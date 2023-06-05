use super::super::beefy_authority_set::BeefyAuthoritySet;
use crate::clients::ics10_grandpa::error::Error;
use crate::prelude::*;
use ibc_proto::ibc::lightclients::grandpa::v1::BeefyMmr as RawBeefyMmr;
use ibc_proto::protobuf::Protobuf;
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
}

impl Protobuf<RawBeefyMmr> for BeefyMmr {}

impl TryFrom<RawBeefyMmr> for BeefyMmr {
    type Error = Error;
    fn try_from(raw: RawBeefyMmr) -> Result<Self, Self::Error> {
        Ok(Self {
            signed_commitment: raw.signed_commitment.map(Into::into),
            signature_proofs: raw.signature_proofs,
            mmr_leaves_and_batch_proof: raw
                .mmr_leaves_and_batch_proof
                .map(TryInto::try_into)
                .transpose()?,
        })
    }
}

impl From<BeefyMmr> for RawBeefyMmr {
    fn from(value: BeefyMmr) -> Self {
        Self {
            signed_commitment: value.signed_commitment.map(Into::into),
            signature_proofs: value.signature_proofs,
            mmr_leaves_and_batch_proof: value.mmr_leaves_and_batch_proof.map(Into::into),
        }
    }
}
