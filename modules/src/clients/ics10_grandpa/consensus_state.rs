use alloc::format;
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
use tendermint::Time;
use tendermint_proto::Protobuf;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsensusState {
    pub timestamp: Time,
    pub root: CommitmentRoot,
}

impl ConsensusState {
    pub fn new(root: Vec<u8>, timestamp: Time) -> Self {
        Self {
            timestamp,
            root: root.into(),
        }
    }

    // TODO
    pub fn from_solchain_header(header: BlockHeader) -> Result<Self, Error> {
        todo!()
    }
}

impl Default for ConsensusState {
    fn default() -> Self {
        Self {
            timestamp: Time::from_unix_timestamp(0, 0).unwrap(),
            root: CommitmentRoot::from(vec![0]),
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
        let ibc_proto::google::protobuf::Timestamp { seconds, nanos } = raw
            .timestamp
            .ok_or_else(|| Error::invalid_raw_consensus_state("mising timestamp".into()))?;
        let proto_timestamp = tendermint_proto::google::protobuf::Timestamp { seconds, nanos };

        let timestamp = proto_timestamp
            .try_into()
            .map_err(|e| Error::invalid_raw_consensus_state(format!("invalid timestamp: {}", e)))?;

        Ok(Self {
            timestamp,
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
        let tendermint_proto::google::protobuf::Timestamp { seconds, nanos } =
            value.timestamp.into();
        let timestamp = ibc_proto::google::protobuf::Timestamp { seconds, nanos };
        Self {
            timestamp: Some(timestamp),
            root: Some(ibc_proto::ibc::core::commitment::v1::MerkleRoot {
                hash: value.root.into_vec(),
            }),
        }
    }
}

impl From<Header> for ConsensusState {
    fn from(header: Header) -> Self {
        let ret = ConsensusState::from_solchain_header(header.block_header);
        ret.unwrap()
    }
}
