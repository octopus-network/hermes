use crate::chain::ic::identity::create_identity;
use crate::chain::ic::types::*;
use candid::{Decode, Encode};

async fn query_ic(
    canister_id: &str,
    method_name: &str,
    args: u64,
    is_mainnet: bool,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let url = if is_mainnet { MAIN_NET } else { LOCAL_NET };
    let agent = ic_agent::Agent::builder()
        .with_url(url)
        .with_identity(create_identity())
        .build()
        .expect("should work");

    if !is_mainnet {
        agent.fetch_root_key().await?;
    }

    let canister_id = ic_cdk::export::Principal::from_text(canister_id)?;

    let mut update_builder =
        ic_agent::agent::QueryBuilder::new(&agent, canister_id, method_name.to_string());
    let update_builder_with_args = update_builder.with_arg(&Encode!(&args)?);

    let response = update_builder_with_args.call().await?;
    Ok(response)
}

async fn query_ic_and_get_u64(
    canister_id: &str,
    method_name: &str,
    args: u64,
    is_mainnet: bool,
) -> Result<U64Result, Box<dyn std::error::Error>> {
    let response = query_ic(canister_id, method_name, args, is_mainnet).await?;
    let response = Decode!(response.as_slice(), U64Result)?;

    Ok(response)
}

async fn call_ic_and_get_u64(
    canister_id: &str,
    method_name: &str,
    args: u64,
    is_mainnet: bool,
) -> Result<u64, String> {
    let result = query_ic_and_get_u64(canister_id, method_name, args, is_mainnet)
        .await
        .map_err(|e| e.to_string())?;

    match result {
        U64Result::Ok(data) => Ok(data),
        U64Result::Err(e) => Err(e),
    }
}

pub async fn query_sequence_times(
    canister_id: &str,
    is_mainnet: bool,
    sequence: u64,
) -> Result<u64, String> {
    let method_name = "get_sequence_times";
    call_ic_and_get_u64(canister_id, method_name, sequence, is_mainnet).await
}
