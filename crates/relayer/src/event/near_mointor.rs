use crate::chain::substrate::config::ibc_node;
use crate::chain::near::REVISION_NUMBER;
use crate::chain::tracking::TrackingId;
use crate::event::IbcEventWithHeight;
use crate::util::retry::{retry_with_index, RetryResult};
use alloc::sync::Arc;
use core::cmp::Ordering;
use crossbeam_channel as channel;
use futures::{
    stream::{self, StreamExt},
    Stream,
};
use ibc_relayer_types::{
    core::{ics02_client::height::Height, ics24_host::identifier::ChainId},
    events::IbcEvent,
};
use subxt::events::EventDetails;
use subxt::{OnlineClient, SubstrateConfig}; //todo!()//Bob,
use tendermint_rpc::{event::Event as RpcEvent, Url};
use tokio::{runtime::Runtime as TokioRuntime, sync::mpsc};
use tracing::{debug, error, info, trace};
mod retry_strategy {  //todo!()//Bob,
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
    client: OnlineClient<SubstrateConfig>, //todo!()//Bob,
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
        client: OnlineClient<SubstrateConfig>, //todo!()//Bob
        node_addr: Url,
        rt: Arc<TokioRuntime>,
    ) -> Result<(Self, EventReceiver, TxMonitorCmd)> {
        let (tx_batch, rx_batch) = channel::unbounded();
        let (tx_cmd, rx_cmd) = channel::unbounded();

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

    /// Clear the current subscriptions, and subscribe again to all queries.
    pub fn subscribe(&mut self) -> Result<()> {
        let mut subscriptions = vec![];

        // todo unwrap
        let subscription = self.rt.block_on(subscribe_events(self.client.clone()));

        info!(
            "in near_mointor: [subscribe] subscription: {:?}",
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
            .block_on(OnlineClient::from_url(format!(  //todo!()//Bob,
                "{}",
                &self.node_addr.clone()
            )))
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
                "successfully reconnected to WebSocket endpoint {}",
                self.node_addr
            ),
            Err(e) => error!(
                "failed to reconnect to {} after {} retries",
                self.node_addr, e.tries
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
        info!("in near_mointor: [run_loop]");

        let client = self.client.clone();
        let chain_id = self.chain_id.clone();
        let send_batch = self.tx_batch.clone();

        let sub_event = async move {
            // Subscribe to any events that occur:
            let mut event_sub = client.events().subscribe().await.unwrap();

            // Our subscription will see the events emitted as a result of this:
            while let Some(events) = event_sub.next().await {
                let events = events.expect("handler result events have meet trouble!");

                for event in events.iter() {
                    let event: EventDetails =
                        event.expect("handler result events have meet trouble");

                    let raw_event = event.clone();

                    let client = client.clone();
                    let chain_id = chain_id.clone();
                    let send_batch = send_batch.clone();

                    tokio::spawn(async move {
                        handle_single_event(raw_event, client, chain_id, send_batch).await;
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
        info!("in near_mointor: [propagate_error]");

        self.tx_batch
            .send(Err(error))
            .map_err(|_| Error::channel_send_failed())?;

        Ok(())
    }

    /// Collect the IBC events from the subscriptions
    fn process_batch(&self, batch: EventBatch) -> Result<()> {
        trace!("in near_mointor: [process_batch]");

        self.tx_batch
            .send(Ok(batch))
            .map_err(|_| Error::channel_send_failed())?;

        Ok(())
    }
}

fn process_batch_for_near(
    send_tx: channel::Sender<Result<EventBatch>>,
    batch: EventBatch,
) -> Result<()> {
    trace!("in near_mointor: [relayer_process_channel_events]");

    send_tx
        .try_send(Ok(batch))
        .map_err(|_| Error::channel_send_failed())?;
    Ok(())
}

/// Collect the IBC events from an RPC event
fn collect_events(
    chain_id: &ChainId,
    event: RpcEvent,
) -> impl Stream<Item = Result<IbcEventWithHeight>> {
    trace!("in near_mointor: [collect_events]");

    let events = crate::event::rpc::get_all_events(chain_id, event).unwrap_or_default();
    stream::iter(events).map(Ok)
}

/// Sort the given events by putting the NewBlock event first,
/// and leaving the other events as is.
fn sort_events(events: &mut [IbcEvent]) {
    trace!("in near_mointor: [sort_events]");

    events.sort_by(|a, b| match (a, b) {
        (IbcEvent::NewBlock(_), _) => Ordering::Less,
        _ => Ordering::Equal,
    })
}

/// Subscribe Events
async fn subscribe_events(client: OnlineClient<SubstrateConfig> /*//todo!()//Bob*/) -> EventDetails {
    info!("In near_monitor: [subscribe_events]");
    todo!()//Bob
}

fn from_raw_event_to_batch_event(
    raw_event: EventDetails,
    chain_id: ChainId,
    height: u64,
) -> Result<EventBatch> {
    trace!(
        "In near: [from_raw_event_to_batch_event] >> raw Event: {:?}",
        raw_event
    );
    let height = Height::new(REVISION_NUMBER, height).expect("REVISION_NUMBER");
    let variant = raw_event.variant_name();
    match variant {
        "CreateClient" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::CreateClient>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [subscribe_events] >> CreateClient Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("Empty Event".to_string()))
            }
        }
        "UpdateClient" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::UpdateClient>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [subscribe_events] >> UpdateClient Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("Empty Event".to_string()))
            }
        }
        "ClientMisbehaviour" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::ClientMisbehaviour>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [subscribe_events] >> ClientMisbehaviour Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "OpenInitConnection" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::OpenInitConnection>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [subscribe_events] >> OpenInitConnection Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "OpenTryConnection" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::OpenTryConnection>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [subscribe_events] >> OpenTryConnection Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "OpenAckConnection" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::OpenAckConnection>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [subscribe_events] >> OpenAckConnection Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "OpenConfirmConnection" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::OpenConfirmConnection>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [subscribe_events] >> OpenConfirmConnection Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }

        "OpenInitChannel" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::OpenInitChannel>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [subscribe_events] >> OpenInitChannel Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "OpenTryChannel" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::OpenTryChannel>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [subscribe_events] >> OpenTryChannel Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "OpenAckChannel" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::OpenAckChannel>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [subscribe_events] >> OpenAckChannel Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "OpenConfirmChannel" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::OpenConfirmChannel>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [subscribe_events] >> OpenConfirmChannel Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "CloseInitChannel" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::CloseInitChannel>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [subscribe_events] >> CloseInitChannel Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "CloseConfirmChannel" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::CloseConfirmChannel>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [subscribe_events] >> CloseConfirmChannel Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "SendPacket" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::SendPacket>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [near_events] >> SendPacket Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "ReceivePacket" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::ReceivePacket>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [near_events] >> ReceivePacket Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "WriteAcknowledgement" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::WriteAcknowledgement>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [near_events] >> WriteAcknowledgement Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "AcknowledgePacket" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::AcknowledgePacket>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [near_events] >> AcknowledgePacket Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "TimeoutPacket" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::TimeoutPacket>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [near_events] >> TimeoutPacket Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "TimeoutOnClosePacket" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::TimeoutOnClosePacket>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [near_events] >> TimeoutOnClosePacket Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "AppModule" => {
            if let Some(event) = raw_event
                .as_event::<ibc_node::ibc::events::AppModule>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        "ChainError" => {
            // near chain error
            trace!("in near_monitor: [near_events] >> ChainError Event");

            let data = String::from("near chain error");

            let event = IbcEvent::ChainError(data);

            let ibc_event_with_height = IbcEventWithHeight::new(event, height.clone());

            Ok(EventBatch {
                height,
                events: vec![ibc_event_with_height],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "ExtrinsicSuccess" => {
            if let Some(_event) = raw_event
                .as_event::<ibc_node::system::events::ExtrinsicSuccess>()
                .map_err(|_| Error::report_error("invalid_codec_decode".to_string()))?
            {
                trace!("In near_monitor: [subscribe_events] >> ExtrinsicSuccess Event");

                todo!()//Bob,
            } else {
                Err(Error::report_error("empty Event".to_string()))
            }
        }
        _ => Ok(EventBatch {
            height,
            events: vec![],
            chain_id,
            tracking_id: TrackingId::new_uuid(),
        }),
    }
}

pub enum Next {
    Abort,
    Continue,
}

async fn get_latest_height(client: OnlineClient<SubstrateConfig>) -> u64 {
    trace!("In near_monitor: [get_latest_height]");

    todo!()//Bob,
}

async fn handle_single_event(
    raw_event: EventDetails,
    client: OnlineClient<SubstrateConfig>, //todo!()//Bob,
    chain_id: ChainId,
    send_batch: channel::Sender<Result<EventBatch>>,
) {
    trace!("in near_monitor: [handle_single_event]");

    let height = get_latest_height(client).await; // Todo: Do not query for latest height every time
    let batch_event = from_raw_event_to_batch_event(raw_event, chain_id.clone(), height);
    if let Ok(batch_event) = batch_event {
        if !batch_event.events.is_empty() {
            process_batch_for_near(send_batch.clone(), batch_event).unwrap_or_else(|e| {
                error!("[{}] {}", chain_id, e);
            });
        }
    } else {
        trace!("in near monitor:handle_single_event from_raw_event_to_batch_event error");
    }
}
