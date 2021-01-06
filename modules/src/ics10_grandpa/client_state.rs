use crate::ics02_client::client_def::AnyClientState;
use crate::ics02_client::client_type::ClientType;
use crate::ics10_grandpa::error::Error;
use crate::ics10_grandpa::header::Header;
use crate::Height;
use ibc_proto::ibc::lightclients::grandpa::v1::Authority;
use ibc_proto::ibc::lightclients::grandpa::v1::ClientState as RawClientState;
use sp_finality_grandpa::{AuthorityList, SetId};
use std::convert::TryFrom;
use std::convert::TryInto;
use tendermint_proto::Protobuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientState {
    pub chain_id: String,
    pub frozen_height: u64,
    pub latest_height: u64,
    pub set_id: SetId,
    pub authorities: AuthorityList,
}

impl Protobuf<RawClientState> for ClientState {}

impl ClientState {
    pub fn new(
        chain_id: String,
        latest_height: u64,
        frozen_height: u64,
        set_id: SetId,
        authorities: AuthorityList,
    ) -> Result<ClientState, Error> {
        Ok(Self {
            chain_id,
            frozen_height,
            latest_height,
            set_id,
            authorities,
        })
    }

    pub fn latest_height(&self) -> Height {
        Height::new(0, self.latest_height)
    }

    pub fn with_header(self, h: Header) -> Self {
        // TODO: Clarify which fields should update.
        ClientState {
            latest_height: h.height,
            ..self
        }
    }
}

impl crate::ics02_client::state::ClientState for ClientState {
    fn chain_id(&self) -> String {
        self.chain_id.clone()
    }

    fn client_type(&self) -> ClientType {
        ClientType::GRANDPA
    }

    fn latest_height(&self) -> Height {
        Height::new(0, self.latest_height)
    }

    fn is_frozen(&self) -> bool {
        // If 'frozen_height' is set to a non-zero value, then the client state is frozen.
        !self.frozen_height == 0
    }

    fn wrap_any(self) -> AnyClientState {
        AnyClientState::GRANDPA(self)
    }
}

impl TryFrom<RawClientState> for ClientState {
    type Error = Error;

    fn try_from(raw: RawClientState) -> Result<Self, Self::Error> {
        Ok(Self {
            chain_id: raw.chain_id,
            latest_height: raw.latest_height,
            frozen_height: raw.frozen_height,
            set_id: raw.set_id,
            authorities: raw
                .authority_list
                .into_iter()
                .map(|authority| {
                    let id: [u8; 32] = authority.authority_id.try_into().unwrap();
                    (
                        sp_application_crypto::ed25519::Public::from_raw(id).into(),
                        authority.authority_weight,
                    )
                })
                .collect(),
        })
    }
}

impl From<ClientState> for RawClientState {
    fn from(value: ClientState) -> Self {
        RawClientState {
            chain_id: value.chain_id.clone(),
            frozen_height: value.frozen_height,
            latest_height: value.latest_height,
            set_id: value.set_id,
            authority_list: value
                .authorities
                .iter()
                .map(|authority| {
                    let id = sp_application_crypto::ed25519::Public::from(authority.0.clone());
                    Authority {
                        authority_id: id.to_vec(),
                        authority_weight: authority.1,
                    }
                })
                .collect(),
        }
    }
}
