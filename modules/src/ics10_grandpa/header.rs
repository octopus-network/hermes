use std::convert::{TryInto, TryFrom};

use ibc_proto::ibc::lightclients::grandpa::v1::Header as RawHeader;

use bytes::Buf;
use prost::Message;
use serde::{Serialize, Deserialize};
use crate::ics02_client::client_type::ClientType;
use crate::ics02_client::header::AnyHeader;
use crate::ics10_grandpa::error::Error;
use crate::ics24_host::identifier::ChainId;
use crate::Height;
use tendermint_proto::Protobuf;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Header;

impl std::fmt::Debug for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, " Header {{...}}")
    }
}

impl crate::ics02_client::header::Header for Header {
    fn client_type(&self) -> ClientType {
        ClientType::Grandpa
    }

    fn height(&self) -> Height {
        unimplemented!()
    }

    fn wrap_any(self) -> AnyHeader {
        AnyHeader::Grandpa(self)
    }
}

impl Protobuf<RawHeader> for Header {}

impl TryFrom<RawHeader> for Header {
    type Error = Error;

    fn try_from(_raw: RawHeader) -> Result<Self, Self::Error> {
        Ok(Header)
    }
}


pub fn decode_header<B: Buf>(buf: B) -> Result<Header, Error> {
    RawHeader::decode(buf).map_err(Error::decode)?.try_into()
}

impl From<Header> for RawHeader {
    fn from(_value: Header) -> Self {
        Self
    }
}
