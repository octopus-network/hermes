use crate::prelude::*;
use ibc_proto::google::protobuf::Timestamp;
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
