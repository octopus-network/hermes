use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

// mock grandpa as tendermint
use ibc_proto::ibc::lightclients::grandpa::v1::ClientState as RawClientState;

use crate::ics02_client::client_state::AnyClientState;
use crate::ics02_client::client_type::ClientType;
use crate::ics10_grandpa::error::Error;
use crate::ics24_host::identifier::ChainId;
use crate::Height;
use serde::{Deserialize, Serialize};
use tendermint_proto::Protobuf;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientState {
    pub chain_id: ChainId,
    pub latest_height: Height,
}

impl ClientState {
    pub fn new(chain_id: ChainId, latest_height: Height) -> Result<Self, Error> {
        Ok(ClientState {
            chain_id,
            latest_height,
        })
    }

    pub fn latest_height(&self) -> Height {
        self.latest_height
    }
}

impl Protobuf<RawClientState> for ClientState {}

impl crate::ics02_client::client_state::ClientState for ClientState {
    fn chain_id(&self) -> ChainId {
        self.chain_id.clone()
    }

    fn client_type(&self) -> ClientType {
        ClientType::Grandpa
    }

    fn latest_height(&self) -> Height {
        self.latest_height
    }

    fn is_frozen(&self) -> bool {
        // If 'frozen_height' is set to a non-zero value, then the client state is frozen.
        unimplemented!()
    }

    fn wrap_any(self) -> AnyClientState {
        AnyClientState::Grandpa(self)
    }
}

impl TryFrom<RawClientState> for ClientState {
    type Error = Error;

    fn try_from(raw: RawClientState) -> Result<Self, Self::Error> {
        Ok(ClientState {
            chain_id: ChainId::from_str(raw.chain_id.as_str())
                .map_err(Error::invalid_chain_identifier)?,
            latest_height: raw.latest_height
                .ok_or_else(Error::missing_latest_height)?
                .into(),
        })
    }
}

impl From<ClientState> for RawClientState {
    fn from(value: ClientState) -> Self {
        Self {
            chain_id: value.chain_id.to_string(),
            latest_height: Some(value.latest_height.into()),
        }
    }
}
