use crate::clients::ics10_grandpa::error::Error;
use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::error::Error as Ics02Error;
use crate::prelude::*;
use crate::timestamp::Timestamp;
use crate::Height;
use bytes::Buf;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::grandpa::v1::header::Message as RawMessage;
use ibc_proto::ibc::lightclients::grandpa::v1::Header as RawHeader;
use ibc_proto::protobuf::Protobuf;
use prost::Message;
use serde::{Deserialize, Serialize};

pub mod beefy_mmr;
pub mod message;

pub const GRANDPA_HEADER_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.Header";

/// header wrapper
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    /// the latest mmr data
    pub beefy_mmr: beefy_mmr::BeefyMmr,
    /// only one header
    pub message: message::Message,
}

impl Header {
    pub fn height(&self) -> Height {
        // match self.message {}
        todo!()
    }
}

impl Protobuf<RawHeader> for Header {}

impl TryFrom<RawHeader> for Header {
    type Error = Error;

    fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
        Ok(Self {
            beefy_mmr: raw.beefy_mmr.ok_or_else(Error::empty_beefy_mmr)?.into(),
            message: match raw.message.ok_or_else(Error::empty_beefy_mmr)? {
                RawMessage::ParachainHeaderMap(v) => message::Message::ParachainHeaderMap(v.into()),
                RawMessage::SubchainHeaderMap(v) => message::Message::SubchainHeaderMap(v.into()),
            },
        })
    }
}

impl From<Header> for RawHeader {
    fn from(value: Header) -> Self {
        Self {
            beefy_mmr: Some(value.beefy_mmr.into()),
            message: Some(match value.message {
                message::Message::ParachainHeaderMap(v) => RawMessage::ParachainHeaderMap(v.into()),
                message::Message::SubchainHeaderMap(v) => RawMessage::SubchainHeaderMap(v.into()),
            }),
        }
    }
}

impl crate::core::ics02_client::header::Header for Header {
    fn client_type(&self) -> ClientType {
        ClientType::Grandpa
    }

    fn height(&self) -> Height {
        self.height()
    }

    fn timestamp(&self) -> Timestamp {
        todo!()
    }
}

impl Protobuf<Any> for Header {}

impl TryFrom<Any> for Header {
    type Error = Ics02Error;

    fn try_from(raw: Any) -> Result<Self, Ics02Error> {
        use core::ops::Deref;

        fn decode_header<B: Buf>(buf: B) -> Result<Header, Error> {
            RawHeader::decode(buf).map_err(Error::decode)?.try_into()
        }

        match raw.type_url.as_str() {
            GRANDPA_HEADER_TYPE_URL => decode_header(raw.value.deref()).map_err(Into::into),
            _ => Err(Ics02Error::unknown_header_type(raw.type_url)),
        }
    }
}

impl From<Header> for Any {
    fn from(header: Header) -> Self {
        Any {
            type_url: GRANDPA_HEADER_TYPE_URL.to_string(),
            value: Protobuf::<RawHeader>::encode_vec(&header)
                .expect("encoding to `Any` from `GpHeader`"),
        }
    }
}

pub fn decode_header<B: Buf>(buf: B) -> Result<Header, Error> {
    RawHeader::decode(buf).map_err(Error::decode)?.try_into()
}
