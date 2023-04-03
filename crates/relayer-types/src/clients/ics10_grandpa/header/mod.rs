use crate::prelude::*;
use ibc_proto::ibc::lightclients::grandpa::v1::Header as RawHeader;
use serde::{Deserialize, Serialize};
pub mod beefy_mmr;
pub mod message;
use crate::clients::ics10_grandpa::error::Error;
use ibc_proto::protobuf::Protobuf;

pub const GRANDPA_HEADER_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.Header";

/// header wrapper
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    /// the latest mmr data
    pub beefy_mmr: Option<beefy_mmr::BeefyMmr>,
    /// only one header
    pub message: Option<message::Message>,
}

impl Protobuf<RawHeader> for Header {}

impl TryFrom<RawHeader> for Header {
    type Error = Error;

    fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<Header> for RawHeader {
    fn from(value: Header) -> Self {
        todo!()
    }
}
