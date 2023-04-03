use super::super::super::beefy_authority_set::BeefyAuthoritySet;
use crate::clients::ics10_grandpa::error::Error;
use crate::prelude::*;
use ibc_proto::ibc::lightclients::grandpa::v1::{
    MmrBatchProof as RawMmrBatchProof, MmrLeaf as RawMmrLeaf,
    MmrLeavesAndBatchProof as RawMmrLeavesAndBatchProof,
    ParentNumberAndHash as RawParentNumberAndHash,
};
use ibc_proto::protobuf::Protobuf;
use serde::{Deserialize, Serialize};

/// mmr leaves and proofs
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MmrLeavesAndBatchProof {
    /// mmr leaves
    pub leaves: Vec<MmrLeaf>,
    /// mmr batch proof
    pub mmr_batch_proof: Option<MmrBatchProof>,
}

impl Protobuf<RawMmrLeavesAndBatchProof> for MmrLeavesAndBatchProof {}

impl TryFrom<RawMmrLeavesAndBatchProof> for MmrLeavesAndBatchProof {
    type Error = Error;

    fn try_from(raw: RawMmrLeavesAndBatchProof) -> Result<Self, Self::Error> {
        Ok(Self {
            leaves: raw
                .leaves
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, Self::Error>>()?,
            mmr_batch_proof: raw
                .mmr_batch_proof
                .map(TryInto::try_into)
                .map_or(Ok(None), |r| r.map(Some))?,
        })
    }
}

impl From<MmrLeavesAndBatchProof> for RawMmrLeavesAndBatchProof {
    fn from(value: MmrLeavesAndBatchProof) -> Self {
        Self {
            leaves: value.leaves.into_iter().map(Into::into).collect(),
            mmr_batch_proof: value.mmr_batch_proof.map(Into::into),
        }
    }
}

/// mmr batch proof
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MmrBatchProof {
    /// The index of the leaf the proof is for.
    pub leaf_indexes: Vec<u64>,
    /// Number of leaves in MMR, when the proof was generated.
    pub leaf_count: u64,
    /// Proof elements (hashes of siblings of inner nodes on the path to the leaf).
    pub items: Vec<Vec<u8>>,
}
impl Protobuf<RawMmrBatchProof> for MmrBatchProof {}

impl TryFrom<RawMmrBatchProof> for MmrBatchProof {
    type Error = Error;

    fn try_from(raw: RawMmrBatchProof) -> Result<Self, Self::Error> {
        Ok(Self {
            leaf_indexes: raw.leaf_indexes,
            leaf_count: raw.leaf_count,
            items: raw.items,
        })
    }
}

impl From<MmrBatchProof> for RawMmrBatchProof {
    fn from(value: MmrBatchProof) -> Self {
        Self {
            leaf_indexes: value.leaf_indexes,
            leaf_count: value.leaf_count,
            items: value.items,
        }
    }
}
/// MmrLeaf leaf data
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MmrLeaf {
    /// leaf version
    pub version: u32,
    /// parent number and hash
    pub parent_number_and_hash: Option<ParentNumberAndHash>,
    /// beefy next authority set.
    pub beefy_next_authority_set: Option<BeefyAuthoritySet>,
    /// merkle root hash of parachain heads included in the leaf.
    pub parachain_heads: Vec<u8>,
}

impl Protobuf<RawMmrLeaf> for MmrLeaf {}

impl TryFrom<RawMmrLeaf> for MmrLeaf {
    type Error = Error;

    fn try_from(raw: RawMmrLeaf) -> Result<Self, Self::Error> {
        Ok(Self {
            version: raw.version,
            parent_number_and_hash: raw
                .parent_number_and_hash
                .map(TryInto::try_into)
                .map_or(Ok(None), |r| r.map(Some))?,
            beefy_next_authority_set: raw
                .beefy_next_authority_set
                .map(TryInto::try_into)
                .map_or(Ok(None), |r| r.map(Some))?,
            parachain_heads: raw.parachain_heads,
        })
    }
}

impl From<MmrLeaf> for RawMmrLeaf {
    fn from(value: MmrLeaf) -> Self {
        Self {
            version: value.version,
            parent_number_and_hash: value.parent_number_and_hash.map(Into::into),
            beefy_next_authority_set: value.beefy_next_authority_set.map(Into::into),
            parachain_heads: value.parachain_heads,
        }
    }
}

/// parent number and hash
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentNumberAndHash {
    /// parent block for this leaf
    pub parent_number: u32,
    /// parent hash for this leaf
    pub parent_hash: Vec<u8>,
}

impl Protobuf<RawParentNumberAndHash> for ParentNumberAndHash {}

impl TryFrom<RawParentNumberAndHash> for ParentNumberAndHash {
    type Error = Error;

    fn try_from(raw: RawParentNumberAndHash) -> Result<Self, Self::Error> {
        Ok(Self {
            parent_number: raw.parent_number,
            parent_hash: raw.parent_hash,
        })
    }
}

impl From<ParentNumberAndHash> for RawParentNumberAndHash {
    fn from(value: ParentNumberAndHash) -> Self {
        Self {
            parent_number: value.parent_number,
            parent_hash: value.parent_hash,
        }
    }
}
