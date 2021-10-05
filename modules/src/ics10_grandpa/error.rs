use alloc::string::String;

use crate::ics24_host::error::ValidationError;
use flex_error::{define_error, DisplayOnly, TraceError};

define_error! {
     #[derive(Debug, PartialEq, Eq)]
    Error{
        Dummy
            |_| { format_args!("dummy error") },

        Decode
            [ TraceError<prost::DecodeError> ]
            | _ | { "decode error" },

        MissingLatestHeight
            | _ | { "missing latest height" },

        MissingHeight
            | _ | { "missing height" },

        InvalidChainIdentifier
            [ ValidationError ]
            | _ | { "Invalid chain identifier" },

        MissingFrozenHeight
            | _ | { "missing frozen height" },

        InvalidRawConsensusState
            { reason: String }
            | _ | { "invalid raw client consensus state" },
    }
}
