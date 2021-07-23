use std::convert::{TryInto, TryFrom};

// use ibc_proto::ibc::lightclients::tendermint::v1::Header as RawHeader;

use serde::{Serialize, Deserialize};
use crate::ics02_client::client_type::ClientType;
use crate::ics02_client::header::AnyHeader;
use crate::ics10_grandpa::error::{Error, Kind};
use crate::ics24_host::identifier::ChainId;
use crate::Height;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Header;

impl std::fmt::Debug for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, " Header {{...}}")
    }
}

impl crate::ics02_client::header::Header for Header {
    fn client_type(&self) -> ClientType {
        ClientType::Tendermint
    }

    fn height(&self) -> Height {
        self.height()
    }

    fn wrap_any(self) -> AnyHeader {
        unimplemented!()
    }
}

// impl Protobuf<RawHeader> for Header {}

// impl TryFrom<RawHeader> for Header {
//     type Error = Error;
//
//     fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
//         unimplemented!()
//     }
// }
//
// impl From<Header> for RawHeader {
//     fn from(value: Header) -> Self {
//         unimplemented!()
//     }
// }

#[test]
mod test {

}
