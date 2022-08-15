use alloc::vec::Vec;
use alloc::{format, vec};
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
use crate::timestamp::Timestamp;
use tendermint::time::Time;
use tendermint_proto::google::protobuf as tpb;
use tendermint_proto::Protobuf;
// use sp_timestamp::Timestamp;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsensusState {
    ///commitment: Option<Commitment>,used to verify mmr proof
    pub commitment: Commitment,
    /// The state trie merkle root that used to verify storage proof
    pub state_root: CommitmentRoot,
    /// timestamp
    pub timestamp: Time,
}

impl ConsensusState {
    pub fn new(commitment: Commitment, state_root: CommitmentRoot, timestamp: Time) -> Self {
        Self {
            commitment,
            state_root,
            timestamp,
        }
    }
}

impl Default for ConsensusState {
    fn default() -> Self {
        Self {
            commitment: Commitment::default(),
            state_root: CommitmentRoot::from(vec![0]),
            timestamp: Time::from_unix_timestamp(0, 0).unwrap(),
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
        &self.state_root
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
            .ok_or_else(|| Error::invalid_raw_consensus_state("missing timestamp".into()))?;

        let proto_timestamp = tpb::Timestamp { seconds, nanos };
        tracing::trace!(target:"ibc-rs","grandpa consensuse state timestamp! proto_timestamp : {:?} ",proto_timestamp);

        let timestamp = proto_timestamp
            .try_into()
            .map_err(|e| Error::invalid_raw_consensus_state(format!("invalid timestamp: {}", e)))?;

        Ok(Self {
            commitment: raw
                .commitment
                .ok_or_else(Error::empty_latest_commitment)?
                .into(),
            state_root: raw
                .state_root
                .ok_or_else(|| {
                    Error::invalid_raw_consensus_state("missing commitment root".into())
                })?
                .hash
                .into(),
            timestamp,
        })
    }
}

impl From<ConsensusState> for RawConsensusState {
    fn from(value: ConsensusState) -> Self {
        // FIXME: shunts like this are necessary due to
        // https://github.com/informalsystems/tendermint-rs/issues/1053
        let tpb::Timestamp { seconds, nanos } = value.timestamp.into();
        let timestamp = ibc_proto::google::protobuf::Timestamp { seconds, nanos };

        Self {
            commitment: Some(value.commitment.into()),
            state_root: Some(ibc_proto::ibc::core::commitment::v1::MerkleRoot {
                hash: value.state_root.into_vec(),
            }),
            timestamp: Some(timestamp),
        }
    }
}

impl From<Header> for ConsensusState {
    fn from(header: Header) -> Self {
        Self {
            commitment: header.mmr_root.signed_commitment.commitment.unwrap(),
            state_root: CommitmentRoot::from_bytes(&header.block_header.state_root),
            timestamp: header.timestamp,
        }
    }
}
