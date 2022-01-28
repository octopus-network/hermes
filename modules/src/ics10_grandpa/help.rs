use crate::ics10_grandpa::error::Error;
use alloc::vec;
use alloc::vec::Vec;
use beefy_light_client::mmr::MmrLeafVersion;
use beefy_merkle_tree::Hash;
use codec::{Decode, Encode};
use core::convert::TryFrom;
use serde::{Deserialize, Serialize};

use ibc_proto::ibc::lightclients::grandpa::v1::Commitment as RawCommitment;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
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
            payload: vec![0u8; 32],
            validator_set_id: 0,
        }
    }
}

impl From<beefy_light_client::commitment::Commitment> for Commitment {
    fn from(value: beefy_light_client::commitment::Commitment) -> Self {
        Self {
            block_number: value.block_number,
            payload: Vec::from(value.payload),
            validator_set_id: value.validator_set_id,
        }
    }
}

impl From<Commitment> for beefy_light_client::commitment::Commitment {
    fn from(value: Commitment) -> Self {
        Self {
            payload: Hash::try_from(value.payload).unwrap_or([0; 32]),
            block_number: value.block_number,
            validator_set_id: value.validator_set_id,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
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

impl From<beefy_light_client::validator_set::BeefyNextAuthoritySet> for ValidatorSet {
    fn from(value: beefy_light_client::validator_set::BeefyNextAuthoritySet) -> Self {
        Self {
            id: value.id,
            len: value.len,
            root: Vec::from(value.root),
        }
    }
}

impl From<ValidatorSet> for beefy_light_client::validator_set::BeefyNextAuthoritySet {
    fn from(value: ValidatorSet) -> Self {
        Self {
            id: value.id,
            len: value.len,
            root: Hash::try_from(value.root).unwrap_or([0; 32]),
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct MmrLeaf {
    //// Version of the leaf format.
    //// Can be used to enable future format migrations and compatibility.
    pub version: u32,
    //// Current block parent number and hash.
    pub parent_number_and_hash: ParentNumberAndHash,
    //// A merkle root of the next BEEFY authority set.
    pub beefy_next_authority_set: ValidatorSet,
    //// A merkle root of all registered parachain heads.
    pub parachain_heads: Vec<u8>,
}

impl From<beefy_light_client::mmr::MmrLeaf> for MmrLeaf {
    fn from(value: beefy_light_client::mmr::MmrLeaf) -> Self {
        Self {
            version: value.version.0 as u32,
            parent_number_and_hash: ParentNumberAndHash {
                parent_header_number: value.parent_number_and_hash.0,
                parent_header_hash: Vec::from(value.parent_number_and_hash.1),
            },
            beefy_next_authority_set: ValidatorSet::from(value.beefy_next_authority_set),
            parachain_heads: Vec::from(value.parachain_heads),
        }
    }
}

impl From<MmrLeaf> for beefy_light_client::mmr::MmrLeaf {
    fn from(value: MmrLeaf) -> Self {
        Self {
            version: MmrLeafVersion(value.version as u8),
            parent_number_and_hash: (
                value.parent_number_and_hash.parent_header_number,
                Hash::try_from(value.parent_number_and_hash.parent_header_hash).unwrap(),
            ),
            beefy_next_authority_set:
                beefy_light_client::validator_set::BeefyNextAuthoritySet::from(
                    value.beefy_next_authority_set,
                ),
            parachain_heads: Hash::try_from(value.parachain_heads).unwrap(),
        }
    }
}

impl From<RawMmrLeaf> for MmrLeaf {
    fn from(raw: RawMmrLeaf) -> Self {
        Self {
            version: raw.version,
            parent_number_and_hash: raw.parent_number_and_hash.unwrap().into(),
            beefy_next_authority_set: raw.beefy_next_authority_set.unwrap().into(),
            parachain_heads: raw.parachain_heads,
        }
    }
}

impl From<MmrLeaf> for RawMmrLeaf {
    fn from(value: MmrLeaf) -> Self {
        Self {
            version: value.version,
            parent_number_and_hash: Some(value.parent_number_and_hash.into()),
            beefy_next_authority_set: Some(value.beefy_next_authority_set.into()),
            parachain_heads: value.parachain_heads,
        }
    }
}

impl Default for MmrLeaf {
    fn default() -> Self {
        Self {
            version: 0,
            parent_number_and_hash: ParentNumberAndHash::default(),
            beefy_next_authority_set: ValidatorSet::default(),
            parachain_heads: vec![],
        }
    }
}

use ibc_proto::ibc::lightclients::grandpa::v1::ParentNumberAndHash as RawParentNumberAndHash;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct ParentNumberAndHash {
    pub parent_header_number: u32,
    /// header hash
    pub parent_header_hash: Vec<u8>,
}

impl Default for ParentNumberAndHash {
    fn default() -> Self {
        Self {
            parent_header_number: 0,
            parent_header_hash: vec![],
        }
    }
}

impl From<RawParentNumberAndHash> for ParentNumberAndHash {
    fn from(raw: RawParentNumberAndHash) -> Self {
        Self {
            parent_header_number: raw.block_number,
            parent_header_hash: raw.mmr_root,
        }
    }
}

impl From<ParentNumberAndHash> for RawParentNumberAndHash {
    fn from(value: ParentNumberAndHash) -> Self {
        Self {
            block_number: value.parent_header_number,
            mmr_root: value.parent_header_hash,
        }
    }
}

use ibc_proto::ibc::lightclients::grandpa::v1::SignedCommitment as RawSignedCommitment;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct SignedCommitment {
    pub commitment: Option<Commitment>,
    pub signatures: Vec<Signature>,
}

impl SignedCommitment {
    pub fn from_height(height: Height) -> SignedCommitment {
        SignedCommitment {
            commitment: Some(Commitment {
                block_number: height.revision_height as u32,
                payload: vec![],
                validator_set_id: 0,
            }),
            signatures: vec![],
        }
    }
}

impl From<beefy_light_client::commitment::SignedCommitment> for SignedCommitment {
    fn from(value: beefy_light_client::commitment::SignedCommitment) -> Self {
        Self {
            commitment: Some(Commitment::from(value.commitment)),
            signatures: value
                .signatures
                .into_iter()
                .map(|value| Signature::from(value.unwrap()))
                .collect(),
        }
    }
}

impl From<SignedCommitment> for beefy_light_client::commitment::SignedCommitment {
    fn from(value: SignedCommitment) -> Self {
        Self {
            commitment: value.commitment.unwrap().into(),
            signatures: value
                .signatures
                .into_iter()
                .map(|value| Some(value.into()))
                .collect(),
        }
    }
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Decode, Encode)]
pub struct Signature {
    pub signature: Vec<u8>,
}

impl From<beefy_light_client::commitment::Signature> for Signature {
    fn from(value: beefy_light_client::commitment::Signature) -> Self {
        Self {
            signature: Vec::from(value.0),
        }
    }
}

impl From<Signature> for beefy_light_client::commitment::Signature {
    fn from(value: Signature) -> Self {
        Self {
            0: <[u8; 65]>::try_from(value.signature).unwrap(),
        }
    }
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Decode, Encode)]
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

impl From<beefy_light_client::ValidatorMerkleProof> for ValidatorMerkleProof {
    fn from(value: beefy_light_client::ValidatorMerkleProof) -> Self {
        let proof: Vec<Vec<u8>> = value.proof.into_iter().map(|val| Vec::from(val)).collect();
        Self {
            proof,
            number_of_leaves: value.number_of_leaves as u32,
            leaf_index: value.leaf_index as u32,
            leaf: value.leaf,
        }
    }
}

impl From<ValidatorMerkleProof> for beefy_light_client::ValidatorMerkleProof {
    fn from(value: ValidatorMerkleProof) -> Self {
        let mut proofs = vec![];
        for item in value.proof {
            let proof = Hash::try_from(item).unwrap_or([0; 32]);
            proofs.push(proof);
        }

        Self {
            proof: proofs,
            number_of_leaves: value.number_of_leaves as usize,
            leaf_index: value.leaf_index as usize,
            leaf: value.leaf,
        }
    }
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

use crate::Height;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Decode, Encode)]
pub struct MmrLeafProof {
    //// The index of the leaf the proof is for.
    pub leaf_index: u64,
    //// Number of leaves in MMR, when the proof was generated.
    pub leaf_count: u64,
    //// Proof elements (hashes of siblings of inner nodes on the path to the leaf).
    pub items: Vec<Vec<u8>>,
}

impl From<beefy_light_client::mmr::MmrLeafProof> for MmrLeafProof {
    fn from(value: beefy_light_client::mmr::MmrLeafProof) -> Self {
        let items = value
            .items
            .into_iter()
            .map(|value| Vec::from(value))
            .collect();
        Self {
            leaf_index: value.leaf_index,
            leaf_count: value.leaf_count,
            items,
        }
    }
}

impl From<MmrLeafProof> for beefy_light_client::mmr::MmrLeafProof {
    fn from(value: MmrLeafProof) -> Self {
        Self {
            leaf_index: value.leaf_index,
            leaf_count: value.leaf_count,
            items: value
                .items
                .into_iter()
                .map(|value| Hash::try_from(value).unwrap())
                .collect(),
        }
    }
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
            items: vec![vec![0u8; 32], vec![0u8; 32]],
        }
    }
}

use ibc_proto::ibc::lightclients::grandpa::v1::BlockHeader as RawBlockHeader;

/// Block Header
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Decode, Encode)]
pub struct BlockHeader {
    //// The parent hash.
    pub parent_hash: Vec<u8>,
    //// The block number.
    #[codec(compact)]
    pub block_number: u32,
    //// The state trie merkle root
    pub state_root: Vec<u8>,
    //// The merkle root of the extrinsics.
    pub extrinsics_root: Vec<u8>,
    //// A chain-specific digest of data useful for light clients or referencing auxiliary data.
    pub digest: Vec<u8>,
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

impl BlockHeader {
    pub fn hash(&self) -> Hash {
        let beefy_header = beefy_light_client::header::Header::from(self.clone());
        beefy_header.hash()
    }
}

impl From<beefy_light_client::header::Header> for BlockHeader {
    fn from(value: beefy_light_client::header::Header) -> Self {
        Self {
            parent_hash: Vec::from(value.parent_hash),
            block_number: value.number,
            state_root: Vec::from(value.state_root),
            extrinsics_root: Vec::from(value.extrinsics_root),
            digest: value.digest.encode(),
        }
    }
}

impl From<BlockHeader> for beefy_light_client::header::Header {
    fn from(value: BlockHeader) -> Self {
        let digest = beefy_light_client::header::Digest::decode(&mut &value.digest[..]).unwrap();
        Self {
            parent_hash: Hash::try_from(value.parent_hash).unwrap(),
            number: value.block_number,
            state_root: Hash::try_from(value.state_root).unwrap(),
            extrinsics_root: Hash::try_from(value.extrinsics_root).unwrap(),
            digest: digest,
        }
    }
}
impl Default for BlockHeader {
    fn default() -> Self {
        Self {
            parent_hash: vec![],
            block_number: 0,
            state_root: vec![],
            extrinsics_root: vec![],
            digest: vec![],
        }
    }
}

impl From<RawBlockHeader> for BlockHeader {
    fn from(raw: RawBlockHeader) -> Self {
        Self {
            parent_hash: raw.parent_hash,
            block_number: raw.block_number,
            state_root: raw.state_root,
            extrinsics_root: raw.extrinsics_root,
            digest: raw.digest,
        }
    }
}

impl From<BlockHeader> for RawBlockHeader {
    fn from(value: BlockHeader) -> Self {
        Self {
            parent_hash: value.parent_hash,
            block_number: value.block_number,
            state_root: value.state_root,
            extrinsics_root: value.extrinsics_root,
            digest: value.digest,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct MmrRoot {
    pub block_header: BlockHeader,
    pub signed_commitment: SignedCommitment,
    pub validator_merkle_proofs: Vec<ValidatorMerkleProof>,
    // pub mmr_leaf: MmrLeaf,
    // pub mmr_leaf_proof: MmrLeafProof,
    pub mmr_leaf: Vec<u8>,
    pub mmr_leaf_proof: Vec<u8>,
}
