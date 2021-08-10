use std::convert::{TryFrom, TryInto};

use ibc_proto::ibc::lightclients::grandpa::v1::Header as RawHeader;

use crate::ics02_client::client_type::ClientType;
use crate::ics02_client::header::AnyHeader;
use crate::ics10_grandpa::error::Error;
use crate::ics24_host::identifier::ChainId;
use crate::Height;
use bytes::Buf;
use prost::Message;
use serde::{Deserialize, Serialize};
use tendermint_proto::Protobuf;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Header {
    pub height: u64,
}

impl std::fmt::Debug for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, " Header {{...}}")
    }
}

impl Header {
    pub fn height(&self) -> Height {
        Height::new(0, self.height)
    }
}

impl crate::ics02_client::header::Header for Header {
    fn client_type(&self) -> ClientType {
        ClientType::Grandpa
    }

    fn height(&self) -> Height {
        self.height()
    }

    fn wrap_any(self) -> AnyHeader {
        AnyHeader::Grandpa(self)
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
