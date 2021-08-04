use std::convert::{TryFrom, TryInto};
use std::convert::Infallible;

use serde::Serialize;

// use tendermint mock as grandpa
use ibc_proto::ibc::lightclients::grandpa::v1::ConsensusState as RawConsensusState;

use crate::ics02_client::client_consensus::AnyConsensusState;
use crate::ics02_client::client_type::ClientType;
use crate::ics10_grandpa::error::Error;
use crate::ics23_commitment::commitment::CommitmentRoot;
use crate::ics10_grandpa::header::Header;
use tendermint_proto::Protobuf;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsensusState{}


impl ConsensusState {
    pub fn new() -> Self {
        Self{}
    }
}

impl Protobuf<RawConsensusState> for ConsensusState {}

impl crate::ics02_client::client_consensus::ConsensusState for ConsensusState {
    type Error = Infallible;

    fn client_type(&self) -> ClientType {
        ClientType::Grandpa
    }

    fn root(&self) -> &CommitmentRoot {
        unimplemented!()
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

    fn try_from(_raw: RawConsensusState) -> Result<Self, Self::Error> {
        Ok(ConsensusState{})
    }
}

impl From<ConsensusState> for RawConsensusState {
    fn from(_value: ConsensusState) -> Self {
        Self{}
    }
}

// impl From<grandpa::block::Header> for ConsensusState {
//     fn from(header: grandpa::block::Header) -> Self {
//         unimplemented!()
//     }
// }

impl From<Header> for ConsensusState {
    fn from(_header: Header) -> Self {
        Self{}
    }
}