use super::super::beefy_authority_set::BeefyAuthoritySet;
use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub mod mmr_leaves_and_batch_proof;
pub mod signed_commitment;

use mmr_leaves_and_batch_proof::MmrLeavesAndBatchProof;
use signed_commitment::SignedCommitment;

/// mmr data
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeefyMmr {
    /// signed commitment data
    pub signed_commitment: Option<SignedCommitment>,
    /// build merkle tree based on all the signature in signed commitment
    /// and generate the signature proof
    pub signature_proofs: Vec<Vec<u8>>,
    /// mmr proof
    pub mmr_leaves_and_batch_proof: Option<MmrLeavesAndBatchProof>,
    /// size of the mmr for the given proof
    pub mmr_size: u64,
}
