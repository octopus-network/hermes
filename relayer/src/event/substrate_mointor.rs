use alloc::sync::Arc;
use core::cmp::Ordering;

use crossbeam_channel as channel;

use futures::{
    stream::{self, StreamExt},
    Stream,
};

use tokio::{runtime::Runtime as TokioRuntime, sync::mpsc};
use tracing::{debug, error, info, trace};

use tendermint_rpc::{event::Event as RpcEvent, Url};

use octopusxt::ibc_node;
use octopusxt::MyConfig;
use subxt::SubstrateExtrinsicParams;
use subxt::{Client, ClientBuilder, RawEventDetails};

use ibc::core::ics02_client::height::Height;
use ibc::core::ics24_host::identifier::ChainId;
use ibc::events::IbcEvent;

use crate::chain::tracking::TrackingId;
use crate::util::retry::{retry_count, retry_with_index, RetryResult};

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
pub use super::monitor::Result;

pub use super::monitor::EventBatch;
pub use super::monitor::EventReceiver;
pub use super::monitor::EventSender;
pub use super::monitor::MonitorCmd;
pub use super::monitor::TxMonitorCmd;

/// Connect to a Tendermint node, subscribe to a set of queries,
/// receive push events over a websocket, and filter them for the
/// event handler.
///
/// The default events that are queried are:
/// - [`EventType::NewBlock`]
/// - [`EventType::Tx`]
///
/// Those can be extending or overriden using
/// [`EventMonitor::add_query`] and [`EventMonitor::set_queries`].
pub struct EventMonitor {
    chain_id: ChainId,
    /// WebSocket to collect events from
    client: Client<MyConfig>,
    /// Channel to handler where the monitor for this chain sends the events
    tx_batch: channel::Sender<Result<EventBatch>>,
    /// Channel where to receive client driver errors
    rx_err: mpsc::UnboundedReceiver<tendermint_rpc::Error>,
    /// Channel where to send client driver errors
    tx_err: mpsc::UnboundedSender<tendermint_rpc::Error>,
    /// Channel where to receive commands
    rx_cmd: channel::Receiver<MonitorCmd>,
    /// Node Address
    node_addr: Url,
    /// Tokio runtime
    rt: Arc<TokioRuntime>,
}

impl EventMonitor {
    /// Create an event monitor, and connect to a node
    pub fn new(
        chain_id: ChainId,
        node_addr: Url,
        rt: Arc<TokioRuntime>,
    ) -> Result<(Self, EventReceiver, TxMonitorCmd)> {
        let (tx_batch, rx_batch) = channel::unbounded();
        let (tx_cmd, rx_cmd) = channel::unbounded();

        let ws_addr = format!("{}", node_addr);
        let client = rt
            .block_on(async move {
                ClientBuilder::new()
                    .set_url(ws_addr)
                    .build::<MyConfig>()
                    .await
            })
            .map_err(|_| Error::client_creation_failed(chain_id.clone(), node_addr.clone()))?;

        let (tx_err, rx_err) = mpsc::unbounded_channel();

        let monitor = Self {
            rt,
            chain_id,
            client,
            tx_batch,
            rx_err,
            tx_err,
            rx_cmd,
            node_addr,
        };

        Ok((monitor, rx_batch, tx_cmd))
    }

    // /// Set the queries to subscribe to.
    // ///
    // /// ## Note
    // /// For this change to take effect, one has to [`subscribe`] again.
    // pub fn set_queries(&mut self, queries: Vec<Query>) {
    //     self.event_queries = queries;
    // }
    //
    // /// Add a new query to subscribe to.
    // ///
    // /// ## Note
    // /// For this change to take effect, one has to [`subscribe`] again.
    // pub fn add_query(&mut self, query: Query) {
    //     self.event_queries.push(query);
    // }

    /// Clear the current subscriptions, and subscribe again to all queries.
    pub fn subscribe(&mut self) -> Result<()> {
        let mut subscriptions = vec![];

        // todo unwrap
        let subscription = self.rt.block_on(subscribe_events(self.client.clone()));

        tracing::info!(
            "in substrate_mointor: [subscribe] subscription: {:?}",
            subscription
        );

        subscriptions.push(subscription);
        trace!("[{}] subscribed to all queries", self.chain_id);

        Ok(())
    }

    fn try_reconnect(&mut self) -> Result<()> {
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
    fn try_resubscribe(&mut self) -> Result<()> {
        info!("[{}] trying to resubscribe to events", self.chain_id);
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

    /// Event monitor loop
    #[allow(clippy::while_let_loop)]
    pub fn run(mut self) {
        debug!("[{}] starting event monitor", self.chain_id);

        // Continuously run the event loop, so that when it aborts
        // because of WebSocket client restart, we pick up the work again.
        loop {
            match self.run_loop() {
                Next::Continue => continue,
                Next::Abort => break,
            }
        }

        debug!("[{}] event monitor is shutting down", self.chain_id);
    }

    fn run_loop(&mut self) -> Next {
        tracing::info!("in substrate_mointor: [run_loop]");
        let client = self.client.clone();
        let chain_id = self.chain_id.clone();
        let send_batch = self.tx_batch.clone();

        let sub_event = async move {
            let api = client
                .clone()
                .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

            // Subscribe to any events that occur:
            let mut event_sub = api.events().subscribe().await.unwrap();

            // Our subscription will see the events emitted as a result of this:
            while let Some(events) = event_sub.next().await {
                let events = events.unwrap();

                for event in events.iter_raw() {
                    let event: RawEventDetails = event.unwrap();

                    let raw_event = event.clone();

                    let _client = client.clone();
                    let _chain_id = chain_id.clone();
                    let _send_batch = send_batch.clone();

                    tokio::spawn(async move {
                        handle_single_event(raw_event, _client, _chain_id, _send_batch).await;
                    });
                }
            }

            Next::Continue
        };

        self.rt.block_on(sub_event)
    }

    /// Propagate error to subscribers.
    ///
    /// The main use case for propagating RPC errors is for the [`Supervisor`]
    /// to notice that the WebSocket connection or subscription has been closed,
    /// and to trigger a clearing of packets, as this typically means that we have
    /// missed a bunch of events which were emitted after the subscrption was closed.
    /// In that case, this error will be handled in [`Supervisor::handle_batch`].
    fn propagate_error(&self, error: Error) -> Result<()> {
        tracing::info!("in substrate_mointor: [propagate_error]");
        self.tx_batch
            .send(Err(error))
            .map_err(|_| Error::channel_send_failed())?;

        Ok(())
    }

    /// Collect the IBC events from the subscriptions
    fn process_batch(&self, batch: EventBatch) -> Result<()> {
        tracing::trace!("in substrate_mointor: [process_batch]");

        self.tx_batch
            .send(Ok(batch))
            .map_err(|_| Error::channel_send_failed())?;

        Ok(())
    }
}

fn process_batch_for_substrate(
    send_tx: channel::Sender<Result<EventBatch>>,
    batch: EventBatch,
) -> Result<()> {
    tracing::trace!("in substrate_mointor: [relayer_process_channel_events]");
    send_tx
        .try_send(Ok(batch))
        .map_err(|_| Error::channel_send_failed())?;
    Ok(())
}

/// Collect the IBC events from an RPC event
fn collect_events(
    chain_id: &ChainId,
    event: RpcEvent,
) -> impl Stream<Item = Result<(Height, IbcEvent)>> {
    tracing::trace!("in substrate_mointor: [collect_events]");

    let events = crate::event::rpc::get_all_events(chain_id, event).unwrap_or_default();
    stream::iter(events).map(Ok)
}

/// Sort the given events by putting the NewBlock event first,
/// and leaving the other events as is.
fn sort_events(events: &mut [IbcEvent]) {
    tracing::trace!("in substrate_mointor: [sort_events]");

    events.sort_by(|a, b| match (a, b) {
        (IbcEvent::NewBlock(_), _) => Ordering::Less,
        _ => Ordering::Equal,
    })
}

/// Subscribe Events
async fn subscribe_events(client: Client<MyConfig>) -> RawEventDetails {
    tracing::info!("In substrate_monitor: [subscribe_events]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();
    // Subscribe to any events that occur:
    let mut event_sub = api.events().subscribe().await.unwrap();

    // Our subscription will see the events emitted as a result of this:
    while let Some(events) = event_sub.next().await {
        let events = events.unwrap();

        if let Some(event) = events.iter_raw().next() {
            let event = event.unwrap();
            return event;
        };
    }

    unimplemented!()
}

fn from_raw_event_to_batch_event(
    raw_event: RawEventDetails,
    chain_id: ChainId,
) -> Result<EventBatch> {
    let ibc_event = octopusxt::inner_process_ibc_event(raw_event);

    Ok(EventBatch {
        height: ibc_event.height(),
        events: vec![ibc_event],
        chain_id,
        tracking_id: TrackingId::new_uuid(),
    })
}

pub enum Next {
    Abort,
    Continue,
}

async fn get_latest_height(client: Client<MyConfig>) -> u64 {
    tracing::trace!("In substrate_monitor: [get_latest_height]");

    let api = client
        .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateExtrinsicParams<MyConfig>>>();

    let block = api.client.rpc().subscribe_blocks().await;

    let mut block = if let Ok(value) = block {
        value
    } else {
        panic!("subscribe blocks error");
    };

    match block.next().await {
        Some(Ok(header)) => header.number as u64,
        Some(Err(_)) => 0,
        None => 0,
    }
}

async fn handle_single_event(
    raw_event: RawEventDetails,
    _client: Client<MyConfig>,
    chain_id: ChainId,
    send_batch: channel::Sender<Result<EventBatch>>,
) {
    tracing::trace!("in substrate_monitor: [handle_single_event]");

    let batch_event = from_raw_event_to_batch_event(raw_event, chain_id.clone());
    if let Ok(batch_event) = batch_event {
        if !batch_event.events.is_empty() {
            process_batch_for_substrate(send_batch.clone(), batch_event).unwrap_or_else(|e| {
                error!("[{}] {}", chain_id, e);
            });
        }
    } else {
        tracing::trace!(
            "in substrate monitor:handle_single_event from_raw_event_to_batch_event error"
        );
    }
}
