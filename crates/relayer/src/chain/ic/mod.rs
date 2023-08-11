pub mod channel_msg;
pub mod client_msg;
pub mod connection_msg;
pub mod errors;
pub mod get_pubkey;
mod identity;
pub mod packet_msg;
pub mod query_ic;
pub mod start_msg;
mod types;
mod update_ic;

use crate::chain::ic::update_ic::send_msg_for_vec;
use anyhow::Result;

pub async fn deliver(canister_id: &str, is_mainnet: bool, msg: Vec<u8>) -> Result<Vec<u8>> {
    let method_name = "deliver";
    let args = msg;
    send_msg_for_vec(canister_id, method_name, args, is_mainnet).await
}

pub async fn query_client_state(
    canister_id: &str,
    is_mainnet: bool,
    msg: Vec<u8>,
) -> Result<Vec<u8>> {
    let method_name = "query_client_state";
    let args = msg;
    send_msg_for_vec(canister_id, method_name, args, is_mainnet).await
}

pub async fn query_consensus_state(
    canister_id: &str,
    is_mainnet: bool,
    msg: Vec<u8>,
) -> Result<Vec<u8>> {
    let method_name = "query_consensus_state";
    let args = msg;
    send_msg_for_vec(canister_id, method_name, args, is_mainnet).await
}
