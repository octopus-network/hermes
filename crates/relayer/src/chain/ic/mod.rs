pub mod errors;
mod identity;
mod types;

use crate::chain::ic::errors::Error;
use crate::chain::ic::identity::create_identity;
use crate::chain::ic::types::*;
use anyhow::Result;
use candid::Principal;
use candid::{Decode, Encode};
use ic_agent::agent::{QueryBuilder, UpdateBuilder};
use ic_agent::Agent;
use std::path::PathBuf;

async fn query_ic(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    ic_endpoint_url: &str,
) -> Result<Vec<u8>> {
    let agent = Agent::builder()
        .with_url(ic_endpoint_url)
        .build()
        .map_err(Error::AgentError)?;

    if ic_endpoint_url == LOCAL_NET {
        agent.fetch_root_key().await?;
    }

    let canister_id = Principal::from_text(canister_id)?;

    let response = QueryBuilder::new(&agent, canister_id, method_name.into())
        .with_arg(Encode!(&args)?)
        .call()
        .await?;

    Decode!(response.as_slice(), VecResult)?.transder_anyhow()
}

async fn update_ic(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    ic_endpoint_url: &str,
    pem_file: &PathBuf,
) -> Result<Vec<u8>> {
    let agent = Agent::builder()
        .with_url(ic_endpoint_url)
        .with_identity(create_identity(pem_file)?)
        .build()
        .map_err(Error::AgentError)?;

    if ic_endpoint_url == LOCAL_NET {
        agent.fetch_root_key().await?;
    }

    let canister_id = Principal::from_text(canister_id)?;

    let response = UpdateBuilder::new(&agent, canister_id, method_name.into())
        .with_arg(Encode!(&args)?)
        .call_and_wait()
        .await?;

    Decode!(response.as_slice(), VecResult)?.transder_anyhow()
}

pub async fn deliver(
    canister_id: &str,
    ic_endpoint_url: &str,
    msg: Vec<u8>,
    pem_file: &PathBuf,
) -> Result<Vec<u8>> {
    update_ic(canister_id, "deliver", msg, ic_endpoint_url, pem_file).await
}

pub async fn query_client_state(
    canister_id: &str,
    ic_endpoint_url: &str,
    msg: Vec<u8>,
) -> Result<Vec<u8>> {
    query_ic(canister_id, "query_client_state", msg, ic_endpoint_url).await
}

pub async fn query_consensus_state(
    canister_id: &str,
    ic_endpoint_url: &str,
    msg: Vec<u8>,
) -> Result<Vec<u8>> {
    query_ic(canister_id, "query_consensus_state", msg, ic_endpoint_url).await
}
