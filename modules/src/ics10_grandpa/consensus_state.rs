use core::convert::Infallible;
use core::convert::{TryFrom, TryInto};
use alloc::vec::Vec;

use serde::Serialize;

// use tendermint mock as grandpa
use ibc_proto::ibc::lightclients::grandpa::v1::ConsensusState as RawConsensusState;

use crate::ics02_client::client_consensus::AnyConsensusState;
use crate::ics02_client::client_type::ClientType;
use crate::ics10_grandpa::error::Error;
use crate::ics10_grandpa::header::Header;
use crate::ics23_commitment::commitment::CommitmentRoot;
use tendermint_proto::Protobuf;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsensusState {
    pub root: CommitmentRoot,
}

impl ConsensusState {
    pub fn new(root: CommitmentRoot) -> Self {
        Self {
            root
        }
    }
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
        todo!()
    }
}

impl From<ConsensusState> for RawConsensusState {
    fn from(value: ConsensusState) -> Self {
        todo!()
    }
}

// impl From<grandpa::block::Header> for ConsensusState {
//     fn from(header: grandpa::block::Header) -> Self {
//         unimplemented!()
//     }
// }

impl From<Header> for ConsensusState {
    fn from(header: Header) -> Self {
        let mut temp_vec = Vec::new();
        for val in 0..4 {
            temp_vec.push(val);
        }
        Self {
            root: CommitmentRoot::from(temp_vec)
        }
    }
}
