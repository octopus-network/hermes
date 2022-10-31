use alloc::format;
use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};
use core::fmt;

use ibc_proto::ibc::lightclients::grandpa::v1::Header as RawHeader;

use super::help::BlockHeader;
use super::help::MmrLeaf;
use super::help::MmrLeafProof;
use super::help::MmrRoot;
use super::help::SignedCommitment;
use super::help::ValidatorMerkleProof;
use crate::clients::ics10_grandpa::error::Error;
use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::header::AnyHeader;
use crate::core::ics24_host::identifier::ChainId;
use crate::timestamp::Timestamp;
use crate::Height;
use beefy_merkle_tree::Hash;
use bytes::Buf;
use codec::{Decode, Encode};
use prost::Message;
use serde::{Deserialize, Serialize};
use tendermint::time::Time;
use tendermint_proto::google::protobuf as tpb;
use tendermint_proto::Protobuf;

/// block header
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Header {
    pub block_header: BlockHeader,
    pub mmr_root: MmrRoot,
    //// timestamp
    pub timestamp: Time,
}
impl Default for Header {
    fn default() -> Self {
        Self {
            block_header: BlockHeader::default(),
            mmr_root: MmrRoot::default(),
            timestamp: Time::from_unix_timestamp(0, 0).unwrap(),
        }
    }
}
impl Header {
    pub fn new(block_header: BlockHeader, mmr_root: MmrRoot, timestamp: Time) -> Self {
        Self {
            block_header,
            mmr_root,
            timestamp,
        }
    }

    pub fn hash(&self) -> Result<Hash, Error> {
        self.block_header.hash()
    }

    pub fn height(&self) -> Height {
        Height::new(8888, self.block_header.block_number as u64).unwrap()
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
        // Timestamp::none()
        self.timestamp.into()
    }

    fn wrap_any(self) -> AnyHeader {
        AnyHeader::Grandpa(self)
    }
}

impl Protobuf<RawHeader> for Header {}

impl TryFrom<RawHeader> for Header {
    type Error = Error;

    fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
        let ibc_proto::google::protobuf::Timestamp { seconds, nanos } = raw
            .timestamp
            .ok_or_else(|| Error::invalid_raw_header("missing timestamp".into()))?;

        let proto_timestamp = tpb::Timestamp { seconds, nanos };
        let timestamp = proto_timestamp
            .try_into()
            .map_err(|e| Error::invalid_raw_header(format!("invalid timestamp: {}", e)))?;

        Ok(Self {
            block_header: raw
                .block_header
                .ok_or_else(Error::empty_block_header)?
                .into(),
            mmr_root: raw.mmr_root.ok_or_else(Error::empty_mmr_root)?.try_into()?,
            timestamp: timestamp,
        })
    }
}

pub fn decode_header<B: Buf>(buf: B) -> Result<Header, Error> {
    RawHeader::decode(buf).map_err(Error::decode)?.try_into()
}

impl From<Header> for RawHeader {
    fn from(value: Header) -> Self {
        let tpb::Timestamp { seconds, nanos } = value.timestamp.into();
        let timestamp = ibc_proto::google::protobuf::Timestamp { seconds, nanos };
        let mmr_root = value.mmr_root.try_into().unwrap();
        RawHeader {
            block_header: Some(value.block_header.into()),
            mmr_root: Some(mmr_root),
            timestamp: Some(timestamp),
        }
    }
}
