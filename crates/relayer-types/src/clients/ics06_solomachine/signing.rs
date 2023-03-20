use super::error::Error;
use crate::prelude::*;
use ibc_proto::cosmos::tx::signing::v1beta1::signature_descriptor::{
    data::{Single, Sum},
    Data as RawData,
};
use ibc_proto::protobuf::Protobuf;

// pub const SOLOMACHINE_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.solomachine.v2.ClientState";

#[derive(Clone, PartialEq, Eq)]
pub struct Data {
    pub sum: Option<data::Sum>,
}

impl Protobuf<RawData> for Data {}

impl TryFrom<RawData> for Data {
    type Error = Error;

    fn try_from(raw: RawData) -> Result<Self, Self::Error> {
        let sum = raw.sum.unwrap();
        match sum {
            Sum::Single(Single { mode, signature }) => Ok(Self {
                sum: Some(data::Sum::Single(data::Single { mode, signature })),
            }),
            _ => Err(Error::solomachine()),
        }
    }
}

impl From<Data> for RawData {
    fn from(value: Data) -> Self {
        let single = if let data::Sum::Single(single) = value.sum.unwrap() {
            single
        } else {
            todo!()
        };

        RawData {
            sum: Some(Sum::Single(Single {
                mode: single.mode,
                signature: single.signature,
            })),
        }
    }
}

pub mod data {
    use super::*;
    #[derive(Clone, PartialEq, Eq)]
    pub struct Single {
        pub mode: i32,
        pub signature: Vec<u8>,
    }

    #[derive(Clone, PartialEq, Eq)]
    pub struct Multi;

    #[derive(Clone, PartialEq, Eq)]
    pub enum Sum {
        Single(Single),
        Multi(Multi),
    }
}
