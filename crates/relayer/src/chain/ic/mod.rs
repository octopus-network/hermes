pub mod errors;
mod identity;
mod types;

use std::path::PathBuf;

use anyhow::Result;

use crate::chain::ic::errors::Error;
use crate::chain::ic::identity::create_identity;
use crate::chain::ic::types::*;
use candid::{Decode, Encode};

async fn query_ic(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    ic_endpoint_url: &str,
) -> Result<Vec<u8>> {
    let agent = ic_agent::Agent::builder()
        .with_url(ic_endpoint_url)
        .build()
        .map_err(Error::AgentError)?;

    if ic_endpoint_url == LOCAL_NET {
        agent.fetch_root_key().await?;
    }

    let canister_id = candid::Principal::from_text(canister_id)?;

    let query_builder =
        ic_agent::agent::QueryBuilder::new(&agent, canister_id, method_name.to_string());

    let args: Vec<u8> = Encode!(&args)?;
    let query_builder_with_args = query_builder.with_arg(&*args);

    let response = query_builder_with_args.call().await?;

    Decode!(response.as_slice(), VecResult)?.transder_anyhow()
}

async fn update_ic(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    ic_endpoint_url: &str,
    pem_file: &PathBuf,
) -> Result<Vec<u8>> {
    let agent = ic_agent::Agent::builder()
        .with_url(ic_endpoint_url)
        .with_identity(create_identity(pem_file)?)
        .build()
        .map_err(Error::AgentError)?;

    if ic_endpoint_url == LOCAL_NET {
        agent.fetch_root_key().await?;
    }

    let canister_id = candid::Principal::from_text(canister_id)?;

    let update_builder =
        ic_agent::agent::UpdateBuilder::new(&agent, canister_id, method_name.to_string());

    let args: Vec<u8> = Encode!(&args)?;
    let update_builder_with_args = update_builder.with_arg(&*args);

    let response = update_builder_with_args.call_and_wait().await?;

    Decode!(response.as_slice(), VecResult)?.transder_anyhow()
}

pub async fn deliver(
    canister_id: &str,
    ic_endpoint_url: &str,
    msg: Vec<u8>,
    pem_file: &PathBuf,
) -> Result<Vec<u8>> {
    update_ic(canister_id, "deliver", msg, is_mainnet, pem_file).await
}

pub async fn query_client_state(
    canister_id: &str,
    ic_endpoint_url: &str,
    msg: Vec<u8>,
) -> Result<Vec<u8>> {
    query_ic(canister_id, "query_client_state", msg, is_mainnet).await
}

pub async fn query_consensus_state(
    canister_id: &str,
    ic_endpoint_url: &str,
    msg: Vec<u8>,
) -> Result<Vec<u8>> {
    query_ic(canister_id, "query_consensus_state", msg, is_mainnet).await
}
