use alloc::sync::Arc;
use core::cmp::Ordering;
use tendermint::Time;

use crossbeam_channel as channel;
use flex_error::{define_error, TraceError};
use futures::{
    pin_mut,
    stream::{self, select_all, StreamExt},
    Stream, TryStreamExt,
};
use tokio::task::JoinHandle;
use tokio::{runtime::Runtime as TokioRuntime, sync::mpsc};
use tracing::{debug, error, info, trace};

use octopusxt::ibc_node::RuntimeApi;
use octopusxt::MyConfig;
use octopusxt::SubstrateNodeTemplateExtrinsicParams;
use subxt::{
    BlockNumber, Client, ClientBuilder, Error as SubstrateError, PairSigner, SignedCommitment,
};
use tendermint_rpc::{event::Event as RpcEvent, Url};

use crate::error::Error as RelayError;
use crate::util::{
    retry::{retry_count, retry_with_index, RetryResult},
    stream::try_group_while,
};
use beefy_light_client::commitment;
use codec::{Decode, Encode};
use ibc::clients::ics10_grandpa::help::{self, MmrRoot};
use ibc::clients::ics10_grandpa::{header::Header, help::ValidatorMerkleProof};
use ibc::core::ics02_client::height::Height;
use ibc::core::ics24_host::identifier::ChainId;
use ibc::events::IbcEvent;
use sp_core::{hexdisplay::HexDisplay, H256};

use std::future::Future;
use tokio::runtime::Runtime;

mod retry_strategy {
    use crate::util::retry::clamp_total;
    use core::time::Duration;
    use retry::delay::Fibonacci;

    // Default parameters for the retrying mechanism
    const MAX_DELAY: Duration = Duration::from_secs(60); // 1 minute
    const MAX_TOTAL_DELAY: Duration = Duration::from_secs(10 * 60); // 10 minutes
    const INITIAL_DELAY: Duration = Duration::from_secs(1); // 1 second

    pub fn default() -> impl Iterator<Item = Duration> {
        clamp_total(Fibonacci::from(INITIAL_DELAY), MAX_DELAY, MAX_TOTAL_DELAY)
    }
}

pub use super::monitor::Error;
pub type BeefyResult<T> = Result<T, Error>;
pub type BeefySender = channel::Sender<BeefyResult<Header>>;
pub type BeefyReceiver = channel::Receiver<BeefyResult<Header>>;

pub use super::monitor::MonitorCmd;
pub use super::monitor::TxMonitorCmd;

#[derive(Debug)]
pub enum BeefyMonitorCtrl {
    None {
        /// Empty channel for when the None case
        never: BeefyReceiver,
    },
    Live {
        beefy_receiver: BeefyReceiver,
        /// Sender channel to terminate the event monitor
        tx_monitor_cmd: TxMonitorCmd,
    },
}

impl BeefyMonitorCtrl {
    pub fn none() -> Self {
        Self::None {
            never: channel::never(),
        }
    }

    pub fn live(beefy_receiver: BeefyReceiver, tx_monitor_cmd: TxMonitorCmd) -> Self {
        Self::Live {
            beefy_receiver,
            tx_monitor_cmd,
        }
    }

    pub fn enable(&mut self, beefy_receiver: BeefyReceiver, tx_monitor_cmd: TxMonitorCmd) {
        *self = Self::live(beefy_receiver, tx_monitor_cmd);
    }

    pub fn recv(&self) -> &BeefyReceiver {
        match self {
            Self::None { ref never } => never,
            Self::Live {
                ref beefy_receiver, ..
            } => beefy_receiver,
        }
    }

    pub fn shutdown(&self) -> Result<(), RelayError> {
        match self {
            Self::None { .. } => Ok(()),
            Self::Live {
                ref tx_monitor_cmd, ..
            } => tx_monitor_cmd
                .send(MonitorCmd::Shutdown)
                .map_err(RelayError::send),
        }
    }

    pub fn is_live(&self) -> bool {
        matches!(self, Self::Live { .. })
    }
}

/// Connect to a substrate node, subscribe to beefy info,
/// receive push signed commitment over a websocket, and build the mmr root from signed commitment
pub struct BeefyMonitor {
    chain_id: ChainId,
    /// WebSocket to collect events from
    client: Client<MyConfig>,
    /// Channel to handler where the monitor for this chain sends the events
    tx_beefy: channel::Sender<BeefyResult<Header>>,
    /// Channel where to receive client driver errors
    rx_err: mpsc::UnboundedReceiver<tendermint_rpc::Error>,
    /// Channel where to send client driver errors
    tx_err: mpsc::UnboundedSender<tendermint_rpc::Error>,
    /// Channel where to receive commands
    rx_cmd: channel::Receiver<MonitorCmd>,
    /// Node Address
    node_addr: Url,
    /// beefy subscription
    subscription: Option<SignedCommitment>,
    /// Tokio runtime
    rt: Arc<TokioRuntime>,
}

impl BeefyMonitor {
    /// Create an event monitor, and connect to a node
    pub fn new(
        chain_id: ChainId,
        client: Client<MyConfig>,
        node_addr: Url,
        rt: Arc<TokioRuntime>,
    ) -> BeefyResult<(Self, BeefyReceiver, TxMonitorCmd)> {
        let (tx_beefy, rx_beefy) = channel::unbounded();
        let (tx_cmd, rx_cmd) = channel::unbounded();
        let (tx_err, rx_err) = mpsc::unbounded_channel();

        let monitor = Self {
            rt,
            chain_id,
            client,
            tx_beefy,
            rx_err,
            tx_err,
            rx_cmd,
            node_addr,
            subscription: None,
        };

        Ok((monitor, rx_beefy, tx_cmd))
    }

    ///subscribe beefy
    pub fn subscribe(&mut self) -> BeefyResult<()> {
        info!("in beefy_mointor: [subscribe] ");
        let sub = self.rt.block_on(self.subscribe_beefy()).unwrap();

        self.subscription = Some(sub);

        Ok(())
    }

    fn try_reconnect(&mut self) -> BeefyResult<()> {
        info!(
            "[{}] trying to reconnect to WebSocket endpoint {}",
            self.chain_id, self.node_addr
        );

        // Try to reconnect
        let mut client = self
            .rt
            .block_on(
                ClientBuilder::new()
                    .set_url(format!("{}", &self.node_addr.clone()))
                    .build::<MyConfig>(),
            )
            .map_err(|_| {
                Error::client_creation_failed(self.chain_id.clone(), self.node_addr.clone())
            })?;

        // Swap the new client with the previous one which failed,
        // so that we can shut the latter down gracefully.
        core::mem::swap(&mut self.client, &mut client);

        trace!(
            "[{}] reconnected to WebSocket endpoint {}",
            self.chain_id,
            self.node_addr
        );

        // Shut down previous client
        trace!(
            "[{}] gracefully shutting down previous client",
            self.chain_id
        );

        trace!("[{}] previous client successfully shutdown", self.chain_id);

        Ok(())
    }

    /// Try to resubscribe to events
    fn try_resubscribe(&mut self) -> BeefyResult<()> {
        info!("[{}] trying to resubscribe to beefy", self.chain_id);
        self.subscribe()
    }

    /// Attempt to reconnect the WebSocket client using the given retry strategy.
    ///
    /// See the [`retry`](https://docs.rs/retry) crate and the
    /// [`crate::util::retry`] module for more information.
    fn reconnect(&mut self) {
        let result = retry_with_index(retry_strategy::default(), |_| {
            // Try to reconnect
            if let Err(e) = self.try_reconnect() {
                trace!("[{}] error when reconnecting: {}", self.chain_id, e);
                return RetryResult::Retry(());
            }

            // Try to resubscribe
            if let Err(e) = self.try_resubscribe() {
                trace!("[{}] error when resubscribing: {}", self.chain_id, e);
                return RetryResult::Retry(());
            }

            RetryResult::Ok(())
        });

        match result {
            Ok(()) => info!(
                "[{}] successfully reconnected to WebSocket endpoint {}",
                self.chain_id, self.node_addr
            ),
            Err(retries) => error!(
                "[{}] failed to reconnect to {} after {} retries",
                self.chain_id,
                self.node_addr,
                retry_count(&retries)
            ),
        }
    }

    /// beefy monitor loop
    #[allow(clippy::while_let_loop)]
    pub fn run(self) {
        debug!("[{}] starting beefy monitor", self.chain_id);
        trace!(
            "in beefy_monitor: [run], [{}] starting beefy monitor ",
            self.chain_id
        );

        let api = self
            .client
            .clone()
            .to_runtime_api::<RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>(
            );

        let sub = self
            .rt
            .block_on(api.client.rpc().subscribe_beefy_justifications())
            .unwrap();

        let mut beefy_sub = sub;

        trace!("in beefy_monitor: [run], beefy subscript success ! ");

        // let mut beefy_sub = self.rt.block_on(self.subscribe_beefy());
        // Work around double borrow
        let rt = self.rt.clone();

        // msg loop for handle the beefy SignedCommitment
        loop {
            let raw_sc = self.rt.block_on(beefy_sub.next()).unwrap().unwrap();
            trace!(
                "in beefy_monitor: [run], from {:?} received beefy signed commitment : {:?} ",
                self.chain_id.as_str(),
                raw_sc
            );

            let _ = self.process_beefy_msg(raw_sc);
        }
    }

    fn run_loop(&mut self) -> Next {
        info!("in beefy_mointor: [run_loop]");
        Next::Continue
    }

    /// Propagate error to subscribers.
    ///
    /// The main use case for propagating RPC errors is for the [`Supervisor`]
    /// to notice that the WebSocket connection or subscription has been closed,
    /// and to trigger a clearing of packets, as this typically means that we have
    /// missed a bunch of events which were emitted after the subscrption was closed.
    /// In that case, this error will be handled in [`Supervisor::handle_batch`].
    fn propagate_error(&self, error: Error) -> BeefyResult<()> {
        info!("in beefy_mointor: [propagate_error]");
        self.tx_beefy
            .send(Err(error))
            .map_err(|_| Error::channel_send_failed())?;

        Ok(())
    }

    /// Collect the IBC events from the subscriptions
    fn process_beefy_msg(&self, raw_sc: SignedCommitment) -> BeefyResult<()> {
        trace!("in beefy_mointor: [process_beefy_msg]");

        let header = self.rt.block_on(self.build_header(raw_sc));
        trace!(
            "in beefy_monitor: [process_beefy_msg], build header: {:?} ",
            header
        );

        if let Ok(h) = header {
            // send to msg queue
            trace!(
                "in beefy_monitor: [process_beefy_msg], send mmr root : {:?} ",
                h
            );

            self.tx_beefy
                .send(Ok(h))
                .map_err(|_| Error::channel_send_failed())?;
        }

        Ok(())
    }
    /// Subscribe beefy msg
    pub async fn subscribe_beefy(&self) -> Result<SignedCommitment, Box<dyn std::error::Error>> {
        info!("in beefy monitor: [subscribe_beefy]");
        let api = self
            .client
            .clone()
            .to_runtime_api::<RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>(
            );

        let mut sub = api.client.rpc().subscribe_beefy_justifications().await?;

        let raw = sub.next().await.unwrap().unwrap();

        Ok(raw)
    }

    pub async fn build_header(&self, raw_sc: SignedCommitment) -> Result<Header, RelayError> {
        trace!("in beefy monitor: [build_header]");

        // decode signed commitment
        let signed_commmitment: commitment::SignedCommitment =
            Decode::decode(&mut &raw_sc.0[..]).unwrap();
        trace!(
                "in beefy monitor: [build_header] decode signed commitment : {:?},", signed_commmitment);
        // get commitment
        let commitment::Commitment {
            payload,
            block_number,
            validator_set_id,
        } = signed_commmitment.commitment.clone();
        trace!(
            "in beefy monitor: [build_header] new mmr root block_number : {:?},", block_number);
        // build validator proof
        let validator_merkle_proofs: Vec<ValidatorMerkleProof> =
            octopusxt::update_client_state::build_validator_proof(
                self.client.clone(),
                block_number,
            )
            .await
            .map_err(|_| RelayError::get_validator_merkle_proof())?;

        // build mmr proof
        let api = self
            .client
            .clone()
            .to_runtime_api::<RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>(
            );

        // get block hash
        let block_hash: Option<H256> = api
            .client
            .rpc()
            .block_hash(Some(BlockNumber::from(block_number)))
            .await
            .map_err(|_| RelayError::get_block_hash_error())?;

        trace!(
            "in beefy monitor: [build_header] block_number:{:?} >> block_hash{:?}",block_number,
            block_hash
        );

        // create proof
        let mmr_leaf_and_mmr_leaf_proof = octopusxt::ibc_rpc::get_mmr_leaf_and_mmr_proof(
            Some(BlockNumber::from(block_number - 1)),
            block_hash,
            self.client.clone(),
        )
        .await
        .map_err(|_| RelayError::get_mmr_leaf_and_mmr_proof_error())?;
        trace!(
            "in beefy monitor: [build_header] get_mmr_leaf_and_mmr_proof block_hash{:?}",
            mmr_leaf_and_mmr_leaf_proof.0
        );

        // build new mmr root
        let mmr_root = MmrRoot {
            signed_commitment: signed_commmitment.into(),
            validator_merkle_proofs: validator_merkle_proofs,
            mmr_leaf: mmr_leaf_and_mmr_leaf_proof.1,
            mmr_leaf_proof: mmr_leaf_and_mmr_leaf_proof.2,
        };

        let api = self
            .client
            .clone()
            .to_runtime_api::<RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>(
            );

        // get block header
        let block_header = octopusxt::ibc_rpc::get_header_by_block_number(
            Some(BlockNumber::from(block_number)),
            self.client.clone(),
        )
        .await
        .map_err(|_| RelayError::get_header_by_block_number_error())?;

        trace!(
            "in beefy monitor: [build_header] >> block_header = {:?}",
            block_header
        );

        //build timestamp
        let timestamp = Time::from_unix_timestamp(0, 0).unwrap();

        trace!(
            "in beefy monitor: [build_header] >> timestamp = {:?}",
            timestamp
        );

        //TODO: test verify mmr root and verify header
        trace!("in beefy monitor: [build_header] ---------------------test[begin]-----------------------");

        // decode mmr leaf
        let mmr_leaf: Vec<u8> = Decode::decode(&mut &mmr_root.mmr_leaf.clone()[..]).unwrap();
        trace!("in beefy monitor: [build_header] mmr_leaf decode to Vec<u8>: {:?}",mmr_leaf);
        let mmr_leaf: beefy_light_client::mmr::MmrLeaf = Decode::decode(&mut &*mmr_leaf).unwrap();
        trace!("in beefy monitor: [build_header] mmr_leaf to data struct: {:?}",mmr_leaf);

        // decode mmr leaf proof
        let mmr_leaf_proof = beefy_light_client::mmr::MmrLeafProof::decode(
            &mut &mmr_root.mmr_leaf_proof.clone()[..],
        )
        .unwrap();
        trace!("in beefy monitor: [build_header] block_header.block_number:{:?},mmr root heigh:{:?},mmr_leaf.parent_number:{:?},mmr_leaf_proof.leaf_index{:?},mmr_leaf_proof.leaf_count: {:?}",block_header.block_number,block_number,mmr_leaf.parent_number_and_hash.0,mmr_leaf_proof.leaf_index, mmr_leaf_proof.leaf_count);

        // log mmr leaf
        trace!("in beefy monitor: [build_header] block_header.parent_hash: {:?}",block_header.parent_hash);
        trace!("in beefy monitor: [build_header] mmr_leaf.parent_number_and_hash.1.to_vec(): {:?}",mmr_leaf.parent_number_and_hash.1.to_vec());

        // verfiy block header
        if block_header.parent_hash != mmr_leaf.parent_number_and_hash.1.to_vec() {
            trace!("in beefy monitor: [build_header] header.block_header.parent_hash != mmr_leaf.parent_number_and_hash.1.to_vec()");
        } else {
            trace!("in beefy monitor: [build_header] header.block_header.parent_hash == mmr_leaf.parent_number_and_hash.1.to_vec()");
        }

        let beefy_header =
            beefy_light_client::header::Header::try_from(block_header.clone()).unwrap();
        let header_hash = beefy_header.hash();
        trace!("in beefy monitor: [build_header] header_hash: {:?}",header_hash);
        trace!("in beefy monitor: [build_header] mmr_leaf.parent_number_and_hash.1: {:?}",mmr_leaf.parent_number_and_hash.1);
        if header_hash != mmr_leaf.parent_number_and_hash.1 {
            trace!("in beefy monitor: [build_header] header_hash != mmr_leaf.parent_number_and_hash.1");
        } else {
            trace!("in beefy monitor: [build_header] header_hash == mmr_leaf.parent_number_and_hash.1");
        }

        trace!("in beefy monitor: [build_header] ---------------------test[end]-----------------------");

        // build header
        let grandpa_header = Header {
            mmr_root: mmr_root,
            block_header: block_header,
            timestamp: timestamp,
        };

        trace!(
            "in beefy monitor: [build_header] >> grandpa_header = {:?}",
            grandpa_header
        );
        Ok(grandpa_header)
    }
}

pub enum Next {
    Abort,
    Continue,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::error::Error;
    use alloc::sync::Arc;
    use ibc::core::ics24_host::identifier::ChainId;
    use octopusxt::ibc_node::{self, RuntimeApi};
    use octopusxt::MyConfig;
    use subxt::{Client, ClientBuilder, Error as SubstrateError, PairSigner, SignedCommitment};
    use tendermint_rpc::Url;
    use tokio::runtime::Runtime as TokioRuntime;

    use super::BeefyMonitor;

    #[test]
    fn test_beefy_monitor_run() {
        let chain_id = ChainId::from_string("ibc-0");
        let websocket_url = Url::from_str("ws://127.0.0.1:9944/websocket").unwrap();
        let rt = Arc::new(TokioRuntime::new().unwrap());

        let mut client = rt
            .block_on(
                ClientBuilder::new()
                    .set_url(format!("{}", websocket_url))
                    .build::<MyConfig>(),
            )
            .unwrap();

        let (beefy_monitor, receiver, tx_cmd) =
            BeefyMonitor::new(chain_id, client, websocket_url, rt).unwrap();
        beefy_monitor.run()
    }
}
