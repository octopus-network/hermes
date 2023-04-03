use super::super::super::beefy_authority_set::BeefyAuthoritySet;
use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// mmr leaves and proofs
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MmrLeavesAndBatchProof {
    /// mmr leaves
    pub leaves: Vec<MmrLeaf>,
    /// mmr batch proof
    pub mmr_batch_proof: Option<MmrBatchProof>,
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

/// parent number and hash
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentNumberAndHash {
    /// parent block for this leaf
    pub parent_number: u32,
    /// parent hash for this leaf
    pub parent_hash: Vec<u8>,
}
