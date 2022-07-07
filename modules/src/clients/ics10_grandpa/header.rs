use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};
use core::fmt;

use ibc_proto::ibc::lightclients::grandpa::v1::Header as RawHeader;

use super::help::BlockHeader;
use super::help::MmrLeaf;
use super::help::MmrLeafProof;
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
use tendermint_proto::Protobuf;

/// block header
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Encode, Decode, Default)]
pub struct Header {
    pub block_header: BlockHeader,
    pub mmr_leaf: MmrLeaf,
    pub mmr_leaf_proof: MmrLeafProof,
}

impl Header {
    pub fn new(block_header: BlockHeader, mmr_leaf: MmrLeaf, mmr_leaf_proof: MmrLeafProof) -> Self {
        Self {
            block_header,
            mmr_leaf,
            mmr_leaf_proof,
        }
    }

    pub fn hash(&self) -> Result<Hash, Error> {
        self.block_header.hash()
    }

    pub fn height(&self) -> Height {
        // todo(davirian) this height new function construct revision_number is not zero.
        // this need to know revision_number is WHAT?
        Height::new(1, self.block_header.block_number as u64).unwrap()
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
        Timestamp::none()
    }

    fn wrap_any(self) -> AnyHeader {
        AnyHeader::Grandpa(self)
    }
}

impl Protobuf<RawHeader> for Header {}

impl TryFrom<RawHeader> for Header {
    type Error = Error;

    fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
        Ok(Self {
            block_header: raw
                .block_header
                .ok_or_else(Error::empty_block_header)?
                .into(),
            mmr_leaf: raw.mmr_leaf.ok_or_else(Error::empty_mmr_leaf)?.try_into()?,
            mmr_leaf_proof: raw
                .mmr_leaf_proof
                .ok_or_else(Error::empty_mmr_leaf_proof)?
                .into(),
        })
    }
}

pub fn decode_header<B: Buf>(buf: B) -> Result<Header, Error> {
    RawHeader::decode(buf).map_err(Error::decode)?.try_into()
}

impl From<Header> for RawHeader {
    fn from(value: Header) -> Self {
        RawHeader {
            block_header: Some(value.block_header.into()),
            mmr_leaf: Some(value.mmr_leaf.into()),
            mmr_leaf_proof: Some(value.mmr_leaf_proof.into()),
        }
    }
}
