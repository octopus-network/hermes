use std::convert::{TryFrom, TryInto};

use tendermint_proto::Protobuf;

// use ibc_proto::ibc::lightclients::tendermint::v1::Misbehaviour as RawMisbehaviour;

use crate::ics02_client::misbehaviour::AnyMisbehaviour;
use crate::ics10_grandpa::error::Error;
use crate::ics10_grandpa::header::Header;
use crate::ics24_host::identifier::ClientId;
use crate::Height;

#[derive(Clone, Debug, PartialEq)]
pub struct Misbehaviour {
    pub client_id: ClientId,
    pub header1: Header,
    pub header2: Header,
}

impl crate::ics02_client::misbehaviour::Misbehaviour for Misbehaviour {
    fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    fn height(&self) -> Height {
        unimplemented!()
    }

    fn wrap_any(self) -> AnyMisbehaviour {
        AnyMisbehaviour::Grandpa(self)
    }
}

// impl Protobuf<RawMisbehaviour> for Misbehaviour {}

// impl TryFrom<RawMisbehaviour> for Misbehaviour {
//     type Error = Error;
//
//     fn try_from(raw: RawMisbehaviour) -> Result<Self, Self::Error> {
//        unimplemented!()
//     }
// }
//
// impl From<Misbehaviour> for RawMisbehaviour {
//     fn from(value: Misbehaviour) -> Self {
//         unimplemented!()
//     }
// }

impl std::fmt::Display for Misbehaviour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        unimplemented!()
    }
}
