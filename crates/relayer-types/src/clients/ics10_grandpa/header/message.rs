// Nested message and enum types in `Header`.
use crate::clients::ics10_grandpa::error::Error;
use crate::prelude::*;
use alloc::collections::BTreeMap;
use ibc_proto::ibc::lightclients::grandpa::v1::header::Message as RawMessage;
use ibc_proto::ibc::lightclients::grandpa::v1::{
    ParachainHeader as RawParachainHeader, ParachainHeaderMap as RawParachainHeaderMap,
    StateProof as RawStateProof, SubchainHeader as RawSubchainHeader,
    SubchainHeaderMap as RawSubchainHeaderMap,
};
use ibc_proto::protobuf::Protobuf;
use serde::{Deserialize, Serialize};

/// only one header
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    /// solochain headers and their proofs
    SolochainHeaderMap(SubchainHeaderMap),
    /// parachain headers and their proofs
    ParachainHeaderMap(ParachainHeaderMap),
}

// todo fix ibc-proto-rs RawMessage
// impl Protobuf<RawMessage> for Message {}

// impl TryFrom<RawMessage> for Message {
//     type Error = Error;

//     fn try_from(raw: RawMessage) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }

// impl From<Message> for RawMessage {
//     fn from(value: Message) -> Self {
//         todo!()
//     }
// }

/// substrate chain header map
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubchainHeaderMap {
    /// LatestMMR latest_mmr = 1;
    /// map<blocknumber,scale-encoded blockheader>
    ///
    /// map<uint32,Timestamp> timestamp_map=2;
    pub solochain_header_map: BTreeMap<u32, SubchainHeader>,
}

impl Protobuf<RawSubchainHeaderMap> for SubchainHeaderMap {}

impl TryFrom<RawSubchainHeaderMap> for SubchainHeaderMap {
    type Error = Error;

    fn try_from(raw: RawSubchainHeaderMap) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<SubchainHeaderMap> for RawSubchainHeaderMap {
    fn from(value: SubchainHeaderMap) -> Self {
        todo!()
    }
}
/// solochain header
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubchainHeader {
    /// scale-encoded solochain header bytes
    pub block_header: Vec<u8>,
    /// timestamp and proof
    pub timestamp: Option<StateProof>,
}

impl Protobuf<RawSubchainHeader> for SubchainHeader {}

impl TryFrom<RawSubchainHeader> for SubchainHeader {
    type Error = Error;

    fn try_from(raw: RawSubchainHeader) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<SubchainHeader> for RawSubchainHeader {
    fn from(value: SubchainHeader) -> Self {
        todo!()
    }
}

/// / Parachain headers and their merkle proofs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParachainHeaderMap {
    /// map<blocknumber,ParachainHeader>
    ///
    ///   map<uint32,Timestamp> timestamp_map=2;
    pub parachain_header_map: BTreeMap<u32, ParachainHeader>,
}

impl Protobuf<RawParachainHeaderMap> for ParachainHeaderMap {}

impl TryFrom<RawParachainHeaderMap> for ParachainHeaderMap {
    type Error = Error;

    fn try_from(raw: RawParachainHeaderMap) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<ParachainHeaderMap> for RawParachainHeaderMap {
    fn from(value: ParachainHeaderMap) -> Self {
        todo!()
    }
}
/// data needed to prove parachain header inclusion in mmr
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParachainHeader {
    /// para id
    pub parachain_id: u32,
    /// scale-encoded parachain header bytes
    pub block_header: Vec<u8>,
    /// proofs for parachain header in the mmr_leaf.parachain_heads
    pub proofs: Vec<Vec<u8>>,
    /// merkle leaf index for parachain heads proof
    pub header_index: u32,
    /// total number of para heads in parachain_heads_root
    pub header_count: u32,
    /// timestamp and proof
    pub timestamp: Option<StateProof>,
}

impl Protobuf<RawParachainHeader> for ParachainHeader {}

impl TryFrom<RawParachainHeader> for ParachainHeader {
    type Error = Error;

    fn try_from(raw: RawParachainHeader) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<ParachainHeader> for RawParachainHeader {
    fn from(value: ParachainHeader) -> Self {
        todo!()
    }
}

/// state value and proof
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateProof {
    /// state key
    pub key: Vec<u8>,
    /// the state value
    pub value: Vec<u8>,
    /// these proof gets from parachain by rpc methord:state_getReadProof
    pub proofs: Vec<Vec<u8>>,
}

impl Protobuf<RawStateProof> for StateProof {}

impl TryFrom<RawStateProof> for StateProof {
    type Error = Error;

    fn try_from(raw: RawStateProof) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<StateProof> for RawStateProof {
    fn from(value: StateProof) -> Self {
        todo!()
    }
}
