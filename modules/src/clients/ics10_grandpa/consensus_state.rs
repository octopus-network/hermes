use alloc::vec;
use alloc::vec::Vec;
use core::convert::Infallible;
use core::convert::{TryFrom, TryInto};

use serde::Serialize;

// use tendermint mock as grandpa
use ibc_proto::ibc::lightclients::grandpa::v1::ConsensusState as RawConsensusState;

use super::help::Commitment;
use crate::clients::ics10_grandpa::error::Error;
use crate::clients::ics10_grandpa::header::Header;
use crate::clients::ics10_grandpa::help::BlockHeader;
use crate::core::ics02_client::client_consensus::AnyConsensusState;
use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics23_commitment::commitment::CommitmentRoot;
use tendermint_proto::Protobuf;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsensusState {
    // TODO NEED timestamp, because ics02 have timestamp function
    // commitmentroot save mrr root 
    pub root: CommitmentRoot,
}

impl ConsensusState {
    pub fn new(value: Vec<u8>) -> Self {
        Self {
            root: CommitmentRoot::from(value),
        }
    }

    pub fn from_commitment_root(root: CommitmentRoot) -> Self {
        Self {
            root: root,
        }
    }
}

impl Default for ConsensusState {
    fn default() -> Self {
        Self {
            root: CommitmentRoot::from(vec![]),
        }
    }
}
impl Protobuf<RawConsensusState> for ConsensusState {}

impl crate::core::ics02_client::client_consensus::ConsensusState for ConsensusState {
    type Error = Infallible;

    fn client_type(&self) -> ClientType {
        ClientType::Grandpa
    }

    fn root(&self) -> &CommitmentRoot {
        &self.root
    }

    fn wrap_any(self) -> AnyConsensusState {
        AnyConsensusState::Grandpa(self)
    }
}

impl TryFrom<RawConsensusState> for ConsensusState {
    type Error = Error;

    fn try_from(raw: RawConsensusState) -> Result<Self, Self::Error> {
        Ok(Self {
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
            root: Some(ibc_proto::ibc::core::commitment::v1::MerkleRoot {
                hash: value.root.into_vec(),
            }),
        }
    }
}

// impl From<Header> for ConsensusState {
//     fn from(header: Header) -> Self {
//         Self {
//             root: CommitmentRoot::from(header.block_header.extrinsics_root),
//         }
//     }
// }
