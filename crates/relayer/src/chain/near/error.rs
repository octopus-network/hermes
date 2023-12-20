use crate::chain::ic::errors::VpError;
use core::num::ParseIntError;
use flex_error::{define_error, TraceError};
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use near_jsonrpc_client::errors::JsonRpcError;
use near_jsonrpc_primitives::types::blocks::RpcBlockError;
use near_jsonrpc_primitives::types::changes::RpcStateChangesError;
use near_jsonrpc_primitives::types::query::RpcQueryError;
use near_jsonrpc_primitives::types::transactions::RpcTransactionError;
use std::string::FromUtf8Error;

define_error! {
    NearError {
        SerdeJsonError
            { line: String }
            [ TraceError<serde_json::Error>]
            |e| {
                format!("serde json failure \n{}", e.line)
            },

        CustomError
            { reason: String, line: String }
            |e| {
                format!("Custom error: {} \n{}", e.reason, e.line)
            },

        RpcQueryError
            { line: String }
            [ TraceError<JsonRpcError<RpcQueryError>> ]
            | e | {
                format!("near rpc query error \n{}", e.line)
            },

        RpcTransactionError
            { line: String }
            [ TraceError<JsonRpcError<RpcTransactionError>>]
            | e | {
                format!("near rpc transaction error \n{}", e.line)
            },

        RpcBlockError
            { line: String }
            [ TraceError<JsonRpcError<RpcBlockError>>]
            | e | {
                format!("near rpc block error \n{}", e.line)
            },

        RpcStateChangesError
            { line: String }
            [ TraceError<JsonRpcError<RpcStateChangesError>>]
            | e | {
                format!("near rpc state changes error \n{}", e.line)
            },

        DeliverError
            { line: String }
            | e | {
                format!("near chain Deliver failed \n{}", e.line)
            },

        VpDeliverError
            { line: String }
            [ TraceError<VpError>]
            | e | {
                format!("call vp deliver error, \n{}", e.line)
            },

        DecodeVpDeliverResultFailed
            { line: String }
            [ TraceError<prost::DecodeError>]
            | e | {
                format!("decode vp deliver result failed, \n{}", e.line)
            },

        BuildVpClientError
            { line: String }
            | e | {
                format!("near chain bootstrap build VpClientFailed \n{}", e.line)
            },

        BuildIbcHeightError
            { line: String }
            [ TraceError<Ics02Error>]
            | e | {
                format!("build ibc height failed \n{}", e.line)
            },

        VpError
            { line: String }
            [ TraceError<VpError> ]
            | e | {
                format!("vp error \n{}", e.line)
            },

        NextBpsEmpty
            { line: String }
            | e | {
                format!("next bps empty \n{}",e.line)
            },

        ParseIntError
            { line: String }
            [ TraceError<ParseIntError> ]
            | e | {
                format!("parse int error \n{}", e.line)
            },

        DecodeStringError
            { line: String }
            [ TraceError<FromUtf8Error> ]
            | e | {
                format!("decode string error \n{}", e.line)
            },

        BuildNearProofsFailed
            { line: String }
            [ TraceError<std::io::Error>]
            | e | {
                format!("build near proofs failed \n{}", e.line)
            }
    }
}
