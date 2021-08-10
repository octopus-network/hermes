use flex_error::{define_error, DisplayOnly, TraceError};
use crate::ics24_host::error::ValidationError;

define_error! {
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
    }
}
