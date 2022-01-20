use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};
use core::fmt;

use ibc_proto::ibc::lightclients::grandpa::v1::Header as RawHeader;

use super::help::MmrLeaf;
use super::help::MmrLeafProof;
use super::help::SignedCommitment;
use super::help::ValidatorMerkleProof;
use crate::ics02_client::client_type::ClientType;
use crate::ics02_client::header::AnyHeader;
use crate::ics10_grandpa::error::Error;
use crate::ics24_host::identifier::ChainId;
use crate::Height;
use bytes::Buf;
use prost::Message;
use serde::{Deserialize, Serialize};
use tendermint_proto::Protobuf;
use codec::{Encode, Decode};
use beefy_merkle_tree::Hash;
use super::help::BlockHeader;

/// block header
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Encode, Decode)]
pub struct Header {
    pub block_header: BlockHeader,
    pub mmr_leaf: MmrLeaf,
    pub mmr_leaf_proof: MmrLeafProof,
}


impl Default for Header {
    fn default() -> Self {
        Self {
            block_header: BlockHeader::default(),
            mmr_leaf: MmrLeaf::default(),
            mmr_leaf_proof: MmrLeafProof::default(),
        }
    }
}

impl Header {
    pub fn new(
        block_header: BlockHeader,
        mmr_leaf: MmrLeaf,
        mmr_leaf_proof: MmrLeafProof,
    ) -> Self {
        Self {
            block_header,
            mmr_leaf,
            mmr_leaf_proof,
        }
    }

    pub fn hash(&self) -> Hash {
        blake2_256(&codec::Encode::encode(&self.block_header))
    }

    pub fn height(&self) -> Height {
        Height::new(0, self.block_header.block_number as u64)
    }
}

/// Do a Blake2 256-bit hash and place result in `dest`.
fn blake2_256_into(data: &[u8], dest: &mut [u8; 32]) {
    dest.copy_from_slice(blake2_rfc::blake2b::blake2b(32, &[], data).as_bytes());
}
/// Do a Blake2 256-bit hash and return result.
fn blake2_256(data: &[u8]) -> [u8; 32] {
    let mut r = [0; 32];
    blake2_256_into(data, &mut r);
    r
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
        Ok(Self {
            block_header: raw.block_header.unwrap().into(),
            mmr_leaf: raw.mmr_leaf.unwrap().into(),
            mmr_leaf_proof: raw.mmr_leaf_proof.unwrap().into(),
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
