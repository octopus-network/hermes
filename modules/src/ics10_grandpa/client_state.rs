use std::convert::{TryFrom, TryInto};

// mock grandpa as tendermint
// use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as RawClientState;

use serde::{Deserialize, Serialize};
use crate::ics10_grandpa::error::{Error, Kind};
use crate::ics02_client::client_state::AnyClientState;
use crate::ics02_client::client_type::ClientType;
use crate::ics24_host::identifier::ChainId;
use crate::Height;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientState{
    pub latest_height: Height,
    pub frozen_height: Height,
}

impl ClientState {
    fn new(latest_height: Height, frozen_height: Height) -> Result<Self, Error> {
        Ok(Self{
            latest_height,
            frozen_height,
        })
    }
}

// impl Protobuf<RawClientState> for ClientState {}

impl crate::ics02_client::client_state::ClientState for ClientState {
    fn chain_id(&self) -> ChainId {
        unimplemented!()
    }

    fn client_type(&self) -> ClientType {
        unimplemented!()
    }

    fn latest_height(&self) -> Height {
        self.latest_height
    }

    fn is_frozen(&self) -> bool {
        // If 'frozen_height' is set to a non-zero value, then the client state is frozen.
        !self.frozen_height.is_zero()
    }

    fn wrap_any(self) -> AnyClientState {
        unimplemented!()
    }
}

// impl TryFrom<RawClientState> for ClientState {
//     type Error = Error;
//
//     fn try_from(raw: RawClientState) -> Result<Self, Self::Error> {
//        unimplemented!()
//     }
// }
//
// impl From<ClientState> for RawClientState {
//     fn from(value: ClientState) -> Self {
//         unimplemented!()
//     }
// }

#[test]
mod tests {

}