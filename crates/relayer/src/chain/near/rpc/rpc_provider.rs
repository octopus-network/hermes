pub const NEAR_OFFICIAL_MAINNET_RPC_URL: &str = "https://rpc.mainnet.near.org";
pub const NEAR_OFFICIAL_TESTNET_RPC_URL: &str = "https://rpc.testnet.near.org";

pub const BLOCKPI_MAINNET_RPC_URL: &str = "https://public-rpc.blockpi.io/http/near";
pub const BLOCKPI_TESTNET_RPC_URL: &str = "https://public-rpc.blockpi.io/http/near-testnet";

#[derive(Debug, Clone)]
pub enum NearEnv {
    Testnet,
    Mainnet,
}

pub enum RpcProvider {
    NearOfficial,
    BlockPi,
}

impl RpcProvider {
    pub fn get_rpc_by_env(&self, env: &NearEnv) -> &str {
        match self {
            RpcProvider::NearOfficial => match env {
                NearEnv::Testnet => NEAR_OFFICIAL_TESTNET_RPC_URL,
                NearEnv::Mainnet => NEAR_OFFICIAL_MAINNET_RPC_URL,
            },
            RpcProvider::BlockPi => match env {
                NearEnv::Testnet => BLOCKPI_TESTNET_RPC_URL,
                NearEnv::Mainnet => BLOCKPI_MAINNET_RPC_URL,
            },
        }
    }
}
