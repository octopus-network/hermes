use crate::clients::ics10_grandpa::error::Error;
use crate::prelude::*;
use ibc_proto::google::protobuf::Timestamp;
use ibc_proto::ibc::lightclients::grandpa::v1::ConsensusState as RawConsensusState;
use ibc_proto::protobuf::Protobuf;
use serde::{Deserialize, Serialize};

/// ConsensusState
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusState {
    /// timestamp that corresponds to the block height in which the ConsensusState
    /// was stored.
    pub timestamp: Option<Timestamp>,
    /// parachain header.state_root that used to verify chain storage proof
    pub root: Vec<u8>,
}

impl Protobuf<RawConsensusState> for ConsensusState {}

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
