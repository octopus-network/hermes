use flex_error::{define_error, TraceError};

define_error! {
    NearError {
        InvalidAccountId
            |_| { "Invalid account ID" },

        GetSignerFailure
            [ TraceError<crate::error::Error> ]
            |_| { "get signer failure" },

        ParserInMemorySignerFailure
            [ TraceError<std::io::Error> ]
            |_| { "Parser InMemorySigner failure" },

        ParserNearAccountIdFailure
            [ TraceError<near_account_id::ParseAccountError> ]
            |_| { "Parser Near Account Id failure" },

        SerdeJsonError
            [ TraceError<serde_json::Error>]
            |_| { "serde json failure" },

        CustomError
            { reason: String }
            |e| { format!("Custom error: {}", e.reason) },
    }
}
