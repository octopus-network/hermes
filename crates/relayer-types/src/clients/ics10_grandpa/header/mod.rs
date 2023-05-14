use crate::clients::ics10_grandpa::error::Error;
use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::error::Error as Ics02Error;
use crate::prelude::*;
use crate::timestamp::Timestamp;
use crate::Height;
use alloc::collections::BTreeMap;
use bytes::Buf;
use codec::Decode;
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
    pub beefy_mmr: Option<beefy_mmr::BeefyMmr>,
    /// only one header
    pub message: message::Message,
}

impl Header {
    // https://github.com/octopus-network/ibc-go/blob/cc25e9b73c3daa2269081f65b23971e7030864d5/modules/light-clients/10-grandpa/types/header.go#L41
    pub fn height(&self) -> Height {
        let (height, _) = get_lastest_block_header(self.clone()).unwrap();
        Height::new(0, height).unwrap()
    }
}

impl Protobuf<RawHeader> for Header {}

impl TryFrom<RawHeader> for Header {
    type Error = Error;

    fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
        let message = raw
            .message
            .and_then(|msg| match msg {
                RawMessage::ParachainHeaders(v) => Some(
                    v.try_into()
                        .map(message::Message::ParachainHeaders)
                        .map_err(|_| Error::missing_header_message()),
                ),
                RawMessage::SubchainHeaders(v) => Some(
                    v.try_into()
                        .map(message::Message::SubchainHeaders)
                        .map_err(|_| Error::missing_header_message()),
                ),
            })
            .ok_or_else(Error::missing_header_message)??;
        let beefy_mmr = raw.beefy_mmr.map(TryInto::try_into).transpose()?;
        Ok(Self { beefy_mmr, message })
    }
}

impl From<Header> for RawHeader {
    fn from(value: Header) -> Self {
        Self {
            beefy_mmr: value.beefy_mmr.map(Into::into),
            message: match value.message {
                message::Message::ParachainHeaders(v) => {
                    Some(RawMessage::ParachainHeaders(v.into()))
                }
                message::Message::SubchainHeaders(v) => Some(RawMessage::SubchainHeaders(v.into())),
            },
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

    // https://github.com/octopus-network/ibc-go/blob/cc25e9b73c3daa2269081f65b23971e7030864d5/modules/light-clients/10-grandpa/types/header.go#L62
    fn timestamp(&self) -> Timestamp {
        let (_, time) = get_lastest_block_header(self.clone()).unwrap();
        Timestamp::from_nanoseconds(time).unwrap()
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

pub fn get_lastest_block_header(h: Header) -> Result<(u64, u64), Error> {
    // let mut latest_height = 0;

    match h.message {
        message::Message::SubchainHeaders(mut v) => {
            v.subchain_headers
                .sort_by(|h1, h2| h2.block_number.cmp(&h1.block_number));
            let latest_header = v.subchain_headers.first().unwrap();
            let latest_height = latest_header.block_number;
            let timestamp_value = &latest_header.timestamp.value;
            let timestamp: u64 = codec::Decode::decode(&mut &timestamp_value[..]).unwrap();
            Ok((latest_height as u64, timestamp))
        }
        message::Message::ParachainHeaders(mut v) => {
            v.parachain_headers
                .sort_by(|h1, h2| h2.relayer_chain_number.cmp(&h1.relayer_chain_number));

            let latest_header = v.parachain_headers.first().unwrap();
            let decoded_header =
                beefy_light_client::header::Header::decode(&mut &latest_header.block_header[..])
                    .unwrap();

            let latest_height = decoded_header.number;
            let timestamp_value = &latest_header.timestamp.value;
            let timestamp: u64 = codec::Decode::decode(&mut &timestamp_value[..]).unwrap();
            Ok((latest_height as u64, timestamp))
        }
    }
}