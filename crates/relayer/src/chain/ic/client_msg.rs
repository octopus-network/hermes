use crate::chain::ic::types::SmState;
use crate::chain::ic::update_ic::{send_msg_for_smstate, send_msg_for_vec};
use anyhow::Result;

pub async fn create_client(canister_id: &str, is_mainnet: bool, msg: Vec<u8>) -> Result<SmState> {
    let method_name = "create_client";
    let args = msg;
    send_msg_for_smstate(canister_id, method_name, args, is_mainnet).await
}

pub async fn update_client(canister_id: &str, is_mainnet: bool, msg: Vec<u8>) -> Result<Vec<u8>> {
    let method_name = "update_client";
    let args = msg;
    send_msg_for_vec(canister_id, method_name, args, is_mainnet).await
}
