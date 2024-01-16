use std::sync::Arc;

use crate::chain::near::contract::NearIbcContract;
use crate::chain::near::error::NearError;
use crate::chain::near::rpc::client::NearRpcClient;
use crate::chain::near::rpc::tool::convert_ibc_event_to_hermes_ibc_event;
use crossbeam_channel as channel;
use near_primitives::types::AccountId;
use serde_json::json;
use tokio::{
    runtime::Runtime as TokioRuntime,
    time::{sleep, Duration, Instant},
};
use tracing::{debug, error, error_span, trace};

use ibc_relayer_types::{
    core::{
        ics02_client::{events::NewBlock, height::Height},
        ics24_host::identifier::ChainId,
    },
    events::IbcEvent,
};

use crate::{
    chain::tracking::TrackingId,
    event::{bus::EventBus, source::Error, IbcEventWithHeight},
    telemetry,
    util::retry::ConstantGrowth,
};

use crate::event::source::{EventBatch, EventSourceCmd, TxEventSourceCmd};

pub type Result<T> = core::result::Result<T, Error>;

/// An RPC endpoint that serves as a source of events for a given chain.
pub struct EventSource {
    /// Chain identifier
    chain_id: ChainId,

    /// near ibc address
    near_ibc_address: AccountId,

    /// The NEAR rpc client to collect IBC events
    rpc_client: NearRpcClient,

    /// Poll interval
    poll_interval: Duration,

    /// Event bus for broadcasting events
    event_bus: EventBus<Arc<Result<EventBatch>>>,

    /// Channel where to receive commands
    rx_cmd: channel::Receiver<EventSourceCmd>,

    /// Tokio runtime
    rt: Arc<TokioRuntime>,

    /// Last fetched block height
    last_fetched_height: Height,
}

impl NearIbcContract for EventSource {
    type Error = NearError;

    fn get_contract_id(&self) -> AccountId {
        self.near_ibc_address.clone()
    }

    fn get_client(&self) -> &NearRpcClient {
        &self.rpc_client
    }

    fn get_rt(&self) -> &Arc<TokioRuntime> {
        &self.rt
    }
}

impl EventSource {
    pub fn new(
        chain_id: ChainId,
        near_ibc_address: AccountId,
        rpc_client: NearRpcClient,
        poll_interval: Duration,
        rt: Arc<TokioRuntime>,
    ) -> Result<(Self, TxEventSourceCmd)> {
        let event_bus = EventBus::new();
        let (tx_cmd, rx_cmd) = channel::unbounded();

        let source = Self {
            rt,
            chain_id,
            near_ibc_address,
            rpc_client,
            poll_interval,
            event_bus,
            rx_cmd,
            last_fetched_height: Height::new(0, 1).unwrap(), // ibc height starts from 1 height is not zero
        };

        Ok((source, TxEventSourceCmd(tx_cmd)))
    }

    pub fn run(mut self) {
        let _span = error_span!("event_source.rpc", chain.id = %self.chain_id).entered();

        debug!("collecting events");

        let rt = self.rt.clone();

        rt.block_on(async {
            let mut backoff = poll_backoff(self.poll_interval);

            // Initialize the latest fetched height
            if let Ok(latest_height) = self.latest_height().await {
                self.last_fetched_height = latest_height;
            }

            // Continuously run the event loop, so that when it aborts
            // because of WebSocket client restart, we pick up the work again.
            loop {
                let before_step = Instant::now();

                match self.step().await {
                    Ok(Next::Abort) => break,

                    Ok(Next::Continue) => {
                        // Reset the backoff
                        backoff = poll_backoff(self.poll_interval);

                        // Check if we need to wait some more before the next iteration.
                        let delay = self.poll_interval.checked_sub(before_step.elapsed());

                        if let Some(delay_remaining) = delay {
                            sleep(delay_remaining).await;
                        }

                        continue;
                    }

                    Err(e) => {
                        error!("event source encountered an error: {e}");

                        // Let's backoff the little bit to give the chain some time to recover.
                        let delay = backoff.next().expect("backoff is an infinite iterator");

                        error!("retrying in {delay:?}...");
                        sleep(delay).await;
                    }
                }
            }
        });

        debug!("shutting down event source");
    }

    async fn step(&mut self) -> Result<Next> {
        // Process any shutdown or subscription commands before we start doing any work
        if let Next::Abort = self.try_process_cmd() {
            return Ok(Next::Abort);
        }

        let latest_height = self.latest_height().await?;

        let batches = if latest_height > self.last_fetched_height {
            trace!(
                "latest height ({latest_height}) > latest fetched height ({})",
                self.last_fetched_height
            );

            self.fetch_batches(latest_height).await.map(Some)?
        } else {
            trace!(
                "latest height ({latest_height}) <= latest fetched height ({})",
                self.last_fetched_height
            );

            None
        };

        // Before handling the batch, check if there are any pending shutdown or subscribe commands.
        //
        // This avoids having the supervisor process an event batch after the event source has been shutdown.
        //
        // It also allows subscribers to receive the latest event batch even if they
        // subscribe while the batch being fetched.
        if let Next::Abort = self.try_process_cmd() {
            return Ok(Next::Abort);
        }

        for batch in batches.unwrap_or_default() {
            self.broadcast_batch(batch);
        }

        Ok(Next::Continue)
    }

    /// Process any pending commands, if any.
    fn try_process_cmd(&mut self) -> Next {
        if let Ok(cmd) = self.rx_cmd.try_recv() {
            match cmd {
                EventSourceCmd::Shutdown => return Next::Abort,

                EventSourceCmd::Subscribe(tx) => {
                    if let Err(e) = tx.send(self.event_bus.subscribe()) {
                        error!("failed to send back subscription: {e}");
                    }
                }
            }
        }

        Next::Continue
    }

    async fn fetch_batches(&mut self, latest_height: Height) -> Result<Vec<EventBatch>> {
        let start_height = self.last_fetched_height.increment();

        trace!("fetching blocks from {start_height} to {latest_height}");

        let heights = HeightRangeInclusive::new(start_height, latest_height);
        let mut batches = Vec::with_capacity(heights.len());

        for height in heights {
            trace!("collecting events at height {height}");

            let result = self.collect_events(&self.chain_id, height).await;

            match result {
                Ok(batch) => {
                    self.last_fetched_height = height;

                    if let Some(batch) = batch {
                        batches.push(batch);
                    }
                }
                Err(e) => {
                    error!(%height, "failed to collect events: {e}");
                    break;
                }
            }
        }

        Ok(batches)
    }

    /// Collect the IBC events from the subscriptions
    fn broadcast_batch(&mut self, batch: EventBatch) {
        telemetry!(ws_events, &batch.chain_id, batch.events.len() as u64);

        trace!(
            chain = %batch.chain_id,
            count = %batch.events.len(),
            height = %batch.height,
            "broadcasting batch of {} events",
            batch.events.len()
        );

        self.event_bus.broadcast(Arc::new(Ok(batch)));
    }

    /// Collect the IBC events from an RPC event
    async fn collect_events(
        &self,
        chain_id: &ChainId,
        latest_block_height: Height,
    ) -> Result<Option<EventBatch>> {
        let near_events = self.query_events_at_height(&latest_block_height).await?;
        let mut block_events: Vec<IbcEventWithHeight> = near_events
            .into_iter()
            .filter(|e| {
                matches!(
                    e.event,
                    IbcEvent::SendPacket(_)
                        | IbcEvent::ReceivePacket(_)
                        | IbcEvent::WriteAcknowledgement(_)
                        | IbcEvent::AcknowledgePacket(_)
                        | IbcEvent::TimeoutPacket(_)
                        | IbcEvent::TimeoutOnClosePacket(_)
                )
            })
            .collect();

        let new_block_event = IbcEventWithHeight::new(
            IbcEvent::NewBlock(NewBlock::new(latest_block_height)),
            latest_block_height,
        );

        let mut events = Vec::with_capacity(block_events.len() + 1);
        events.push(new_block_event);
        events.append(&mut block_events);

        trace!(
            "collected {events_len} events at height {height}: {events:#?}",
            events_len = events.len(),
            height = latest_block_height,
        );

        Ok(Some(EventBatch {
            chain_id: chain_id.clone(),
            tracking_id: TrackingId::new_uuid(),
            height: latest_block_height,
            events,
        }))
    }

    async fn query_events_at_height(&self, height: &Height) -> Result<Vec<IbcEventWithHeight>> {
        let events_result = self
            .get_client()
            .view(
                self.get_contract_id(),
                "get_ibc_events_at".to_string(),
                json!({ "height": height }).to_string().into_bytes(),
            )
            .await
            .map(|result| {
                let ibc_events: Vec<ibc::core::handler::types::events::IbcEvent> =
                    result.json().map_err(|e| {
                        Error::custom_error(format!(
                            "failed to parse ibc events: {}",
                            e.to_string()
                        ))
                    })?;
                let events = ibc_events
                    .iter()
                    .map(|event| {
                        let event = convert_ibc_event_to_hermes_ibc_event(event).map_err(|e| {
                            Error::custom_error(format!("failed to convert ibc event: {}", e))
                        })?;
                        Ok(IbcEventWithHeight {
                            height: *height,
                            event,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;

                Ok(events)
            })
            .map_err(|e| {
                Error::custom_error(format!(
                    "failed to query events at height {}: Error({})",
                    height.revision_height(),
                    e
                ))
            })?;
        events_result
    }

    async fn latest_height(&self) -> Result<Height> {
        self.get_client()
            .view(
                self.get_contract_id(),
                "get_latest_height".to_string(),
                json!({}).to_string().into_bytes(),
            )
            .await
            .map(|result| {
                let height: Height = result.json().map_err(|e| {
                    Error::custom_error(format!("failed to parse ibc events: {}", e.to_string()))
                })?;
                Ok(height)
            })
            .map_err(|e| {
                Error::custom_error(format!("failed to get latest height: Error({})", e))
            })?
    }
}

fn poll_backoff(poll_interval: Duration) -> impl Iterator<Item = Duration> {
    ConstantGrowth::new(poll_interval, Duration::from_millis(500))
        .clamp(poll_interval * 5, usize::MAX)
}

pub enum Next {
    Abort,
    Continue,
}

pub struct HeightRangeInclusive {
    current: Height,
    end: Height,
}

impl HeightRangeInclusive {
    pub fn new(start: Height, end: Height) -> Self {
        Self {
            current: start,
            end,
        }
    }
}

impl Iterator for HeightRangeInclusive {
    type Item = Height;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            None
        } else {
            let current = self.current;
            self.current = self.current.increment();
            Some(current)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.end.revision_height() - self.current.revision_height() + 1;
        (size as usize, Some(size as usize))
    }
}

impl ExactSizeIterator for HeightRangeInclusive {}
