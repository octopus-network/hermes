use crate::chain::SubstrateChain;
use crate::error::Error;

use crate::light_client::Verified;
use ibc::ics02_client::client_state::AnyClientState;
use ibc::ics02_client::events::UpdateClient;
use ibc::ics02_client::header::Header;
use ibc::ics02_client::misbehaviour::{MisbehaviourEvidence, AnyMisbehaviour};
use ibc::ics10_grandpa::header::Header as GPHeader;
use ibc::Height;
use ibc::ics02_client::header::AnyHeader;
use ibc::ics24_host::identifier::ClientId;

pub struct LightClient {}

impl LightClient {
    pub fn new() -> Self {
        Self {}
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

        Ok(Verified{
            // target: GPHeader::new(target.revision_height),
            target: GPHeader::default(),
            supporting: Vec::new(),
        })
    }

    fn verify(
        &mut self,
        trusted: Height,
        target: Height,
        client_state: &AnyClientState,
    ) -> Result<Verified<()>, Error> {
        tracing::info!("In grandpa: [verify]");

        Ok(Verified {
            target: (),
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
        Ok(None)  // Todo: May need to implement the same logic of check_misbehaviour in tendermint.rs
    }

    fn fetch(&mut self, height: Height) -> Result<(), Error> {
        tracing::info!("in grandpa: [fetch]");

        Ok(())
    }
}
