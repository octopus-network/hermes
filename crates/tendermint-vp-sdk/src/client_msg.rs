use crate::types::SmState;
use crate::update_ic::{send_msg_for_smstate, send_msg_for_vec};

pub async fn create_client(
    canister_id: &str,
    is_mainnet: bool,
    msg: Vec<u8>,
) -> Result<SmState, String> {
    let method_name = "create_client";
    let args = msg;
    send_msg_for_smstate(canister_id, method_name, args, is_mainnet)
        .await
        .map_err(|e| e.to_string())
}

pub async fn update_client(
    canister_id: &str,
    is_mainnet: bool,
    msg: Vec<u8>,
) -> Result<Vec<u8>, String> {
    let method_name = "update_client";
    let args = msg;
    send_msg_for_vec(canister_id, method_name, args, is_mainnet)
        .await
        .map_err(|e| e.to_string())
}
