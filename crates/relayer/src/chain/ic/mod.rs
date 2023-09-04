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

#[derive(Debug)]
pub struct VpClient {
    pub agent: Agent,
}

impl VpClient {
    const LOCAL_NET: &str = "http://localhost:4943";

    pub async fn new(ic_endpoint_url: &str, pem_file: &PathBuf) -> Result<Self> {
        let agent = Agent::builder()
            .with_url(ic_endpoint_url)
            .with_identity(create_identity(pem_file)?)
            .build()
            .map_err(Error::AgentError)?;

        if ic_endpoint_url == Self::LOCAL_NET {
            agent.fetch_root_key().await?;
        }

        Ok(VpClient { agent })
    }

    async fn query_ic(
        &self,
        canister_id: &str,
        method_name: &str,
        args: Vec<u8>,
    ) -> Result<Vec<u8>> {
        let canister_id = Principal::from_text(canister_id)?;

        let response = QueryBuilder::new(&self.agent, canister_id, method_name.into())
            .with_arg(Encode!(&args)?)
            .call()
            .await?;

        Decode!(response.as_slice(), VecResult)?.transder_anyhow()
    }

    async fn update_ic(
        &self,
        canister_id: &str,
        method_name: &str,
        args: Vec<u8>,
    ) -> Result<Vec<u8>> {
        let canister_id = Principal::from_text(canister_id)?;

        let response = UpdateBuilder::new(&self.agent, canister_id, method_name.into())
            .with_arg(Encode!(&args)?)
            .call_and_wait()
            .await?;

        Decode!(response.as_slice(), VecResult)?.transder_anyhow()
    }

    pub async fn query_client_state(&self, canister_id: &str, msg: Vec<u8>) -> Result<Vec<u8>> {
        self.query_ic(canister_id, "query_client_state", msg).await
    }

    pub async fn query_consensus_state(&self, canister_id: &str, msg: Vec<u8>) -> Result<Vec<u8>> {
        self.query_ic(canister_id, "query_consensus_state", msg)
            .await
    }

    pub async fn deliver(&self, canister_id: &str, msg: Vec<u8>) -> Result<Vec<u8>> {
        self.update_ic(canister_id, "deliver", msg).await
    }
}

use core::ops::Deref;

impl Deref for VpClient {
    type Target = Agent;
    fn deref(&self) -> &Agent {
        &self.agent
    }
}
