use crate::core::ics02_client::error::Error as Ics02Error;
use flex_error::{define_error, TraceError};

define_error! {
    #[derive(Debug, PartialEq, Eq)]
    Error {
        Solomachine
            |_| { "solomachine" },
        Decode
            [ TraceError<prost::DecodeError> ]
            | _ | { "decode error" },
    }
}

impl From<Error> for Ics02Error {
    fn from(e: Error) -> Self {
        Self::client_specific(e.to_string())
    }
}
