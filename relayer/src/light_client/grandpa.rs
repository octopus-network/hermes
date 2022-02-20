use crate::chain::SubstrateChain;
use crate::error::Error;
use octopusxt::ibc_node;
use std::future::Future;
use std::sync::Arc;
use subxt::{BlockNumber, ClientBuilder};
use tokio::runtime::Runtime as TokioRuntime;

use crate::config::{ChainConfig, Strategy};
use crate::light_client::Verified;
use ibc::ics02_client::client_state::AnyClientState;
use ibc::ics02_client::events::UpdateClient;
use ibc::ics02_client::header::AnyHeader;
use ibc::ics02_client::header::Header;
use ibc::ics02_client::misbehaviour::{AnyMisbehaviour, MisbehaviourEvidence};
use ibc::ics10_grandpa::header::Header as GPHeader;
use ibc::ics10_grandpa::help::{BlockHeader, Commitment, SignedCommitment};
use ibc::ics24_host::identifier::ChainId;
use ibc::ics24_host::identifier::ClientId;
use ibc::Height;

pub struct LightClient {
    chain_id: ChainId,
    websocket_url: String,
    rt: Arc<TokioRuntime>,
    beefy_light_client: beefy_light_client::LightClient,
}

impl LightClient {
    pub fn from_config(
        config: &ChainConfig,
        websocket_url: String,
        rt: Arc<TokioRuntime>,
        initial_public_keys: Vec<String>,
    ) -> Self {
        let chain_id = config.id.clone();
        let beefy_light_client = beefy_light_client::new(initial_public_keys);
        Self {
            chain_id,
            websocket_url,
            rt,
            beefy_light_client,
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
        client_state: &AnyClientState,
    ) -> Result<Verified<GPHeader>, Error> {
        tracing::info!("In grandpa: [header_and_minimal_set]");
        use ibc::ics10_grandpa::help::Commitment;

        Ok(Verified {
            // target: GPHeader::new(target.revision_height),
            target: GPHeader {
                block_header: BlockHeader {
                    parent_hash: vec![],
                    block_number: target.revision_height as u32,
                    state_root: vec![],
                    extrinsics_root: vec![],
                    digest: vec![],
                },
                mmr_leaf: Default::default(),
                mmr_leaf_proof: Default::default(),
            },
            supporting: vec![GPHeader {
                block_header: BlockHeader {
                    parent_hash: vec![],
                    block_number: trusted.revision_height as u32,
                    state_root: vec![],
                    extrinsics_root: vec![],
                    digest: vec![],
                },
                mmr_leaf: Default::default(),
                mmr_leaf_proof: Default::default(),
            }],
        })
    }

    fn verify(
        &mut self,
        trusted: Height,
        target: Height,
        client_state: &AnyClientState,
    ) -> Result<Verified<GPHeader>, Error> {
        tracing::info!("In grandpa: [verify]");

        let block_header = async {
            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();

            // get block header
            let block_header = octopusxt::call_ibc::get_header_by_block_number(
                client.clone(),
                Some(BlockNumber::from(target.revision_height as u32)),
            )
            .await
            .unwrap();

            block_header
        };

        let result = self.block_on(block_header);

        Ok(Verified {
            target: GPHeader {
                block_header: result,
                mmr_leaf: Default::default(),
                mmr_leaf_proof: Default::default(),
            },
            supporting: Vec::new(),
        })
    }

    fn check_misbehaviour(
        &mut self,
        update: UpdateClient,
        client_state: &AnyClientState,
    ) -> Result<Option<MisbehaviourEvidence>, Error> {
        tracing::info!("in grandpa: [check_misbehaviour]");
        // Uncomment below will return a good misbehaviour which will be sent to Cosmos chain and freeze the Grandpa client
        /*        let anyheader = update.header.unwrap();

        Ok(Some(MisbehaviourEvidence{
            misbehaviour: AnyMisbehaviour::Grandpa(ibc::ics10_grandpa::misbehaviour::Misbehaviour{
                client_id: update.common.client_id,
                header1: GPHeader::new(anyheader.height().revision_height),
                header2: GPHeader::new(anyheader.height().revision_height),
            }),
            supporting_headers: vec![anyheader],
        }))*/
        Ok(None) // Todo: May need to implement the same logic of check_misbehaviour in tendermint.rs
    }

    fn fetch(&mut self, height: Height) -> Result<GPHeader, Error> {
        tracing::info!("in grandpa: [fetch]");

        Ok(GPHeader::default())
    }
}
