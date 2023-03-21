use super::error::Error;
use crate::prelude::*;
use ibc_proto::ibc::lightclients::solomachine::v2::TimestampedSignatureData as RawTimestampedSignatureData;
use ibc_proto::protobuf::Protobuf;

// pub const SOLOMACHINE_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.solomachine.v2.ClientState";

#[derive(Clone, PartialEq, Eq)]

pub struct TimestampedSignatureData {
    pub signature_data: Vec<u8>,
    pub timestamp: u64,
}

impl Protobuf<RawTimestampedSignatureData> for TimestampedSignatureData {}

impl TryFrom<RawTimestampedSignatureData> for TimestampedSignatureData {
    type Error = Error;

    fn try_from(raw: RawTimestampedSignatureData) -> Result<Self, Self::Error> {
        Ok(Self {
            signature_data: raw.signature_data,
            timestamp: raw.timestamp,
        })
    }
}

impl From<TimestampedSignatureData> for RawTimestampedSignatureData {
    fn from(value: TimestampedSignatureData) -> Self {
        RawTimestampedSignatureData {
            signature_data: value.signature_data,
            timestamp: value.timestamp,
        }
    }
}
