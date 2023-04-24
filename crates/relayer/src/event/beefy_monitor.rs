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

// use octopusxt::ibc_node::RuntimeApi;
// use octopusxt::MyConfig;
// use octopusxt::SubstrateNodeTemplateExtrinsicParams;
// use subxt::{
//     BlockNumber, Client, ClientBuilder, Error as SubstrateError, PairSigner, SignedCommitment,
// };
use tendermint_rpc::{event::Event as RpcEvent, Url};

use crate::util::{
    retry::{retry_with_index, RetryResult},
    stream::try_group_while,
};
use crate::{error::Error as RelayError, telemetry};
use beefy_light_client::commitment;
use codec::{Decode, Encode};
use ibc_relayer_types::clients::ics10_grandpa::consensus_state::ConsensusState as GpConsensusState;
use ibc_relayer_types::clients::ics10_grandpa::header::Header as GpHeader;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics24_host::identifier::{
    ChainId, ChannelId, ClientId, ConnectionId, PortId,
};
// use ibc::events::IbcEvent;
use sp_core::{hexdisplay::HexDisplay, H256};

use std::future::Future;
use tokio::runtime::Runtime;

use super::{bus::EventBus, IbcEventWithHeight};
use crate::chain::handle::BeefySubscription;
use crate::error::Error;
use serde::{Deserialize, Serialize};
use subxt::blocks::Block;
use subxt::blocks::BlocksClient;
use subxt::error::RpcError as SubxtRpcError;
use subxt::rpc::types::BlockNumber;
use subxt::rpc::RpcClient as SubxtRpcClient;
use subxt::rpc::Subscription as SubxtSubscription;
use subxt::rpc_params;
use subxt::Error as SubxtError;
use subxt::{tx::PairSigner, OnlineClient, PolkadotConfig, SubstrateConfig};
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

pub use super::monitor::error::Error as MonitorError;
pub use super::monitor::error::*;
pub type BeefyResult<T> = Result<T, MonitorError>;
pub type BeefySender = channel::Sender<BeefyResult<GpHeader>>;
pub type BeefyReceiver = channel::Receiver<BeefyResult<GpHeader>>;

// pub type Result<T> = core::result::Result<T, Error>;
// pub use super::monitor::MonitorCmd;
// pub use super::monitor::TxMonitorCmd;

#[derive(Clone, Debug)]
pub struct TxMonitorCmd(pub channel::Sender<MonitorCmd>);

impl TxMonitorCmd {
    pub fn shutdown(&self) -> BeefyResult<()> {
        self.0
            .send(MonitorCmd::Shutdown)
            .map_err(|_| MonitorError::channel_send_failed())
    }

    pub fn subscribe(&self) -> BeefyResult<BeefySubscription> {
        let (tx, rx) = crossbeam_channel::bounded(1);

        self.0
            .send(MonitorCmd::Subscribe(tx))
            .map_err(|_| MonitorError::channel_send_failed())?;

        let subscription = rx.recv().map_err(|_| MonitorError::channel_recv_failed())?;
        Ok(subscription)
    }
}

#[derive(Debug)]
pub enum MonitorCmd {
    Shutdown,
    Subscribe(channel::Sender<BeefySubscription>),
}

/// Connect to a substrate node, subscribe to beefy info,
/// receive push signed commitment over a websocket, and build the mmr root from signed commitment
pub struct BeefyMonitor {
    chain_id: ChainId,
    /// WebSocket to collect events from
    beefy_node_addr: Url,
    /// connect to beefy client
    beefy_rpc_client: OnlineClient<PolkadotConfig>,
    /// Event bus for broadcasting events
    event_bus: EventBus<Arc<BeefyResult<GpHeader>>>,
    /// Channel where to receive client driver errors
    rx_err: mpsc::UnboundedReceiver<SubxtError>,
    /// Channel where to send client driver errors
    tx_err: mpsc::UnboundedSender<SubxtError>,
    /// Channel where to receive commands
    rx_cmd: channel::Receiver<MonitorCmd>,

    /// beefy stream subscription
    beefy_sub: SubxtSubscription<SignedCommitment>,
    /// Tokio runtime
    rt: Arc<TokioRuntime>,
}

impl BeefyMonitor {
    /// Create an event monitor, and connect to a node
    pub fn new(
        chain_id: ChainId,
        beefy_node_addr: Url,
        rt: Arc<TokioRuntime>,
    ) -> BeefyResult<(Self, TxMonitorCmd)> {
        let event_bus = EventBus::new();
        let (tx_cmd, rx_cmd) = channel::unbounded();
        let (tx_err, rx_err) = mpsc::unbounded_channel();

        let beefy_rpc_client = rt
            .block_on(OnlineClient::<PolkadotConfig>::from_url(
                beefy_node_addr.to_string(),
            ))
            .unwrap();
        let beefy_sub = rt
            .block_on(subscribe_beefy_message(beefy_rpc_client.rpc()))
            .unwrap();

        let monitor = Self {
            rt,
            chain_id,
            beefy_node_addr,
            beefy_rpc_client,
            event_bus,
            rx_err,
            tx_err,
            rx_cmd,
            beefy_sub,
        };

        Ok((monitor, TxMonitorCmd(tx_cmd)))
    }

    ///subscribe beefy
    pub fn init_subscriptions(&mut self) -> BeefyResult<()> {
        info!("in beefy_mointor: [subscribe] ");
        // subcribe beefy msg
        self.beefy_sub = self
            .rt
            .block_on(subscribe_beefy_message(self.beefy_rpc_client.rpc()))
            .unwrap();

        Ok(())
    }

    fn try_reconnect(&mut self) -> BeefyResult<()> {
        trace!(
            "[{}] trying to reconnect to WebSocket endpoint {}",
            self.chain_id,
            self.beefy_node_addr
        );

        let mut beefy_rpc_client = self
            .rt
            .block_on(OnlineClient::<PolkadotConfig>::from_url(
                self.beefy_node_addr.to_string(),
            ))
            .unwrap();
        // Swap the new client with the previous one which failed,
        // so that we can shut the latter down gracefully.
        core::mem::swap(&mut self.beefy_rpc_client, &mut beefy_rpc_client);

        Ok(())
    }

    /// Try to resubscribe to events
    fn try_resubscribe(&mut self) -> BeefyResult<()> {
        info!("[{}] trying to resubscribe to beefy", self.chain_id);
        self.init_subscriptions()
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
                self.chain_id, self.beefy_node_addr
            ),
            Err(retries) => error!(
                "failed to reconnect to {} after {} retries",
                self.chain_id, self.beefy_node_addr,
            ),
        }
    }

    /// beefy monitor loop
    #[allow(clippy::while_let_loop)]
    pub fn run(mut self) {
        debug!("[{}] starting beefy monitor", self.chain_id);
        trace!(
            "in beefy_monitor: [run], [{}] starting beefy monitor ",
            self.chain_id
        );

        loop {
            match self.run_loop() {
                Next::Continue => continue,
                Next::Abort => break,
            }
        }

        // close connection
        let _ = self.beefy_rpc_client;

        trace!("[{}] beefy monitor is shutting down", self.chain_id);
    }

    fn run_loop(&mut self) -> Next {
        info!("in beefy_mointor: [run_loop]");

        // Work around double borrow
        let rt = self.rt.clone();

        // get beefy_sub ownership
        let mut beefy_sub = rt
            .block_on(subscribe_beefy_message(self.beefy_rpc_client.rpc()))
            .unwrap();

        loop {
            if let Ok(cmd) = self.rx_cmd.try_recv() {
                match cmd {
                    MonitorCmd::Shutdown => return Next::Abort,
                    MonitorCmd::Subscribe(tx) => {
                        if let Err(e) = tx.send(self.event_bus.subscribe()) {
                            error!("failed to send back subscription: {e}");
                        }
                    }
                }
            }
            let beefy_result = rt.block_on(async {
                tokio::select! {

                    //select beefy subcription
                    Some(beefy_msg) = beefy_sub.next() => {
                        let msg = beefy_msg.unwrap();
                         trace!(" received beefy msg {:?}", msg);

                          Ok(msg)

                    },
                    Some(e) = self.rx_err.recv() => Err(MonitorError::subxt_error(e)),
                }
            });

            match beefy_result {
                Ok(msg) => {
                    trace!(" need to build header based on beefy msg {:?}", msg);
                    self.process_beefy_msg(msg)
                }
                Err(e) => {
                    error!("subscription dropped, need to reconnect !");

                    self.propagate_error(e);
                    telemetry!(ws_reconnect, &self.chain_id);

                    // Reconnect to the WebSocket endpoint, and subscribe again to the queries.
                    self.reconnect();

                    // Abort this event loop, the `run` method will start a new one.
                    // We can't just write `return self.run()` here because Rust
                    // does not perform tail call optimization, and we would
                    // thus potentially blow up the stack after many restarts.
                    return Next::Continue;
                }
            }
        }
    }

    /// Propagate error to subscribers.
    ///
    /// The main use case for propagating RPC errors is for the [`Supervisor`]
    /// to notice that the WebSocket connection or subscription has been closed,
    /// and to trigger a clearing of packets, as this typically means that we have
    /// missed a bunch of events which were emitted after the subscrption was closed.
    /// In that case, this error will be handled in [`Supervisor::handle_batch`].
    fn propagate_error(&mut self, error: MonitorError) {
        self.event_bus.broadcast(Arc::new(Err(error)));
    }

    /// Collect the IBC events from the subscriptions
    fn process_beefy_msg(&mut self, sc: SignedCommitment) {
        trace!("in beefy_mointor: [process_beefy_msg]");

        let header = self.rt.block_on(self.build_header(sc));
        trace!(
            "in beefy_monitor: [process_beefy_msg], build header: {:?} ",
            header
        );

        if let Ok(h) = header {
            // send to msg queue
            trace!(
                "in beefy_monitor: [process_beefy_msg], send beefy header : {:?} ",
                h
            );

            self.event_bus.broadcast(Arc::new(Ok(h)));
        }
    }

    pub async fn build_header(&self, raw_sc: SignedCommitment) -> Result<GpHeader, RelayError> {
        trace!("in beefy monitor: [build_header]");
        // decode signed commitment
        let beefy_light_client::commitment::VersionedFinalityProof::V1(signed_commitment) =
            beefy_light_client::commitment::VersionedFinalityProof::decode(&mut &raw_sc.0[..])
                .unwrap();
        trace!(
            "in beefy_monitor: [build_header], decode signed_commitment : {:?} ",
            signed_commitment
        );
        todo!()
    }
}

#[derive(codec::Encode, codec::Decode, PartialEq, Eq)]
pub struct EncodableOpaqueLeaf(pub Vec<u8>);

/// An encoded signed commitment proving that the given header has been finalized.
/// The given bytes should be the SCALE-encoded representation of a
/// `beefy_primitives::SignedCommitment`.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignedCommitment(pub sp_core::Bytes);

/// Subscribe to beefy justifications.
pub async fn subscribe_beefy_message(
    client: &SubxtRpcClient,
) -> Result<SubxtSubscription<SignedCommitment>, subxt::Error> {
    let subscription = client
        .subscribe(
            "beefy_subscribeJustifications",
            rpc_params![],
            "beefy_unsubscribeJustifications",
        )
        .await?;
    Ok(subscription)
}

pub enum Next {
    Abort,
    Continue,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use beefy_light_client::Hash;
    use codec::{Decode, Encode};
    // use mmr_rpc::LeavesProof;
    use alloc::sync::Arc;
    use ibc_relayer_types::core::ics24_host::identifier::{
        ChainId, ChannelId, ClientId, ConnectionId, PortId,
    };
    use serde::{Deserialize, Serialize};
    use sp_core::H256;
    use subxt::rpc::types::BlockNumber;
    use subxt::rpc::RpcClient;
    use subxt::rpc::Subscription;
    use subxt::{rpc_params, OnlineClient, PolkadotConfig};
    use tendermint_rpc::Url;
    use tokio::{runtime::Runtime as TokioRuntime, sync::mpsc};
    // #[subxt::subxt(runtime_metadata_path = "metadata/metadata.scale")]
    // #[subxt::subxt(runtime_metadata_url = "wss://rococo-rpc.polkadot.io:443")]
    #[subxt::subxt(runtime_metadata_url = "ws://127.0.0.1:9944")]
    pub mod polkadot {}

    use super::BeefyMonitor;

    #[test]
    fn test_beefy_monitor_run() {
        let chain_id = ChainId::from_string("ibc-0");
        let beefy_node_addr = Url::from_str("ws://127.0.0.1:9944").unwrap();
        let rt = Arc::new(TokioRuntime::new().unwrap());

        let mut beefy_rpc_client = rt
            .block_on(OnlineClient::<PolkadotConfig>::from_url(
                beefy_node_addr.to_string(),
            ))
            .unwrap();

        let (beefy_monitor, tx_cmd) = BeefyMonitor::new(chain_id, beefy_node_addr, rt).unwrap();
        beefy_monitor.run()
    }
}
