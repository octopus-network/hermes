use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// signed commitment data
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedCommitment {
    /// commitment data being signed
    pub commitment: Option<Commitment>,
    /// all the signatures
    pub signatures: Vec<Signature>,
}

/// Signature with it`s index in merkle tree
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature {
    /// signature leaf index in the merkle tree.
    pub index: u32,
    /// signature bytes
    pub signature: Vec<u8>,
}

/// Commitment message signed by beefy validators
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Commitment {
    /// array of payload items signed by Beefy validators
    pub payloads: Vec<PayloadItem>,
    /// block number for this commitment
    pub block_number: u32,
    /// validator set that signed this commitment
    pub validator_set_id: u64,
}

/// Actual payload items
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PayloadItem {
    /// 2-byte payload id
    pub id: Vec<u8>,
    /// arbitrary length payload data., eg mmr_root_hash
    pub data: Vec<u8>,
}
