use std::convert::{TryFrom, TryInto};

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
    pub latest_height: Height,
}

impl ClientState {
    pub fn new(latest_height: Height) -> Result<Self, Error> {
        Ok(ClientState {
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
        unimplemented!()
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
            latest_height: raw.latest_height
                .ok_or_else(Error::missing_latest_height)?
                .into(),
        })
    }
}

impl From<ClientState> for RawClientState {
    fn from(value: ClientState) -> Self {
        Self {
            latest_height: Some(value.latest_height.into()),
        }
    }
}
