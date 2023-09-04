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
    let result = Decode!(response.as_slice(), VecResult)?;

    match result {
        VecResult::Ok(value) => Ok(value),
        VecResult::Err(e) => Err(anyhow::anyhow!(e)),
    }
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

    // let waiter = garcon::Delay::builder()
    //     .throttle(std::time::Duration::from_millis(500))
    //     .timeout(std::time::Duration::from_secs(60 * 5))
    //     .build();

    let response = update_builder_with_args.call_and_wait().await?;
    let result = Decode!(response.as_slice(), VecResult)?;

    match result {
        VecResult::Ok(value) => Ok(value),
        VecResult::Err(e) => Err(anyhow::anyhow!(e)),
    }
}

pub async fn deliver(
    canister_id: &str,
    ic_endpoint_url: &str,
    msg: Vec<u8>,
    pem_file: &PathBuf,
) -> Result<Vec<u8>> {
    let method_name = "deliver";
    let args = msg;
    update_ic(canister_id, method_name, args, ic_endpoint_url, pem_file).await
}

pub async fn query_client_state(
    canister_id: &str,
    ic_endpoint_url: &str,
    msg: Vec<u8>,
) -> Result<Vec<u8>> {
    let method_name = "query_client_state";
    let args = msg;
    query_ic(canister_id, method_name, args, ic_endpoint_url).await
}

pub async fn query_consensus_state(
    canister_id: &str,
    ic_endpoint_url: &str,
    msg: Vec<u8>,
) -> Result<Vec<u8>> {
    let method_name = "query_consensus_state";
    let args = msg;
    query_ic(canister_id, method_name, args, ic_endpoint_url).await
}
