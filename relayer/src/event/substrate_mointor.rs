use alloc::sync::Arc;
use core::cmp::Ordering;

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


use tendermint_rpc::{event::Event as RpcEvent, Url};

use substrate_subxt::{
    ClientBuilder, PairSigner, Client,
    EventSubscription, system::ExtrinsicSuccessEvent, RawEvent,
    Error as SubstrateError,
};


use ibc::{events::IbcEvent, ics02_client::height::Height, ics24_host::identifier::ChainId};

use crate::util::{
    retry::{retry_count, retry_with_index, RetryResult},
    stream::try_group_while,
};
use calls::NodeRuntime;

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
//
// define_error! {
//     #[derive(Debug, Clone)]
//     Error {
//         WebSocketDriver
//             |_| { "WebSocket driver failed" },
//
//         ClientCreationFailed
//             { chain_id: ChainId, address: Url }
//             |e| { format!("failed to create WebSocket driver for chain {0} with address {1}", e.chain_id, e.address) },
//
//         ClientTerminationFailed
//             [ TraceError<tokio::task::JoinError> ]
//             |_| { "failed to terminate previous WebSocket driver" },
//
//         ClientCompletionFailed
//             [ TraceError<SubstrateError> ]
//             |_| { "failed to run previous WebSocket driver to completion" },
//
//         ClientSubscriptionFailed
//             [ TraceError<SubstrateError> ]
//             |_| { "failed to run previous WebSocket driver to completion" },
//
//         NextEventBatchFailed
//             [ TraceError<SubstrateError> ]
//             |_| { "failed to collect events over WebSocket subscription" },
//
//         CollectEventsFailed
//             { reason: String }
//             |e| { format!("failed to extract IBC events: {0}", e.reason) },
//
//         ChannelSendFailed
//             |_| { "failed to send event batch through channel" },
//
//         SubscriptionCancelled
//             [ TraceError<SubstrateError> ]
//             |_| { "subscription cancelled" },
//
//         Rpc
//             [ TraceError<SubstrateError> ]
//             |_| { "RPC error" },
//     }
// }
//
//
// pub type Result<T> = core::result::Result<T, Error>;

pub use super::monitor::Result;
pub use super::monitor::Error;

// /// A batch of events from a chain at a specific height
// #[derive(Clone, Debug)]
// pub struct EventBatch {
//     pub chain_id: ChainId,
//     pub height: Height,
//     pub events: Vec<IbcEvent>,
// }

type SubscriptionResult = core::result::Result<RpcEvent, SubstrateError>;
type SubscriptionStream = dyn Stream<Item = SubscriptionResult> + Send + Sync + Unpin;


// pub type EventSender = channel::Sender<Result<EventBatch>>;
// pub type EventReceiver = channel::Receiver<Result<EventBatch>>;
// pub type TxMonitorCmd = channel::Sender<MonitorCmd>;


pub use super::monitor::EventSender;
pub use super::monitor::EventReceiver;
pub use super::monitor::TxMonitorCmd;
pub use super::monitor::EventBatch;
pub use super::monitor::MonitorCmd;

// #[derive(Debug)]
// pub enum MonitorCmd {
//     Shutdown,
// }

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
    client: Client<NodeRuntime>,
    /// Async task handle for the WebSocket client's driver
    // driver_handle: JoinHandle<()>,
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
    /// Queries
    // event_queries: Vec<Query>,
    /// All subscriptions combined in a single stream
    subscriptions: Box<SubscriptionStream>,
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

        let ws_addr = format!("{}", node_addr.clone());
        let client = rt
            .block_on(async move { ClientBuilder::<NodeRuntime>::new()
                .set_url(ws_addr)
                .build().await
            })
            .map_err(|_| Error::client_creation_failed(chain_id.clone(), node_addr.clone()))?;

        let (tx_err, rx_err) = mpsc::unbounded_channel();
        // let websocket_driver_handle = rt.spawn(run_driver(driver, tx_err.clone()));

        // TODO: move them to config file(?)
        // let event_queries = vec![Query::from(EventType::Tx), Query::from(EventType::NewBlock)];

        let monitor = Self {
            rt,
            chain_id,
            client,
            // driver_handle: websocket_driver_handle,
            // event_queries,
            tx_batch,
            rx_err,
            tx_err,
            rx_cmd,
            node_addr,
            subscriptions: Box::new(futures::stream::empty()),
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

        // for query in &self.event_queries {
        //     trace!("[{}] subscribing to query: {}", self.chain_id, query);

        let subscription = self
            .rt
            .block_on(subscribe_events(self.client.clone()))
            .map_err(Error::subscribe_substrate_failed)?;
        tracing::info!("in substrate_mointor: [subscribe] subscription: {:?}", subscription);

        subscriptions.push(subscription);
        // }

        // self.subscriptions = Box::new(select_all(subscriptions));

        trace!("[{}] subscribed to all queries", self.chain_id);

        Ok(())
    }

    fn try_reconnect(&mut self) -> Result<()> {
        trace!(
            "[{}] trying to reconnect to WebSocket endpoint {}",
            self.chain_id,
            self.node_addr
        );

        // Try to reconnect
        let mut client = self
            .rt
            .block_on(
                ClientBuilder::<NodeRuntime>::new()
                .set_url(format!("{}", &self.node_addr.clone()))
                .build()
            )
            .map_err(|_| {
                Error::client_creation_failed(self.chain_id.clone(), self.node_addr.clone())
            })?;

        // let mut driver_handle = self.rt.spawn(run_driver(driver, self.tx_err.clone()));

        // Swap the new client with the previous one which failed,
        // so that we can shut the latter down gracefully.
        core::mem::swap(&mut self.client, &mut client);
        // core::mem::swap(&mut self.driver_handle, &mut driver_handle);

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

        // let _ = client.close();

        // self.rt
        //     .block_on(driver_handle)
        //     .map_err(Error::client_termination_failed)?;

        trace!("[{}] previous client successfully shutdown", self.chain_id);

        Ok(())
    }

    /// Try to resubscribe to events
    fn try_resubscribe(&mut self) -> Result<()> {
        trace!("[{}] trying to resubscribe to events", self.chain_id);
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

        // Close the WebSocket connection
        // let _ = self.client.close();

        // Wait for the WebSocket driver to finish
        // let _ = self.rt.block_on(self.driver_handle);

        trace!(
            "[{}] event monitor has successfully shut down",
            self.chain_id
        );
    }

    fn run_loop(&mut self) -> Next {
        tracing::info!("in substrate_mointor: [run_loop]");

        // Take ownership of the subscriptions
        let subscriptions =
            core::mem::replace(&mut self.subscriptions, Box::new(futures::stream::empty()));


        let (send_tx, recv_tx) = channel::unbounded();


        let client = self.client.clone();
        let _client = self.client.clone();
        let send_tx = send_tx.clone();
        let chain_id = self.chain_id.clone();
        let send_batch = self.tx_batch.clone();

        let sub_event = async move {
            let sub = client.subscribe_finalized_events().await.unwrap();
            let decoder = client.events_decoder();
            let mut sub = EventSubscription::<NodeRuntime>::new(sub, decoder);

            while let Some(raw_event) = sub.next().await {
                if let Err(err) = raw_event {
                    tracing::info!("In substrate_mointor: [run_loop] >> raw_event error: {:?}", err);
                    continue;
                }
                let raw_event = raw_event.unwrap();
                tracing::info!("in substrate_mointor: [run_loop] >> raw_event : {:?}", raw_event);
                send_tx.send(raw_event);  // Todo: remove these 2 lines?
                for item in recv_tx.clone().recv() {
                    tracing::info!("in substrate_mointor: [run_loop] >> recv raw_event : {:?}", item);

                    let height = get_latest_height(_client.clone()).await; // Todo: Do not query for latest height every time
                    let batch_event = from_raw_event_to_batch_event(item, chain_id.clone(), height);
                    process_batch_for_substrate(send_batch.clone(), batch_event).unwrap_or_else(|e| {
                                error!("[{}] {}", chain_id.clone(), e);
                    });
                }
            }

            Next::Continue
        };
        //
        let ret = self.rt.block_on(sub_event);
        ret

        // Convert the stream of RPC events into a stream of event batches.
        // let batches = stream_batches(subscriptions, self.chain_id.clone());

        // Needed to be able to poll the stream
        // pin_mut!(batches);

        // Work around double borrow
        // let rt = self.rt.clone();

        // loop {
        //     if let Ok(cmd) = self.rx_cmd.try_recv() {
        //         match cmd {
        //             MonitorCmd::Shutdown => return Next::Abort,
        //         }
        //     }



            // let result = rt.block_on(async {
            //     tokio::select! {
            //         Some(batch) = batches.next() => batch,
            //         Some(e) = self.rx_err.recv() => Err(Error::web_socket_driver()),
            //     }
            // });

            // match result {
            //     Ok(batch) => self.process_batch(batch).unwrap_or_else(|e| {
            //         error!("[{}] {}", self.chain_id, e);
            //     }),
            //     Err(e) => {
            //         match e.detail() {
            //             ErrorDetail::SubscriptionCancelled(reason) => {
            //                 error!(
            //                     "[{}] subscription cancelled, reason: {}",
            //                     self.chain_id, reason
            //                 );
            //
            //                 self.propagate_error(e).unwrap_or_else(|e| {
            //                     error!("[{}] {}", self.chain_id, e);
            //                 });
            //
            //                 // Reconnect to the WebSocket endpoint, and subscribe again to the queries.
            //                 self.reconnect();
            //
            //                 // Abort this event loop, the `run` method will start a new one.
            //                 // We can't just write `return self.run()` here because Rust
            //                 // does not perform tail call optimization, and we would
            //                 // thus potentially blow up the stack after many restarts.
            //                 return Next::Continue;
            //             }
            //             _ => {
            //                 error!("[{}] failed to collect events: {}", self.chain_id, e);
            //
            //                 // Reconnect to the WebSocket endpoint, and subscribe again to the queries.
            //                 self.reconnect();
            //
            //                 // Abort this event loop, the `run` method will start a new one.
            //                 // We can't just write `return self.run()` here because Rust
            //                 // does not perform tail call optimization, and we would
            //                 // thus potentially blow up the stack after many restarts.
            //                 return Next::Continue;
            //             }
            //         }
            //     }
            // }
        // }
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
        tracing::info!("in substrate_mointor: [process_batch]");

        self.tx_batch
            .send(Ok(batch))
            .map_err(|_| Error::channel_send_failed())?;

        Ok(())
    }


}

fn process_batch_for_substrate(send_tx: channel::Sender<Result<EventBatch>>, batch: EventBatch) -> Result<()> {
    tracing::info!("in substrate_mointor: [process_batch_for_substrate]");

    send_tx
        .send(Ok(batch))
        .map_err(|_| Error::channel_send_failed())?;

    Ok(())
}

/// Collect the IBC events from an RPC event
fn collect_events(
    chain_id: &ChainId,
    event: RpcEvent,
) -> impl Stream<Item = Result<(Height, IbcEvent)>> {
    tracing::info!("in substrate_mointor: [collect_events]");

    let events = crate::event::rpc::get_all_events(chain_id, event).unwrap_or_default();
    tracing::info!("in substrate_mointor: [collect_events] : events: {:?}", events);
    stream::iter(events).map(Ok)
}
//
// /// Convert a stream of RPC event into a stream of event batches
// fn stream_batches(
//     subscriptions: Box<SubscriptionStream>,
//     // subscriptions: channel::Receiver<RawEvent>,
//     chain_id: ChainId,
// ) -> impl Stream<Item = Result<EventBatch>> {
//     tracing::info!("in substrate_mointor: [stream_batches]");
//     let id = chain_id.clone();
//
//     // Collect IBC events from each RPC event
//     let events = subscriptions
//         .map_ok(move |raw_event| {
//             collect_events(&id, raw_event)
//         })
//         .map_err(Error::subscription_cancelled)
//         .try_flatten();
//
//     // Group events by height
//     let grouped = try_group_while(events, |(h0, _), (h1, _)| h0 == h1);
//
//     // Convert each group to a batch
//     grouped.map_ok(move |events| {
//         let height = events
//             .first()
//             .map(|(h, _)| h)
//             .copied()
//             .expect("internal error: found empty group"); // SAFETY: upheld by `group_while`
//
//         let mut events = events.into_iter().map(|(_, e)| e).collect();
//         sort_events(&mut events);
//
//         EventBatch {
//             height,
//             events,
//             chain_id: chain_id.clone(),
//         }
//     })
// }



/// Sort the given events by putting the NewBlock event first,
/// and leaving the other events as is.
fn sort_events(events: &mut Vec<IbcEvent>) {
    tracing::info!("in substrate_mointor: [sort_events]");

    events.sort_by(|a, b| match (a, b) {
        (IbcEvent::NewBlock(_), _) => Ordering::Less,
        _ => Ordering::Equal,
    })
}

use calls::ibc::{
    NewBlockEvent,
    CreateClientEvent, OpenInitConnectionEvent, UpdateClientEvent, ClientMisbehaviourEvent,
    OpenTryConnectionEvent, OpenAckConnectionEvent, OpenConfirmConnectionEvent,
    OpenInitChannelEvent, OpenTryChannelEvent, OpenAckChannelEvent,
    OpenConfirmChannelEvent,  CloseInitChannelEvent, CloseConfirmChannelEvent,
    SendPacketEvent, ReceivePacketEvent, WriteAcknowledgementEvent,
    AcknowledgePacketEvent, TimeoutPacketEvent, TimeoutOnClosePacketEvent,
    EmptyEvent, ChainErrorEvent,
};

use codec::Decode;

/// Subscribe Events
async fn subscribe_events(
    client: Client<NodeRuntime>,
)  -> core::result::Result<RawEvent, SubstrateError> {
    tracing::info!("In substrate_mointor: [subscribe_events]");

    let sub = client.subscribe_finalized_events().await?;
    let decoder = client.events_decoder();
    let mut sub = EventSubscription::<NodeRuntime>::new(sub, decoder);

    let result = sub.next().await.unwrap();

    result
}


fn from_raw_event_to_batch_event(raw_event: RawEvent, chain_id: ChainId, height: u64) -> EventBatch {
    tracing::info!("In substrate: [from_raw_event_to_batch_event] >> raw Event: {:?}", raw_event);
    let variant = raw_event.variant;
    tracing::info!("In substrate: [from_raw_event_to_batch_event] >> variant: {:?}", variant);
    match variant.as_str() {
        "CreateClient" => {
            let event = CreateClientEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [from_raw_event_to_batch_event] >> CreateClient Event");

            let height = event.height;
            let client_id = event.client_id;
            let client_type = event.client_type;
            let consensus_height = event.consensus_height;
            use ibc::ics02_client::events::Attributes;
            let event = IbcEvent::CreateClient(ibc::ics02_client::events::CreateClient(Attributes {
                height: height.clone().to_ibc_height(),
                client_id: client_id.to_ibc_client_id(),
                client_type: client_type.to_ibc_client_type(),
                consensus_height: consensus_height.to_ibc_height()
            }));

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        },
        "UpdateClient" => {
            let event = UpdateClientEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [from_raw_event_to_batch_event] >> UpdateClient Event");

            let height = event.height;
            let client_id = event.client_id;
            let client_type = event.client_type;
            let consensus_height = event.consensus_height;
            use ibc::ics02_client::events::Attributes;
            let event = IbcEvent::UpdateClient(ibc::ics02_client::events::UpdateClient{
                common: Attributes {
                    height: height.clone().to_ibc_height(),
                    client_id: client_id.to_ibc_client_id(),
                    client_type: client_type.to_ibc_client_type(),
                    consensus_height: consensus_height.to_ibc_height(),
                },
                header: None,
            });
            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        },
        "ClientMisbehaviour" => {
            let event = ClientMisbehaviourEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [from_raw_event_to_batch_event] >> ClientMisbehaviour Event");

            let height = event.height;
            let client_id = event.client_id;
            let client_type = event.client_type;
            let consensus_height = event.consensus_height;
            use ibc::ics02_client::events::Attributes;
            let event = IbcEvent::ClientMisbehaviour(ibc::ics02_client::events::ClientMisbehaviour(
                Attributes {
                    height: height.clone().to_ibc_height(),
                    client_id: client_id.to_ibc_client_id(),
                    client_type: client_type.to_ibc_client_type(),
                    consensus_height: consensus_height.to_ibc_height(),
                }
            ));

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        },
        "OpenInitConnection" => {
            let event = OpenInitConnectionEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [from_raw_event_to_batch_event] >> OpenInitConnection Event");

            let height = event.height;
            let connection_id = event.connection_id.map(|val| val.to_ibc_connection_id());
            let client_id = event.client_id;
            let counterparty_connection_id = event.counterparty_connection_id.map(|val| val.to_ibc_connection_id());
            let counterparty_client_id = event.counterparty_client_id;
            use ibc::ics03_connection::events::Attributes;
            let event = IbcEvent::OpenInitConnection(ibc::ics03_connection::events::OpenInit(Attributes {
                height: height.clone().to_ibc_height(),
                connection_id,
                client_id: client_id.to_ibc_client_id(),
                counterparty_connection_id,
                counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
            }));

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        },
        "OpenTryConnection" => {
            let event = OpenTryConnectionEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [from_raw_event_to_batch_event] >> OpenTryConnection Event");

            let height = event.height;
            let connection_id = event.connection_id.map(|val| val.to_ibc_connection_id());
            let client_id = event.client_id;
            let counterparty_connection_id = event.counterparty_connection_id.map(|val| val.to_ibc_connection_id());
            let counterparty_client_id = event.counterparty_client_id;
            use ibc::ics03_connection::events::Attributes;
            let event = IbcEvent::OpenTryConnection(ibc::ics03_connection::events::OpenTry(Attributes {
                height: height.clone().to_ibc_height(),
                connection_id,
                client_id: client_id.to_ibc_client_id(),
                counterparty_connection_id,
                counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
            }));

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        },
        "OpenAckConnection" => {
            let event = OpenAckConnectionEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [from_raw_event_to_batch_event] >> OpenAckConnection Event");

            let height = event.height;
            let connection_id = event.connection_id.map(|val| val.to_ibc_connection_id());
            let client_id = event.client_id;
            let counterparty_connection_id = event.counterparty_connection_id.map(|val| val.to_ibc_connection_id());
            let counterparty_client_id = event.counterparty_client_id;
            use ibc::ics03_connection::events::Attributes;
            let event = IbcEvent::OpenAckConnection(ibc::ics03_connection::events::OpenAck(Attributes {
                height: height.clone().to_ibc_height(),
                connection_id,
                client_id: client_id.to_ibc_client_id(),
                counterparty_connection_id,
                counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
            }));

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        },
        "OpenConfirmConnection" => {
            let event = OpenConfirmConnectionEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [from_raw_event_to_batch_event] >> OpenConfirmConnection Event");

            let height = event.height;
            let connection_id = event.connection_id.map(|val| val.to_ibc_connection_id());
            let client_id = event.client_id;
            let counterparty_connection_id = event.counterparty_connection_id.map(|val| val.to_ibc_connection_id());
            let counterparty_client_id = event.counterparty_client_id;
            use ibc::ics03_connection::events::Attributes;
            let event = IbcEvent::OpenConfirmConnection(ibc::ics03_connection::events::OpenConfirm(Attributes {
                height: height.clone().to_ibc_height(),
                connection_id,
                client_id: client_id.to_ibc_client_id(),
                counterparty_connection_id,
                counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
            }));

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }

        "OpenInitChannel" => {
            let event = OpenInitChannelEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [from_raw_event_to_batch_event] >> OpenInitChannel Event");

            let height = event.height;
            let port_id = event.port_id;
            let channel_id = event.channel_id.map(|val| val.to_ibc_channel_id());
            let connection_id = event.connection_id;
            let counterparty_port_id = event.counterparty_port_id;
            let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.to_ibc_channel_id());
            use ibc::ics04_channel::events::Attributes;
            let event = IbcEvent::OpenInitChannel(ibc::ics04_channel::events::OpenInit(Attributes{
                height: height.clone().to_ibc_height(),
                port_id: port_id.to_ibc_port_id(),
                channel_id: channel_id,
                connection_id: connection_id.to_ibc_connection_id(),
                counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                counterparty_channel_id: counterparty_channel_id,
            }));

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "OpenTryChannel" => {
            let event = OpenTryChannelEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [from_raw_event_to_batch_event] >> OpenTryChannel Event");

            let height = event.height;
            let port_id = event.port_id;
            let channel_id = event.channel_id.map(|val| val.to_ibc_channel_id());
            let connection_id = event.connection_id;
            let counterparty_port_id = event.counterparty_port_id;
            let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.to_ibc_channel_id());
            use ibc::ics04_channel::events::Attributes;
            let event = IbcEvent::OpenTryChannel(ibc::ics04_channel::events::OpenTry(Attributes{
                height: height.clone().to_ibc_height(),
                port_id: port_id.to_ibc_port_id(),
                channel_id: channel_id,
                connection_id: connection_id.to_ibc_connection_id(),
                counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                counterparty_channel_id: counterparty_channel_id,
            }));

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "OpenAckChannel" => {
            let event = OpenAckChannelEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [from_raw_event_to_batch_event] >> OpenAckChannel Event");

            let height = event.height;
            let port_id = event.port_id;
            let channel_id = event.channel_id.map(|val| val.to_ibc_channel_id());
            let connection_id = event.connection_id;
            let counterparty_port_id = event.counterparty_port_id;
            let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.to_ibc_channel_id());
            use ibc::ics04_channel::events::Attributes;
            let event = IbcEvent::OpenAckChannel(ibc::ics04_channel::events::OpenAck(Attributes{
                height: height.clone().to_ibc_height(),
                port_id: port_id.to_ibc_port_id(),
                channel_id: channel_id,
                connection_id: connection_id.to_ibc_connection_id(),
                counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                counterparty_channel_id: counterparty_channel_id,
            }));

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "OpenConfirmChannel" => {
            let event = OpenConfirmChannelEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [from_raw_event_to_batch_event] >> OpenConfirmChannel Event");

            let height = event.height;
            let port_id = event.port_id;
            let channel_id = event.channel_id.map(|val| val.to_ibc_channel_id());
            let connection_id = event.connection_id;
            let counterparty_port_id = event.counterparty_port_id;
            let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.to_ibc_channel_id());
            use ibc::ics04_channel::events::Attributes;
            let event = IbcEvent::OpenConfirmChannel(ibc::ics04_channel::events::OpenConfirm(Attributes{
                height: height.clone().to_ibc_height(),
                port_id: port_id.to_ibc_port_id(),
                channel_id: channel_id,
                connection_id: connection_id.to_ibc_connection_id(),
                counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                counterparty_channel_id: counterparty_channel_id,
            }));

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "CloseInitChannel" => {
            let event = CloseInitChannelEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [subscribe_events] >> CloseInitChannel Event");

            let height = event.height;
            let port_id = event.port_id;
            let channel_id = event.channel_id.map(|val| val.to_ibc_channel_id());
            let connection_id = event.connection_id;
            let counterparty_port_id = event.counterparty_port_id;
            let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.to_ibc_channel_id());
            use ibc::ics04_channel::events::Attributes;
            let event = IbcEvent::CloseInitChannel(ibc::ics04_channel::events::CloseInit(Attributes{
                height: height.clone().to_ibc_height(),
                port_id: port_id.to_ibc_port_id(),
                channel_id: channel_id,
                connection_id: connection_id.to_ibc_connection_id(),
                counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                counterparty_channel_id: counterparty_channel_id,
            }));

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "CloseConfirmChannel" => {
            let event = CloseConfirmChannelEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [subscribe_events] >> CloseConfirmChannel Event");

            let height = event.height;
            let port_id = event.port_id;
            let channel_id = event.channel_id.map(|val| val.to_ibc_channel_id());
            let connection_id = event.connection_id;
            let counterparty_port_id = event.counterparty_port_id;
            let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.to_ibc_channel_id());
            use ibc::ics04_channel::events::Attributes;
            let event = IbcEvent::CloseConfirmChannel(ibc::ics04_channel::events::CloseConfirm(Attributes{
                height: height.clone().to_ibc_height(),
                port_id: port_id.to_ibc_port_id(),
                channel_id: channel_id,
                connection_id: connection_id.to_ibc_connection_id(),
                counterparty_port_id: counterparty_port_id.to_ibc_port_id(),
                counterparty_channel_id: counterparty_channel_id,
            }));

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }

        }
        "SendPacket" => {
            let event = SendPacketEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [from_raw_event_to_batch_event] >> SendPacket Event");

            let height = event.height;
            let packet = event.packet;
            let event = IbcEvent::SendPacket(ibc::ics04_channel::events::SendPacket{
                height: height.clone().to_ibc_height(),
                packet: packet.to_ibc_packet(),
            });

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "ReceivePacket" => {
            let event = ReceivePacketEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [substrate_events] >> ReceivePacket Event");

            let height = event.height;
            let packet = event.packet;
            let event = IbcEvent::ReceivePacket(ibc::ics04_channel::events::ReceivePacket{
                height: height.clone().to_ibc_height(),
                packet: packet.to_ibc_packet(),
            });

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "WriteAcknowledgement" => {
            let event = WriteAcknowledgementEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [substrate_events] >> WriteAcknowledgement Event");

            let height = event.height;
            let packet = event.packet;
            let event = IbcEvent::WriteAcknowledgement(ibc::ics04_channel::events::WriteAcknowledgement{
                height: height.clone().to_ibc_height(),
                packet: packet.to_ibc_packet(),
                ack: event.ack,
            });

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }

        }
        "AcknowledgePacket" => {
            let event = AcknowledgePacketEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [substrate_events] >> AcknowledgePacket Event");

            let height = event.height;
            let packet = event.packet;
            let event = IbcEvent::AcknowledgePacket(ibc::ics04_channel::events::AcknowledgePacket{
                height: height.clone().to_ibc_height(),
                packet: packet.to_ibc_packet(),
            });

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "TimeoutPacket" => {
            let event = TimeoutPacketEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [substrate_events] >> TimeoutPacket Event");

            let height = event.height;
            let packet = event.packet;
            let event = IbcEvent::TimeoutPacket(ibc::ics04_channel::events::TimeoutPacket{
                height: height.clone().to_ibc_height(),
                packet: packet.to_ibc_packet(),
            });

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "TimeoutOnClosePacket" => {
            let event = TimeoutOnClosePacketEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [substrate_events] >> TimeoutOnClosePacket Event");

            let height = event.height;
            let packet = event.packet;
            let event = IbcEvent::TimeoutOnClosePacket(ibc::ics04_channel::events::TimeoutOnClosePacket{
                height: height.clone().to_ibc_height(),
                packet: packet.to_ibc_packet(),
            });

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "Empty" => {
            let event =  EmptyEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("in substrate: [substrate_events] >> Empty Event");

            let data = String::from_utf8(event.data).unwrap();
            let event = IbcEvent::Empty(data);

            EventBatch {
                height: Height::default(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "ChainError" => {
            let event =  ChainErrorEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("in substrate: [substrate_events] >> ChainError Event");

            let data = String::from_utf8(event.data).unwrap();
            let event = IbcEvent::Empty(data);

            EventBatch {
                height: Height::default(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "ExtrinsicSuccess" => {
            let event = ExtrinsicSuccessEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [from_raw_event_to_batch_event] >> SystemEvent: {:?}", event);

            let event = IbcEvent::NewBlock(ibc::ics02_client::events::NewBlock{
                height: Height::new(0, height)  // Todo: to set revision_number
            });

            EventBatch {
                height: Height::new(0, height),  // Todo: to set revision_number
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        _ =>  {
            tracing::info!("In substrate: [from_raw_event_to_batch_event] >> Unknown event");
            unimplemented!()
        }
    }
}


pub enum Next {
    Abort,
    Continue,
}

async fn get_latest_height(client: Client<NodeRuntime>) -> u64 {
    tracing::info!("In substrate_monitor: [get_latest_height]");
    // let mut blocks = client.subscribe_finalized_blocks().await?;
    let mut blocks = client.subscribe_blocks().await.unwrap();
    let height= match blocks.next().await {
        Ok(Some(header)) => {
            header.number as u64
        },
        Ok(None) => {
            tracing::info!("In Substrate: [get_latest_height] >> None");
            0
        },
        Err(err) =>  {
            tracing::info!(" In substrate: [get_latest_height] >> error: {:?} ", err);
            0
        },
    };
    tracing::info!("In Substrate: [get_latest_height] >> height: {:?}", height);
    height
}