use flex_error::{define_error, TraceError};
use near_jsonrpc_client::errors::JsonRpcError;
use near_jsonrpc_primitives::types::blocks::RpcBlockError;
use near_jsonrpc_primitives::types::query::RpcQueryError;
use near_jsonrpc_primitives::types::transactions::RpcTransactionError;

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

        RpcQueryError
            [ TraceError<JsonRpcError<RpcQueryError>> ]
            | _ | { "near rpc query error"},

        RpcTransactionError
            [ TraceError<JsonRpcError<RpcTransactionError>>]
            | _ | { "near rpc transaction error"},

        RpcBlockError
            [ TraceError<JsonRpcError<RpcBlockError>>]
            | _ | { "near rpc block error" },

        DeliverError
            |_| { "near chain Deliver failed" },

        BuildVpClientError
            | _ | { "near chain bootstrap build VpClientFailed" },
    }
}
