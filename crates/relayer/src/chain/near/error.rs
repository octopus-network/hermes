use crate::chain::ic::errors::VpError;
use flex_error::{define_error, TraceError};
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use near_jsonrpc_client::errors::JsonRpcError;
use near_jsonrpc_primitives::types::blocks::RpcBlockError;
use near_jsonrpc_primitives::types::changes::RpcStateChangesError;
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

        RpcStateChangesError
            [ TraceError<JsonRpcError<RpcStateChangesError>>]
            | _ | { "near rpc state changes error" },

        DeliverError
            |_| { "near chain Deliver failed" },

        BuildVpClientError
            | _ | { "near chain bootstrap build VpClientFailed" },

        BuildIbcHeightError
            [ TraceError<Ics02Error>]
            | _ | { "build ibc height failed" },

        VpError
            [ TraceError<VpError> ]
            | _ | { "vp error" },
    }
}
