use crate::ics10_grandpa::error::Error;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryFrom;
use serde::{Deserialize, Serialize};

use ibc_proto::ibc::lightclients::grandpa::v1::Commitment as RawCommitment;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Commitment {
    /// block height
    pub block_number: u32,
    /// mmr root
    pub payload: Vec<u8>,
    ///validator_set_id
    pub validator_set_id: u64,
}

impl Default for Commitment {
    fn default() -> Self {
        Self {
            block_number: 0,
            payload: vec![],
            validator_set_id: 0,
        }
    }
}

impl From<RawCommitment> for Commitment {
    fn from(raw: RawCommitment) -> Self {
        Self {
            block_number: raw.block_number,
            payload: raw.payload,
            validator_set_id: raw.validator_set_id,
        }
    }
}
impl From<Commitment> for RawCommitment {
    fn from(value: Commitment) -> Self {
        Self {
            block_number: value.block_number,
            payload: value.payload,
            validator_set_id: value.validator_set_id,
        }
    }
}

use ibc_proto::ibc::lightclients::grandpa::v1::ValidatorSet as RawValidatorSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatorSet {
    /// Id of the next set.
    ///
    /// Id is required to correlate BEEFY signed commitments with the validator set.
    /// Light Client can easily verify that the commitment witness it is getting is
    /// produced by the latest validator set.
    pub id: u64,
    /// Number of validators in the set.
    ///
    /// Some BEEFY Light Clients may use an interactive protocol to verify only subset
    /// of signatures. We put set length here, so that these clients can verify the minimal
    /// number of required signatures.
    pub len: u32,
    /// Merkle Root Hash build from BEEFY AuthorityIds.
    ///
    /// This is used by Light Clients to confirm that the commitments are signed by the correct
    /// validator set. Light Clients using interactive protocol, might verify only subset of
    /// signatures, hence don't require the full list here (will receive inclusion proofs).
    pub root: Vec<u8>,
}

impl Default for ValidatorSet {
    fn default() -> Self {
        Self {
            id: 0,
            len: 0,
            root: vec![],
        }
    }
}
impl From<RawValidatorSet> for ValidatorSet {
    fn from(raw: RawValidatorSet) -> Self {
        Self {
            id: raw.id,
            len: raw.len,
            root: raw.root,
        }
    }
}

impl From<ValidatorSet> for RawValidatorSet {
    fn from(value: ValidatorSet) -> Self {
        Self {
            id: value.id,
            len: value.len,
            root: value.root,
        }
    }
}

use ibc_proto::ibc::lightclients::grandpa::v1::MmrLeaf as RawMmrLeaf;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MmrLeaf {
    //// Version of the leaf format.
    //// Can be used to enable future format migrations and compatibility.
    pub version: u32,
    //// Current block parent number and hash.
    pub parent_number_and_hash: Option<ParentNumberAndHash>,
    //// A merkle root of the next BEEFY authority set.
    pub beefy_next_authority_set: Option<ValidatorSet>,
    //// A merkle root of all registered parachain heads.
    pub parachain_heads: Vec<u8>,
}

impl From<RawMmrLeaf> for MmrLeaf {
    fn from(raw: RawMmrLeaf) -> Self {
        Self {
            version: raw.version,
            parent_number_and_hash: Some(raw.parent_number_and_hash.unwrap().into()),
            beefy_next_authority_set: Some(raw.beefy_next_authority_set.unwrap().into()),
            parachain_heads: raw.parachain_heads,
        }
    }
}

impl From<MmrLeaf> for RawMmrLeaf {
    fn from(value: MmrLeaf) -> Self {
        Self {
            version: value.version,
            parent_number_and_hash: Some(value.parent_number_and_hash.unwrap().into()),
            beefy_next_authority_set: Some(value.beefy_next_authority_set.unwrap().into()),
            parachain_heads: value.parachain_heads,
        }
    }
}

impl Default for MmrLeaf {
    fn default() -> Self {
        Self {
            version: 0,
            parent_number_and_hash: Some(ParentNumberAndHash::default()),
            beefy_next_authority_set: Some(ValidatorSet::default()),
            parachain_heads: vec![],
        }
    }
}

use ibc_proto::ibc::lightclients::grandpa::v1::ParentNumberAndHash as RawParentNumberAndHash;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentNumberAndHash {
    pub block_number: u32,
    /// header hash
    pub mmr_root: Vec<u8>,
}

impl Default for ParentNumberAndHash {
    fn default() -> Self {
        Self {
            block_number: 0,
            mmr_root: vec![],
        }
    }
}

impl From<RawParentNumberAndHash> for ParentNumberAndHash {
    fn from(raw: RawParentNumberAndHash) -> Self {
        Self {
            block_number: raw.block_number,
            mmr_root: raw.mmr_root,
        }
    }
}

impl From<ParentNumberAndHash> for RawParentNumberAndHash {
    fn from(value: ParentNumberAndHash) -> Self {
        Self {
            block_number: value.block_number,
            mmr_root: value.mmr_root,
        }
    }
}

use ibc_proto::ibc::lightclients::grandpa::v1::SignedCommitment as RawSignedCommitment;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedCommitment {
    pub commitment: Option<Commitment>,
    pub signatures: Vec<Signature>,
}

impl From<RawSignedCommitment> for SignedCommitment {
    fn from(raw: RawSignedCommitment) -> Self {
        Self {
            commitment: Some(raw.commitment.unwrap().into()),
            signatures: raw
                .signatures
                .into_iter()
                .map(|value| value.into())
                .collect(),
        }
    }
}

impl From<SignedCommitment> for RawSignedCommitment {
    fn from(value: SignedCommitment) -> Self {
        Self {
            commitment: Some(value.commitment.unwrap().into()),
            signatures: value
                .signatures
                .into_iter()
                .map(|value| value.into())
                .collect(),
        }
    }
}

impl Default for SignedCommitment {
    fn default() -> Self {
        Self {
            commitment: Some(Commitment::default()),
            signatures: vec![],
        }
    }
}

use ibc_proto::ibc::lightclients::grandpa::v1::Signature as RawSignature;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature {
    pub signature: Vec<u8>,
}

impl From<RawSignature> for Signature {
    fn from(raw: RawSignature) -> Self {
        Self {
            signature: raw.signature,
        }
    }
}

impl From<Signature> for RawSignature {
    fn from(value: Signature) -> Self {
        Self {
            signature: value.signature,
        }
    }
}

impl Default for Signature {
    fn default() -> Self {
        Self { signature: vec![] }
    }
}

use ibc_proto::ibc::lightclients::grandpa::v1::ValidatorMerkleProof as RawValidatorMerkleProof;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatorMerkleProof {
    //// Proof items (does not contain the leaf hash, nor the root obviously).
    ////
    //// This vec contains all inner node hashes necessary to reconstruct the root hash given the
    //// leaf hash.
    pub proof: Vec<Vec<u8>>,
    //// Number of leaves in the original tree.
    ////
    //// This is needed to detect a case where we have an odd number of leaves that "get promoted"
    //// to upper layers.
    //// pub number_of_leaves: usize,
    pub number_of_leaves: u32,
    //// Index of the leaf the proof is for (0-based).
    //// pub leaf_index: usize,
    pub leaf_index: u32,
    //// Leaf content.
    ////pub leaf: Vec<u8>,
    pub leaf: Vec<u8>,
}

impl From<RawValidatorMerkleProof> for ValidatorMerkleProof {
    fn from(raw: RawValidatorMerkleProof) -> Self {
        Self {
            proof: raw.proof,
            number_of_leaves: raw.number_of_leaves,
            leaf_index: raw.leaf_index,
            leaf: raw.leaf,
        }
    }
}

impl From<ValidatorMerkleProof> for RawValidatorMerkleProof {
    fn from(value: ValidatorMerkleProof) -> Self {
        Self {
            proof: value.proof,
            number_of_leaves: value.number_of_leaves,
            leaf_index: value.leaf_index,
            leaf: value.leaf,
        }
    }
}

impl Default for ValidatorMerkleProof {
    fn default() -> Self {
        Self {
            proof: vec![],
            number_of_leaves: 0,
            leaf_index: 0,
            leaf: vec![],
        }
    }
}

use ibc_proto::ibc::lightclients::grandpa::v1::MmrLeafProof as RawMmrLeafProof;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MmrLeafProof {
    //// The index of the leaf the proof is for.
    pub leaf_index: u64,
    //// Number of leaves in MMR, when the proof was generated.
    pub leaf_count: u64,
    //// Proof elements (hashes of siblings of inner nodes on the path to the leaf).
    pub items: Vec<Vec<u8>>,
}

impl From<RawMmrLeafProof> for MmrLeafProof {
    fn from(raw: RawMmrLeafProof) -> Self {
        Self {
            leaf_index: raw.leaf_index,
            leaf_count: raw.leaf_count,
            items: raw.items,
        }
    }
}

impl From<MmrLeafProof> for RawMmrLeafProof {
    fn from(value: MmrLeafProof) -> Self {
        Self {
            leaf_index: value.leaf_index,
            leaf_count: value.leaf_count,
            items: value.items,
        }
    }
}

impl Default for MmrLeafProof {
    fn default() -> Self {
        Self {
            leaf_index: 0,
            leaf_count: 0,
            items: vec![],
        }
    }
}
