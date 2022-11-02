use crate::chain::substrate::SubstrateChain;
use crate::error::Error;

use std::future::Future;
use std::sync::Arc;
use tokio::runtime::Runtime as TokioRuntime;

use crate::chain::substrate::rpc::get_header_by_block_number;
use crate::config::ChainConfig;
use crate::light_client::AnyClientState;
use crate::light_client::Verified;
use crate::misbehaviour::MisbehaviourEvidence;
use ibc_relayer_types::clients::ics10_grandpa::header::Header as GPHeader;
use ibc_relayer_types::clients::ics10_grandpa::help::{BlockHeader, MmrRoot};
use ibc_relayer_types::core::ics02_client::events::UpdateClient;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::Height;
use subxt::rpc::BlockNumber;
use subxt::OnlineClient;
use tendermint::time::Time;

pub struct LightClient {
    chain_id: ChainId,
    websocket_url: String,
    rt: Arc<TokioRuntime>,
}

impl LightClient {
    pub fn from_config(
        config: &ChainConfig,
        websocket_url: String,
        rt: Arc<TokioRuntime>,
        _initial_public_keys: Vec<String>,
    ) -> Self {
        let chain_id = config.id.clone();
        // let beefy_light_client = beefy_light_client::new(initial_public_keys);
        Self {
            chain_id,
            websocket_url,
            rt,
            // beefy_light_client,
        }
    }

    /// Run a future to completion on the Tokio runtime.
    pub fn block_on<F: Future>(&self, f: F) -> F::Output {
        crate::time!("block_on");
        self.rt.block_on(f)
    }
}

impl super::LightClient<SubstrateChain> for LightClient {
    fn header_and_minimal_set(
        &mut self,
        trusted: Height,
        target: Height,
        _client_state: &AnyClientState,
    ) -> Result<Verified<GPHeader>, Error> {
        tracing::info!("In grandpa: [header_and_minimal_set]");

        Ok(Verified {
            target: GPHeader {
                block_header: BlockHeader {
                    parent_hash: vec![],
                    block_number: target.revision_height() as u32,
                    state_root: vec![],
                    extrinsics_root: vec![],
                    digest: vec![],
                },
                mmr_root: MmrRoot::default(),
                timestamp: Time::from_unix_timestamp(0, 0).unwrap(),
            },
            supporting: vec![GPHeader {
                block_header: BlockHeader {
                    parent_hash: vec![],
                    block_number: trusted.revision_height() as u32,
                    state_root: vec![],
                    extrinsics_root: vec![],
                    digest: vec![],
                },
                mmr_root: MmrRoot::default(),
                timestamp: Time::from_unix_timestamp(0, 0).unwrap(),
            }],
        })
    }

    fn verify(
        &mut self,
        _trusted: Height,
        target: Height,
        _client_state: &AnyClientState,
    ) -> Result<Verified<GPHeader>, Error> {
        tracing::info!("In grandpa: [verify]");

        let block_header = async {
            let client = OnlineClient::from_url(&self.websocket_url.clone())
                .await
                .map_err(|_| Error::report_error("substrate client builder error".to_string()))?;

            // get block header
            let block_header = get_header_by_block_number(
                Some(BlockNumber::from(target.revision_height() as u32)),
                client.clone(),
            )
            .await
            .map_err(|_| Error::report_error("get header by block number error".to_string()))?;

            Ok(block_header)
        };

        let result = self.block_on(block_header)?;
        //TODO: get mmr root and timestamp

        Ok(Verified {
            target: GPHeader {
                block_header: result,
                mmr_root: MmrRoot::default(),
                timestamp: Time::from_unix_timestamp(0, 0).unwrap(),
            },
            supporting: Vec::new(),
        })
    }

    fn check_misbehaviour(
        &mut self,
        _update: &UpdateClient,
        _client_state: &AnyClientState,
    ) -> Result<Option<MisbehaviourEvidence>, Error> {
        tracing::info!("in grandpa: [check_misbehaviour]");

        Ok(None) // Todo: May need to implement the same logic of check_misbehaviour in tendermint.rs
    }

    fn fetch(&mut self, _height: Height) -> Result<GPHeader, Error> {
        tracing::info!("in grandpa: [fetch]");

        Ok(GPHeader::default())
    }
}
