use flex_error::{define_error, DisplayOnly, TraceError};

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

        Infallible
            { reason: String }
            [ DisplayOnly<std::convert::Infallible> ]
            | _ | { "invalid header, failed basic validation" },
    }
}
