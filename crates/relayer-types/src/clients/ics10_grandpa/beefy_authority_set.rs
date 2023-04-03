use crate::clients::ics10_grandpa::error::Error;
use crate::prelude::*;
use ibc_proto::ibc::lightclients::grandpa::v1::BeefyAuthoritySet as RawBeefyAuthoritySet;
use ibc_proto::protobuf::Protobuf;
use serde::{Deserialize, Serialize};

/// Beefy Authority Info
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeefyAuthoritySet {
    /// Id of the authority set, it should be strictly increasing
    pub id: u64,
    /// Number of validators in the set.
    pub len: u32,
    /// Merkle Root Hash build from BEEFY uncompressed AuthorityIds.
    pub root: Vec<u8>,
}

impl Protobuf<RawBeefyAuthoritySet> for BeefyAuthoritySet {}

impl TryFrom<RawBeefyAuthoritySet> for BeefyAuthoritySet {
    type Error = Error;

    fn try_from(raw: RawBeefyAuthoritySet) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<BeefyAuthoritySet> for RawBeefyAuthoritySet {
    fn from(value: BeefyAuthoritySet) -> Self {
        todo!()
    }
}
