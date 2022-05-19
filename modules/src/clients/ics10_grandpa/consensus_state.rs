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
    //// timestamp
    pub timestamp: Time,
}

impl ConsensusState {
    pub fn new(header: BlockHeader) -> Self {
        Self {
            parent_hash: header.clone().parent_hash,
            block_number: header.block_number,
            state_root: header.clone().state_root,
            extrinsics_root: header.clone().extrinsics_root,
            digest: vec![],
            root: CommitmentRoot::from(header.extrinsics_root),
            //TODO: better to get timestamp from header
            timestamp: Time::from_unix_timestamp(0, 0).unwrap(),
        }
    }
}

impl Default for ConsensusState {
    fn default() -> Self {
        Self {
            parent_hash: vec![0; 10],
            block_number: 0,
            state_root: vec![0; 10],
            extrinsics_root: vec![0; 10],
            digest: vec![0; 10],
            root: CommitmentRoot::from(vec![1, 2, 3]),
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
            .ok_or_else(|| Error::invalid_raw_consensus_state("missing timestamp".into()))?;
        // FIXME: shunts like this are necessary due to
        // https://github.com/informalsystems/tendermint-rs/issues/1053
        let proto_timestamp = tpb::Timestamp { seconds, nanos };
        let timestamp = proto_timestamp
            .try_into()
            .map_err(|e| Error::invalid_raw_consensus_state(format!("invalid timestamp: {}", e)))?;

        // let t_time = Time::from_unix_timestamp(seconds, nanos as u32)
        //     .map_err(|e| Error::invalid_raw_consensus_state(format!("invalid timestamp: {}", e)))?;
        // let timestamp = t_time.into();

        // let timestamp = Timestamp::new(seconds as u64);

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
            parent_hash: value.parent_hash,
            block_number: value.block_number,
            state_root: value.state_root,
            extrinsics_root: value.extrinsics_root,
            digest: value.digest,
            root: Some(ibc_proto::ibc::core::commitment::v1::MerkleRoot {
                hash: value.root.into_vec(),
            }),
            timestamp: Some(timestamp),
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
            digest: header.clone().block_header.digest,
            root: CommitmentRoot::from(header.block_header.extrinsics_root),
            //TODO: better to get timestamp from header
            timestamp: Time::from_unix_timestamp(0, 0).unwrap(),
        }
    }
}
