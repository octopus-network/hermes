use crate::util::retry::{retry_count, retry_with_index, RetryResult};

use crate::chain::substrate::REVISION_NUMBER;
use crate::chain::tracking::TrackingId;
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
use crate::chain::substrate::config::MyConfig;
use crate::chain::substrate::config::SubstrateNodeTemplateExtrinsicParams;
use subxt::{Client, ClientBuilder, RawEventDetails};
use tendermint_rpc::{event::Event as RpcEvent, Url};
use tokio::{runtime::Runtime as TokioRuntime, sync::mpsc};
use tracing::{debug, error, info, trace};

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
            client: Client<MyConfig>,
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
        info!("in substrate_mointor: [run_loop]");

        let client = self.client.clone();
        let chain_id = self.chain_id.clone();
        let send_batch = self.tx_batch.clone();

        let sub_event = async move {
            let api = client
            .clone()
            .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

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
        info!("in substrate_mointor: [propagate_error]");

        self.tx_batch
        .send(Err(error))
        .map_err(|_| Error::channel_send_failed())?;

        Ok(())
    }

    /// Collect the IBC events from the subscriptions
    fn process_batch(&self, batch: EventBatch) -> Result<()> {
        trace!("in substrate_mointor: [process_batch]");

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
    trace!("in substrate_mointor: [relayer_process_channel_events]");

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
    trace!("in substrate_mointor: [collect_events]");

    let events = crate::event::rpc::get_all_events(chain_id, event).unwrap_or_default();
    stream::iter(events).map(Ok)
}

/// Sort the given events by putting the NewBlock event first,
/// and leaving the other events as is.
fn sort_events(events: &mut [IbcEvent]) {
    trace!("in substrate_mointor: [sort_events]");

    events.sort_by(|a, b| match (a, b) {
        (IbcEvent::NewBlock(_), _) => Ordering::Less,
        _ => Ordering::Equal,
    })
}

/// Subscribe Events
async fn subscribe_events(client: Client<MyConfig>) -> RawEventDetails {
    info!("In substrate_monitor: [subscribe_events]");

    let api = client
    .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();
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
        height: u64,
        ) -> Result<EventBatch> {
    trace!(
            "In substrate: [from_raw_event_to_batch_event] >> raw Event: {:?}",
    raw_event
    );
    let variant = raw_event.variant;
    match variant.as_str() {
        "CreateClient" => {
            let event = <ibc_node::ibc::events::CreateClient as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;

            trace!("In substrate_monitor: [subscribe_events] >> CreateClient Event");


            let client_id = event.client_id;
            let client_type = event.client_type;
            let consensus_height = event.consensus_height;

            use ibc_relayer_types::core::ics02_client::events::Attributes;
            let event = IbcEvent::CreateClient(
                    ibc_relayer_types::core::ics02_client::events::CreateClient::from(Attributes {

                        client_id: client_id.into(),
                        client_type: client_type.into(),
                        consensus_height: consensus_height.into(),
                    }),
            );

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "UpdateClient" => {
            let event = <ibc_node::ibc::events::UpdateClient as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [subscribe_events] >> UpdateClient Event");

            let height = event.height;
            let client_id = event.client_id;
            let client_type = event.client_type;
            let consensus_height = event.consensus_height;

            use ibc_relayer_types::core::ics02_client::events::Attributes;
            let event = IbcEvent::UpdateClient(
                    ibc_relayer_types::core::ics02_client::events::UpdateClient::from(Attributes {

                        client_id: client_id.into(),
                        client_type: client_type.into(),
                        consensus_height: consensus_height.into(),
                    }),
            );
            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "ClientMisbehaviour" => {
            let event = <ibc_node::ibc::events::ClientMisbehaviour as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [subscribe_events] >> ClientMisbehaviour Event");

            let height = event.height;
            let client_id = event.client_id;
            let client_type = event.client_type;
            let consensus_height = event.consensus_height;

            use ibc_relayer_types::core::ics02_client::events::Attributes;
            let event = IbcEvent::ClientMisbehaviour(
                    ibc_relayer_types::core::ics02_client::events::ClientMisbehaviour::from(Attributes {

                        client_id: client_id.into(),
                        client_type: client_type.into(),
                        consensus_height: consensus_height.into(),
                    }),
            );

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "OpenInitConnection" => {
            let event = <ibc_node::ibc::events::OpenInitConnection as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [subscribe_events] >> OpenInitConnection Event");

            let height = event.height;
            let connection_id = event.connection_id.map(|val| val.into());
            let client_id = event.client_id;
            let counterparty_connection_id = event.counterparty_connection_id.map(|val| val.into());
            let counterparty_client_id = event.counterparty_client_id;

            use ibc_relayer_types::core::ics03_connection::events::Attributes;
            let event = IbcEvent::OpenInitConnection(
                    ibc_relayer_types::core::ics03_connection::events::OpenInit::from(Attributes {

                        connection_id,
                        client_id: client_id.into(),
                        counterparty_connection_id,
                        counterparty_client_id: counterparty_client_id.into(),
                    }),
            );

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "OpenTryConnection" => {
            let event = <ibc_node::ibc::events::OpenTryConnection as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [subscribe_events] >> OpenTryConnection Event");

            let height = event.height;
            let connection_id = event.connection_id.map(|val| val.into());
            let client_id = event.client_id;
            let counterparty_connection_id = event.counterparty_connection_id.map(|val| val.into());
            let counterparty_client_id = event.counterparty_client_id;

            use ibc_relayer_types::core::ics03_connection::events::Attributes;
            let event = IbcEvent::OpenTryConnection(
                    ibc_relayer_types::core::ics03_connection::events::OpenTry::from(Attributes {

                        connection_id,
                        client_id: client_id.into(),
                        counterparty_connection_id,
                        counterparty_client_id: counterparty_client_id.into(),
                    }),
            );

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "OpenAckConnection" => {
            let event = <ibc_node::ibc::events::OpenAckConnection as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [subscribe_events] >> OpenAckConnection Event");


            let connection_id = event.connection_id.map(|val| val.into());
            let client_id = event.client_id;
            let counterparty_connection_id = event.counterparty_connection_id.map(|val| val.into());
            let counterparty_client_id = event.counterparty_client_id;

            use ibc_relayer_types::core::ics03_connection::events::Attributes;
            let event = IbcEvent::OpenAckConnection(
                    ibc_relayer_types::core::ics03_connection::events::OpenAck::from(Attributes {

                        connection_id,
                        client_id: client_id.into(),
                        counterparty_connection_id,
                        counterparty_client_id: counterparty_client_id.into(),
                    }),
            );

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "OpenConfirmConnection" => {
            let event = <ibc_node::ibc::events::OpenConfirmConnection as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [subscribe_events] >> OpenConfirmConnection Event");

            let height = event.height;
            let connection_id = event.connection_id.map(|val| val.into());
            let client_id = event.client_id;
            let counterparty_connection_id = event.counterparty_connection_id.map(|val| val.into());
            let counterparty_client_id = event.counterparty_client_id;

            use ibc_relayer_types::core::ics03_connection::events::Attributes;
            let event = IbcEvent::OpenConfirmConnection(
                    ibc_relayer_types::core::ics03_connection::events::OpenConfirm::from(Attributes {
                        height: height.into(),
                        connection_id,
                        client_id: client_id.into(),
                        counterparty_connection_id,
                        counterparty_client_id: counterparty_client_id.into(),
                    }),
            );

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }

        "OpenInitChannel" => {
            let event = <ibc_node::ibc::events::OpenInitChannel as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [subscribe_events] >> OpenInitChannel Event");

            let height = event.height;
            let port_id = event.port_id;
            let channel_id = event.channel_id.map(|val| val.into());
            let connection_id = event.connection_id;
            let counterparty_port_id = event.counterparty_port_id;
            let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.into());

            let event = IbcEvent::OpenInitChannel(ibc::core::ics04_channel::events::OpenInit {
                height: height.into(),
                port_id: port_id.into(),
                channel_id,
                connection_id: connection_id.into(),
                counterparty_port_id: counterparty_port_id.into(),
                counterparty_channel_id,
            });

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "OpenTryChannel" => {
            let event = <ibc_node::ibc::events::OpenTryChannel as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [subscribe_events] >> OpenTryChannel Event");

            let port_id = event.port_id;
            let channel_id = event.channel_id.map(|val| val.into());
            let connection_id = event.connection_id;
            let counterparty_port_id = event.counterparty_port_id;
            let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.into());

            let event = IbcEvent::OpenTryChannel(ibc_relayer_types::core::ics04_channel::events::OpenTry {

                port_id: port_id.into(),
                channel_id,
                connection_id: connection_id.into(),
                counterparty_port_id: counterparty_port_id.into(),
                counterparty_channel_id,
            });

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "OpenAckChannel" => {
            let event = <ibc_node::ibc::events::OpenAckChannel as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [subscribe_events] >> OpenAckChannel Event");


            let port_id = event.port_id;
            let channel_id = event.channel_id.map(|val| val.into());
            let connection_id = event.connection_id;
            let counterparty_port_id = event.counterparty_port_id;
            let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.into());

            let event = IbcEvent::OpenAckChannel(ibc_relayer_types::core::ics04_channel::events::OpenAck {

                port_id: port_id.into(),
                channel_id,
                connection_id: connection_id.into(),
                counterparty_port_id: counterparty_port_id.into(),
                counterparty_channel_id,
            });

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "OpenConfirmChannel" => {
            let event = <ibc_node::ibc::events::OpenConfirmChannel as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [subscribe_events] >> OpenConfirmChannel Event");

            let port_id = event.port_id;
            let channel_id = event.channel_id.map(|val| val.into());
            let connection_id = event.connection_id;
            let counterparty_port_id = event.counterparty_port_id;
            let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.into());

            let event =
            IbcEvent::OpenConfirmChannel(ibc_relayer_types::core::ics04_channel::events::OpenConfirm {

                port_id: port_id.into(),
                channel_id,
                connection_id: connection_id.into(),
                counterparty_port_id: counterparty_port_id.into(),
                counterparty_channel_id,
            });

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "CloseInitChannel" => {
            let event = <ibc_node::ibc::events::CloseInitChannel as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [subscribe_events] >> CloseInitChannel Event");


            let port_id = event.port_id;
            let channel_id = event.channel_id.map(|val| val.into());
            let connection_id = event.connection_id;
            let counterparty_port_id = event.counterparty_port_id;
            let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.into());

            let event = IbcEvent::CloseInitChannel(ibc_relayer_types::core::ics04_channel::events::CloseInit {

                port_id: port_id.into(),
                channel_id: channel_id.unwrap_or_default(),
                connection_id: connection_id.into(),
                counterparty_port_id: counterparty_port_id.into(),
                counterparty_channel_id,
            });

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "CloseConfirmChannel" => {
            let event = <ibc_node::ibc::events::CloseConfirmChannel as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [subscribe_events] >> CloseConfirmChannel Event");


            let port_id = event.port_id;
            let channel_id = event.channel_id.map(|val| val.into());
            let connection_id = event.connection_id;
            let counterparty_port_id = event.counterparty_port_id;
            let counterparty_channel_id = event.counterparty_channel_id.map(|val| val.into());

            let event =
            IbcEvent::CloseConfirmChannel(ibc_relayer_types::core::ics04_channel::events::CloseConfirm {
                port_id: port_id.into(),
                channel_id,
                connection_id: connection_id.into(),
                counterparty_port_id: counterparty_port_id.into(),
                counterparty_channel_id,
            });

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "SendPacket" => {
            let event = <ibc_node::ibc::events::SendPacket as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [substrate_events] >> SendPacket Event");

            let packet = event.packet;

            let event = IbcEvent::SendPacket(ibc_relayer_types::core::ics04_channel::events::SendPacket {

                packet: packet.into(),
            });

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "ReceivePacket" => {
            let event = <ibc_node::ibc::events::ReceivePacket as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [substrate_events] >> ReceivePacket Event");

            let packet = event.packet;

            let event = IbcEvent::ReceivePacket(ibc_relayer_types::core::ics04_channel::events::ReceivePacket {

                packet: packet.into(),
            });

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "WriteAcknowledgement" => {
            let event = <ibc_node::ibc::events::WriteAcknowledgement as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [substrate_events] >> WriteAcknowledgement Event");


            let packet = event.packet;
            let ack = event.ack;

            let event = IbcEvent::WriteAcknowledgement(
                    ibc_relayer_types::core::ics04_channel::events::WriteAcknowledgement {
                        packet: packet.into(),
                        ack,
                    },
            );

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "AcknowledgePacket" => {
            let event = <ibc_node::ibc::events::AcknowledgePacket as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [substrate_events] >> AcknowledgePacket Event");


            let packet = event.packet;

            let event =
            IbcEvent::AcknowledgePacket(ibc_relayer_types::core::ics04_channel::events::AcknowledgePacket {
                packet: packet.into(),
            });

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "TimeoutPacket" => {
            let event = <ibc_node::ibc::events::TimeoutPacket as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [substrate_events] >> TimeoutPacket Event");

            let packet = event.packet;

            let event = IbcEvent::TimeoutPacket(ibc_relayer_types::core::ics04_channel::events::TimeoutPacket {

                packet: packet.into(),
            });

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "TimeoutOnClosePacket" => {
            let event = <ibc_node::ibc::events::TimeoutOnClosePacket as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [substrate_events] >> TimeoutOnClosePacket Event");


            let packet = event.packet;

            let event = IbcEvent::TimeoutOnClosePacket(
                    ibc_relayer_types::core::ics04_channel::events::TimeoutOnClosePacket {

                        packet: packet.into(),
                    },
            );

            Ok(EventBatch {
                height: height.into(),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "AppModule" => {
            let event = <ibc_node::ibc::events::AppModule as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .unwrap();

            let app_module = ibc_relayer_types::events::ModuleEvent {
                kind: String::from_utf8(event.0.kind).expect("convert kind error"),
                module_name: event.0.module_name.into(),
                attributes: event
                .0
                .attributes
                .into_iter()
                .map(|attribute| attribute.into())
                .collect(),
            };

            let event = IbcEvent::AppModule(app_module);

            Ok(EventBatch {
                height: Height::new(REVISION_NUMBER, height).expect("REVISION_NUMBER"),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "ChainError" => {
            let event = <ibc_node::ibc::events::ChainError as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("in substrate_monitor: [substrate_events] >> ChainError Event");

            let data = String::from_utf8(event.0).map_err(|_| Error::invalid_from_utf8())?;

            let event = IbcEvent::ChainError(data);

            Ok(EventBatch {
                height: Height::new(REVISION_NUMBER, height).expect("REVISION_NUMBER"),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        "ExtrinsicSuccess" => {
            let event = <ibc_node::system::events::ExtrinsicSuccess as codec::Decode>::decode(
                    &mut &raw_event.data[..],
            )
            .map_err(Error::invalid_codec_decode)?;
            trace!("In substrate_monitor: [subscribe_events] >> ExtrinsicSuccess Event");

            let event = IbcEvent::NewBlock(ibc_relayer_types::core::ics02_client::events::NewBlock {
                height: Height::new(REVISION_NUMBER, height).expect("REVISION_NUMBER"),
            });

            Ok(EventBatch {
                height: Height::new(REVISION_NUMBER, height).expect("REVISION_NUMBER"),
                events: vec![event],
                chain_id,
                tracking_id: TrackingId::new_uuid(),
            })
        }
        _ => Ok(EventBatch {
            height: Height::new(REVISION_NUMBER, height).expect("REVISION_NUMBER"),
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

async fn get_latest_height(client: Client<MyConfig>) -> u64 {
    trace!("In substrate_monitor: [get_latest_height]");

    let api = client
    .to_runtime_api::<ibc_node::RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

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
        client: Client<MyConfig>,
        chain_id: ChainId,
        send_batch: channel::Sender<Result<EventBatch>>,
        ) {
    trace!("in substrate_monitor: [handle_single_event]");

    let height = get_latest_height(client).await; // Todo: Do not query for latest height every time
    let batch_event = from_raw_event_to_batch_event(raw_event, chain_id.clone(), height);
    if let Ok(batch_event) = batch_event {
        if !batch_event.events.is_empty() {
            process_batch_for_substrate(send_batch.clone(), batch_event).unwrap_or_else(|e| {
                error!("[{}] {}", chain_id, e);
            });
        }
    } else {
        trace!("in substrate monitor:handle_single_event from_raw_event_to_batch_event error");
    }
}
