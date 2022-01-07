use crate::chain::SubstrateChain;
use crate::error::Error;

use crate::config::{ChainConfig, Strategy};
use crate::light_client::Verified;
use ibc::ics02_client::client_state::AnyClientState;
use ibc::ics02_client::events::UpdateClient;
use ibc::ics02_client::header::AnyHeader;
use ibc::ics02_client::header::Header;
use ibc::ics02_client::misbehaviour::{AnyMisbehaviour, MisbehaviourEvidence};
use ibc::ics10_grandpa::header::Header as GPHeader;
use ibc::ics24_host::identifier::ChainId;
use ibc::ics24_host::identifier::ClientId;
use ibc::Height;
use ibc::ics10_grandpa::help::{Commitment, SignedCommitment};

pub struct LightClient {
    chain_id: ChainId,
    beefy_light_client: beefy_light_client::LightClient,
}

impl LightClient {
    pub fn from_config(config: &ChainConfig, initial_public_keys: Vec<String>) -> Self {
        let chain_id = config.id.clone();
        let beefy_light_client = beefy_light_client::new(initial_public_keys);
        Self {
            chain_id,
            beefy_light_client,
        }
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
                signed_commitment: SignedCommitment { commitment: Some(Commitment {
                    block_number: target.revision_height as u32,
                    payload: vec![],
                    validator_set_id: 0
                }) , signatures: vec![] },
                validator_merkle_proof: Default::default(),
                mmr_leaf: Default::default(),
                mmr_leaf_proof: Default::default()
            },
            supporting: vec![GPHeader {
                signed_commitment: SignedCommitment { commitment: Some(Commitment {
                    block_number: trusted.revision_height as u32,
                    payload: vec![],
                    validator_set_id: 0
                }) , signatures: vec![] },
                validator_merkle_proof: Default::default(),
                mmr_leaf: Default::default(),
                mmr_leaf_proof: Default::default()
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

        Ok(Verified {
            target: GPHeader::default(),
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
