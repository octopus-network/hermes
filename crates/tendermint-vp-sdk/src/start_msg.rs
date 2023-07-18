use crate::update_ic::call_args_is_string_function;
use crate::update_ic::send_msg;
use anyhow::Result;

pub async fn greet(canister_id: &str, is_mainnet: bool) -> Result<String> {
    let method_name = "greet";
    let args = "davirain".to_string();
    let ret = call_args_is_string_function(canister_id, method_name, args, is_mainnet).await?;

    String::from_utf8(ret).map_err(|e| anyhow::anyhow!(e))
}

pub async fn start_vp(canister_id: &str, is_mainnet: bool) -> Result<()> {
    let method_name = "start";
    let args = vec![];
    send_msg(canister_id, method_name, args, is_mainnet).await
}

pub async fn restart_vp(canister_id: &str, is_mainnet: bool) -> Result<()> {
    let method_name = "restart";
    let args = vec![];
    send_msg(canister_id, method_name, args, is_mainnet).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    const PATH: &str = "tests/resource/canister_id";
    const HELLO_PATH: &str = "tests/resource/hello_id";

    #[test]
    fn start_works() {
        let canister_id = fs::read_to_string(PATH).expect("Read file error");
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let ret = start_vp(&canister_id, false).await;
            assert!(ret.is_ok());
        });
    }

    // [crates/tendermint-vp-sdk/src/start_msg.rs:46] ret  = "DIDL\0\u{1}q\u{10}Hello, davirain!"
    // test start_msg::tests::start_hello_works ... ok
    #[test]
    fn start_hello_works() {
        let canister_id = fs::read_to_string(HELLO_PATH).expect("Read file error");
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let ret = greet(&canister_id, false).await.unwrap();
            dbg!(ret);
        });
    }

    #[test]
    fn restart_works() {
        let canister_id = fs::read_to_string(PATH).expect("Read file error");
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let ret = restart_vp(&canister_id, false).await;
            match ret {
                Ok(r) => {
                    dbg!("{:?}", r);
                }
                Err(e) => {
                    println!("error: {:?}", e);
                    assert!(e.contains("unauthorized"));
                }
            }
        });
    }
}
