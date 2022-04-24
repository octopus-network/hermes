use crate::prelude::*;

use crate::core::ics04_channel::error::Error;
use ibc_proto::ibc::core::channel::v1::acknowledgement::Response as RawResponse;
use ibc_proto::ibc::core::channel::v1::Acknowledgement as RawAcknowledgement;
use tendermint_proto::Protobuf;

pub const TYPE_URL: &str = "/ibc.core.channel.v1.Acknowledgement";

#[derive(Clone, Debug, PartialEq)]
pub struct Acknowledgement {
    /// response contains either a result or an error and must be non-empty
    pub response: Option<acknowledgement::Response>,
}

impl Acknowledgement {
    pub fn new_success(value: Vec<u8>) -> Self {
        Self {
            response: Some(acknowledgement::Response::Result(value)),
        }
    }

    pub fn new_error(value: String) -> Self {
        Self {
            response: Some(acknowledgement::Response::Error(value)),
        }
    }

    pub fn success(&self) -> Result<bool, Error> {
        match self
            .response
            .as_ref()
            .ok_or_else(Error::empty_acknowledge_response)?
        {
            acknowledgement::Response::Result(_value) => Ok(true),
            acknowledgement::Response::Error(_e) => Ok(false),
        }
    }
}

/// Nested message and enum types in `Acknowledgement`.
pub mod acknowledgement {
    use crate::prelude::*;

    /// response contains either a result or an error and must be non-empty
    #[derive(Clone, Debug, PartialEq)]
    pub enum Response {
        Result(Vec<u8>),
        Error(String),
    }
}

impl Protobuf<RawAcknowledgement> for Acknowledgement {}

impl TryFrom<RawAcknowledgement> for Acknowledgement {
    type Error = Error;

    fn try_from(raw_acknowledgement: RawAcknowledgement) -> Result<Self, Self::Error> {
        let acknowledgement = if raw_acknowledgement.response.is_some() {
            let response = raw_acknowledgement.response.unwrap();

            let inner_response = match response {
                RawResponse::Result(value) => acknowledgement::Response::Result(value),
                RawResponse::Error(value) => acknowledgement::Response::Error(value),
            };

            Acknowledgement {
                response: Some(inner_response),
            }
        } else {
            Acknowledgement { response: None }
        };

        Ok(acknowledgement)
    }
}

impl From<Acknowledgement> for RawAcknowledgement {
    fn from(acknowledgement: Acknowledgement) -> Self {
        if acknowledgement.response.is_some() {
            let raw_response = acknowledgement.response.unwrap();
            let inner_response = match raw_response {
                acknowledgement::Response::Result(value) => RawResponse::Result(value),
                acknowledgement::Response::Error(value) => RawResponse::Error(value),
            };

            RawAcknowledgement {
                response: Some(inner_response),
            }
        } else {
            RawAcknowledgement { response: None }
        }
    }
}
