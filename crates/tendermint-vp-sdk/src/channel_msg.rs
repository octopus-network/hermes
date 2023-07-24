use crate::types::Proofs;
use crate::update_ic::{send_msg, send_msg_for_proofs};
use anyhow::Result;

pub async fn channel_open_init(canister_id: &str, is_mainnet: bool, msg: Vec<u8>) -> Result<()> {
    let method_name = "chan_open_init";
    let args = msg;
    send_msg(canister_id, method_name, args, is_mainnet).await
}

pub async fn channel_open_try(canister_id: &str, is_mainnet: bool, msg: Vec<u8>) -> Result<Proofs> {
    let method_name = "chan_open_try";
    let args = msg;
    send_msg_for_proofs(canister_id, method_name, args, is_mainnet).await
}

pub async fn channel_open_ack(canister_id: &str, is_mainnet: bool, msg: Vec<u8>) -> Result<Proofs> {
    let method_name = "chan_open_ack";
    let args = msg;
    send_msg_for_proofs(canister_id, method_name, args, is_mainnet).await
}

pub async fn channel_open_confirm(
    canister_id: &str,
    is_mainnet: bool,
    msg: Vec<u8>,
) -> Result<Proofs> {
    let method_name = "chan_open_confirm";
    let args = msg;
    send_msg_for_proofs(canister_id, method_name, args, is_mainnet).await
}

pub async fn channel_close_init(canister_id: &str, is_mainnet: bool, msg: Vec<u8>) -> Result<()> {
    let method_name = "chan_close_init";
    let args = msg;
    send_msg(canister_id, method_name, args, is_mainnet).await
}
