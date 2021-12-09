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

use subxt::{
    ClientBuilder, PairSigner, Client,
    EventSubscription, RawEvent,
    Error as SubstrateError,
};
use calls::ibc_node;


use ibc::{events::IbcEvent, ics02_client::height::Height, ics24_host::identifier::ChainId};

use crate::util::{
    retry::{retry_count, retry_with_index, RetryResult},
    stream::try_group_while,
};

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

pub use super::monitor::Result;
pub use super::monitor::Error;

type SubscriptionResult = core::result::Result<RpcEvent, SubstrateError>;
type SubscriptionStream = dyn Stream<Item = SubscriptionResult> + Send + Sync + Unpin;



pub use super::monitor::EventSender;
pub use super::monitor::EventReceiver;
pub use super::monitor::TxMonitorCmd;
pub use super::monitor::EventBatch;
pub use super::monitor::MonitorCmd;


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
    client: Client<ibc_node::DefaultConfig>,
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
            .block_on(async move { ClientBuilder::new()
                .set_url(ws_addr)
                .build::<ibc_node::DefaultConfig>().await
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
                ClientBuilder::new()
                .set_url(format!("{}", &self.node_addr.clone()))
                .build::<ibc_node::DefaultConfig>()
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
        use core::time::Duration;
        tracing::info!("in substrate_mointor: [run_loop]");
        let client = self.client.clone();
        let chain_id = self.chain_id.clone();
        let send_batch = self.tx_batch.clone();

        let sub_event = async move {
            let api = client.clone().to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();
            let sub = api.client.rpc().subscribe_events().await.unwrap();
            let decoder = api.client.events_decoder();
            let mut sub = EventSubscription::<ibc_node::DefaultConfig>::new(sub, decoder);

            while let Some(raw_event) = sub.next().await {
                let _client = client.clone();
                let _chain_id = chain_id.clone();
                let _send_batch = send_batch.clone();

                if let Err(err) = raw_event {
                    tracing::info!("In substrate_mointor: [run_loop] >> raw_event error: {:?}", err);
                    continue;
                }

                tokio::spawn(async move {
                handle_single_event(raw_event.unwrap(), _client, _chain_id, _send_batch).await;
                });
            }
            Next::Continue
        };

        let ret = self.rt.block_on(sub_event);
        ret
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

// use calls::ibc::{
//     NewBlockEvent,
//     CreateClientEvent, OpenInitConnectionEvent, UpdateClientEvent, ClientMisbehaviourEvent,
//     OpenTryConnectionEvent, OpenAckConnectionEvent, OpenConfirmConnectionEvent,
//     OpenInitChannelEvent, OpenTryChannelEvent, OpenAckChannelEvent,
//     OpenConfirmChannelEvent,  CloseInitChannelEvent, CloseConfirmChannelEvent,
//     SendPacketEvent, ReceivePacketEvent, WriteAcknowledgementEvent,
//     AcknowledgePacketEvent, TimeoutPacketEvent, TimeoutOnClosePacketEvent,
//     EmptyEvent, ChainErrorEvent,
// };

use codec::Decode;

/// Subscribe Events
async fn subscribe_events(
    client: Client<ibc_node::DefaultConfig>,
)  -> core::result::Result<RawEvent, SubstrateError> {
    tracing::info!("In substrate_mointor: [subscribe_events]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();
    let sub = api.client.rpc().subscribe_events().await?;
    let decoder = api.client.events_decoder();
    let mut sub = EventSubscription::<ibc_node::DefaultConfig>::new(sub, decoder);

    let result = sub.next().await.unwrap();

    result
}


fn from_raw_event_to_batch_event(raw_event: RawEvent, chain_id: ChainId, height: u64) -> EventBatch {
    // tracing::info!("In substrate: [from_raw_event_to_batch_event] >> raw Event: {:?}", raw_event);
    let variant = raw_event.variant;
    // tracing::info!("In substrate: [from_raw_event_to_batch_event] >> variant: {:?}", variant);
    match variant.as_str() {
        "CreateClient" => {
            let event  = <ibc_node::ibc::events::CreateClient as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [subscribe_events] >> CreateClient Event");

            // let height = event.height;
            let height = event.0;
            // let client_id = event.client_id;
            let client_id = event.1;
            // let client_type = event.client_type;
            let client_type = event.2;

            // let consensus_height = event.consensus_height;
            let consensus_height = event.3;

            use ibc::ics02_client::events::Attributes;
            let event = IbcEvent::CreateClient(ibc::ics02_client::events::CreateClient(Attributes {
                height: height.to_ibc_height(),
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
            let event = <ibc_node::ibc::events::UpdateClient as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [subscribe_events] >> UpdateClient Event");

            // let height = event.height;
            let height = event.0;
            // let client_id = event.client_id;
            let client_id = event.1;
            // let client_type = event.client_type;
            let client_type = event.2;
            // let consensus_height = event.consensus_height;
            let consensus_height = event.3;

            use ibc::ics02_client::events::Attributes;
            let event = IbcEvent::UpdateClient(ibc::ics02_client::events::UpdateClient{
                common: Attributes {
                    height: height.to_ibc_height(),
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
            let event = <ibc_node::ibc::events::ClientMisbehaviour as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [subscribe_events] >> ClientMisbehaviour Event");

            // let height = event.height;
            let height = event.0;
            // let client_id = event.client_id;
            let client_id = event.1;
            // let client_type = event.client_type;
            let client_type = event.2;
            // let consensus_height = event.consensus_height;
            let consensus_height = event.3;

            use ibc::ics02_client::events::Attributes;
            let event = IbcEvent::ClientMisbehaviour(ibc::ics02_client::events::ClientMisbehaviour(
                Attributes {
                    height: height.to_ibc_height(),
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
            let event = <ibc_node::ibc::events::OpenInitConnection as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [subscribe_events] >> OpenInitConnection Event");

            // let height = event.height;
            let height = event.0;
            // let connection_id = event.connection_id.map(|val| val.to_ibc_connection_id());
            let connection_id = event.1.map(|val| val.to_ibc_connection_id());
            // let client_id = event.client_id;
            let client_id = event.2;
            // let counterparty_connection_id = event.counterparty_connection_id.map(|val| val.to_ibc_connection_id());
            let counterparty_connection_id = event.3.map(|val| val.to_ibc_connection_id());
            // let counterparty_client_id = event.counterparty_client_id;
            let counterparty_client_id = event.4;

            use ibc::ics03_connection::events::Attributes;
            let event = IbcEvent::OpenInitConnection(ibc::ics03_connection::events::OpenInit(Attributes {
                height: height.to_ibc_height(),
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
            let event = <ibc_node::ibc::events::OpenTryConnection as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [subscribe_events] >> OpenTryConnection Event");

            // let height = event.height;
            let height = event.0;
            // let connection_id = event.connection_id.map(|val| val.to_ibc_connection_id());
            let connection_id = event.1.map(|val| val.to_ibc_connection_id());
            // let client_id = event.client_id;
            let client_id = event.2;
            // let counterparty_connection_id = event.counterparty_connection_id.map(|val| val.to_ibc_connection_id());
            let counterparty_connection_id = event.3.map(|val| val.to_ibc_connection_id());
            // let counterparty_client_id = event.counterparty_client_id;
            let counterparty_client_id = event.4;

            use ibc::ics03_connection::events::Attributes;
            let event = IbcEvent::OpenTryConnection(ibc::ics03_connection::events::OpenTry(Attributes {
                height: height.to_ibc_height(),
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
            let event = <ibc_node::ibc::events::OpenAckConnection as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [subscribe_events] >> OpenAckConnection Event");

            // let height = event.height;
            let height = event.0;
            // let connection_id = event.connection_id.map(|val| val.to_ibc_connection_id());
            let connection_id = event.1.map(|val| val.to_ibc_connection_id());
            // let client_id = event.client_id;
            let client_id = event.2;
            // let counterparty_connection_id = event.counterparty_connection_id.map(|val| val.to_ibc_connection_id());
            let counterparty_connection_id = event.3.map(|val| val.to_ibc_connection_id());
            // let counterparty_client_id = event.counterparty_client_id;
            let counterparty_client_id = event.4;

            use ibc::ics03_connection::events::Attributes;
            let event = IbcEvent::OpenAckConnection(ibc::ics03_connection::events::OpenAck(Attributes {
                height: height.to_ibc_height(),
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
            let event = <ibc_node::ibc::events::OpenConfirmConnection as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [subscribe_events] >> OpenConfirmConnection Event");


            // let height = event.height;
            let height = event.0;
            // let connection_id = event.connection_id.map(|val| val.to_ibc_connection_id());
            let connection_id = event.1.map(|val| val.to_ibc_connection_id());
            // let client_id = event.client_id;
            let client_id = event.2;
            // let counterparty_connection_id = event.counterparty_connection_id.map(|val| val.to_ibc_connection_id());
            let counterparty_connection_id = event.3.map(|val| val.to_ibc_connection_id());
            // let counterparty_client_id = event.counterparty_client_id;
            let counterparty_client_id = event.4;

            use ibc::ics03_connection::events::Attributes;
            let event = IbcEvent::OpenConfirmConnection(ibc::ics03_connection::events::OpenConfirm(Attributes {
                height: height.to_ibc_height(),
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
            let event = <ibc_node::ibc::events::OpenInitChannel as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [subscribe_events] >> OpenInitChannel Event");

            // let height = event.height;
            let height = event.0;
            // let port_id = event.port_id;
            let port_id = event.1;
            // let channel_id = event.channel_id.map(|val| val.to_ibc_channel_id());
            let channel_id = event.2.map(|val| val.to_ibc_channel_id());
            // let connection_id = event.connection_id;
            let connection_id = event.3;
            // let counterparty_port_id = event.counterparty_port_id;
            let counterparty_port_id = event.4;
            // let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.to_ibc_channel_id());
            let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

            use ibc::ics04_channel::events::Attributes;
            let event = IbcEvent::OpenInitChannel(ibc::ics04_channel::events::OpenInit(Attributes{
                height: height.to_ibc_height(),
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
            let event = <ibc_node::ibc::events::OpenTryChannel as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [subscribe_events] >> OpenTryChannel Event");

            // let height = event.height;
            let height = event.0;
            // let port_id = event.port_id;
            let port_id = event.1;
            // let channel_id = event.channel_id.map(|val| val.to_ibc_channel_id());
            let channel_id = event.2.map(|val| val.to_ibc_channel_id());
            // let connection_id = event.connection_id;
            let connection_id = event.3;
            // let counterparty_port_id = event.counterparty_port_id;
            let counterparty_port_id = event.4;
            // let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.to_ibc_channel_id());
            let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

            use ibc::ics04_channel::events::Attributes;
            let event = IbcEvent::OpenTryChannel(ibc::ics04_channel::events::OpenTry(Attributes{
                height: height.to_ibc_height(),
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
            let event = <ibc_node::ibc::events::OpenAckChannel as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [subscribe_events] >> OpenAckChannel Event");

            // let height = event.height;
            let height = event.0;
            // let port_id = event.port_id;
            let port_id = event.1;
            // let channel_id = event.channel_id.map(|val| val.to_ibc_channel_id());
            let channel_id = event.2.map(|val| val.to_ibc_channel_id());
            // let connection_id = event.connection_id;
            let connection_id = event.3;
            // let counterparty_port_id = event.counterparty_port_id;
            let counterparty_port_id = event.4;
            // let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.to_ibc_channel_id());
            let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

            use ibc::ics04_channel::events::Attributes;
            let event = IbcEvent::OpenAckChannel(ibc::ics04_channel::events::OpenAck(Attributes{
                height: height.to_ibc_height(),
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
            let event = <ibc_node::ibc::events::OpenConfirmChannel as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [subscribe_events] >> OpenConfirmChannel Event");

            // let height = event.height;
            let height = event.0;
            // let port_id = event.port_id;
            let port_id = event.1;
            // let channel_id = event.channel_id.map(|val| val.to_ibc_channel_id());
            let channel_id = event.2.map(|val| val.to_ibc_channel_id());
            // let connection_id = event.connection_id;
            let connection_id = event.3;
            // let counterparty_port_id = event.counterparty_port_id;
            let counterparty_port_id = event.4;
            // let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.to_ibc_channel_id());
            let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

            use ibc::ics04_channel::events::Attributes;
            let event = IbcEvent::OpenConfirmChannel(ibc::ics04_channel::events::OpenConfirm(Attributes{
                height: height.to_ibc_height(),
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
            let event = <ibc_node::ibc::events::CloseInitChannel as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [subscribe_events] >> CloseInitChannel Event");

            // let height = event.height;
            let height = event.0;
            // let port_id = event.port_id;
            let port_id = event.1;
            // let channel_id = event.channel_id.map(|val| val.to_ibc_channel_id());
            let channel_id = event.2.map(|val| val.to_ibc_channel_id());
            // let connection_id = event.connection_id;
            let connection_id = event.3;
            // let counterparty_port_id = event.counterparty_port_id;
            let counterparty_port_id = event.4;
            // let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.to_ibc_channel_id());
            let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

            use ibc::ics04_channel::events::Attributes;
            let event = IbcEvent::CloseInitChannel(ibc::ics04_channel::events::CloseInit(Attributes{
                height: height.to_ibc_height(),
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
            let event = <ibc_node::ibc::events::CloseConfirmChannel as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [subscribe_events] >> CloseConfirmChannel Event");
            // let height = event.height;
            let height = event.0;
            // let port_id = event.port_id;
            let port_id = event.1;
            // let channel_id = event.channel_id.map(|val| val.to_ibc_channel_id());
            let channel_id = event.2.map(|val| val.to_ibc_channel_id());
            // let connection_id = event.connection_id;
            let connection_id = event.3;
            // let counterparty_port_id = event.counterparty_port_id;
            let counterparty_port_id = event.4;
            // let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.to_ibc_channel_id());
            let counterparty_channel_id = event.5.map(|val| val.to_ibc_channel_id());

            use ibc::ics04_channel::events::Attributes;
            let event = IbcEvent::CloseConfirmChannel(ibc::ics04_channel::events::CloseConfirm(Attributes{
                height: height.to_ibc_height(),
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
            let event = <ibc_node::ibc::events::SendPacket as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [substrate_events] >> SendPacket Event");

            // let height = event.height;
            let height = event.0;
            // let packet = event.packet;
            let packet = event.1;
            let event = IbcEvent::SendPacket(ibc::ics04_channel::events::SendPacket{
                height: height.to_ibc_height(),
                packet: packet.to_ibc_packet(),
            });

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "ReceivePacket" => {
            let event = <ibc_node::ibc::events::ReceivePacket as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [substrate_events] >> ReceivePacket Event");
            // let height = event.height;
            let height = event.0;
            // let packet = event.packet;
            let packet = event.1;

            let event = IbcEvent::ReceivePacket(ibc::ics04_channel::events::ReceivePacket{
                height: height.to_ibc_height(),
                packet: packet.to_ibc_packet(),
            });

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "WriteAcknowledgement" => {
            let event = <ibc_node::ibc::events::WriteAcknowledgement as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [substrate_events] >> WriteAcknowledgement Event");

            // let height = event.height;
            let height = event.0;
            // let packet = event.packet;
            let packet = event.1;

            let ack = event.2;

            let event = IbcEvent::WriteAcknowledgement(ibc::ics04_channel::events::WriteAcknowledgement{
                height: height.to_ibc_height(),
                packet: packet.to_ibc_packet(),
                ack: ack,
            });

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }

        }
        "AcknowledgePacket" => {
            let event = <ibc_node::ibc::events::AcknowledgePacket as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [substrate_events] >> AcknowledgePacket Event");

            // let height = event.height;
            let height = event.0;
            // let packet = event.packet;
            let packet = event.1;

            let event = IbcEvent::AcknowledgePacket(ibc::ics04_channel::events::AcknowledgePacket{
                height: height.to_ibc_height(),
                packet: packet.to_ibc_packet(),
            });

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "TimeoutPacket" => {
            let event = <ibc_node::ibc::events::TimeoutPacket as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [substrate_events] >> TimeoutPacket Event");

            // let height = event.height;
            let height = event.0;
            // let packet = event.packet;
            let packet = event.1;

            let event = IbcEvent::TimeoutPacket(ibc::ics04_channel::events::TimeoutPacket{
                height: height.to_ibc_height(),
                packet: packet.to_ibc_packet(),
            });

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "TimeoutOnClosePacket" => {
            let event = <ibc_node::ibc::events::TimeoutOnClosePacket as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [substrate_events] >> TimeoutOnClosePacket Event");

            // let height = event.height;
            let height = event.0;
            // let packet = event.packet;
            let packet = event.1;

            let event = IbcEvent::TimeoutOnClosePacket(ibc::ics04_channel::events::TimeoutOnClosePacket{
                height: height.to_ibc_height(),
                packet: packet.to_ibc_packet(),
            });

            EventBatch {
                height: height.to_ibc_height(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "Empty" => {
            let event =  <ibc_node::ibc::events::Empty as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("in substrate: [substrate_events] >> Empty Event");

            let data = String::from_utf8(event.0).unwrap();

            let event = IbcEvent::Empty(data);

            EventBatch {
                height: Height::default(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "ChainError" => {
            let event =  <ibc_node::ibc::events::ChainError as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("in substrate: [substrate_events] >> ChainError Event");

            let data = String::from_utf8(event.0).unwrap();

            let event = IbcEvent::Empty(data);

            EventBatch {
                height: Height::default(),
                events: vec![event],
                chain_id: chain_id.clone(),
            }
        }
        "ExtrinsicSuccess" => {
            let event = <ibc_node::system::events::ExtrinsicSuccess as codec::Decode>::decode(&mut &raw_event.data[..]).unwrap();
            tracing::info!("In substrate: [subscribe_events] >> ExtrinsicSuccess Event");

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
            // tracing::info!("In substrate: [from_raw_event_to_batch_event]: unknown event");

            EventBatch {
                height: Height::new(0, height),  // Todo: to set revision_number
                events: vec![],
                chain_id: chain_id.clone(),
            }

        }
    }
}


pub enum Next {
    Abort,
    Continue,
}

async fn get_latest_height(client: Client<ibc_node::DefaultConfig>) -> u64 {
    tracing::info!("In substrate_monitor: [get_latest_height]");

    let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

    let mut block = api
        .client
        .rpc()
        .subscribe_blocks()
        .await.unwrap();

    let height= match block.next().await {
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

async fn handle_single_event(raw_event: RawEvent, client: Client<ibc_node::DefaultConfig>,
                             chain_id: ChainId, send_batch: channel::Sender<Result<EventBatch>>) {
    tracing::info!("in substrate_mointor: [run_loop] >> raw_event : {:?}", raw_event);
    let height = get_latest_height(client).await; // Todo: Do not query for latest height every time
    let batch_event = from_raw_event_to_batch_event(raw_event, chain_id.clone(), height);
    process_batch_for_substrate(send_batch.clone(), batch_event).unwrap_or_else(|e| {
        error!("[{}] {}", chain_id, e);
    });
}
