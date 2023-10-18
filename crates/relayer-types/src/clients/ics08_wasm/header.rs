use super::proto::wasm::ClientMessage as RawHeader;
use crate::clients::ics12_near::header::Header as NearHeader;
use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::error::Error;
use crate::timestamp::Timestamp;
use crate::Height;
use bytes::Buf;
use ibc_proto::google::protobuf::Any;
use ibc_proto::protobuf::Protobuf;
use prost::Message;
use serde::{Deserialize, Serialize};

pub const WASM_HEADER_TYPE_URL: &str = "/ibc.lightclients.wasm.v1.ClientMessage";

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    pub data: Vec<u8>,
}

impl Header {
    fn near_header(&self) -> NearHeader {
        let any = Any::decode(self.data.as_slice()).unwrap();
        NearHeader::try_from(any).unwrap()
    }
}

impl From<NearHeader> for Header {
    fn from(value: NearHeader) -> Self {
        let any = Any::from(value);
        Self {
            data: any.encode_to_vec(),
        }
    }
}

impl crate::core::ics02_client::header::Header for Header {
    fn client_type(&self) -> ClientType {
        ClientType::Wasm
    }

    fn height(&self) -> Height {
        let header = self.near_header();
        Height::new(0, header.light_client_block.inner_lite.height)
            .expect("failed to create ibc height") // TODO: julian, see revision number in tm header
    }

    fn timestamp(&self) -> Timestamp {
        let header = self.near_header();
        Timestamp::from_nanoseconds(header.light_client_block.inner_lite.timestamp)
            .expect("failed to create Timestamp")
    }
}

impl Protobuf<RawHeader> for Header {}

impl TryFrom<RawHeader> for Header {
    type Error = Error;

    fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
        let header = Self { data: raw.data };

        Ok(header)
    }
}

impl From<Header> for RawHeader {
    fn from(value: Header) -> Self {
        RawHeader { data: value.data }
    }
}

impl Protobuf<Any> for Header {}

impl TryFrom<Any> for Header {
    type Error = Error;

    fn try_from(raw: Any) -> Result<Self, Error> {
        use core::ops::Deref;

        match raw.type_url.as_str() {
            WASM_HEADER_TYPE_URL => decode_header(raw.value.deref()).map_err(Into::into),
            _ => Err(Error::unknown_header_type(raw.type_url)),
        }
    }
}

impl From<Header> for Any {
    fn from(header: Header) -> Self {
        Any {
            type_url: WASM_HEADER_TYPE_URL.to_string(),
            value: Protobuf::<RawHeader>::encode_vec(&header),
        }
    }
}

pub fn decode_header<B: Buf>(buf: B) -> Result<Header, Error> {
    RawHeader::decode(buf).map_err(Error::decode)?.try_into()
}
