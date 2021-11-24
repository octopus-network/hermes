use crate::chain::SubstrateChain;
use crate::error::Error;

use crate::light_client::Verified;
use ibc::core::ics02_client::client_state::AnyClientState;
use ibc::core::ics02_client::events::UpdateClient;
use ibc::core::ics02_client::misbehaviour::{MisbehaviourEvidence, AnyMisbehaviour};
use ibc::clients::ics10_grandpa::header::{Header as GPHeader, Header};
use ibc::Height;
use ibc::core::ics02_client::header::AnyHeader;
use ibc::core::ics24_host::identifier::ClientId;

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
            target: GPHeader::new(target.revision_height),
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
        let anyheader = update.header.unwrap();

        Ok(Some(MisbehaviourEvidence{
            misbehaviour: AnyMisbehaviour::Grandpa(ibc::clients::ics10_grandpa::misbehaviour::Misbehaviour {
                client_id: ClientId::default(),
                header1: Header::new(0),
                header2: Header::new(0),
            }),
            supporting_headers: vec![anyheader],
        }))
    }

    fn fetch(&mut self, height: Height) -> Result<(), Error> {
        tracing::info!("in grandpa: [fetch]");

        Ok(())
    }
}
