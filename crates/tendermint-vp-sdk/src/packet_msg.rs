use crate::types::Proofs;
use crate::update_ic::send_msg_for_proofs;
use anyhow::Result;

pub async fn recv_packet(canister_id: &str, is_mainnet: bool, msg: Vec<u8>) -> Result<Proofs> {
    let method_name = "recv_packet";
    let args = msg;
    send_msg_for_proofs(canister_id, method_name, args, is_mainnet).await
}

pub async fn ack_packet(canister_id: &str, is_mainnet: bool, msg: Vec<u8>) -> Result<Proofs> {
    let method_name = "ack_packet";
    let args = msg;
    send_msg_for_proofs(canister_id, method_name, args, is_mainnet).await
}
