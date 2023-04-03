use crate::clients::ics10_grandpa::error::Error;
use crate::core::ics02_client::error::Error as Ics02Error;
use crate::prelude::*;
use ibc_proto::google::protobuf::Any;
use ibc_proto::google::protobuf::Timestamp;
use ibc_proto::ibc::lightclients::grandpa::v1::ConsensusState as RawConsensusState;
use ibc_proto::protobuf::Protobuf;
use serde::{Deserialize, Serialize};

pub const GRANDPA_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.ConsensusState";

/// ConsensusState
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusState {
    /// timestamp that corresponds to the block height in which the ConsensusState
    /// was stored.
    // toto(davirian), maybe timestamp need replace to anyther time.
    pub timestamp: Option<Timestamp>,
    /// parachain header.state_root that used to verify chain storage proof
    pub root: Vec<u8>,
}

impl Protobuf<RawConsensusState> for ConsensusState {}

impl TryFrom<RawConsensusState> for ConsensusState {
    type Error = Error;

    fn try_from(raw: RawConsensusState) -> Result<Self, Self::Error> {
        Ok(Self {
            timestamp: raw.timestamp,
            root: raw.root,
        })
    }
}

impl From<ConsensusState> for RawConsensusState {
    fn from(value: ConsensusState) -> Self {
        Self {
            timestamp: value.timestamp,
            root: value.root,
        }
    }
}

impl Protobuf<Any> for ConsensusState {}

impl TryFrom<Any> for ConsensusState {
    type Error = Ics02Error;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use bytes::Buf;
        use core::ops::Deref;
        use prost::Message;

        fn decode_consensus_state<B: Buf>(buf: B) -> Result<ConsensusState, Error> {
            RawConsensusState::decode(buf)
                .map_err(Error::decode)?
                .try_into()
        }

        match raw.type_url.as_str() {
            GRANDPA_CONSENSUS_STATE_TYPE_URL => {
                decode_consensus_state(raw.value.deref()).map_err(Into::into)
            }
            _ => Err(Ics02Error::unknown_consensus_state_type(raw.type_url)),
        }
    }
}

impl From<ConsensusState> for Any {
    fn from(consensus_state: ConsensusState) -> Self {
        Any {
            type_url: GRANDPA_CONSENSUS_STATE_TYPE_URL.to_string(),
            value: Protobuf::<RawConsensusState>::encode_vec(&consensus_state)
                .expect("encoding to `Any` from `GpConsensusState`"),
        }
    }
}
