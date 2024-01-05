pub mod rpc;

use std::{sync::Arc, time::Duration};

use crate::chain::near::rpc::client::NearRpcClient;
use crate::event::source::TxEventSourceCmd;

use near_primitives::types::AccountId;
use tokio::runtime::Runtime as TokioRuntime;

use ibc_relayer_types::core::ics24_host::identifier::ChainId;

pub use super::error::{Error, ErrorDetail};

pub type Result<T> = core::result::Result<T, Error>;

pub enum EventSource {
    Rpc(rpc::EventSource),
}

impl EventSource {
    pub fn rpc(
        chain_id: ChainId,
        near_ibc_address: AccountId,
        rpc_client: NearRpcClient,
        poll_interval: Duration,
        rt: Arc<TokioRuntime>,
    ) -> Result<(Self, TxEventSourceCmd)> {
        let (source, tx) =
            rpc::EventSource::new(chain_id, near_ibc_address, rpc_client, poll_interval, rt)?;
        Ok((Self::Rpc(source), tx))
    }

    pub fn run(self) {
        match self {
            Self::Rpc(source) => source.run(),
        }
    }
}
