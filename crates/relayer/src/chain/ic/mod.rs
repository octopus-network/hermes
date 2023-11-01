pub mod errors;
mod identity;
pub mod types;

use crate::chain::ic::errors::VpError;
use crate::chain::ic::identity::create_identity;
use crate::chain::ic::types::VecResult;
use candid::Principal;
use candid::{Decode, Encode};
use core::ops::Deref;
use ic_agent::agent::{QueryBuilder, UpdateBuilder};
use ic_agent::Agent;
use std::path::PathBuf;

#[derive(Debug)]
pub struct VpClient {
    pub agent: Agent,
}

impl VpClient {
    const LOCAL_NET: &'static str = "http://localhost:4943";
    #[allow(dead_code)]
    const MAIN_NET: &'static str = "https://ic0.app";

    pub async fn new(ic_endpoint_url: &str, pem_file: &PathBuf) -> Result<Self, VpError> {
        let agent = Agent::builder()
            .with_url(ic_endpoint_url)
            .with_identity(create_identity(pem_file).map_err(VpError::create_identity_error)?)
            .build()
            .map_err(VpError::agent_error)?;

        if ic_endpoint_url == Self::LOCAL_NET {
            agent.fetch_root_key().await.map_err(VpError::agent_error)?;
        }

        Ok(VpClient { agent })
    }

    async fn query_ic(
        &self,
        canister_id: &str,
        method_name: &str,
        args: Vec<u8>,
    ) -> Result<Vec<u8>, VpError> {
        let canister_id = Principal::from_text(canister_id).map_err(VpError::principal_error)?;

        let response = QueryBuilder::new(&self.agent, canister_id, method_name.into())
            .with_arg(Encode!(&args).map_err(VpError::decode_ic_type_error)?)
            .call()
            .await
            .map_err(|e| {
                tracing::error!("query_ic: {:?}", e);
                VpError::agent_error(e)
            })?;

        if Decode!(response.as_slice(), VecResult).is_err() {
            tracing::error!("query_ic: {:?}", response);
            if let Ok(value) = Decode!(response.as_slice(), String) {
                Ok(value.into_bytes())
            } else {
                Err(VpError::custom_error("decode to string error".to_string()))
            }
        } else {
            Decode!(response.as_slice(), VecResult)
                .map_err(|e| {
                    tracing::error!("update_ic: {:?}", e);
                    VpError::decode_ic_type_error(e)
                })?
                .transfer_anyhow()
        }
    }

    async fn update_ic(
        &self,
        canister_id: &str,
        method_name: &str,
        args: Vec<u8>,
    ) -> Result<Vec<u8>, VpError> {
        let canister_id = Principal::from_text(canister_id).map_err(VpError::principal_error)?;

        let response = UpdateBuilder::new(&self.agent, canister_id, method_name.into())
            .with_arg(Encode!(&args).map_err(VpError::decode_ic_type_error)?)
            .call_and_wait()
            .await
            .map_err(|e| {
                tracing::error!("query_ic: {:?}", e);
                VpError::agent_error(e)
            })?;

        if Decode!(response.as_slice(), VecResult).is_err() {
            tracing::error!("query_ic: {:?}", response);
            if let Ok(value) = Decode!(response.as_slice(), String) {
                Ok(value.into_bytes())
            } else {
                Err(VpError::custom_error("decode to string error".to_string()))
            }
        } else {
            Decode!(response.as_slice(), VecResult)
                .map_err(|e| {
                    tracing::error!("update_ic: {:?}", e);
                    VpError::decode_ic_type_error(e)
                })?
                .transfer_anyhow()
        }
    }

    pub async fn query_client_state(
        &self,
        canister_id: &str,
        msg: Vec<u8>,
    ) -> Result<Vec<u8>, VpError> {
        self.query_ic(canister_id, "query_client_state", msg).await
    }

    pub async fn query_consensus_state(
        &self,
        canister_id: &str,
        msg: Vec<u8>,
    ) -> Result<Vec<u8>, VpError> {
        self.query_ic(canister_id, "query_consensus_state", msg)
            .await
    }

    pub async fn deliver(&self, canister_id: &str, msg: Vec<u8>) -> Result<Vec<u8>, VpError> {
        self.update_ic(canister_id, "deliver", msg).await
    }
}

impl Deref for VpClient {
    type Target = Agent;
    fn deref(&self) -> &Agent {
        &self.agent
    }
}
