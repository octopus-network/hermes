use alloc::vec;
use alloc::vec::Vec;
use core::convert::Infallible;
use core::convert::{TryFrom, TryInto};

use serde::Serialize;

// use tendermint mock as grandpa
use ibc_proto::ibc::lightclients::grandpa::v1::ConsensusState as RawConsensusState;

use super::help::Commitment;
use crate::ics02_client::client_consensus::AnyConsensusState;
use crate::ics02_client::client_type::ClientType;
use crate::ics10_grandpa::error::Error;
use crate::ics10_grandpa::header::Header;
use crate::ics10_grandpa::help::BlockHeader;
use crate::ics23_commitment::commitment::CommitmentRoot;
use tendermint_proto::Protobuf;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsensusState {
    //// The parent hash.
    pub parent_hash: Vec<u8>,
    //// The block number
    pub block_number: u32,
    //// The state trie merkle root
    pub state_root: Vec<u8>,
    //// The merkle root of the extrinsics.
    pub extrinsics_root: Vec<u8>,
    //// A chain-specific digest of data useful for light clients or referencing auxiliary data.
    pub digest: Vec<u8>,
    // // TODO NEED timestamp, because ics02 have timestamp function
    pub root: CommitmentRoot,
}

impl ConsensusState {
    pub fn new(header: BlockHeader) -> Self {
        Self {
            parent_hash: header.clone().parent_hash,
            block_number: header.clone().block_number,
            state_root: header.clone().state_root,
            extrinsics_root: header.clone().extrinsics_root,
            digest: vec![],
            root: CommitmentRoot::from(header.extrinsics_root.clone()),
        }
    }

    // pub fn from_commit(root_commit: Commitment) -> Self {
    //     let encode_root_commit = serde_json::to_string(&root_commit)
    //         .unwrap()
    //         .as_bytes()
    //         .to_vec();
    //
    //     Self {
    //         root: CommitmentRoot::from_bytes(&encode_root_commit),
    //     }
    // }
}

impl Protobuf<RawConsensusState> for ConsensusState {}

impl crate::ics02_client::client_consensus::ConsensusState for ConsensusState {
    type Error = Infallible;

    fn client_type(&self) -> ClientType {
        ClientType::Grandpa
    }

    fn root(&self) -> &CommitmentRoot {
        &self.root
    }

    fn validate_basic(&self) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn wrap_any(self) -> AnyConsensusState {
        AnyConsensusState::Grandpa(self)
    }
}

impl TryFrom<RawConsensusState> for ConsensusState {
    type Error = Error;

    fn try_from(raw: RawConsensusState) -> Result<Self, Self::Error> {
        Ok(Self {
            parent_hash: raw.parent_hash,
            block_number: raw.block_number,
            state_root: raw.state_root,
            extrinsics_root: raw.extrinsics_root,
            digest: raw.digest,
            root: raw
                .root
                .ok_or_else(|| {
                    Error::invalid_raw_consensus_state("missing commitment root".into())
                })?
                .hash
                .into(),
        })
    }
}

impl From<ConsensusState> for RawConsensusState {
    fn from(value: ConsensusState) -> Self {
        Self {
            parent_hash: value.parent_hash,
            block_number: value.block_number,
            state_root: value.state_root,
            extrinsics_root: value.extrinsics_root,
            digest: value.digest,
            root: Some(ibc_proto::ibc::core::commitment::v1::MerkleRoot {
                hash: value.root.into_vec(),
            }),
        }
    }
}

impl From<Header> for ConsensusState {
    fn from(header: Header) -> Self {
        Self {
            parent_hash: header.clone().block_header.parent_hash,
            block_number: header.clone().block_header.block_number,
            state_root: header.clone().block_header.state_root,
            extrinsics_root: header.clone().block_header.extrinsics_root,
            digest: vec![],
            root: CommitmentRoot::from(header.block_header.extrinsics_root),
        }
    }
}
