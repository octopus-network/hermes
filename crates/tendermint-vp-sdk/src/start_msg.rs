use crate::update_ic::send_msg;

pub async fn start_vp(canister_id: &str, is_mainnet: bool) -> Result<(), String> {
    let method_name = "start";
    let args = vec![];
    send_msg(canister_id, method_name, args, is_mainnet).await
}

pub async fn restart_vp(canister_id: &str, is_mainnet: bool) -> Result<(), String> {
    let method_name = "restart";
    let args = vec![];
    send_msg(canister_id, method_name, args, is_mainnet).await
}

#[cfg(test)]
mod tests {
    use super::{restart_vp, start_vp};
    use std::fs;
    const PATH: &str = "tests/resource/canister_id";

    #[test]
    fn start_works() {
        let canister_id = fs::read_to_string(PATH).expect("Read file error");
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let ret = start_vp(&canister_id, false).await;
            assert!(ret.is_ok());
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
