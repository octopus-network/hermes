// Nested message and enum types in `Header`.
use crate::clients::ics10_grandpa::error::Error;
use crate::core::ics24_host::identifier::ChainId;
use crate::prelude::*;
use alloc::collections::BTreeMap;
use codec::{Encode, Decode};
use ibc_proto::ibc::lightclients::grandpa::v1::header::Message as RawMessage;
use ibc_proto::ibc::lightclients::grandpa::v1::{
    ParachainHeader as RawParachainHeader, ParachainHeaders as RawParachainHeaders,
    StateProof as RawStateProof, SubchainHeader as RawSubchainHeader,
    SubchainHeaders as RawSubchainHeaders,
};
use ibc_proto::protobuf::Protobuf;

use super::beefy_mmr::mmr_leaves_and_batch_proof;
use mmr_leaves_and_batch_proof::MmrLeavesAndBatchProof;
use serde::{Deserialize, Serialize};
/// only one header
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    /// solochain headers and their proofs
    SubchainHeaders(SubchainHeaders),
    /// parachain headers and their proofs
    ParachainHeaders(ParachainHeaders),
}

/// substrate chain header map
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubchainHeaders {
    pub subchain_headers: Vec<SubchainHeader>,
    pub mmr_leaves_and_batch_proof: Option<MmrLeavesAndBatchProof>,
}

impl SubchainHeaders {
    pub fn new() -> Self {
        Self {
            subchain_headers: Vec::new(),
            mmr_leaves_and_batch_proof: None,
        }
    }
}

impl Protobuf<RawSubchainHeaders> for SubchainHeaders {}

impl TryFrom<RawSubchainHeaders> for SubchainHeaders {
    type Error = Error;
    fn try_from(raw: RawSubchainHeaders) -> Result<Self, Self::Error> {
        Ok(Self {
            subchain_headers: raw
                .subchain_headers
                .into_iter()
                .map(|h| SubchainHeader::try_from(h).unwrap())
                .collect::<Vec<_>>(),
            mmr_leaves_and_batch_proof: raw
                .mmr_leaves_and_batch_proof
                .map(TryInto::try_into)
                .transpose()?,
        })
    }
}

impl From<SubchainHeaders> for RawSubchainHeaders {
    fn from(value: SubchainHeaders) -> Self {
        Self {
            subchain_headers: value
                .subchain_headers
                .into_iter()
                .map(|h| RawSubchainHeader::from(h))
                .collect(),
            mmr_leaves_and_batch_proof: value.mmr_leaves_and_batch_proof.map(Into::into),
        }
    }
}
/// solochain header
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubchainHeader {
    pub chain_id: ChainId,
    pub block_number: u32,
    /// scale-encoded solochain header bytes
    pub block_header: Vec<u8>,
    /// timestamp and proof
    pub timestamp: StateProof,
}

impl Protobuf<RawSubchainHeader> for SubchainHeader {}

impl TryFrom<RawSubchainHeader> for SubchainHeader {
    type Error = Error;
    fn try_from(raw: RawSubchainHeader) -> Result<Self, Self::Error> {
        Ok(Self {
            chain_id: ChainId::from_string(raw.chain_id.as_str()),
            block_number: raw.block_number,
            block_header: raw.block_header,
            timestamp: raw
                .timestamp
                .map(Into::into)
                .ok_or_else(|| Error::missing_timestamp())?,
        })
    }
}

impl From<SubchainHeader> for RawSubchainHeader {
    fn from(value: SubchainHeader) -> Self {
        Self {
            chain_id: value.chain_id.to_string(),
            block_number: value.block_number,
            block_header: value.block_header,
            timestamp: Some(value.timestamp.into()),
        }
    }
}

/// / Parachain headers and their merkle proofs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParachainHeaders {
    /// map<blocknumber,ParachainHeader>
    ///
    ///   map<uint32,Timestamp> timestamp_map=2;
    pub parachain_headers: Vec<ParachainHeader>,
    pub mmr_leaves_and_batch_proof: Option<MmrLeavesAndBatchProof>,
}

impl Protobuf<RawParachainHeaders> for ParachainHeaders {}

impl TryFrom<RawParachainHeaders> for ParachainHeaders {
    type Error = Error;
    fn try_from(raw: RawParachainHeaders) -> Result<Self, Self::Error> {
        Ok(Self {
            parachain_headers: raw
                .parachain_headers
                .into_iter()
                .map(|h| ParachainHeader::try_from(h).unwrap())
                .collect::<Vec<_>>(),
            mmr_leaves_and_batch_proof: raw
                .mmr_leaves_and_batch_proof
                .map(TryInto::try_into)
                .transpose()?,
        })
    }
}

impl From<ParachainHeaders> for RawParachainHeaders {
    fn from(value: ParachainHeaders) -> Self {
        Self {
            parachain_headers: value
                .parachain_headers
                .into_iter()
                .map(|h| RawParachainHeader::from(h))
                .collect(),
            mmr_leaves_and_batch_proof: value.mmr_leaves_and_batch_proof.map(Into::into),
        }
    }
}
/// data needed to prove parachain header inclusion in mmr
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParachainHeader {
    pub chain_id: ChainId,
    /// para id
    pub parachain_id: u32,
    /// This block number is relayer chain blocknumber that parachain header packed into relayer block
    pub relayer_chain_number: u32,
    /// scale-encoded parachain header bytes
    pub block_header: Vec<u8>,
    /// proofs for parachain header in the mmr_leaf.parachain_heads
    pub proofs: Vec<Vec<u8>>,
    /// merkle leaf index for parachain heads proof
    pub header_index: u32,
    /// total number of para heads in parachain_heads_root
    pub header_count: u32,
    /// timestamp and proof
    pub timestamp: StateProof,
}

impl Protobuf<RawParachainHeader> for ParachainHeader {}

impl TryFrom<RawParachainHeader> for ParachainHeader {
    type Error = Error;

    fn try_from(raw: RawParachainHeader) -> Result<Self, Self::Error> {
        Ok(Self {
            chain_id: ChainId::from_string(raw.chain_id.as_str()),
            parachain_id: raw.parachain_id,
            relayer_chain_number: raw.relayer_chain_number,
            block_header: raw.block_header,
            proofs: raw.proofs,
            header_index: raw.header_index,
            header_count: raw.header_count,
            timestamp: raw
                .timestamp
                .map(Into::into)
                .ok_or_else(|| Error::missing_timestamp())?,
        })
    }
}

impl From<ParachainHeader> for RawParachainHeader {
    fn from(value: ParachainHeader) -> Self {
        Self {
            chain_id: value.chain_id.to_string(),
            parachain_id: value.parachain_id,
            relayer_chain_number: value.relayer_chain_number,
            block_header: value.block_header,
            proofs: value.proofs,
            header_index: value.header_index,
            header_count: value.header_count,
            timestamp: Some(value.timestamp.into()),
        }
    }
}

/// state value and proof
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct StateProof {
    /// state key
    pub key: Vec<u8>,
    /// the state value
    pub value: Vec<u8>,
    /// these proof gets from parachain by rpc methord:state_getReadProof
    pub proofs: Vec<Vec<u8>>,
}

impl Protobuf<RawStateProof> for StateProof {}

impl From<RawStateProof> for StateProof {
    fn from(raw: RawStateProof) -> Self {
        Self {
            key: raw.key,
            value: raw.value,
            proofs: raw.proofs,
        }
    }
}

impl From<StateProof> for RawStateProof {
    fn from(value: StateProof) -> Self {
        Self {
            key: value.key,
            value: value.value,
            proofs: value.proofs,
        }
    }
}
