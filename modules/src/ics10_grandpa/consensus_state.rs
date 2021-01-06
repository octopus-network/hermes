use crate::ics02_client::{client_def::AnyConsensusState, client_type::ClientType};
use crate::ics10_grandpa::error::{Error, Kind};
use crate::ics23_commitment::commitment::CommitmentRoot;
use chrono::{TimeZone, Utc};
use ibc_proto::ibc::lightclients::grandpa::v1::ConsensusState as RawConsensusState;
use sp_core::H256;
use std::convert::TryFrom;
use tendermint::time::Time;
use tendermint_proto::Protobuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConsensusState {
    pub timestamp: Time,
    pub root: H256,
}

impl ConsensusState {
    pub fn new(timestamp: Time, root: H256) -> Self {
        Self { timestamp, root }
    }
}

impl crate::ics02_client::state::ConsensusState for ConsensusState {
    fn client_type(&self) -> ClientType {
        ClientType::GRANDPA
    }

    fn root(&self) -> &CommitmentRoot {
        // &CommitmentRoot::from_bytes(self.root.as_bytes())
        unimplemented!()
    }

    fn validate_basic(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn wrap_any(self) -> AnyConsensusState {
        AnyConsensusState::GRANDPA(self)
    }
}

impl Protobuf<RawConsensusState> for ConsensusState {}

impl TryFrom<RawConsensusState> for ConsensusState {
    type Error = Error;

    fn try_from(raw: RawConsensusState) -> Result<Self, Self::Error> {
        let proto_timestamp = raw
            .timestamp
            .ok_or_else(|| Kind::InvalidRawConsensusState.context("missing timestamp"))?;

        Ok(Self {
            timestamp: Utc
                .timestamp(proto_timestamp.seconds, proto_timestamp.nanos as u32)
                .into(),
            root: H256::from_slice(&raw.root),
        })
    }
}

impl From<ConsensusState> for RawConsensusState {
    fn from(value: ConsensusState) -> Self {
        RawConsensusState {
            timestamp: Some(value.timestamp.to_system_time().unwrap().into()),
            root: value.root.as_bytes().to_vec(),
        }
    }
}
