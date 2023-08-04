use crate::chain::ic::update_ic::send_msg_for_vec;

pub async fn get_pubkey(canister_id: &str, is_mainnet: bool) -> Result<Vec<u8>, String> {
    let method_name = "public_key";
    let args = vec![];
    send_msg_for_vec(canister_id, method_name, args, is_mainnet)
        .await
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::get_pubkey;
    use std::fs;
    const PATH: &str = "tests/resource/canister_id";

    #[test]
    fn query_pubkey_works() {
        let canister_id = fs::read_to_string(PATH).expect("Read file error");
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let ret = get_pubkey(&canister_id, false).await;
            assert!(ret.is_ok());
        });
    }
}
