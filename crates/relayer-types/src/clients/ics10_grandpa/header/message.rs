// Nested message and enum types in `Header`.
use crate::prelude::*;
use alloc::collections::BTreeMap;
use serde::{Deserialize, Serialize};
/// only one header
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    /// solochain headers and their proofs
    SolochainHeaderMap(SolochainHeaderMap),
    /// parachain headers and their proofs
    ParachainHeaderMap(ParachainHeaderMap),
}

/// solochain header map
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolochainHeaderMap {
    /// LatestMMR latest_mmr = 1;
    /// map<blocknumber,scale-encoded blockheader>
    ///
    /// map<uint32,Timestamp> timestamp_map=2;
    pub solochain_header_map: BTreeMap<u32, SolochainHeader>,
}
/// solochain header
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolochainHeader {
    /// scale-encoded solochain header bytes
    pub block_header: Vec<u8>,
    /// timestamp and proof
    pub timestamp: Option<StateProof>,
}

/// / Parachain headers and their merkle proofs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParachainHeaderMap {
    /// map<blocknumber,ParachainHeader>
    ///
    ///   map<uint32,Timestamp> timestamp_map=2;
    pub parachain_header_map: BTreeMap<u32, ParachainHeader>,
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
