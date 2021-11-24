use core::convert::{TryFrom, TryInto};
use core::fmt;

use ibc_proto::ibc::lightclients::grandpa::v1::Header as RawHeader;

use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::header::AnyHeader;
use crate::clients::ics10_grandpa::error::Error;
use crate::core::ics24_host::identifier::ChainId;
use crate::Height;
use crate::timestamp::Timestamp;
use bytes::Buf;
use prost::Message;
use serde::{Deserialize, Serialize};
use tendermint_proto::Protobuf;

#[derive(Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct Header {
    pub height: u64,
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, " Header {{...}}")
    }
}

impl Header {
    pub fn new(height: u64) -> Self {
        Self {
            height,
        }
    }
    pub fn height(&self) -> Height {
        Height::new(0, self.height)
    }
}

impl crate::core::ics02_client::header::Header for Header {
    fn client_type(&self) -> ClientType {
        ClientType::Grandpa
    }

    fn height(&self) -> Height {
        self.height()
    }

    fn wrap_any(self) -> AnyHeader {
        AnyHeader::Grandpa(self)
    }

    fn timestamp(&self) -> Timestamp {
        // TODO
        Timestamp::now()
    }
}

impl Protobuf<RawHeader> for Header {}

impl TryFrom<RawHeader> for Header {
    type Error = Error;

    fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
        Ok(Header {
            height: raw
                .height
                .ok_or_else(Error::missing_height)?
                .revision_height,
        })
    }
}

pub fn decode_header<B: Buf>(buf: B) -> Result<Header, Error> {
    RawHeader::decode(buf).map_err(Error::decode)?.try_into()
}

impl From<Header> for RawHeader {
    fn from(value: Header) -> Self {
        Self {
            height: Some(Height::new(0, value.height).into()),
        }
    }
}
