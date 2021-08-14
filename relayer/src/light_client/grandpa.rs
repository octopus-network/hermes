use crate::chain::SubstrateChain;
use crate::error::Error;

use crate::light_client::Verified;
use ibc::ics02_client::client_state::AnyClientState;
use ibc::ics02_client::events::UpdateClient;
use ibc::ics02_client::misbehaviour::MisbehaviourEvidence;
use ibc::ics10_grandpa::header::Header as GPHeader;
use ibc::Height;

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
        tracing::info!("in header and minimal set");

        todo!()
    }

    fn verify(
        &mut self,
        trusted: Height,
        target: Height,
        client_state: &AnyClientState,
    ) -> Result<Verified<()>, Error> {
        tracing::info!("in verify");

        todo!()
    }

    fn check_misbehaviour(
        &mut self,
        update: UpdateClient,
        client_state: &AnyClientState,
    ) -> Result<Option<MisbehaviourEvidence>, Error> {
        tracing::info!("in check misbehaviour");

        todo!()
    }

    fn fetch(&mut self, height: Height) -> Result<(), Error> {
        tracing::info!("in fetch");

        todo!()
    }
}
