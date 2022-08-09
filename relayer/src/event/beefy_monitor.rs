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
use octopusxt::MyConfig;
use octopusxt::ibc_node::RuntimeApi;
use octopusxt::SubstrateNodeTemplateExtrinsicParams;
use subxt::{
    Client, ClientBuilder, Error as SubstrateError,
    PairSigner, SignedCommitment,
};

use ibc::clients::ics10_grandpa::help::{self, MmrRoot};
use ibc::core::ics02_client::height::Height;
use ibc::core::ics24_host::identifier::ChainId;
use ibc::events::IbcEvent;

use crate::error::Error as RelayError;
use crate::util::{
    retry::{retry_count, retry_with_index, RetryResult},
    stream::try_group_while,
};
use beefy_light_client::commitment;
use sp_core::hexdisplay::HexDisplay;

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
// pub use super::monitor::Result;
// pub type Result<T> = core::result::Result<T, Error>;
// pub type EventSender = channel::Sender<Result<EventBatch>>;
// pub type EventReceiver = channel::Receiver<Result<EventBatch>>;

pub type BeefyResult<T> = Result<T, Error>;
pub type BeefySender = channel::Sender<BeefyResult<MmrRoot>>;
pub type BeefyReceiver = channel::Receiver<BeefyResult<MmrRoot>>;

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
    /// Async task handle for the WebSocket client's driver
    // driver_handle: JoinHandle<()>,
    /// Channel to handler where the monitor for this chain sends the events
    tx_beefy: channel::Sender<BeefyResult<MmrRoot>>,
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
        node_addr: Url,
        rt: Arc<TokioRuntime>,
    ) -> BeefyResult<(Self, BeefyReceiver, TxMonitorCmd)> {
        let (tx_beefy, rx_beefy) = channel::unbounded();
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
            // driver_handle: websocket_driver_handle,
            // event_queries,
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
        tracing::info!("in beefy_mointor: [subscribe] ");
        let sub = self.rt.block_on(self.subscribe_beefy()).unwrap();

        self.subscription = Some(sub);
        // trace!("[{}] subscribed to all queries", self.chain_id);

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

        // let _ = client.close();

        // self.rt
        //     .block_on(driver_handle)
        //     .map_err(Error::client_termination_failed)?;

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
        tracing::trace!(
            "in beefy_monitor: [run], [{}] starting beefy monitor ",
            self.chain_id
        );
        println!(
            "in beefy_monitor: [run], [{}] starting beefy monitor ",
            self.chain_id
        );
        // Continuously run the event loop, so that when it aborts
        // because of WebSocket client restart, we pick up the work again.
        // loop {
        //     match self.run_loop() {
        //         Next::Continue => continue,
        //         Next::Abort => break,
        //     }
        // }
        // let &mut sub = match self.subscription {
        //     Some(ref sub) => sub,
        //     // None => Next::Abort,
        //     None => return,
        // };

        // let mut sub: BeefySubscription = if let Some(ref sub) = self.subscription {
        //     &sub
        // } else {
        //     return;
        // };

        // let &mut sub = self.subscription.as_ref().unwrap();
        
        let api = self.client.clone().to_runtime_api::<RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

        let sub = self
            .rt
            .block_on(api.client.rpc().subscribe_beefy_justifications())
            .unwrap();

        let mut beefy_sub = sub;

        tracing::trace!("in beefy_monitor: [run], beefy subscripte success ! ");
        println!(
            "in beefy_monitor: [run], Successful beefy subscription to {:?} ! ",
            self.chain_id.as_str()
        );

        // let mut beefy_sub = self.rt.block_on(self.subscribe_beefy());
        // Work around double borrow
        let rt = self.rt.clone();

        // msg loop for handle the beefy SignedCommitment
        loop {
            let raw_sc = self.rt.block_on(beefy_sub.next()).unwrap().unwrap();
            tracing::trace!(
                "in beefy_monitor: [run], from {:?} received beefy signed commitment : {:?} ",
                self.chain_id.as_str(),
                raw_sc
            );
            println!(
                "in beefy_monitor: [run], from {:?} received beefy signed commitment : {:?} ",
                self.chain_id.as_str(),
                raw_sc
            );

            let _ = self.process_beefy_msg(raw_sc);
            //build mmr root
            // let result = self
            //     .rt
            //     .block_on(octopusxt::build_mmr_root(self.client.clone(), raw_sc));

            // if let Ok(mmr_root) = result {
            //     // send to msg queue
            //     self.tx_beefy
            //         .send(Ok(mmr_root))
            //         .map_err(|_| Error::channel_send_failed())?;
            // }
        }

        // loop {
        //     if let Ok(MonitorCmd::Shutdown) = self.rx_cmd.try_recv() {
        //         // return Next::Abort;
        //         break;
        //     }

        //     let result = rt.block_on(sub.next());

        //     // Repeat check of shutdown command here, as previous recv()
        //     // may block for a long time.
        //     if let Ok(MonitorCmd::Shutdown) = self.rx_cmd.try_recv() {
        //         // return Next::Abort;
        //         break;
        //     }

        //     if let Some(raw_sc) = result {
        //         // process beefy msg
        //         self.process_beefy_msg(raw_sc);
        //     }
        // }

        // debug!("[{}] beefy monitor is shutting down", self.chain_id);
    }

    fn run_loop(&mut self) -> Next {
        use core::time::Duration;
        tracing::info!("in beefy_mointor: [run_loop]");
        // Take ownership of the subscriptions
        // let sub = core::mem::replace(&mut self.subscription, Box::new(futures::stream::empty()));
        // if let Some(sub) = self.subscription {
        //    return
        // }

        // let sub = match self.subscription {
        //     Some(sub) => sub,
        //     None => return Next::Abort,
        // };
        // // Work around double borrow
        // let rt = self.rt.clone();

        // loop {
        //     if let Ok(MonitorCmd::Shutdown) = self.rx_cmd.try_recv() {
        //         return Next::Abort;
        //     }

        //     let result = rt.block_on(sub.next());

        //     // Repeat check of shutdown command here, as previous recv()
        //     // may block for a long time.
        //     if let Ok(MonitorCmd::Shutdown) = self.rx_cmd.try_recv() {
        //         return Next::Abort;
        //     }

        //     if let Some(raw_sc) = result {
        //         // process beefy msg
        //         self.process_beefy_msg(raw_sc);
        //     }
        // }
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
        tracing::info!("in beefy_mointor: [propagate_error]");
        self.tx_beefy
            .send(Err(error))
            .map_err(|_| Error::channel_send_failed())?;

        Ok(())
    }

    /// Collect the IBC events from the subscriptions
    fn process_beefy_msg(&self, raw_sc: SignedCommitment) -> BeefyResult<()> {
        tracing::trace!("in beefy_mointor: [process_beefy_msg]");
        println!("in beefy_mointor: [process_beefy_msg]");
        //build mmr root
        let result = self
            .rt
            .block_on(octopusxt::build_mmr_root(self.client.clone(), raw_sc));
        tracing::trace!(
            "in beefy_monitor: [process_beefy_msg], build mmr root : {:?} ",
            result
        );
        // println!(
        //     "in beefy_monitor: [process_beefy_msg], build mmr root : {:?} ",
        //     result
        // );
        if let Ok(mmr_root) = result {
            // send to msg queue
            tracing::trace!(
                "in beefy_monitor: [process_beefy_msg], send mmr root : {:?} ",
                mmr_root
            );
            println!(
                "in beefy_monitor: [process_beefy_msg], chain id: {:?} ",
                self.chain_id.as_str(),
            );
            self.tx_beefy
                .send(Ok(mmr_root))
                .map_err(|_| Error::channel_send_failed())?;
        }
        // if result.is_ok() {
        //     self.tx_beefy
        //         .send(result)
        //         .map_err(|_| Error::channel_send_failed())?;
        // }

        Ok(())
    }
    /// Subscribe beefy msg
    pub async fn subscribe_beefy(
        &self,
    ) -> Result<SignedCommitment, Box<dyn std::error::Error>> {
        tracing::info!("In call_ibc: [subscribe_beefy_justifications]");
        let api = self.client.clone()
            .to_runtime_api::<RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();
    
        let mut sub = api.client.rpc().subscribe_beefy_justifications().await?;
    
        let raw = sub.next().await.unwrap().unwrap();
    
        Ok(raw)
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
    use octopusxt::ibc_node::{self, DefaultConfig, RuntimeApi};
    use subxt::{
        BeefySubscription, Client, ClientBuilder, Error as SubstrateError, EventSubscription,
        PairSigner, RawEvent, SignedCommitment,
    };
    use tendermint_rpc::Url;
    use tokio::runtime::Runtime as TokioRuntime;

    use super::BeefyMonitor;

    #[test]
    fn test_beefy_monitor_run() {
        let chain_id = ChainId::from_string("ibc-0");
        println!(
            "in beefy_monitor: [test_beefy_monitor_run], chain id : {:?} ",
            chain_id
        );
        let websocket_url = Url::from_str("ws://127.0.0.1:9944/websocket").unwrap();
        println!(
            "in beefy_monitor: [test_beefy_monitor_run], websocket url : {:?} ",
            websocket_url
        );
        let rt = Arc::new(TokioRuntime::new().unwrap());
        // let client = async {
        //     ClientBuilder::new()
        //         .set_url(websocket_url.clone())
        //         .build::<DefaultConfig>()
        //         .await
        //         .map_err(|_| Error::substrate_client_builder_error())
        // };

        // let cleint = rt.block_on(client);
        let (beefy_monitor, receiver, tx_cmd) =
            BeefyMonitor::new(chain_id, websocket_url, rt).unwrap();
        beefy_monitor.run()
    }
}
