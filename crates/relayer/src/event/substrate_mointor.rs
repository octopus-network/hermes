use crate::util::retry::{retry_with_index, RetryResult};
use codec::{Decode, Encode};
use crate::chain::substrate::REVISION_NUMBER;
use crate::chain::tracking::TrackingId;
use alloc::sync::Arc;
use core::cmp::Ordering;
use crossbeam_channel as channel;
use futures::{
    stream::{self, StreamExt},
    Stream,
};
use ibc_relayer_types::core::ics02_client::events::NewBlock;

use std::{future::Future, pin::Pin};

use ibc_relayer_types::{
    core::{ics02_client::height::Height, ics24_host::identifier::ChainId},
    events::IbcEvent,
};
use ibc_relayer_types::core::ics04_channel::events::SendPacket;

use super::{bus::EventBus, IbcEventWithHeight};
use tendermint_rpc::{event::Event as RpcEvent, Url};
use tokio::{runtime::Runtime as TokioRuntime, sync::mpsc};
use tracing::{debug, error, info, instrument, trace};

use subxt::blocks::Block;
use subxt::blocks::BlocksClient;
use subxt::error::RpcError as SubxtRpcError;
use subxt::rpc::RpcClient as SubxtRpcClient;
use subxt::rpc::Subscription as SubxtSubscription;
use subxt::rpc_params;
use subxt::Error as SubxtError;
use subxt::{tx::PairSigner, OnlineClient, PolkadotConfig, SubstrateConfig};
use crate::error::Error;
use serde::{Deserialize, Serialize};
use subxt::rpc::types::BlockNumber;

use crate::chain::substrate::relaychain::relaychain_node as ibc_node;
use crate::chain::substrate::utils;
use crate::chain::substrate::RpcClient;
use crate::telemetry;
// use tendermint_rpc::error::ErrorDetail;
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

// mod error;
// pub use error::*;
pub use super::monitor::error::*;
pub use super::monitor::error::Error as MonitorError;
pub use super::monitor::EventBatch;
pub use super::monitor::EventReceiver;
pub use super::monitor::EventSender;
pub use super::monitor::Next;
pub use super::monitor::Result;



/// Connect to a substrate node, subscribe ibc event,
/// receive push events over a websocket, and filter them for the
/// event handler.
///

type BlockStream<T> = Pin<Box<dyn Stream<Item = core::result::Result<T, SubxtError>> + Send>>;
type BlockStreamRes<T> = core::result::Result<BlockStream<T>, SubxtError>;
// pub type Result<T> = core::result::Result<T, SubxtError>;

pub type Subscription = channel::Receiver<Arc<Result<EventBatch>>>;

#[derive(Clone, Debug)]
pub struct TxMonitorCmd(pub channel::Sender<MonitorCmd>);

impl TxMonitorCmd {
    pub fn shutdown(&self) -> Result<()> {
        self.0
            .send(MonitorCmd::Shutdown)
            .map_err(|_| MonitorError::channel_send_failed())
    }

    pub fn subscribe(&self) -> Result<Subscription> {
        let (tx, rx) = crossbeam_channel::bounded(1);

        self.0
            .send(MonitorCmd::Subscribe(tx))
            .map_err(|_| MonitorError::channel_send_failed())?;

        let subscription = rx
            .recv()
            .map_err(|_| MonitorError::channel_recv_failed())?;
        Ok(subscription)
    }
}

#[derive(Debug)]
pub enum MonitorCmd {
    Shutdown,
    Subscribe(channel::Sender<Subscription>),
}

pub struct EventMonitor {
    chain_id: ChainId,
    ibc_node_addr: Url,
    /// WebSocket to  substrate chain integrated pallet-ibc
    ibc_rpc_client: OnlineClient<SubstrateConfig>,
    /// Event bus for broadcasting events
    event_bus: EventBus<Arc<Result<EventBatch>>>,
    /// Channel where to receive client driver errors
    rx_err: mpsc::UnboundedReceiver<SubxtError>,
    /// Channel where to send client driver errors
    tx_err: mpsc::UnboundedSender<SubxtError>,
    /// Channel where to receive commands
    rx_cmd: channel::Receiver<MonitorCmd>,
    /// block stream subscription
    block_sub: BlockStream<Block<SubstrateConfig, OnlineClient<SubstrateConfig>>>,
    /// Tokio runtime
    rt: Arc<TokioRuntime>,
}

impl EventMonitor {
    /// Create an event monitor, and connect to a node
    pub fn new(
        chain_id: ChainId,
        ibc_node_addr: Url,
        rt: Arc<TokioRuntime>,
    ) -> Result<(Self, TxMonitorCmd)> {
        let event_bus = EventBus::new();
        let (tx_cmd, rx_cmd) = channel::unbounded();

        let (tx_err, rx_err) = mpsc::unbounded_channel();

        let ibc_rpc_client = rt
            .block_on(OnlineClient::<SubstrateConfig>::from_url(
                ibc_node_addr.to_string(),
            ))
            .unwrap();
       

        let monitor = Self {
            rt,
            chain_id,
            ibc_node_addr,
            ibc_rpc_client,
            event_bus,
            rx_err,
            tx_err,
            rx_cmd,
            block_sub: Box::pin(futures::stream::empty()),
        };

        Ok((monitor, TxMonitorCmd(tx_cmd)))
    }

    #[instrument(name = "event_monitor.init_subscriptions", skip_all, fields(chain = %self.chain_id))]
    pub fn init_subscriptions(&mut self) -> Result<()> {
        // subcribe ibc event

        self.block_sub = self
            .rt
            .block_on(self.ibc_rpc_client.blocks().subscribe_finalized())
            .unwrap();

        
        Ok(())
    }

    #[instrument(
        name = "event_monitor.try_reconnect",
        level = "error",
        skip_all,
        fields(chain = %self.chain_id)
    )]
    fn try_reconnect(&mut self) -> Result<()> {
        trace!(
            "[{}] trying to reconnect to WebSocket endpoint {}",
            self.chain_id,
            self.ibc_node_addr
        );
        let mut ibc_rpc_client = self
            .rt
            .block_on(OnlineClient::<SubstrateConfig>::from_url(
                self.ibc_node_addr.to_string(),
            ))
            .unwrap();
        

        // Swap the new client with the previous one which failed,
        // so that we can shut the latter down gracefully.
        core::mem::swap(&mut self.ibc_rpc_client, &mut ibc_rpc_client);

        Ok(())
    }

    /// Try to resubscribe to events
    #[instrument(
        name = "event_monitor.try_resubscribe",
        level = "error",
        skip_all,
        fields(chain = %self.chain_id)
    )]
    fn try_resubscribe(&mut self) -> Result<()> {
        trace!("[{}] trying to resubscribe to events", self.chain_id);
        self.init_subscriptions()
    }

    /// Attempt to reconnect the node using the given retry strategy.
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
                "[{}] successfully reconnected to ibc endpoint: {} ",
                self.chain_id, self.ibc_node_addr
            ),
            Err(e) => error!(
                "failed to reconnect tto ibc endpoint: {} after {} retries",
                self.ibc_node_addr,  e.tries
            ),
        }
    }

    /// Event monitor loop
    #[allow(clippy::while_let_loop)]
    pub fn run(mut self) {
        debug!("[{}] substrate_monitor::run -> starting event monitor", self.chain_id);

        // Continuously run the event loop, so that when it aborts
        // because of WebSocket client restart, we pick up the work again.
        loop {
            match self.run_loop() {
                Next::Continue => continue,
                Next::Abort => break,
            }
        }

        // close connection
        let _ = self.ibc_rpc_client;
       
        trace!("[{}] substrate_monitor::run -> event monitor is shutting down", self.chain_id);
    }

    fn run_loop(&mut self) -> Next {
        trace!("substrate_monitor::run_loop");
        // Take ownership of the subscriptions
        let mut event_sub = core::mem::replace(&mut self.block_sub, Box::pin(futures::stream::empty()));
       
        // Work around double borrow
        let rt = self.rt.clone();

        loop {
            if let Ok(cmd) = self.rx_cmd.try_recv() {
                match cmd {
                    MonitorCmd::Shutdown => return Next::Abort,
                    MonitorCmd::Subscribe(tx) => {
                        if let Err(e) = tx.send(self.event_bus.subscribe()) {
                            error!("substrate_monitor::run_loop -> failed to send back subscription: {e}");
                        }
                    }
                }
            }

            let event_result = rt.block_on(async {
                tokio::select! {
                    Some(block) = event_sub.next() => {
                        let block = block.unwrap();

                        // Ask for the events for this block.
                        let events = block.events().await.unwrap();

                        let block_number = block.number();
                        let height = Height::new(REVISION_NUMBER,block_number.into()).unwrap();

                        let mut events_with_heights = Vec::new();
        
                        // add new block event for every new block
                        let new_block = IbcEvent::NewBlock(NewBlock { height });
                        events_with_heights.push(IbcEventWithHeight::new(new_block, height));
        
                        // filter ibc core events
                        let ibc_events_result:  core::result::Result<Vec<ibc_node::ibc::events::IbcEvents>, SubxtError> = events
                            .find::<ibc_node::ibc::events::IbcEvents>()
                            .collect();
                        match ibc_events_result {
                            Ok(ibc_tao_events) => {
                                //conver event
                                for ibc_events in ibc_tao_events.into_iter() {
                                    ibc_events.events.into_iter().for_each(|event| {
                                        //   let c_event = *event.clone();
                                        // let new_event = event.into();
                                            debug!("🐙🐙 ics10::substrate_monitor::run_loop -> ibc_tao_event: {:?}" ,event);
                                            events_with_heights.push(IbcEventWithHeight::new(event.into(), height));
                                        });
                                    }
                                }

                                Err(e) => {
                                    error!("🐙🐙 ics10::substrate_monitor::run_loop -> block: {}, filter ibc tao event error: {}", block_number, e);
                                }
                            }
                        
                        // filter ibc ics20 events
                        let send_packet_events_result:  core::result::Result<Vec<ibc_node::ics20_transfer::events::SendPacket>, SubxtError> = events
                            .find::<ibc_node::ics20_transfer::events::SendPacket>()
                            .collect();
                        match send_packet_events_result {
                            Ok(send_packet_events) => {
                                //conver event
                                for send_packet in send_packet_events.into_iter() {
                                        let event = SendPacket::from(send_packet.0);
                                        let event_with_height = IbcEventWithHeight {
                                            event: event.into(),
                                            height,
                                        };
                                        debug!("🐙🐙 ics10::substrate_monitor::run_loop -> send_packet: {:?}", event_with_height);
                                        events_with_heights.push(event_with_height)
                                    }
                                }

                                Err(e) => {
                                    error!("🐙🐙 ics10::substrate_monitor::run_loop -> block: {}, filter send packet event error: {}", block_number, e);
                                }
                            }
                          

                        Ok((height,events_with_heights))

                    },
                
                    Some(e) = self.rx_err.recv() => Err(MonitorError::subxt_error(e)),
                }
            
            });

            match event_result {
                Ok((height, events)) => self.process_batch(height, events),
                
                Err(e) => {
                    error!("🐙🐙 ics10::substrate_monitor::run_loop -> subscription dropped, need to reconnect !");

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

    /// broadcast the IBC events to subscriber
    fn process_batch(
        &mut self,
        height: Height,
        events: Vec<IbcEventWithHeight>,
    ) {

        let event_batch = EventBatch {
            height,
            events,
            chain_id: self.chain_id.clone(),
            tracking_id: TrackingId::new_uuid(),
        };
        debug!("🐙🐙 ics10::substrate_monitor::process_batch -> event_batch: {:?}", event_batch);

        // broadcast        
        self.event_bus.broadcast(Arc::new(Ok(event_batch)));

      
    }
}


#[cfg(test)]
mod tests {
    use futures::StreamExt;
    use sp_keyring::AccountKeyring;
    use std::time::Duration;
    use subxt::{error::Error, tx::PairSigner, OnlineClient, PolkadotConfig, SubstrateConfig};
    use ibc_relayer_types::{
        core::{ics02_client::height::Height, ics24_host::identifier::ChainId},
        events::IbcEvent,
    };
    use ibc_relayer_types::core::ics02_client::events::NewBlock;
    use super::{EventBus, IbcEventWithHeight};
    use crate::chain::substrate::REVISION_NUMBER;
    use crate::chain::substrate::relaychain::relaychain_node as ibc_node;
    use super::EventBatch;
    use crate::chain::tracking::TrackingId;
    // use ibc_relayer_types::core::ics24_host::identifier::ClientId;
    //  use ibc_relayer_types::core::ics02_client::client_type::ClientType;
    

    #[subxt::subxt(runtime_metadata_path = "./polkadot.scale")]
    // #[subxt::subxt(runtime_metadata_url = "ws://127.0.0.1:9944")]
    pub mod polkadot {}
    #[subxt::subxt(runtime_metadata_path = "./parachain.scale")]
    pub mod parachain_ibc {}
    /// Subscribe to all events, and then manually look through them and
    /// pluck out the events that we care about.
    #[tokio::test]
    async fn subscribe_polkadot_block_events() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt::init();

        // Create a client to use:
        let api = OnlineClient::<PolkadotConfig>::new().await?;

        // Subscribe to (in this case, finalized) blocks.
        let mut block_sub = api.blocks().subscribe_finalized().await?;

        // While this subscription is active, balance transfers are made somewhere:
        tokio::task::spawn({
            let api = api.clone();
            async move {
                let signer = PairSigner::new(AccountKeyring::Alice.pair());
                let mut transfer_amount = 1_000_000_000;

                // Make small balance transfers from Alice to Bob in a loop:
                loop {
                    let transfer_tx = polkadot::tx()
                        .balances()
                        .transfer(AccountKeyring::Bob.to_account_id().into(), transfer_amount);
                    api.tx()
                        .sign_and_submit_default(&transfer_tx, &signer)
                        .await
                        .unwrap();

                    tokio::time::sleep(Duration::from_secs(10)).await;
                    transfer_amount += 100_000_000;
                }
            }
        });

        // Get each finalized block as it arrives.
        while let Some(block) = block_sub.next().await {
            let block = block?;

            // Ask for the events for this block.
            let events = block.events().await?;

            let block_hash = block.hash();

            // We can dynamically decode events:
            println!("  Dynamic event details: {block_hash:?}:");
            for event in events.iter() {
                let event: subxt::events::EventDetails = event?;
                let is_balance_transfer = event
                    .as_event::<polkadot::balances::events::Transfer>()?
                    .is_some();
                let pallet = event.pallet_name();
                let variant = event.variant_name();
                println!("    {pallet}::{variant} (is balance transfer? {is_balance_transfer})");
            }

            // Or we can find the first transfer event, ignoring any others:
            let transfer_event = events.find_first::<polkadot::balances::events::Transfer>()?;

            if let Some(ev) = transfer_event {
                println!("  - Balance transfer success: value: {:?}", ev.amount);
            } else {
                println!("  - No balance transfer event found in this block");
            }
        }

        Ok(())
    }
   
    #[tokio::test]
    async fn subscribe_parachain_block_events() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt::init();

        // Create a client to use:
        let api = OnlineClient::<PolkadotConfig>::new().await?;
        // let api = OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9988").await?;

        // Subscribe to (in this case, finalized) blocks.
        let mut block_sub = api.blocks().subscribe_finalized().await?;


        // Get each finalized block as it arrives.
        while let Some(block) = block_sub.next().await {
            let block = block?;

            // Ask for the events for this block.
            let events = block.events().await?;

            let block_hash = block.hash();
            let block_number = block.number().into();
            
       
         
            let ibc_events: Result<Vec<_>, Error> = events
                .find::<ibc_node::ibc::events::IbcEvents>()
                .collect();
            println!("  All the ibc events: {:?}", ibc_events);
          
          
            match ibc_events {
                Ok(batch_event) => {
                  
    
                    let height = Height::new(REVISION_NUMBER,block_number).unwrap();
    
                    let mut events_with_heights = Vec::new();
    
                    // add new block
                    let new_block = IbcEvent::NewBlock(NewBlock { height });
                    events_with_heights.push(IbcEventWithHeight::new(new_block, height));
    
                    //conver event
                    for ibc_events in batch_event.into_iter() {
                        ibc_events.events.into_iter().for_each(|event| {
                        //   let c_event = *event.clone();
                          let new_event = event.into();
                            
                            events_with_heights.push(IbcEventWithHeight::new(new_event, height));
                        });
                    }
                    // let tm_client_id = ClientId::new(ClientType::Tendermint, 0);
                    let epoch_number = 10;
                    let chain_id  = ChainId::new("earth".to_string(), epoch_number);
                    
                    let event_batch = EventBatch {
                        height,
                        events: events_with_heights,
                        chain_id: chain_id,
                        tracking_id: TrackingId::new_uuid(),
                    };
    
                    println!("  event_batch: {:?}", event_batch);
                }
                Err(e) => {
                    println!("block: {}, found ibc event error: {}", block_number, e);
                }
            };  
        }
        Ok(())
    }    
        
}
