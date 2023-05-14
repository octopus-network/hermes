use alloc::sync::Arc;
use bytes::{Buf, Bytes};
use core::{
    convert::{TryFrom, TryInto},
    future::Future,
    str::FromStr,
    time::Duration,
};
use digest::typenum::U264;
use futures::future::join_all;
use num_bigint::BigInt;
use std::{cmp::Ordering, thread};

use tokio::runtime::Runtime as TokioRuntime;
use tonic::{codegen::http::Uri, metadata::AsciiMetadataValue};
use tracing::{debug, error, instrument, trace, warn};

use ibc_proto::cosmos::base::node::v1beta1::ConfigResponse;
use ibc_proto::cosmos::staking::v1beta1::Params as StakingParams;
use ibc_proto::protobuf::Protobuf;
use ibc_relayer_types::applications::ics31_icq::response::CrossChainQueryResponse;
use ibc_relayer_types::clients::ics10_grandpa::client_state::{
    AllowUpdate, ClientState as GpClientState,
};
use ibc_relayer_types::clients::ics10_grandpa::consensus_state::ConsensusState as GpConsensusState;
use ibc_relayer_types::clients::ics10_grandpa::header::Header as GpHeader;
use ibc_relayer_types::core::ics02_client::client_type::ClientType;
use ibc_relayer_types::core::ics02_client::error::Error as ClientError;
use ibc_relayer_types::core::ics02_client::events::UpdateClient;
use ibc_relayer_types::core::ics02_client::msgs::update_client::MsgUpdateClient;
use ibc_relayer_types::core::ics03_connection::connection::{
    ConnectionEnd, IdentifiedConnectionEnd,
};
use ibc_relayer_types::core::ics04_channel::channel::{ChannelEnd, IdentifiedChannelEnd};
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentPrefix;
use ibc_relayer_types::core::ics23_commitment::merkle::MerkleProof;
use ibc_relayer_types::core::ics24_host::identifier::{
    ChainId, ChannelId, ClientId, ConnectionId, PortId,
};
use ibc_relayer_types::core::ics24_host::path::{
    AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath, CommitmentsPath,
    ConnectionsPath, ReceiptsPath, SeqRecvsPath,
};
use ibc_relayer_types::core::ics24_host::{
    ClientUpgradePath, Path, IBC_QUERY_PATH, SDK_UPGRADE_QUERY_PATH,
};
use ibc_relayer_types::signer::Signer;
use ibc_relayer_types::Height as ICSHeight;
use tendermint::block::Height as TmHeight;
use tendermint::node::info::TxIndexStatus;
use tendermint_rpc::endpoint::broadcast::tx_sync::Response;
use tendermint_rpc::endpoint::status;
use tendermint_rpc::{Client, HttpClient, Order};

use crate::chain::client::ClientSettings;
use crate::chain::cosmos::batch::{
    send_batched_messages_and_wait_check_tx, send_batched_messages_and_wait_commit,
    sequential_send_batched_messages_and_wait_commit,
};
use crate::chain::endpoint::{ChainEndpoint, ChainStatus, HealthCheck};
use crate::chain::handle::Subscription;
use crate::chain::requests::*;
use crate::chain::tracking::TrackedMsgs;
use crate::client_state::{AnyClientState, IdentifiedAnyClientState};
use crate::config::{parse_gas_prices, ChainConfig, GasPrice};
use crate::consensus_state::AnyConsensusState;
use crate::denom::DenomTrace;
use crate::error::Error;
use crate::event::beefy_monitor::{BeefyMonitor, BeefyReceiver, TxMonitorCmd as BeefyTxMonitorCmd};
use crate::event::substrate_mointor::{EventMonitor, TxMonitorCmd};
use crate::event::IbcEventWithHeight;
use crate::keyring::{KeyRing, Secp256k1KeyPair, SigningKeyPair};
use crate::light_client::tendermint::LightClient as TmLightClient;
use crate::light_client::{LightClient, Verified};
use crate::misbehaviour::MisbehaviourEvidence;
use crate::util::pretty::{
    PrettyIdentifiedChannel, PrettyIdentifiedClientState, PrettyIdentifiedConnection,
};
use crate::{account::Balance, config::default::para_chain_addr};
use codec::Decode;
use ibc_relayer_types::timestamp::Timestamp;
// substrate
use serde::{Deserialize, Serialize};
use subxt::rpc::RpcClient as SubxtRpcClient;
use subxt::rpc::Subscription as SubxtSubscription;
use subxt::rpc_params;
use subxt::Error as SubxtError;
use subxt::{tx::PairSigner, OnlineClient, PolkadotConfig, SubstrateConfig};
// use subxt::rpc::types::into_block_number;
pub mod beefy;
// #[subxt::subxt(runtime_metadata_path = "./metadata.scale")]
pub mod parachain;
pub mod relaychain;
pub mod utils;

use parachain::parachain_node;
use relaychain::relaychain_node;

use beefy_primitives::{known_payloads::MMR_ROOT_ID, Payload, VersionedFinalityProof};

const MAX_QUERY_TIMES: u64 = 800;
pub const REVISION_NUMBER: u64 = 0;
pub const BEEFY_ACTIVATION_HEIGHT: u32 = 0;

/// `beefy_primitives::SignedCommitment`.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EncodedVersionedFinalityProof(pub sp_core::Bytes);
/// Subscribe to beefy justifications.
pub async fn subscribe_beefy_justifications(
    client: &SubxtRpcClient,
) -> Result<SubxtSubscription<EncodedVersionedFinalityProof>, subxt::Error> {
    let subscription = client
        .subscribe(
            "beefy_subscribeJustifications",
            rpc_params![],
            "beefy_unsubscribeJustifications",
        )
        .await?;
    Ok(subscription)
}

/// Defines an upper limit on how large any transaction can be.
/// This upper limit is defined as a fraction relative to the block's
/// maximum bytes. For example, if the fraction is `0.9`, then
/// `max_tx_size` will not be allowed to exceed 0.9 of the
/// maximum block size of any Cosmos SDK network.
///
/// The default fraction we use is `0.9`; anything larger than that
/// would be risky, as transactions might be rejected; a smaller value
/// might be un-necessarily restrictive on the relayer side.
/// The [default max. block size in Tendermint 0.37 is 21MB](tm-37-max).
/// With a fraction of `0.9`, then Hermes will never permit the configuration
/// of `max_tx_size` to exceed ~18.9MB.
///
/// [tm-37-max]: https://github.com/tendermint/tendermint/blob/v0.37.0-rc1/types/params.go#L79
pub const BLOCK_MAX_BYTES_MAX_FRACTION: f64 = 0.9;
pub struct SubstrateChain {
    rpc_client: RpcClient,
    config: ChainConfig,
    rt: Arc<TokioRuntime>,
    relay_chain_addr: String,
    para_chain_addr: String,
    keybase: KeyRing<Secp256k1KeyPair>,
    tx_event_monitor_cmd: Option<TxMonitorCmd>,
    tx_beefy_monitor_cmd: Option<BeefyTxMonitorCmd>,
}

pub enum RpcClient {
    ParachainRpc {
        relay_rpc: OnlineClient<PolkadotConfig>,
        para_rpc: OnlineClient<SubstrateConfig>,
    },
    SubChainRpc {
        rpc: OnlineClient<PolkadotConfig>,
    },
}

impl SubstrateChain {
    /// Get a reference to the configuration for this chain.
    pub fn config(&self) -> &ChainConfig {
        &self.config
    }

    /// The maximum size of any transaction sent by the relayer to this chain
    fn max_tx_size(&self) -> usize {
        todo!()
    }

    fn key(&self) -> Result<Secp256k1KeyPair, Error> {
        self.keybase()
            .get_key(&self.config.key_name)
            .map_err(Error::key_base)
    }

    /// Fetches the trusting period as a `Duration` from the chain config.
    /// If no trusting period exists in the config, the trusting period is calculated
    /// as two-thirds of the `unbonding_period`.
    fn trusting_period(&self, unbonding_period: Duration) -> Duration {
        todo!()
    }

    /// Performs validation of the relayer's configuration
    /// for a specific chain against the parameters of that chain.
    ///
    /// Currently, validates the following:
    ///     - the configured `max_tx_size` is appropriate
    ///     - the trusting period is greater than zero
    ///     - the trusting period is smaller than the unbonding period
    ///     - the default gas is smaller than the max gas
    ///
    /// Emits a log warning in case any error is encountered and
    /// exits early without doing subsequent validations.
    pub fn validate_params(&self) -> Result<(), Error> {
        Ok(())
    }

    fn init_event_monitor(&mut self) -> Result<TxMonitorCmd, Error> {
        crate::time!("init_event_monitor");

        tracing::debug!(
            "substrate::init_event_mointor -> websocket addr: {:?}",
            self.config.websocket_addr.clone()
        );

        let (mut event_monitor, monitor_tx) = EventMonitor::new(
            self.config.id.clone(),
            self.config.websocket_addr.clone(),
            self.rt.clone(),
        )
        .map_err(Error::event_monitor)?;

        event_monitor
            .init_subscriptions()
            .map_err(Error::event_monitor)?;

        // thread::spawn(move || event_monitor.run());

        Ok(monitor_tx)
    }

    fn init_beefy_monitor(&mut self) -> Result<BeefyTxMonitorCmd, Error> {
        crate::time!("init_beefy_monitor");

        tracing::debug!(
            "substrate::init_beefy_mointor -> websocket addr: {:?}",
            self.config.websocket_addr.clone()
        );

        let (mut beefy_monitor, monitor_tx) = BeefyMonitor::new(
            self.config.id.clone(),
            self.config.websocket_addr.clone(),
            self.rt.clone(),
        )
        .map_err(Error::event_monitor)?;

        beefy_monitor
            .init_subscriptions()
            .map_err(Error::event_monitor)?;

        thread::spawn(move || beefy_monitor.run());
        debug!("substrate::init_beefy_mointor ->  beefy monitor is running ...");

        Ok(monitor_tx)
    }

    /// Query the chain staking parameters
    pub fn query_staking_params(&self) -> Result<StakingParams, Error> {
        crate::time!("query_staking_params");
        crate::telemetry!(query, self.id(), "query_staking_params");
        todo!()
    }

    /// Query the node for its configuration parameters.
    ///
    /// ### Note: This query endpoint was introduced in SDK v0.46.3/v0.45.10. Not available before that.
    ///
    /// Returns:
    ///     - `Ok(Some(..))` if the query was successful.
    ///     - `Ok(None) in case the query endpoint is not available.
    ///     - `Err` for any other error.
    pub fn query_config_params(&self) -> Result<Option<ConfigResponse>, Error> {
        crate::time!("query_config_params");
        crate::telemetry!(query, self.id(), "query_config_params");
        todo!()
    }

    /// The minimum gas price that this node accepts
    pub fn min_gas_price(&self) -> Result<Vec<GasPrice>, Error> {
        crate::time!("min_gas_price");

        todo!()
    }

    /// The unbonding period of this chain
    pub fn unbonding_period(&self) -> Result<Duration, Error> {
        crate::time!("unbonding_period");

        todo!()
    }

    /// The number of historical entries kept by this chain
    pub fn historical_entries(&self) -> Result<u32, Error> {
        crate::time!("historical_entries");

        todo!()
    }

    /// Run a future to completion on the Tokio runtime.
    fn block_on<F: Future>(&self, f: F) -> F::Output {
        crate::time!("block_on");
        self.rt.block_on(f)
    }

    /// Perform an ABCI query against the client upgrade sub-store.
    ///
    /// The data is returned in its raw format `Vec<u8>`, and is either the
    /// client state (if the target path is [`UpgradedClientState`]), or the
    /// client consensus state ([`UpgradedClientConsensusState`]).
    ///
    /// Note: This is a special query in that it will only succeed if the chain
    /// is halted after reaching the height proposed in a successful governance
    /// proposal to upgrade the chain. In this scenario, let P be the height at
    /// which the chain is planned to upgrade. We assume that the chain is
    /// halted at height P. Tendermint will be at height P (as reported by the
    /// /status RPC query), but the application will be at height P-1 (as
    /// reported by the /abci_info RPC query).
    ///
    /// Therefore, `query_height` needs to be P-1. However, the path specified
    /// in `query_data` needs to be constructed with height `P`, as this is how
    /// the chain will have stored it in its upgrade sub-store.
    fn query_client_upgrade_state(
        &self,
        query_data: ClientUpgradePath,
        query_height: ICSHeight,
    ) -> Result<(Vec<u8>, MerkleProof), Error> {
        todo!()
    }

    /// Query the chain status via an RPC query.
    ///
    /// Returns an error if the node is still syncing and has not caught up,
    /// ie. if `sync_info.catching_up` is `true`.
    fn chain_status(&self) -> Result<status::Response, Error> {
        crate::time!("chain_status");
        crate::telemetry!(query, self.id(), "status");

        todo!()
    }

    /// Query the chain's latest height
    pub fn query_chain_latest_height(&self) -> Result<ICSHeight, Error> {
        crate::time!("query_latest_height");
        crate::telemetry!(query, self.id(), "query_latest_height");

        todo!()
    }

    #[instrument(
        name = "do_send_messages_and_wait_commit",
        level = "error",
        skip_all,
        fields(
            chain = %self.id(),
            tracking_id = %tracked_msgs.tracking_id()
        ),
    )]
    fn do_send_messages_and_wait_commit(
        &mut self,
        tracked_msgs: TrackedMsgs,
    ) -> Result<Vec<IbcEventWithHeight>, Error> {
        use ibc_relayer_types::applications::transfer::msgs::transfer::TYPE_URL as TRANSFER_TYPE_URL;
        use ibc_relayer_types::events::IbcEvent;
        use sp_keyring::AccountKeyring;
        debug!(
            "substrate::do_send_messages_and_wait_commit -> tracked_msgs: {:?}",
            tracked_msgs
        );
        let proto_msgs = tracked_msgs.msgs;
        let mut is_transfer_msg = false;

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => {
                let msg: Vec<parachain_node::runtime_types::ibc_proto::google::protobuf::Any> =
                    proto_msgs
                        .iter()
                        .map(|m| {
                            match m.type_url.as_str() {
                                TRANSFER_TYPE_URL => {
                                    is_transfer_msg = true;
                                    debug!(
                                        "substrate::do_send_messages_and_wait_commit -> transfer msg: {:?}",
                                        m.clone()
                                    );
                                 },
                                _ => is_transfer_msg = false,
                            }
                            parachain_node::runtime_types::ibc_proto::google::protobuf::Any {
                                type_url: m.type_url.clone(),
                                value: m.value.clone(),
                            }
                        })
                        .collect();
                // use default signer
                let signer = PairSigner::new(AccountKeyring::Alice.pair());

                let binding = para_rpc.tx();
                let runtime = self.rt.clone();
                match is_transfer_msg {
                    true => {
                        let tx = parachain_node::tx().ics20_transfer().raw_transfer(msg);

                        let transfer = binding.sign_and_submit_then_watch_default(&tx, &signer);
                        let result = runtime.block_on(transfer);
                        let events = runtime.block_on(result.unwrap().wait_for_finalized_success());

                        let ibc_events = events
                            .unwrap()
                            .find_first::<parachain_node::ibc::events::IbcEvents>()
                            .unwrap()
                            .unwrap();
                        use subxt::config::Header;
                        let finalized_head_hash =
                            runtime.block_on(para_rpc.rpc().finalized_head()).unwrap();

                        let block = runtime
                            .block_on(para_rpc.rpc().block(Some(finalized_head_hash)))
                            .unwrap();

                        let height =
                            ICSHeight::new(0, u64::from(block.unwrap().block.header.number()))
                                .unwrap();

                        let es: Vec<IbcEventWithHeight> = ibc_events
                            .events
                            .into_iter()
                            .map(|e| IbcEventWithHeight {
                                event: IbcEvent::from(e),
                                height: height.clone(),
                            })
                            .collect();

                        Ok(es)
                    }
                    false => {
                        let tx = parachain_node::tx().ibc().deliver(msg);

                        let deliver = binding.sign_and_submit_then_watch_default(&tx, &signer);
                        let result = runtime.block_on(deliver);
                        let events = runtime.block_on(result.unwrap().wait_for_finalized_success());

                        let ibc_events = events
                            .unwrap()
                            .find_first::<parachain_node::ibc::events::IbcEvents>()
                            .unwrap()
                            .unwrap();
                        use subxt::config::Header;
                        let finalized_head_hash =
                            runtime.block_on(para_rpc.rpc().finalized_head()).unwrap();

                        let block = runtime
                            .block_on(para_rpc.rpc().block(Some(finalized_head_hash)))
                            .unwrap();

                        let height =
                            ICSHeight::new(0, u64::from(block.unwrap().block.header.number()))
                                .unwrap();

                        let es: Vec<IbcEventWithHeight> = ibc_events
                            .events
                            .into_iter()
                            .map(|e| IbcEventWithHeight {
                                event: IbcEvent::from(e),
                                height: height.clone(),
                            })
                            .collect();

                        Ok(es)
                    }
                }
            }
            RpcClient::SubChainRpc { rpc } => {
                let msg: Vec<relaychain_node::runtime_types::ibc_proto::google::protobuf::Any> =
                    proto_msgs
                        .iter()
                        .map(|m| {
                            match m.type_url.as_str() {
                                TRANSFER_TYPE_URL => {
                                    is_transfer_msg = true;
                                    debug!(
                                        "substrate::do_send_messages_and_wait_commit -> transfer msg: {:?}",
                                        m.clone()
                                    );
                                 },

                                _ => is_transfer_msg = false,
                            }
                            relaychain_node::runtime_types::ibc_proto::google::protobuf::Any {
                                type_url: m.type_url.clone(),
                                value: m.value.clone(),
                            }
                        })
                        .collect();
                // use default signer
                let signer = PairSigner::new(AccountKeyring::Alice.pair());

                let binding = rpc.tx();

                let runtime = self.rt.clone();
                match is_transfer_msg {
                    true => {
                        let tx = relaychain_node::tx().ics20_transfer().raw_transfer(msg);
                        let deliver = binding.sign_and_submit_then_watch_default(&tx, &signer);
                        let result = runtime.block_on(deliver);

                        let events = runtime.block_on(result.unwrap().wait_for_finalized_success());

                        let ibc_events = events
                            .unwrap()
                            .find_first::<relaychain_node::ibc::events::IbcEvents>()
                            .unwrap()
                            .unwrap();
                        use subxt::config::Header;
                        let finalized_head_hash =
                            runtime.block_on(rpc.rpc().finalized_head()).unwrap();

                        let block = runtime
                            .block_on(rpc.rpc().block(Some(finalized_head_hash)))
                            .unwrap();
                        let height =
                            ICSHeight::new(0, u64::from(block.unwrap().block.header.number()))
                                .unwrap();

                        let es: Vec<IbcEventWithHeight> = ibc_events
                            .events
                            .into_iter()
                            .map(|e| IbcEventWithHeight {
                                event: IbcEvent::from(e),
                                height: height.clone(),
                            })
                            .collect();

                        Ok(es)
                    }
                    false => {
                        let tx = relaychain_node::tx().ibc().deliver(msg);
                        let deliver = binding.sign_and_submit_then_watch_default(&tx, &signer);
                        let result = runtime.block_on(deliver);

                        let events = runtime.block_on(result.unwrap().wait_for_finalized_success());

                        let ibc_events = events
                            .unwrap()
                            .find_first::<relaychain_node::ibc::events::IbcEvents>()
                            .unwrap()
                            .unwrap();
                        use subxt::config::Header;
                        let finalized_head_hash =
                            runtime.block_on(rpc.rpc().finalized_head()).unwrap();

                        let block = runtime
                            .block_on(rpc.rpc().block(Some(finalized_head_hash)))
                            .unwrap();
                        let height =
                            ICSHeight::new(0, u64::from(block.unwrap().block.header.number()))
                                .unwrap();

                        let es: Vec<IbcEventWithHeight> = ibc_events
                            .events
                            .into_iter()
                            .map(|e| IbcEventWithHeight {
                                event: IbcEvent::from(e),
                                height: height.clone(),
                            })
                            .collect();

                        Ok(es)
                    }
                }
            }
        }
    }

    #[instrument(
        name = "send_messages_and_wait_check_tx",
        level = "error",
        skip_all,
        fields(
            chain = %self.id(),
            tracking_id = %tracked_msgs.tracking_id()
        ),
    )]
    async fn do_send_messages_and_wait_check_tx(
        &mut self,
        tracked_msgs: TrackedMsgs,
    ) -> Result<Vec<Response>, Error> {
        crate::time!("send_messages_and_wait_check_tx");
        todo!()
    }

    fn query_packet_from_block(
        &self,
        request: &QueryPacketEventDataRequest,
        seqs: &[Sequence],
        block_height: &ICSHeight,
    ) -> Result<(Vec<IbcEventWithHeight>, Vec<IbcEventWithHeight>), Error> {
        crate::time!("query_block: query block packet events");
        crate::telemetry!(query, self.id(), "query_block");

        todo!()
    }

    fn query_packets_from_blocks(
        &self,
        request: &QueryPacketEventDataRequest,
    ) -> Result<(Vec<IbcEventWithHeight>, Vec<IbcEventWithHeight>), Error> {
        crate::time!("query_blocks: query block packet events");
        crate::telemetry!(query, self.id(), "query_blocks");
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubLightBlock {}

impl ChainEndpoint for SubstrateChain {
    type LightBlock = SubLightBlock;
    type Header = GpHeader;
    type ConsensusState = GpConsensusState;
    type ClientState = GpClientState;
    type SigningKeyPair = Secp256k1KeyPair;

    fn bootstrap(config: ChainConfig, rt: Arc<TokioRuntime>) -> Result<Self, Error> {
        let relay_chain_addr = config
            .relay_chain_addr
            .clone()
            .map(|v| v.to_string())
            .unwrap_or("ws://127.0.0.1:9944".to_string());

        let para_chain_addr = config
            .para_chain_addr
            .clone()
            .map(|v| v.to_string())
            .unwrap_or("ws://127.0.0.1:9945".to_string());

        // Initialize key store and load key
        let keybase = KeyRing::new(config.key_store_type, &config.account_prefix, &config.id)
            .map_err(Error::key_base)?;

        // judge by para_chain rpc client can init define SubChainRpc or ParachianRpc
        let rpc_client = if rt
            .block_on(OnlineClient::<SubstrateConfig>::from_url(&para_chain_addr))
            .is_err()
        {
            RpcClient::SubChainRpc {
                rpc: rt
                    .block_on(OnlineClient::<PolkadotConfig>::from_url(&relay_chain_addr))
                    .unwrap(),
            }
        } else {
            RpcClient::ParachainRpc {
                relay_rpc: rt
                    .block_on(OnlineClient::<PolkadotConfig>::from_url(&relay_chain_addr))
                    .unwrap(),
                para_rpc: rt
                    .block_on(OnlineClient::<SubstrateConfig>::from_url(&para_chain_addr))
                    .unwrap(),
            }
        };

        Ok(Self {
            config,
            rpc_client,
            rt,
            keybase,
            relay_chain_addr,
            para_chain_addr,
            tx_event_monitor_cmd: None,
            tx_beefy_monitor_cmd: None,
        })
    }

    fn shutdown(self) -> Result<(), Error> {
        if let Some(monitor_tx) = self.tx_event_monitor_cmd {
            monitor_tx.shutdown().map_err(Error::event_monitor)?;
        }

        Ok(())
    }

    fn keybase(&self) -> &KeyRing<Self::SigningKeyPair> {
        &self.keybase
    }

    fn keybase_mut(&mut self) -> &mut KeyRing<Self::SigningKeyPair> {
        &mut self.keybase
    }

    fn subscribe(&mut self) -> Result<Subscription, Error> {
        tracing::info!("substrate::subscribe -> requst to subscribe substrate event msg !",);
        let tx_monitor_cmd = match &self.tx_event_monitor_cmd {
            Some(tx_monitor_cmd) => tx_monitor_cmd,
            None => {
                let tx_monitor_cmd = self.init_event_monitor()?;
                self.tx_event_monitor_cmd = Some(tx_monitor_cmd);
                self.tx_event_monitor_cmd.as_ref().unwrap()
            }
        };

        let subscription = tx_monitor_cmd.subscribe().map_err(Error::event_monitor)?;
        Ok(subscription)
    }
    fn subscribe_beefy(&mut self) -> Result<super::handle::BeefySubscription, Error> {
        tracing::info!("substrate::subscribe_beefy -> reqeust to subscribe substrate beefy msg !",);
        let tx_beefy_monitor_cmd = match &self.tx_beefy_monitor_cmd {
            Some(tx_beefy_monitor_cmd) => tx_beefy_monitor_cmd,
            None => {
                let tx_beefy_monitor_cmd = self.init_beefy_monitor()?;
                self.tx_beefy_monitor_cmd = Some(tx_beefy_monitor_cmd);
                self.tx_beefy_monitor_cmd.as_ref().unwrap()
            }
        };

        let beefy_subscription = tx_beefy_monitor_cmd
            .subscribe()
            .map_err(Error::event_monitor)?;
        Ok(beefy_subscription)
    }
    /// Does multiple RPC calls to the full node, to check for
    /// reachability and some basic APIs are available.
    ///
    /// Currently this checks that:
    ///     - the node responds OK to `/health` RPC call;
    ///     - the node has transaction indexing enabled;
    ///     - the SDK & IBC versions are supported;
    ///
    /// Emits a log warning in case anything is amiss.
    /// Exits early if any health check fails, without doing any
    /// further checks.
    fn health_check(&self) -> Result<HealthCheck, Error> {
        Ok(HealthCheck::Healthy)
    }

    /// Fetch a header from the chain at the given height and verify it.
    fn verify_header(
        &mut self,
        trusted: ICSHeight,
        target: ICSHeight,
        client_state: &AnyClientState,
    ) -> Result<Self::LightBlock, Error> {
        async fn verify_header(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            trusted: ICSHeight,
            target: ICSHeight,
            client_state: &AnyClientState,
        ) -> Result<SubLightBlock, Error> {
            Ok(SubLightBlock {})
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(verify_header(
                relay_rpc,
                Some(para_rpc),
                trusted,
                target,
                client_state,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(verify_header(rpc, None, trusted, target, client_state))
            }
        }
    }

    /// Given a client update event that includes the header used in a client update,
    /// look for misbehaviour by fetching a header at same or latest height.
    fn check_misbehaviour(
        &mut self,
        update: &UpdateClient,
        client_state: &AnyClientState,
    ) -> Result<Option<MisbehaviourEvidence>, Error> {
        async fn check_misbehaviour(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            update: &UpdateClient,
            client_state: &AnyClientState,
        ) -> Result<Option<MisbehaviourEvidence>, Error> {
            Ok(None)
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(check_misbehaviour(
                relay_rpc,
                Some(para_rpc),
                update,
                client_state,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(check_misbehaviour(rpc, None, update, client_state))
            }
        }
    }

    // Queries

    /// Send one or more transactions that include all the specified messages.
    /// The `proto_msgs` are split in transactions such they don't exceed the configured maximum
    /// number of messages per transaction and the maximum transaction size.
    /// Then `send_tx()` is called with each Tx. `send_tx()` determines the fee based on the
    /// on-chain simulation and if this exceeds the maximum gas specified in the configuration file
    /// then it returns error.
    /// TODO - more work is required here for a smarter split maybe iteratively accumulating/ evaluating
    /// msgs in a Tx until any of the max size, max num msgs, max fee are exceeded.
    fn send_messages_and_wait_commit(
        &mut self,
        tracked_msgs: TrackedMsgs,
    ) -> Result<Vec<IbcEventWithHeight>, Error> {
        self.do_send_messages_and_wait_commit(tracked_msgs)
    }

    fn send_messages_and_wait_check_tx(
        &mut self,
        tracked_msgs: TrackedMsgs,
    ) -> Result<Vec<Response>, Error> {
        use tendermint::abci::Code;
        use tendermint::Hash;
        let ret = self.do_send_messages_and_wait_commit(tracked_msgs)?;

        let json = "\"ChYKFGNvbm5lY3Rpb25fb3Blbl9pbml0\"";
        let tx_re = Response {
            code: Code::default(),
            data: serde_json::from_str(json).unwrap(),
            log: String::from("test_test"),
            hash: Hash::Sha256([0u8; 32]),
        };

        Ok(vec![tx_re])
    }

    /// Get the account for the signer
    fn get_signer(&self) -> Result<Signer, Error> {
        crate::time!("get_signer");
        use crate::chain::cosmos::encode::key_pair_to_signer;

        // Get the key from key seed file
        let key_pair = self.key()?;

        let signer = key_pair_to_signer(&key_pair)?;

        Ok(signer)
    }

    /// Get the chain configuration
    fn config(&self) -> &ChainConfig {
        &self.config
    }

    fn ibc_version(&self) -> Result<Option<semver::Version>, Error> {
        async fn ibc_version(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
        ) -> Result<Option<semver::Version>, Error> {
            Ok(None)
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(ibc_version(relay_rpc, Some(para_rpc))),
            RpcClient::SubChainRpc { rpc } => self.block_on(ibc_version(rpc, None)),
        }
    }

    fn query_balance(&self, key_name: Option<&str>, denom: Option<&str>) -> Result<Balance, Error> {
        async fn query_balance(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            key_name: Option<&str>,
            denom: Option<&str>,
        ) -> Result<Balance, Error> {
            Ok(Balance {
                amount: String::default(),
                denom: String::default(),
            })
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_balance(relay_rpc, Some(para_rpc), key_name, denom)),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_balance(rpc, None, key_name, denom))
            }
        }
    }

    fn query_all_balances(&self, key_name: Option<&str>) -> Result<Vec<Balance>, Error> {
        async fn query_all_balances(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            key_name: Option<&str>,
        ) -> Result<Vec<Balance>, Error> {
            Ok(vec![])
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_all_balances(relay_rpc, Some(para_rpc), key_name)),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_all_balances(rpc, None, key_name))
            }
        }
    }

    fn query_denom_trace(&self, hash: String) -> Result<DenomTrace, Error> {
        fn query_denom_trace(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            hash: String,
        ) -> Result<DenomTrace, Error> {
            Ok(DenomTrace {
                /// The chain of port/channel identifiers used for tracing the source of the coin.
                path: String::default(),
                /// The base denomination for that coin
                base_denom: String::default(),
            })
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_denom_trace(self.rt.clone(), relay_rpc, Some(para_rpc), hash),
            RpcClient::SubChainRpc { rpc } => query_denom_trace(self.rt.clone(), rpc, None, hash),
        }
    }

    fn query_commitment_prefix(&self) -> Result<CommitmentPrefix, Error> {
        crate::time!("query_commitment_prefix");
        crate::telemetry!(query, self.id(), "query_commitment_prefix");

        // TODO - do a real chain query
        CommitmentPrefix::try_from(self.config.store_prefix.as_bytes().to_vec())
            .map_err(|_| Error::ics02(ClientError::empty_prefix()))
    }

    /// Query the application status
    fn query_application_status(&self) -> Result<ChainStatus, Error> {
        crate::time!("query_application_status");
        crate::telemetry!(query, self.id(), "query_application_status");

        async fn query_application_state(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
        ) -> Result<ChainStatus, Error> {
            if let Some(rpc_client) = para_rpc_client {
                use subxt::config::Header;
                let finalized_head_hash = rpc_client.rpc().finalized_head().await.unwrap();

                let block = rpc_client
                    .rpc()
                    .block(Some(finalized_head_hash))
                    .await
                    .unwrap();

                Ok(ChainStatus {
                    height: ICSHeight::new(0, u64::from(block.unwrap().block.header.number()))
                        .unwrap(),
                    timestamp: Timestamp::default(),
                })
            } else {
                use subxt::config::Header;
                let finalized_head_hash = relay_rpc_client.rpc().finalized_head().await.unwrap();

                let block = relay_rpc_client
                    .rpc()
                    .block(Some(finalized_head_hash))
                    .await
                    .unwrap();

                Ok(ChainStatus {
                    height: ICSHeight::new(0, u64::from(block.unwrap().block.header.number()))
                        .unwrap(),
                    timestamp: Timestamp::default(),
                })
            }
        }

        match &self.rpc_client {
            RpcClient::SubChainRpc { rpc } => self.block_on(query_application_state(rpc, None)),
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_application_state(relay_rpc, Some(para_rpc))),
        }
    }

    fn query_clients(
        &self,
        request: QueryClientStatesRequest,
    ) -> Result<Vec<IdentifiedAnyClientState>, Error> {
        crate::time!("query_clients");
        crate::telemetry!(query, self.id(), "query_clients");

        async fn query_clients(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryClientStatesRequest,
        ) -> Result<Vec<IdentifiedAnyClientState>, Error> {
            if let Some(rpc_client) = para_rpc_client {
                let key_addr = parachain_node::storage().ibc().client_states_root();

                let mut iter = rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let key = parachain_node::runtime_types::ibc::core::ics24_host::path::ClientStatePath::decode(&mut &*raw_key).unwrap();
                    let client_id = ClientId::from(key.0);
                    // todo (davirian) maybe this have error
                    let client_state = AnyClientState::decode_vec(&value).map_err(Error::decode)?;

                    result.push(IdentifiedAnyClientState {
                        client_id,
                        client_state,
                    });
                }
                Ok(result)
            } else {
                let key_addr = relaychain_node::storage().ibc().client_states_root();

                let mut iter = relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let key = relaychain_node::runtime_types::ibc::core::ics24_host::path::ClientStatePath::decode(&mut &*raw_key).unwrap();
                    let client_id = ClientId::from(key.0);
                    // todo (davirian) maybe this have error
                    let client_state = AnyClientState::decode_vec(&value).map_err(Error::decode)?;

                    result.push(IdentifiedAnyClientState {
                        client_id,
                        client_state,
                    });
                }
                Ok(result)
            }
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_clients(relay_rpc, Some(para_rpc), request)),
            RpcClient::SubChainRpc { rpc } => self.block_on(query_clients(rpc, None, request)),
        }
    }

    fn query_client_state(
        &self,
        request: QueryClientStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyClientState, Option<MerkleProof>), Error> {
        crate::time!("query_client_state");
        crate::telemetry!(query, self.id(), "query_client_state");

        println!("query_client_state: request: {:?}", request);
        println!("query_client_state: include_proof: {:?}", include_proof);

        async fn query_client_state(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryClientStateRequest,
            include_proof: IncludeProof,
        ) -> Result<(AnyClientState, Option<MerkleProof>), Error> {
            if let Some(rpc_client) = para_rpc_client {
                let client_id =
                    parachain_node::runtime_types::ibc::core::ics24_host::identifier::ClientId(
                        request.client_id.to_string(),
                    );
                let client_state_path =
                    parachain_node::runtime_types::ibc::core::ics24_host::path::ClientStatePath(
                        client_id,
                    );
                let storage = parachain_node::storage()
                    .ibc()
                    .client_states(client_state_path);

                let query_hash = match request.height {
                    QueryHeight::Latest => rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };
                // get client_state pb encoded value
                let client_state = rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();

                // let client_state =
                //     AnyClientState::decode_vec(&client_state).map_err(Error::decode)?;

                // Ok((client_state, None))
                let any_client_state =
                    AnyClientState::decode_vec(&client_state).map_err(Error::decode)?;

                debug!(
                    "substrate::query_client_state -> states: {:?}",
                    any_client_state
                );
                match include_proof {
                    IncludeProof::Yes => {
                        // scale encode client_state
                        // let value = codec::Encode::encode(&client_state);
                        let value = client_state.clone();
                        let state_proof = utils::build_state_proof(
                            relay_rpc_client,
                            query_hash,
                            storage.to_bytes(),
                            value,
                        )
                        .await;
                        // debug!(
                        //     "substrate::query_client_state -> state_proof: {:?}",
                        //     state_proof
                        // );
                        let merkle_proof = utils::build_ics23_merkle_proof(state_proof.unwrap());
                        Ok((any_client_state, merkle_proof))
                    }
                    IncludeProof::No => Ok((any_client_state, None)),
                }
            } else {
                let client_id =
                    relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ClientId(
                        request.client_id.to_string(),
                    );
                let client_state_path =
                    relaychain_node::runtime_types::ibc::core::ics24_host::path::ClientStatePath(
                        client_id,
                    );
                let storage = relaychain_node::storage()
                    .ibc()
                    .client_states(client_state_path);

                let query_hash = match request.height {
                    QueryHeight::Latest => relay_rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => relay_rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };
                let client_state = relay_rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();

                // let client_state =
                //     AnyClientState::decode_vec(&client_state.unwrap()).map_err(Error::decode)?;

                // Ok((client_state, None))
                let any_client_state =
                    AnyClientState::decode_vec(&client_state).map_err(Error::decode)?;

                debug!(
                    "substrate::query_client_state -> states: {:?}",
                    any_client_state
                );
                match include_proof {
                    IncludeProof::Yes => {
                        // scale encode client_state
                        // let value = codec::Encode::encode(&client_state);
                        let value = client_state.clone();
                        let state_proof = utils::build_state_proof(
                            relay_rpc_client,
                            query_hash,
                            storage.to_bytes(),
                            value,
                        )
                        .await;
                        // debug!(
                        //     "substrate::query_client_state -> state_proof: {:?}",
                        //     state_proof
                        // );
                        let merkle_proof = utils::build_ics23_merkle_proof(state_proof.unwrap());
                        Ok((any_client_state, merkle_proof))
                    }
                    IncludeProof::No => Ok((any_client_state, None)),
                }
            }
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_client_state(
                relay_rpc,
                Some(para_rpc),
                request,
                include_proof,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_client_state(rpc, None, request, include_proof))
            }
        }
    }

    fn query_upgraded_client_state(
        &self,
        request: QueryUpgradedClientStateRequest,
    ) -> Result<(AnyClientState, MerkleProof), Error> {
        crate::time!("query_upgraded_client_state");
        crate::telemetry!(query, self.id(), "query_upgraded_client_state");
        async fn query_upgraded_client_state(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryUpgradedClientStateRequest,
        ) -> Result<(AnyClientState, MerkleProof), Error> {
            todo!() // can todo
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_upgraded_client_state(
                relay_rpc,
                Some(para_rpc),
                request,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_upgraded_client_state(rpc, None, request))
            }
        }
    }

    fn query_upgraded_consensus_state(
        &self,
        request: QueryUpgradedConsensusStateRequest,
    ) -> Result<(AnyConsensusState, MerkleProof), Error> {
        crate::time!("query_upgraded_consensus_state");
        crate::telemetry!(query, self.id(), "query_upgraded_consensus_state");
        async fn query_upgraded_consensus_state(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryUpgradedConsensusStateRequest,
        ) -> Result<(AnyConsensusState, MerkleProof), Error> {
            todo!() // can todo
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_upgraded_consensus_state(
                relay_rpc,
                Some(para_rpc),
                request,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_upgraded_consensus_state(rpc, None, request))
            }
        }
    }

    // todo
    fn query_consensus_state_heights(
        &self,
        request: QueryConsensusStateHeightsRequest,
    ) -> Result<Vec<ICSHeight>, Error> {
        async fn query_consensus_state_heights(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryConsensusStateHeightsRequest,
        ) -> Result<Vec<ICSHeight>, Error> {
            if let Some(rpc_client) = para_rpc_client {
                let key_addr = parachain_node::storage().ibc().consensus_states_root();

                let mut iter = rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let key = parachain_node::runtime_types::ibc::core::ics24_host::path::ClientConsensusStatePath::decode(&mut &*raw_key).unwrap();
                    let right_client_id = ClientId::from(key.client_id);
                    if request.client_id == right_client_id {
                        result.push(ICSHeight::new(key.epoch, key.height).unwrap());
                    }
                }
                Ok(result)
            } else {
                let key_addr = relaychain_node::storage().ibc().consensus_states_root();

                let mut iter = relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let key = relaychain_node::runtime_types::ibc::core::ics24_host::path::ClientConsensusStatePath::decode(&mut &*raw_key).unwrap();
                    let right_client_id = ClientId::from(key.client_id);
                    if request.client_id == right_client_id {
                        result.push(ICSHeight::new(key.epoch, key.height).unwrap());
                    }
                }
                Ok(result)
            }
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_consensus_state_heights(
                relay_rpc,
                Some(para_rpc),
                request,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_consensus_state_heights(rpc, None, request))
            }
        }
    }

    fn query_consensus_state(
        &self,
        request: QueryConsensusStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyConsensusState, Option<MerkleProof>), Error> {
        crate::time!("query_consensus_state");
        crate::telemetry!(query, self.id(), "query_consensus_state");

        async fn query_consensus_state(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryConsensusStateRequest,
            include_proof: IncludeProof,
        ) -> Result<(AnyConsensusState, Option<MerkleProof>), Error> {
            if let Some(rpc_client) = para_rpc_client {
                let client_id =
                    parachain_node::runtime_types::ibc::core::ics24_host::identifier::ClientId(
                        request.client_id.to_string(),
                    );

                let client_conesnsus_state_path =
                    parachain_node::runtime_types::ibc::core::ics24_host::path::ClientConsensusStatePath {
                        client_id,
                        epoch: request.consensus_height.revision_number(),
                        height: request.consensus_height.revision_height()
                    };
                let storage = parachain_node::storage()
                    .ibc()
                    .consensus_states(client_conesnsus_state_path);

                let query_hash = match request.query_height {
                    QueryHeight::Latest => rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };

                let consensus_states = rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();

                // let consensus_state = AnyConsensusState::decode_vec(&consensus_states.unwrap())
                //     .map_err(Error::decode)?;

                // Ok((consensus_state, None))

                let any_consensus_state =
                    AnyConsensusState::decode_vec(&consensus_states).map_err(Error::decode)?;

                debug!(
                    "substrate::query_consensus_state -> consensus_state: {:?}",
                    any_consensus_state
                );
                match include_proof {
                    IncludeProof::Yes => {
                        // scale encode consensus_states
                        // let value = codec::Encode::encode(&consensus_states);
                        let value = consensus_states.clone();
                        let state_proof = utils::build_state_proof(
                            relay_rpc_client,
                            query_hash,
                            storage.to_bytes(),
                            value,
                        )
                        .await;
                        // debug!(
                        //     "substrate::query_consensus_state -> state_proof: {:?}",
                        //     state_proof
                        // );
                        let merkle_proof = utils::build_ics23_merkle_proof(state_proof.unwrap());
                        Ok((any_consensus_state, merkle_proof))
                    }
                    IncludeProof::No => Ok((any_consensus_state, None)),
                }
            } else {
                let client_id =
                    relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ClientId(
                        request.client_id.to_string(),
                    );
                let client_conesnsus_state_path =
                    relaychain_node::runtime_types::ibc::core::ics24_host::path::ClientConsensusStatePath {
                        client_id,
                        epoch: request.consensus_height.revision_number(),
                        height: request.consensus_height.revision_height()
                    };
                let storage = relaychain_node::storage()
                    .ibc()
                    .consensus_states(client_conesnsus_state_path);

                let query_hash = match request.query_height {
                    QueryHeight::Latest => relay_rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => relay_rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };
                let consensus_states = relay_rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();

                let any_consensus_state =
                    AnyConsensusState::decode_vec(&consensus_states).map_err(Error::decode)?;

                debug!(
                    "substrate::query_consensus_state -> consensus_state: {:?}",
                    any_consensus_state
                );
                match include_proof {
                    IncludeProof::Yes => {
                        // scale encode consensus_states
                        // let value = codec::Encode::encode(&consensus_states);
                        let value = consensus_states.clone();
                        let state_proof = utils::build_state_proof(
                            relay_rpc_client,
                            query_hash,
                            storage.to_bytes(),
                            value,
                        )
                        .await;
                        // debug!(
                        //     "substrate::query_consensus_state -> state_proof: {:?}",
                        //     state_proof
                        // );
                        let merkle_proof = utils::build_ics23_merkle_proof(state_proof.unwrap());
                        Ok((any_consensus_state, merkle_proof))
                    }
                    IncludeProof::No => Ok((any_consensus_state, None)),
                }
            }
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_consensus_state(
                relay_rpc,
                Some(para_rpc),
                request,
                include_proof,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_consensus_state(rpc, None, request, include_proof))
            }
        }
    }

    fn query_client_connections(
        &self,
        request: QueryClientConnectionsRequest,
    ) -> Result<Vec<ConnectionId>, Error> {
        crate::time!("query_client_connections");
        crate::telemetry!(query, self.id(), "query_client_connections");

        async fn query_client_connections(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryClientConnectionsRequest,
        ) -> Result<Vec<ConnectionId>, Error> {
            let mut conn_ids = Vec::<ConnectionId>::new();
            if let Some(rpc_client) = para_rpc_client {
                let client_id =
                    parachain_node::runtime_types::ibc::core::ics24_host::identifier::ClientId(
                        request.client_id.to_string(),
                    );

                let storage = parachain_node::storage().ibc().connection_client(client_id);

                let connection_id = rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap();

                if let Some(conn) = connection_id {
                    conn_ids.push(conn.into())
                }

                Ok(conn_ids)
            } else {
                let client_id =
                    relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ClientId(
                        request.client_id.to_string(),
                    );

                let storage = relaychain_node::storage()
                    .ibc()
                    .connection_client(client_id);

                let connection_id = relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap();

                if let Some(conn) = connection_id {
                    conn_ids.push(conn.into())
                }

                Ok(conn_ids)
            }
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_client_connections(relay_rpc, Some(para_rpc), request)),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_client_connections(rpc, None, request))
            }
        }
    }

    fn query_connections(
        &self,
        request: QueryConnectionsRequest,
    ) -> Result<Vec<IdentifiedConnectionEnd>, Error> {
        crate::time!("query_connections");
        crate::telemetry!(query, self.id(), "query_connections");

        async fn query_connections(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryConnectionsRequest,
        ) -> Result<Vec<IdentifiedConnectionEnd>, Error> {
            if let Some(parac_rpc) = para_rpc_client {
                let key_addr = parachain_node::storage().ibc().connections_root();

                let mut iter = parac_rpc
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let ret = parachain_node::runtime_types::ibc::core::ics24_host::path::ConnectionsPath::decode(&mut &*raw_key).unwrap();
                    let connection_id = ConnectionId::from(ret.0);
                    let connection_end = ConnectionEnd::from(value);
                    result.push(IdentifiedConnectionEnd {
                        connection_id,
                        connection_end,
                    });
                }
                Ok(result)
            } else {
                let key_addr = relaychain_node::storage().ibc().connections_root();

                let mut iter = relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let ret = relaychain_node::runtime_types::ibc::core::ics24_host::path::ConnectionsPath::decode(&mut &*raw_key).unwrap();
                    let connection_id = ConnectionId::from(ret.0);
                    let connection_end = ConnectionEnd::from(value);
                    result.push(IdentifiedConnectionEnd {
                        connection_id,
                        connection_end,
                    });
                }
                Ok(result)
            }
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_connections(relay_rpc, Some(para_rpc), request)),
            RpcClient::SubChainRpc { rpc } => self.block_on(query_connections(rpc, None, request)),
        }
    }

    fn query_connection(
        &self,
        request: QueryConnectionRequest,
        include_proof: IncludeProof,
    ) -> Result<(ConnectionEnd, Option<MerkleProof>), Error> {
        crate::time!("query_connection");
        crate::telemetry!(query, self.id(), "query_connection");

        async fn query_connection(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryConnectionRequest,
            include_proof: IncludeProof,
        ) -> Result<(ConnectionEnd, Option<MerkleProof>), Error> {
            debug!(
                "substrate::query_connection -> QueryConnectionRequest: {:?}, IncludeProof: {:?}",
                request, include_proof
            );
            if let Some(rpc_client) = para_rpc_client {
                let connection_id =
                    parachain_node::runtime_types::ibc::core::ics24_host::identifier::ConnectionId(
                        request.connection_id.to_string(),
                    );
                let connection_path =
                    parachain_node::runtime_types::ibc::core::ics24_host::path::ConnectionsPath(
                        connection_id,
                    );
                let storage = parachain_node::storage().ibc().connections(connection_path);

                let query_hash = match request.height {
                    QueryHeight::Latest => rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };

                let connection = rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();

                // Ok((connection.into(), None))

                debug!(
                    "substrate::query_connection -> connection: {:?}",
                    connection
                );
                match include_proof {
                    IncludeProof::Yes => {
                        // scale encode consensus_states
                        let value = codec::Encode::encode(&connection);
                        let state_proof = utils::build_state_proof(
                            relay_rpc_client,
                            query_hash,
                            storage.to_bytes(),
                            value,
                        )
                        .await;
                        // debug!(
                        //     "substrate::query_connection -> state_proof: {:?}",
                        //     state_proof
                        // );
                        let merkle_proof = utils::build_ics23_merkle_proof(state_proof.unwrap());
                        Ok((connection.into(), merkle_proof))
                    }
                    IncludeProof::No => Ok((connection.into(), None)),
                }
            } else {
                let connection_id =
                    relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ConnectionId(
                        request.connection_id.to_string(),
                    );
                let connection_path =
                    relaychain_node::runtime_types::ibc::core::ics24_host::path::ConnectionsPath(
                        connection_id,
                    );
                debug!(
                    "substrate::query_connection -> connection_id: {:?} connection_path: {:?}",
                    request.connection_id, connection_path
                );

                let storage = relaychain_node::storage()
                    .ibc()
                    .connections(connection_path);

                debug!(
                    "substrate::query_connection -> storage key: {:?}",
                    hex::encode(storage.to_bytes())
                );

                let query_hash = match request.height {
                    QueryHeight::Latest => {
                        let query_hash = relay_rpc_client.rpc().block_hash(None).await.unwrap();
                        debug!(
                            "substrate::query_connection -> query_height: latest height, query_hash: {:?}",
                            query_hash
                        );
                        query_hash
                    }
                    QueryHeight::Specific(v) => {
                        let query_height =
                            subxt::rpc::types::BlockNumber::from(v.revision_height());
                        let query_hash = relay_rpc_client
                            .rpc()
                            .block_hash(Some(query_height))
                            .await
                            .unwrap();
                        debug!(
                            "substrate::query_connection -> query_height: {:?}, query_hash: {:?}",
                            v, query_hash
                        );
                        query_hash
                    }
                };
                let connection = relay_rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();

                // Ok((connection.into(), None))
                debug!(
                    "substrate::query_connection -> connection: {:?}",
                    connection
                );
                match include_proof {
                    IncludeProof::Yes => {
                        // scale encode consensus_states
                        let value = codec::Encode::encode(&connection);
                        let state_proof = utils::build_state_proof(
                            relay_rpc_client,
                            query_hash,
                            storage.to_bytes(),
                            value,
                        )
                        .await;
                        // debug!(
                        //     "substrate::query_connection -> state_proof: {:?}",
                        //     state_proof
                        // );
                        let merkle_proof = utils::build_ics23_merkle_proof(state_proof.unwrap());
                        Ok((connection.into(), merkle_proof))
                    }
                    IncludeProof::No => Ok((connection.into(), None)),
                }
            }
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_connection(
                relay_rpc,
                Some(para_rpc),
                request,
                include_proof,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_connection(rpc, None, request, include_proof))
            }
        }
    }

    // need todo
    fn query_connection_channels(
        &self,
        request: QueryConnectionChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        crate::time!("query_connection_channels");
        crate::telemetry!(query, self.id(), "query_connection_channels");

        async fn query_connection_channels(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryConnectionChannelsRequest,
        ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
            if let Some(rpc_client) = para_rpc_client {
                let key_addr = parachain_node::storage().ibc().channels_connection_root();

                let mut iter = rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut port_id_and_channel_id = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = parachain_node::runtime_types::ibc::core::ics24_host::identifier::ConnectionId::decode(&mut &*raw_key).unwrap();
                    let connection_id = ConnectionId::from(rets);
                    if request.connection_id == connection_id {
                        port_id_and_channel_id = value;
                    }
                }
                let port_id_and_channel_id = port_id_and_channel_id
                    .into_iter()
                    .map(|(p, c)| {
                        let port_id = PortId::from(p);
                        let channel_id = ChannelId::from(c);
                        (port_id, channel_id)
                    })
                    .collect::<Vec<_>>();

                let key_addr = parachain_node::storage().ibc().channels_root();

                let mut iter = rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = parachain_node::runtime_types::ibc::core::ics24_host::path::ChannelEndsPath::decode(&mut &*raw_key).unwrap();

                    let port_id = PortId::from(rets.0);
                    let channel_id = ChannelId::from(rets.1);
                    let channel_end = ChannelEnd::from(value);
                    for item in port_id_and_channel_id.iter() {
                        if &item.0 == &port_id && &item.1 == &channel_id {
                            result.push(IdentifiedChannelEnd {
                                port_id: port_id.clone(),
                                channel_id: channel_id.clone(),
                                channel_end: channel_end.clone(),
                            })
                        }
                    }
                }
                Ok(result)
            } else {
                let key_addr = relaychain_node::storage().ibc().channels_connection_root();

                let mut iter = relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut port_id_and_channel_id = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ConnectionId::decode(&mut &*raw_key).unwrap();
                    let connection_id = ConnectionId::from(rets);
                    if request.connection_id == connection_id {
                        port_id_and_channel_id = value;
                    }
                }
                let port_id_and_channel_id = port_id_and_channel_id
                    .into_iter()
                    .map(|(p, c)| {
                        let port_id = PortId::from(p);
                        let channel_id = ChannelId::from(c);
                        (port_id, channel_id)
                    })
                    .collect::<Vec<_>>();

                let key_addr = relaychain_node::storage().ibc().channels_root();

                let mut iter = relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = relaychain_node::runtime_types::ibc::core::ics24_host::path::ChannelEndsPath::decode(&mut &*raw_key).unwrap();

                    let port_id = PortId::from(rets.0);
                    let channel_id = ChannelId::from(rets.1);
                    let channel_end = ChannelEnd::from(value);
                    for item in port_id_and_channel_id.iter() {
                        if &item.0 == &port_id && &item.1 == &channel_id {
                            result.push(IdentifiedChannelEnd {
                                port_id: port_id.clone(),
                                channel_id: channel_id.clone(),
                                channel_end: channel_end.clone(),
                            })
                        }
                    }
                }
                Ok(result)
            }
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_connection_channels(
                relay_rpc,
                Some(para_rpc),
                request,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_connection_channels(rpc, None, request))
            }
        }
    }

    fn query_channels(
        &self,
        request: QueryChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        crate::time!("query_channels");
        crate::telemetry!(query, self.id(), "query_channels");

        async fn query_channels(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryChannelsRequest,
        ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
            if let Some(rpc_client) = para_rpc_client {
                let key_addr = parachain_node::storage().ibc().channels_root();

                let mut iter = rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = parachain_node::runtime_types::ibc::core::ics24_host::path::ChannelEndsPath::decode(&mut &*raw_key).unwrap();
                    let port_id = PortId::from(rets.0);
                    let channel_id = ChannelId::from(rets.1);
                    let channel_end = ChannelEnd::from(value);

                    result.push(IdentifiedChannelEnd {
                        port_id,
                        channel_id,
                        channel_end,
                    })
                }
                Ok(result)
            } else {
                let key_addr = relaychain_node::storage().ibc().channels_root();

                let mut iter = relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = relaychain_node::runtime_types::ibc::core::ics24_host::path::ChannelEndsPath::decode(&mut &*raw_key).unwrap();
                    let port_id = PortId::from(rets.0);
                    let channel_id = ChannelId::from(rets.1);
                    let channel_end = ChannelEnd::from(value);

                    result.push(IdentifiedChannelEnd {
                        port_id,
                        channel_id,
                        channel_end,
                    })
                }
                Ok(result)
            }
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_channels(relay_rpc, Some(para_rpc), request)),
            RpcClient::SubChainRpc { rpc } => self.block_on(query_channels(rpc, None, request)),
        }
    }

    fn query_channel(
        &self,
        request: QueryChannelRequest,
        include_proof: IncludeProof,
    ) -> Result<(ChannelEnd, Option<MerkleProof>), Error> {
        crate::time!("query_channel");
        crate::telemetry!(query, self.id(), "query_channel");

        async fn query_channel(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryChannelRequest,
            include_proof: IncludeProof,
        ) -> Result<(ChannelEnd, Option<MerkleProof>), Error> {
            if let Some(rpc_client) = para_rpc_client {
                let port_id =
                    parachain_node::runtime_types::ibc::core::ics24_host::identifier::PortId(
                        request.port_id.to_string(),
                    );
                let channel_id =
                    parachain_node::runtime_types::ibc::core::ics24_host::identifier::ChannelId(
                        request.channel_id.to_string(),
                    );
                let channel_ends_path =
                    parachain_node::runtime_types::ibc::core::ics24_host::path::ChannelEndsPath(
                        port_id, channel_id,
                    );
                let storage = parachain_node::storage().ibc().channels(channel_ends_path);

                let query_hash = match request.height {
                    QueryHeight::Latest => rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };

                let result = rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();

                // Ok((result.into(), None))
                debug!("substrate::query_channel -> channel_end: {:?}", result);
                match include_proof {
                    IncludeProof::Yes => {
                        // scale encode result
                        let value = codec::Encode::encode(&result);
                        let state_proof = utils::build_state_proof(
                            relay_rpc_client,
                            query_hash,
                            storage.to_bytes(),
                            value,
                        )
                        .await;
                        // debug!("substrate::query_channel -> state_proof: {:?}", state_proof);
                        let merkle_proof = utils::build_ics23_merkle_proof(state_proof.unwrap());
                        Ok((result.into(), merkle_proof))
                    }
                    IncludeProof::No => Ok((result.into(), None)),
                }
            } else {
                let port_id =
                    relaychain_node::runtime_types::ibc::core::ics24_host::identifier::PortId(
                        request.port_id.to_string(),
                    );
                let channel_id =
                    relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ChannelId(
                        request.channel_id.to_string(),
                    );
                let channel_ends_path =
                    relaychain_node::runtime_types::ibc::core::ics24_host::path::ChannelEndsPath(
                        port_id, channel_id,
                    );
                let storage = relaychain_node::storage().ibc().channels(channel_ends_path);

                let query_hash = match request.height {
                    QueryHeight::Latest => relay_rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => relay_rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };
                let result = relay_rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();

                // Ok((result.into(), None))
                debug!("substrate::query_channel -> channel_end: {:?}", result);
                match include_proof {
                    IncludeProof::Yes => {
                        // scale encode consensus_states
                        let value = codec::Encode::encode(&result);
                        let state_proof = utils::build_state_proof(
                            relay_rpc_client,
                            query_hash,
                            storage.to_bytes(),
                            value,
                        )
                        .await;
                        // debug!("substrate::query_channel -> state_proof: {:?}", state_proof);
                        let merkle_proof = utils::build_ics23_merkle_proof(state_proof.unwrap());
                        Ok((result.into(), merkle_proof))
                    }
                    IncludeProof::No => Ok((result.into(), None)),
                }
            }
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_channel(
                relay_rpc,
                Some(para_rpc),
                request,
                include_proof,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_channel(rpc, None, request, include_proof))
            }
        }
    }

    // todo
    fn query_channel_client_state(
        &self,
        request: QueryChannelClientStateRequest,
    ) -> Result<Option<IdentifiedAnyClientState>, Error> {
        crate::time!("query_channel_client_state");
        crate::telemetry!(query, self.id(), "query_channel_client_state");

        async fn query_channel_client_state(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryChannelClientStateRequest,
        ) -> Result<Option<IdentifiedAnyClientState>, Error> {
            if let Some(rpc_client) = para_rpc_client {
                let key_addr = parachain_node::storage().ibc().channels_connection_root();

                let mut iter = rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result_connection_id = ConnectionId::default();
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = parachain_node::runtime_types::ibc::core::ics24_host::identifier::ConnectionId::decode(&mut &*raw_key).unwrap();
                    let connection_id = ConnectionId::from(rets);
                    for item in value {
                        if request.port_id == PortId::from(item.0)
                            && request.channel_id == ChannelId::from(item.1)
                        {
                            result_connection_id = connection_id.clone();
                        }
                    }
                }

                let key_addr = parachain_node::storage().ibc().connection_client_root();

                let mut iter = rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result_client_id = ClientId::default();
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = parachain_node::runtime_types::ibc::core::ics24_host::identifier::ClientId::decode(&mut &*raw_key).unwrap();
                    let client_id = ClientId::from(rets);
                    if result_connection_id == ConnectionId::from(value) {
                        result_client_id = client_id;
                    }
                }

                let client_id =
                    parachain_node::runtime_types::ibc::core::ics24_host::identifier::ClientId(
                        result_client_id.to_string(),
                    );
                let client_state_path =
                    parachain_node::runtime_types::ibc::core::ics24_host::path::ClientStatePath(
                        client_id,
                    );

                let storage = parachain_node::storage()
                    .ibc()
                    .client_states(client_state_path);

                let client_state = rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap();

                let client_state =
                    AnyClientState::decode_vec(&client_state.unwrap()).map_err(Error::decode)?;

                Ok(Some(IdentifiedAnyClientState {
                    client_id: result_client_id,
                    client_state,
                }))
            } else {
                let key_addr = relaychain_node::storage().ibc().channels_connection_root();

                let mut iter = relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result_connection_id = ConnectionId::default();
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ConnectionId::decode(&mut &*raw_key).unwrap();
                    let connection_id = ConnectionId::from(rets);
                    for item in value {
                        if request.port_id == PortId::from(item.0)
                            && request.channel_id == ChannelId::from(item.1)
                        {
                            result_connection_id = connection_id.clone();
                        }
                    }
                }

                let key_addr = relaychain_node::storage().ibc().connection_client_root();

                let mut iter = relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result_client_id = ClientId::default();
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ClientId::decode(&mut &*raw_key).unwrap();
                    let client_id = ClientId::from(rets);
                    if result_connection_id == ConnectionId::from(value) {
                        result_client_id = client_id;
                    }
                }

                let client_id =
                    relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ClientId(
                        result_client_id.to_string(),
                    );
                let client_state_path =
                    relaychain_node::runtime_types::ibc::core::ics24_host::path::ClientStatePath(
                        client_id,
                    );

                let storage = relaychain_node::storage()
                    .ibc()
                    .client_states(client_state_path);

                let client_state = relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap();

                let client_state =
                    AnyClientState::decode_vec(&client_state.unwrap()).map_err(Error::decode)?;

                Ok(Some(IdentifiedAnyClientState {
                    client_id: result_client_id,
                    client_state,
                }))
            }
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_channel_client_state(
                relay_rpc,
                Some(para_rpc),
                request,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_channel_client_state(rpc, None, request))
            }
        }
    }

    fn query_packet_commitment(
        &self,
        request: QueryPacketCommitmentRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        async fn query_packet_commitment(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryPacketCommitmentRequest,
            include_proof: IncludeProof,
        ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
            if let Some(rpc_client) = para_rpc_client {
                let port_id =
                    parachain_node::runtime_types::ibc::core::ics24_host::identifier::PortId(
                        request.port_id.to_string(),
                    );
                let channel_id =
                    parachain_node::runtime_types::ibc::core::ics24_host::identifier::ChannelId(
                        request.channel_id.to_string(),
                    );
                let sequence =
                    parachain_node::runtime_types::ibc::core::ics04_channel::packet::Sequence(
                        u64::from(request.sequence),
                    );
                let packet_commitment_path =
                    parachain_node::runtime_types::ibc::core::ics24_host::path::CommitmentsPath {
                        port_id,
                        channel_id,
                        sequence,
                    };
                let storage = parachain_node::storage()
                    .ibc()
                    .packet_commitment(packet_commitment_path);

                let query_hash = match request.height {
                    QueryHeight::Latest => rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };
                let result = rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();
                // Ok((result.0, None))
                debug!(
                    "substrate::query_packet_commitment -> packet commitment: {:?}",
                    result
                );
                match include_proof {
                    IncludeProof::Yes => {
                        // scale encode result
                        let value = codec::Encode::encode(&result);
                        let state_proof = utils::build_state_proof(
                            relay_rpc_client,
                            query_hash,
                            storage.to_bytes(),
                            value,
                        )
                        .await;
                        // debug!("substrate::query_packet_commitment -> state_proof: {:?}", state_proof);
                        let merkle_proof = utils::build_ics23_merkle_proof(state_proof.unwrap());
                        Ok((result.0, merkle_proof))
                    }
                    IncludeProof::No => Ok((result.0, None)),
                }
            } else {
                let port_id =
                    relaychain_node::runtime_types::ibc::core::ics24_host::identifier::PortId(
                        request.port_id.to_string(),
                    );
                let channel_id =
                    relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ChannelId(
                        request.channel_id.to_string(),
                    );
                let sequence =
                    relaychain_node::runtime_types::ibc::core::ics04_channel::packet::Sequence(
                        u64::from(request.sequence),
                    );
                let packet_commitment_path =
                    relaychain_node::runtime_types::ibc::core::ics24_host::path::CommitmentsPath {
                        port_id,
                        channel_id,
                        sequence,
                    };
                let storage = relaychain_node::storage()
                    .ibc()
                    .packet_commitment(packet_commitment_path);

                let query_hash = match request.height {
                    QueryHeight::Latest => relay_rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => relay_rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };

                let result = relay_rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();
                // Ok((result.0, None))
                debug!(
                    "substrate::query_packet_commitment -> packet commitment: {:?}",
                    result
                );
                match include_proof {
                    IncludeProof::Yes => {
                        // scale encode result
                        let value = codec::Encode::encode(&result);
                        let state_proof = utils::build_state_proof(
                            relay_rpc_client,
                            query_hash,
                            storage.to_bytes(),
                            value,
                        )
                        .await;
                        // debug!("substrate::query_packet_commitment -> state_proof: {:?}", state_proof);
                        let merkle_proof = utils::build_ics23_merkle_proof(state_proof.unwrap());
                        Ok((result.0, merkle_proof))
                    }
                    IncludeProof::No => Ok((result.0, None)),
                }
            }
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_packet_commitment(
                relay_rpc,
                Some(para_rpc),
                request,
                include_proof,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_packet_commitment(rpc, None, request, include_proof))
            }
        }
    }

    /// Queries the packet commitment hashes associated with a channel.
    fn query_packet_commitments(
        &self,
        request: QueryPacketCommitmentsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), Error> {
        crate::time!("query_packet_commitments");
        crate::telemetry!(query, self.id(), "query_packet_commitments");

        async fn query_packet_commitments(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryPacketCommitmentsRequest,
        ) -> Result<(Vec<Sequence>, ICSHeight), Error> {
            if let Some(rpc_client) = para_rpc_client {
                let key_addr = parachain_node::storage().ibc().channels_root();

                let mut iter = rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = parachain_node::runtime_types::ibc::core::ics24_host::path::CommitmentsPath::decode(&mut &*raw_key).unwrap();
                    let port_id = PortId::from(rets.port_id);
                    let channel_id = ChannelId::from(rets.channel_id);
                    let sequence = Sequence::from(rets.sequence);

                    if port_id == request.port_id && channel_id == request.channel_id {
                        result.push(sequence);
                    }
                }
                use subxt::config::Header;
                let finalized_head_hash = rpc_client.rpc().finalized_head().await.unwrap();

                let block = rpc_client
                    .rpc()
                    .block(Some(finalized_head_hash))
                    .await
                    .unwrap();
                Ok((
                    result,
                    ICSHeight::new(0, u64::from(block.unwrap().block.header.number())).unwrap(),
                ))
            } else {
                let key_addr = relaychain_node::storage().ibc().channels_root();

                let mut iter = relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = relaychain_node::runtime_types::ibc::core::ics24_host::path::CommitmentsPath::decode(&mut &*raw_key).unwrap();
                    let port_id = PortId::from(rets.port_id);
                    let channel_id = ChannelId::from(rets.channel_id);
                    let sequence = Sequence::from(rets.sequence);

                    if port_id == request.port_id && channel_id == request.channel_id {
                        result.push(sequence);
                    }
                }
                use subxt::config::Header;
                let finalized_head_hash = relay_rpc_client.rpc().finalized_head().await.unwrap();

                let block = relay_rpc_client
                    .rpc()
                    .block(Some(finalized_head_hash))
                    .await
                    .unwrap();
                Ok((
                    result,
                    ICSHeight::new(0, u64::from(block.unwrap().block.header.number())).unwrap(),
                ))
            }
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_packet_commitments(relay_rpc, Some(para_rpc), request)),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_packet_commitments(rpc, None, request))
            }
        }
    }

    fn query_packet_receipt(
        &self,
        request: QueryPacketReceiptRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        async fn query_packet_receipt(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryPacketReceiptRequest,
            include_proof: IncludeProof,
        ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
            if let Some(rpc_client) = para_rpc_client {
                let port_id =
                    parachain_node::runtime_types::ibc::core::ics24_host::identifier::PortId(
                        request.port_id.to_string(),
                    );
                let channel_id =
                    parachain_node::runtime_types::ibc::core::ics24_host::identifier::ChannelId(
                        request.channel_id.to_string(),
                    );
                let sequence =
                    parachain_node::runtime_types::ibc::core::ics04_channel::packet::Sequence(
                        u64::from(request.sequence),
                    );

                let packet_receipt_path =
                    parachain_node::runtime_types::ibc::core::ics24_host::path::ReceiptsPath {
                        port_id,
                        channel_id,
                        sequence,
                    };

                let storage = parachain_node::storage()
                    .ibc()
                    .packet_receipt(packet_receipt_path);

                let query_hash = match request.height {
                    QueryHeight::Latest => rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };
                let result = rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();
                // match result {
                //     parachain_node::runtime_types::ibc::core::ics04_channel::packet::Receipt::Ok => Ok((b"ok".to_vec(), None)),
                // }

                debug!("substrate::query_packet_receipt -> receipt: {:?}", result);
                match result {
                    parachain_node::runtime_types::ibc::core::ics04_channel::packet::Receipt::Ok => {

                        match include_proof {
                            IncludeProof::Yes => {
                                // scale encode result
                                let value = codec::Encode::encode(&result);
                               let state_proof= utils::build_state_proof(relay_rpc_client, query_hash,
                                storage.to_bytes(), value).await;
                                // debug!("substrate::query_packet_commitment -> state_proof: {:?}", state_proof);
                                let merkle_proof = utils::build_ics23_merkle_proof(state_proof.unwrap());
                                Ok((vec![0], merkle_proof))
                            },
                            IncludeProof::No => Ok((vec![0], None)),
                        }
                        // Ok((vec![0], None))
                    }

                }
            } else {
                let port_id =
                    relaychain_node::runtime_types::ibc::core::ics24_host::identifier::PortId(
                        request.port_id.to_string(),
                    );
                let channel_id =
                    relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ChannelId(
                        request.channel_id.to_string(),
                    );
                let sequence =
                    relaychain_node::runtime_types::ibc::core::ics04_channel::packet::Sequence(
                        u64::from(request.sequence),
                    );
                let packet_receipt_path =
                    relaychain_node::runtime_types::ibc::core::ics24_host::path::ReceiptsPath {
                        port_id,
                        channel_id,
                        sequence,
                    };
                let storage = relaychain_node::storage()
                    .ibc()
                    .packet_receipt(packet_receipt_path);

                let query_hash = match request.height {
                    QueryHeight::Latest => relay_rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => relay_rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };
                let result = relay_rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();
                // match result {
                //     relaychain_node::runtime_types::ibc::core::ics04_channel::packet::Receipt::Ok => Ok((b"ok".to_vec(), None)),
                // }
                debug!("substrate::query_packet_receipt -> receipt: {:?}", result);
                match result {
                    relaychain_node::runtime_types::ibc::core::ics04_channel::packet::Receipt::Ok => {

                        match include_proof {
                            IncludeProof::Yes => {
                                // scale encode result
                                let value = codec::Encode::encode(&result);
                               let state_proof= utils::build_state_proof(relay_rpc_client, query_hash,
                                storage.to_bytes(), value).await;
                                // debug!("substrate::query_packet_commitment -> state_proof: {:?}", state_proof);
                                let merkle_proof = utils::build_ics23_merkle_proof(state_proof.unwrap());
                                Ok((vec![0], merkle_proof))
                            },
                            IncludeProof::No => Ok((vec![0], None)),
                        }
                        // Ok((vec![0], None))
                    }

                }
            }
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_packet_receipt(
                relay_rpc,
                Some(para_rpc),
                request,
                include_proof,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_packet_receipt(rpc, None, request, include_proof))
            }
        }
    }

    // todo
    /// Queries the unreceived packet sequences associated with a channel.
    fn query_unreceived_packets(
        &self,
        request: QueryUnreceivedPacketsRequest,
    ) -> Result<Vec<Sequence>, Error> {
        crate::time!("query_unreceived_packets");
        crate::telemetry!(query, self.id(), "query_unreceived_packets");

        async fn query_packet_receipt(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryUnreceivedPacketsRequest,
        ) -> Result<Vec<Sequence>, Error> {
            if let Some(rpc_client) = para_rpc_client {
                let key_addr = parachain_node::storage().ibc().channels_root();

                let mut iter = rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = parachain_node::runtime_types::ibc::core::ics24_host::path::CommitmentsPath::decode(&mut &*raw_key).unwrap();
                    let port_id = PortId::from(rets.port_id);
                    let channel_id = ChannelId::from(rets.channel_id);
                    let sequence = Sequence::from(rets.sequence);

                    if port_id == request.port_id && channel_id == request.channel_id {
                        result.push(sequence);
                    }
                }

                let mut ret = vec![];
                for seq in request.packet_commitment_sequences {
                    for in_seq in result.iter() {
                        if seq != *in_seq {
                            ret.push(seq);
                        }
                    }
                }

                Ok(ret)
            } else {
                let key_addr = relaychain_node::storage().ibc().channels_root();

                let mut iter = relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = relaychain_node::runtime_types::ibc::core::ics24_host::path::CommitmentsPath::decode(&mut &*raw_key).unwrap();
                    let port_id = PortId::from(rets.port_id);
                    let channel_id = ChannelId::from(rets.channel_id);
                    let sequence = Sequence::from(rets.sequence);

                    if port_id == request.port_id && channel_id == request.channel_id {
                        result.push(sequence);
                    }
                }

                let mut ret = vec![];
                for seq in request.packet_commitment_sequences {
                    for in_seq in result.iter() {
                        if seq != *in_seq {
                            ret.push(seq);
                        }
                    }
                }

                Ok(ret)
            }
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_packet_receipt(relay_rpc, Some(para_rpc), request)),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_packet_receipt(rpc, None, request))
            }
        }
    }

    fn query_next_sequence_receive(
        &self,
        request: QueryNextSequenceReceiveRequest,
        include_proof: IncludeProof,
    ) -> Result<(Sequence, Option<MerkleProof>), Error> {
        async fn query_next_sequence_receive(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryNextSequenceReceiveRequest,
            include_proof: IncludeProof,
        ) -> Result<(Sequence, Option<MerkleProof>), Error> {
            if let Some(rpc_client) = para_rpc_client {
                let port_id =
                    parachain_node::runtime_types::ibc::core::ics24_host::identifier::PortId(
                        request.port_id.to_string(),
                    );
                let channel_id =
                    parachain_node::runtime_types::ibc::core::ics24_host::identifier::ChannelId(
                        request.channel_id.to_string(),
                    );

                let next_sequence_recv_path =
                    parachain_node::runtime_types::ibc::core::ics24_host::path::SeqRecvsPath(
                        port_id, channel_id,
                    );

                let storage = parachain_node::storage()
                    .ibc()
                    .next_sequence_recv(next_sequence_recv_path);

                let query_hash = match request.height {
                    QueryHeight::Latest => rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => relay_rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };

                let result = relay_rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();

                // Ok((result.into(), None))

                debug!(
                    "substrate::query_next_sequence_receive -> sequence: {:?}",
                    result
                );

                match include_proof {
                    IncludeProof::Yes => {
                        // // Note: We expect the return to be a u64 encoded in big-endian. Refer to ibc-go:
                        // // https://github.com/cosmos/ibc-go/blob/25767f6bdb5bab2c2a116b41d92d753c93e18121/modules/core/04-channel/client/utils/utils.go#L191
                        // if res.value.len() != 8 {
                        //     return Err(Error::query("next_sequence_receive".into()));
                        // }
                        // let seq: Sequence = Bytes::from(res.value).get_u64().into();

                        // scale encode sequence
                        let value = codec::Encode::encode(&result);
                        let state_proof = utils::build_state_proof(
                            relay_rpc_client,
                            query_hash,
                            storage.to_bytes(),
                            value,
                        )
                        .await;
                        // debug!("substrate::query_packet_commitment -> state_proof: {:?}", state_proof);
                        let merkle_proof = utils::build_ics23_merkle_proof(state_proof.unwrap());
                        Ok((result.into(), merkle_proof))
                    }
                    IncludeProof::No => Ok((result.into(), None)),
                }
            } else {
                let port_id =
                    relaychain_node::runtime_types::ibc::core::ics24_host::identifier::PortId(
                        request.port_id.to_string(),
                    );
                let channel_id =
                    relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ChannelId(
                        request.channel_id.to_string(),
                    );

                let next_sequence_recv_path =
                    relaychain_node::runtime_types::ibc::core::ics24_host::path::SeqRecvsPath(
                        port_id, channel_id,
                    );

                let storage = relaychain_node::storage()
                    .ibc()
                    .next_sequence_recv(next_sequence_recv_path);

                let query_hash = match request.height {
                    QueryHeight::Latest => relay_rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => relay_rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };

                let result = relay_rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();
                // Ok((result.into(), None))
                match include_proof {
                    IncludeProof::Yes => {
                        // // Note: We expect the return to be a u64 encoded in big-endian. Refer to ibc-go:
                        // // https://github.com/cosmos/ibc-go/blob/25767f6bdb5bab2c2a116b41d92d753c93e18121/modules/core/04-channel/client/utils/utils.go#L191
                        // if res.value.len() != 8 {
                        //     return Err(Error::query("next_sequence_receive".into()));
                        // }
                        // let seq: Sequence = Bytes::from(res.value).get_u64().into();

                        // scale encode sequence
                        let value = codec::Encode::encode(&result);
                        let state_proof = utils::build_state_proof(
                            relay_rpc_client,
                            query_hash,
                            storage.to_bytes(),
                            value,
                        )
                        .await;
                        // debug!("substrate::query_packet_commitment -> state_proof: {:?}", state_proof);
                        let merkle_proof = utils::build_ics23_merkle_proof(state_proof.unwrap());
                        Ok((result.into(), merkle_proof))
                    }
                    IncludeProof::No => Ok((result.into(), None)),
                }
            }
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_next_sequence_receive(
                relay_rpc,
                Some(para_rpc),
                request,
                include_proof,
            )),
            RpcClient::SubChainRpc { rpc } => self.block_on(query_next_sequence_receive(
                rpc,
                None,
                request,
                include_proof,
            )),
        }
    }

    fn query_packet_acknowledgement(
        &self,
        request: QueryPacketAcknowledgementRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        async fn query_packet_acknowledgement(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryPacketAcknowledgementRequest,
            include_proof: IncludeProof,
        ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
            if let Some(rpc_client) = para_rpc_client {
                let port_id =
                    parachain_node::runtime_types::ibc::core::ics24_host::identifier::PortId(
                        request.port_id.to_string(),
                    );
                let channel_id =
                    parachain_node::runtime_types::ibc::core::ics24_host::identifier::ChannelId(
                        request.channel_id.to_string(),
                    );
                let sequence =
                    parachain_node::runtime_types::ibc::core::ics04_channel::packet::Sequence(
                        u64::from(request.sequence),
                    );

                let acknowledgement_path =
                    parachain_node::runtime_types::ibc::core::ics24_host::path::AcksPath {
                        port_id,
                        channel_id,
                        sequence,
                    };
                let storage = parachain_node::storage()
                    .ibc()
                    .acknowledgements(acknowledgement_path);

                let query_hash = match request.height {
                    QueryHeight::Latest => rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };
                let result = rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();
                // Ok((result.0, None))

                debug!(
                    "substrate::query_packet_acknowledgement -> ack commitment: {:?}",
                    result
                );

                match include_proof {
                    IncludeProof::Yes => {
                        // scale encode ack commitment
                        let value = codec::Encode::encode(&result);
                        let state_proof = utils::build_state_proof(
                            relay_rpc_client,
                            query_hash,
                            storage.to_bytes(),
                            value,
                        )
                        .await;
                        // debug!("substrate::query_packet_commitment -> state_proof: {:?}", state_proof);
                        let merkle_proof = utils::build_ics23_merkle_proof(state_proof.unwrap());
                        Ok((result.0, merkle_proof))
                    }
                    IncludeProof::No => Ok((result.0, None)),
                }
            } else {
                let port_id =
                    relaychain_node::runtime_types::ibc::core::ics24_host::identifier::PortId(
                        request.port_id.to_string(),
                    );
                let channel_id =
                    relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ChannelId(
                        request.channel_id.to_string(),
                    );
                let sequence =
                    relaychain_node::runtime_types::ibc::core::ics04_channel::packet::Sequence(
                        u64::from(request.sequence),
                    );

                let acknowledgement_path =
                    relaychain_node::runtime_types::ibc::core::ics24_host::path::AcksPath {
                        port_id,
                        channel_id,
                        sequence,
                    };
                let storage = relaychain_node::storage()
                    .ibc()
                    .acknowledgements(acknowledgement_path);

                let query_hash = match request.height {
                    QueryHeight::Latest => relay_rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => relay_rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };
                let result = relay_rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();
                // Ok((result.0, None))
                debug!(
                    "substrate::query_packet_acknowledgement -> ack commitment: {:?}",
                    result
                );

                match include_proof {
                    IncludeProof::Yes => {
                        // scale encode ack commitment
                        let value = codec::Encode::encode(&result);
                        let state_proof = utils::build_state_proof(
                            relay_rpc_client,
                            query_hash,
                            storage.to_bytes(),
                            value,
                        )
                        .await;
                        // debug!("substrate::query_packet_commitment -> state_proof: {:?}", state_proof);
                        let merkle_proof = utils::build_ics23_merkle_proof(state_proof.unwrap());
                        Ok((result.0, merkle_proof))
                    }
                    IncludeProof::No => Ok((result.0, None)),
                }
            }
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_packet_acknowledgement(
                relay_rpc,
                Some(para_rpc),
                request,
                include_proof,
            )),
            RpcClient::SubChainRpc { rpc } => self.block_on(query_packet_acknowledgement(
                rpc,
                None,
                request,
                include_proof,
            )),
        }
    }

    // todo
    /// Queries the packet acknowledgment hashes associated with a channel.
    fn query_packet_acknowledgements(
        &self,
        request: QueryPacketAcknowledgementsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), Error> {
        crate::time!("query_packet_acknowledgements");
        crate::telemetry!(query, self.id(), "query_packet_acknowledgements");

        async fn query_packet_acknowledgements(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryPacketAcknowledgementsRequest,
        ) -> Result<(Vec<Sequence>, ICSHeight), Error> {
            if let Some(rpc_client) = para_rpc_client {
                let key_addr = parachain_node::storage().ibc().acknowledgements_root();

                let mut iter = rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = parachain_node::runtime_types::ibc::core::ics24_host::path::AcksPath::decode(&mut &*raw_key).unwrap();
                    let port_id = PortId::from(rets.port_id);
                    let channel_id = ChannelId::from(rets.channel_id);
                    let sequence = Sequence::from(rets.sequence);

                    if port_id == request.port_id && channel_id == request.channel_id {
                        result.push(sequence);
                    }
                }
                use subxt::config::Header;
                let finalized_head_hash = rpc_client.rpc().finalized_head().await.unwrap();

                let block = rpc_client
                    .rpc()
                    .block(Some(finalized_head_hash))
                    .await
                    .unwrap();

                Ok((
                    result,
                    ICSHeight::new(0, u64::from(block.unwrap().block.header.number())).unwrap(),
                ))
            } else {
                let key_addr = relaychain_node::storage().ibc().acknowledgements_root();

                let mut iter = relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = relaychain_node::runtime_types::ibc::core::ics24_host::path::AcksPath::decode(&mut &*raw_key).unwrap();
                    let port_id = PortId::from(rets.port_id);
                    let channel_id = ChannelId::from(rets.channel_id);
                    let sequence = Sequence::from(rets.sequence);

                    if port_id == request.port_id && channel_id == request.channel_id {
                        result.push(sequence);
                    }
                }
                use subxt::config::Header;
                let finalized_head_hash = relay_rpc_client.rpc().finalized_head().await.unwrap();

                let block = relay_rpc_client
                    .rpc()
                    .block(Some(finalized_head_hash))
                    .await
                    .unwrap();

                // todo ics height need to set
                Ok((
                    result,
                    ICSHeight::new(0, u64::from(block.unwrap().block.header.number())).unwrap(),
                ))
            }
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_packet_acknowledgements(
                relay_rpc,
                Some(para_rpc),
                request,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_packet_acknowledgements(rpc, None, request))
            }
        }
    }

    /// Queries the unreceived acknowledgements sequences associated with a channel.
    fn query_unreceived_acknowledgements(
        &self,
        request: QueryUnreceivedAcksRequest,
    ) -> Result<Vec<Sequence>, Error> {
        crate::time!("query_unreceived_acknowledgements");
        crate::telemetry!(query, self.id(), "query_unreceived_acknowledgements");

        async fn query_unreceived_acknowledgements(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryUnreceivedAcksRequest,
        ) -> Result<Vec<Sequence>, Error> {
            if let Some(rpc_client) = para_rpc_client {
                let key_addr = parachain_node::storage().ibc().acknowledgements_root();

                let mut iter = rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = parachain_node::runtime_types::ibc::core::ics24_host::path::AcksPath::decode(&mut &*raw_key).unwrap();
                    let port_id = PortId::from(rets.port_id);
                    let channel_id = ChannelId::from(rets.channel_id);
                    let sequence = Sequence::from(rets.sequence);

                    if port_id == request.port_id && channel_id == request.channel_id {
                        result.push(sequence);
                    }
                }

                let mut ret = vec![];
                for seq in request.packet_ack_sequences {
                    for in_seq in result.iter() {
                        if seq != *in_seq {
                            ret.push(seq);
                        }
                    }
                }
                Ok(ret)
            } else {
                let key_addr = relaychain_node::storage().ibc().acknowledgements_root();

                let mut iter = relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .iter(key_addr, 10)
                    .await
                    .unwrap();

                let mut result = vec![];
                while let Some((key, value)) = iter.next().await.unwrap() {
                    let raw_key = key.0[48..].to_vec();
                    let rets = relaychain_node::runtime_types::ibc::core::ics24_host::path::AcksPath::decode(&mut &*raw_key).unwrap();
                    let port_id = PortId::from(rets.port_id);
                    let channel_id = ChannelId::from(rets.channel_id);
                    let sequence = Sequence::from(rets.sequence);

                    if port_id == request.port_id && channel_id == request.channel_id {
                        result.push(sequence);
                    }
                }

                let mut ret = vec![];
                for seq in request.packet_ack_sequences {
                    for in_seq in result.iter() {
                        if seq != *in_seq {
                            ret.push(seq);
                        }
                    }
                }
                Ok(ret)
            }
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_unreceived_acknowledgements(
                relay_rpc,
                Some(para_rpc),
                request,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_unreceived_acknowledgements(rpc, None, request))
            }
        }
    }

    /// This function queries transactions for events matching certain criteria.
    /// 1. Client Update request - returns a vector with at most one update client event
    /// 2. Transaction event request - returns all IBC events resulted from a Tx execution
    fn query_txs(&self, request: QueryTxRequest) -> Result<Vec<IbcEventWithHeight>, Error> {
        crate::time!("query_txs");
        crate::telemetry!(query, self.id(), "query_txs");
        async fn query_txs(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryTxRequest,
        ) -> Result<Vec<IbcEventWithHeight>, Error> {
            // todo
            todo!()
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_txs(relay_rpc, Some(para_rpc), request)),
            RpcClient::SubChainRpc { rpc } => self.block_on(query_txs(rpc, None, request)),
        }
    }

    /// This function queries transactions for packet events matching certain criteria.
    /// It returns at most one packet event for each sequence specified in the request.
    ///    Note - there is no way to format the packet query such that it asks for Tx-es with either
    ///    sequence (the query conditions can only be AND-ed).
    ///    There is a possibility to include "<=" and ">=" conditions but it doesn't work with
    ///    string attributes (sequence is emmitted as a string).
    ///    Therefore, for packets we perform one tx_search for each sequence.
    ///    Alternatively, a single query for all packets could be performed but it would return all
    ///    packets ever sent.
    fn query_packet_events(
        &self,
        mut request: QueryPacketEventDataRequest,
    ) -> Result<Vec<IbcEventWithHeight>, Error> {
        crate::time!("query_packet_events");
        crate::telemetry!(query, self.id(), "query_packet_events");

        async fn query_packet_events(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            mut request: QueryPacketEventDataRequest,
        ) -> Result<Vec<IbcEventWithHeight>, Error> {
            if let Some(rpc_client) = para_rpc_client {
                todo!()
            } else {
                todo!()
            }
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_packet_events(relay_rpc, Some(para_rpc), request)),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_packet_events(rpc, None, request))
            }
        }
    }

    fn query_host_consensus_state(
        &self,
        request: QueryHostConsensusStateRequest,
    ) -> Result<Self::ConsensusState, Error> {
        async fn query_host_consensus_state(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryHostConsensusStateRequest,
        ) -> Result<GpConsensusState, Error> {
            use core::time::Duration;
            use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentRoot;
            use tendermint::time::Time;
            if let Some(rpc_client) = para_rpc_client {
                let storage = parachain_node::storage().timestamp().now();

                let query_hash = match request.height {
                    QueryHeight::Latest => rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };
                let result = rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();
                // As the `u64` representation can only represent times up to
                // about year 2554, there is no risk of overflowing `Time`
                // or `OffsetDateTime`.
                let timestamp = time::OffsetDateTime::from_unix_timestamp_nanos(result as i128)
                    .unwrap()
                    .try_into()
                    .unwrap();

                let last_finalized_head_hash = rpc_client.rpc().finalized_head().await.unwrap();
                let finalized_head = rpc_client
                    .rpc()
                    .header(Some(last_finalized_head_hash))
                    .await
                    .unwrap()
                    .unwrap();
                let root = CommitmentRoot::from(finalized_head.state_root.as_bytes().to_vec());
                let consensus_state = GpConsensusState::new(root, timestamp);
                Ok(consensus_state)
            } else {
                let storage = relaychain_node::storage().timestamp().now();

                let query_hash = match request.height {
                    QueryHeight::Latest => relay_rpc_client.rpc().block_hash(None).await.unwrap(),
                    QueryHeight::Specific(v) => relay_rpc_client
                        .rpc()
                        .block_hash(Some(subxt::rpc::types::BlockNumber::from(
                            v.revision_height(),
                        )))
                        .await
                        .unwrap(),
                };
                let result = relay_rpc_client
                    .storage()
                    .at(query_hash)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();
                // As the `u64` representation can only represent times up to
                // about year 2554, there is no risk of overflowing `Time`
                // or `OffsetDateTime`.
                let timestamp = time::OffsetDateTime::from_unix_timestamp_nanos(result as i128)
                    .unwrap()
                    .try_into()
                    .unwrap();
                let last_finalized_head_hash =
                    relay_rpc_client.rpc().finalized_head().await.unwrap();

                let finalized_head = relay_rpc_client
                    .rpc()
                    .header(Some(last_finalized_head_hash))
                    .await
                    .unwrap()
                    .unwrap();
                let root = CommitmentRoot::from(finalized_head.state_root.as_bytes().to_vec());
                let consensus_state = GpConsensusState::new(root, timestamp);
                Ok(consensus_state)
            }
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(query_host_consensus_state(
                relay_rpc,
                Some(para_rpc),
                request,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(query_host_consensus_state(rpc, None, request))
            }
        }
    }

    // need to do
    fn build_client_state(
        &self,
        height: ICSHeight,
        settings: ClientSettings,
    ) -> Result<Self::ClientState, Error> {
        crate::time!("build_client_state");

        fn build_client_state(
            config: ChainConfig,
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            height: ICSHeight,
            settings: ClientSettings,
        ) -> Result<GpClientState, Error> {
            if let Some(rpc_client) = para_rpc_client {
                use codec::Decode;
                use ibc_relayer_types::clients::ics10_grandpa::beefy_authority_set::BeefyAuthoritySet;
                use ibc_relayer_types::clients::ics10_grandpa::client_state::ChaninType;
                use ibc_relayer_types::Height;

                let chain_id = config.id.clone();

                let result = async {
                    let mut sub = subscribe_beefy_justifications(&*relay_rpc_client.rpc())
                        .await
                        .unwrap();

                    sub.next().await.unwrap().unwrap().0
                };
                let raw_signed_commitment = rt.block_on(result);

                // decode signed commitment

                let beefy_light_client::commitment::VersionedFinalityProof::V1(signed_commitment) =
                    beefy_light_client::commitment::VersionedFinalityProof::decode(
                        &mut &raw_signed_commitment[..],
                    )
                    .unwrap();

                // get commitment
                let beefy_light_client::commitment::Commitment {
                    payload,
                    block_number,
                    validator_set_id,
                } = signed_commitment.commitment;

                let mmr_root_hash = payload
                    .get_raw(&beefy_light_client::commitment::known_payload_ids::MMR_ROOT_ID)
                    .unwrap();
                let reversion_number = config.id.version();
                let latest_beefy_height =
                    Height::new(reversion_number, block_number as u64).unwrap();
                let latest_mmr_root = mmr_root_hash.clone();
                // get authorith set
                let storage = relaychain_node::storage().mmr_leaf().beefy_authorities();

                let closure = async {
                    relay_rpc_client
                        .storage()
                        .at(None)
                        .await
                        .unwrap()
                        .fetch(&storage)
                        .await
                };
                let latest_authority_set =
                    rt.block_on(closure).unwrap().map(|v| BeefyAuthoritySet {
                        id: v.id,
                        len: v.len,
                        root: v.root.as_bytes().to_vec(),
                    });

                let (chain_type, parachain_id, latest_chain_height) = (
                    ChaninType::Parachian,
                    2000,
                    // todo(davirian)need query parachain height, maybey revision number neet to change
                    Height::new(reversion_number, block_number as u64).unwrap(),
                );

                let client_state = GpClientState {
                    chain_type,
                    chain_id,
                    parachain_id,
                    latest_beefy_height,
                    latest_mmr_root,
                    latest_chain_height,
                    frozen_height: None,
                    latest_authority_set,
                };
                Ok(client_state)
            } else {
                use codec::Decode;
                use ibc_relayer_types::clients::ics10_grandpa::beefy_authority_set::BeefyAuthoritySet;
                use ibc_relayer_types::clients::ics10_grandpa::client_state::ChaninType;
                use ibc_relayer_types::Height;

                let chain_id = config.id.clone();

                let result = async {
                    let mut sub = subscribe_beefy_justifications(&*relay_rpc_client.rpc())
                        .await
                        .unwrap();

                    sub.next().await.unwrap().unwrap().0
                };
                let raw_signed_commitment = rt.block_on(result);

                // decode signed commitment

                let beefy_light_client::commitment::VersionedFinalityProof::V1(signed_commitment) =
                    beefy_light_client::commitment::VersionedFinalityProof::decode(
                        &mut &raw_signed_commitment[..],
                    )
                    .unwrap();

                // get commitment
                let beefy_light_client::commitment::Commitment {
                    payload,
                    block_number,
                    validator_set_id,
                } = signed_commitment.commitment;

                let mmr_root_hash = payload
                    .get_raw(&beefy_light_client::commitment::known_payload_ids::MMR_ROOT_ID)
                    .unwrap();
                let reversion_number = config.id.version();
                let latest_beefy_height =
                    Height::new(reversion_number, block_number as u64).unwrap();
                let latest_mmr_root = mmr_root_hash.clone();
                // get authorith set
                let storage = relaychain_node::storage().mmr_leaf().beefy_authorities();

                let closure = async {
                    relay_rpc_client
                        .storage()
                        .at(None)
                        .await
                        .unwrap()
                        .fetch(&storage)
                        .await
                };
                let latest_authority_set =
                    rt.block_on(closure).unwrap().map(|v| BeefyAuthoritySet {
                        id: v.id,
                        len: v.len,
                        root: v.root.as_bytes().to_vec(),
                    });

                let (chain_type, parachain_id, latest_chain_height) = (
                    ChaninType::Subchain,
                    0,
                    Height::new(reversion_number, block_number as u64).unwrap(),
                );

                let client_state = GpClientState {
                    chain_type,
                    chain_id,
                    parachain_id,
                    latest_beefy_height,
                    latest_mmr_root,
                    latest_chain_height,
                    frozen_height: None,
                    latest_authority_set,
                };
                Ok(client_state)
            }
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => build_client_state(
                self.config.clone(),
                self.rt.clone(),
                relay_rpc,
                Some(para_rpc),
                height,
                settings,
            ),
            RpcClient::SubChainRpc { rpc } => build_client_state(
                self.config.clone(),
                self.rt.clone(),
                rpc,
                None,
                height,
                settings,
            ),
        }
    }

    // need to do
    fn build_consensus_state(
        &self,
        light_block: Self::LightBlock,
    ) -> Result<Self::ConsensusState, Error> {
        crate::time!("build_consensus_state");
        async fn build_consensus_state(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            light_block: SubLightBlock,
        ) -> Result<GpConsensusState, Error> {
            use core::time::Duration;
            use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentRoot;
            use tendermint::time::Time;
            if let Some(rpc_client) = para_rpc_client {
                let last_finalized_head_hash = rpc_client.rpc().finalized_head().await.unwrap();
                let finalized_head = rpc_client
                    .rpc()
                    .header(Some(last_finalized_head_hash))
                    .await
                    .unwrap()
                    .unwrap();
                let storage = parachain_node::storage().timestamp().now();

                let result = rpc_client
                    .storage()
                    .at(Some(last_finalized_head_hash))
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();
                // As the `u64` representation can only represent times up to
                // about year 2554, there is no risk of overflowing `Time`
                // or `OffsetDateTime`.
                let timestamp = time::OffsetDateTime::from_unix_timestamp_nanos(result as i128)
                    .unwrap()
                    .try_into()
                    .unwrap();
                let root = CommitmentRoot::from(finalized_head.state_root.as_bytes().to_vec());
                let consensus_state = GpConsensusState::new(root, timestamp);
                Ok(consensus_state)
            } else {
                let last_finalized_head_hash =
                    relay_rpc_client.rpc().finalized_head().await.unwrap();

                let finalized_head = relay_rpc_client
                    .rpc()
                    .header(Some(last_finalized_head_hash))
                    .await
                    .unwrap()
                    .unwrap();
                let storage = relaychain_node::storage().timestamp().now();

                let result = relay_rpc_client
                    .storage()
                    .at(Some(last_finalized_head_hash))
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
                    .unwrap()
                    .unwrap();
                // As the `u64` representation can only represent times up to
                // about year 2554, there is no risk of overflowing `Time`
                // or `OffsetDateTime`.
                let timestamp = time::OffsetDateTime::from_unix_timestamp_nanos(result as i128)
                    .unwrap()
                    .try_into()
                    .unwrap();
                let root = CommitmentRoot::from(finalized_head.state_root.as_bytes().to_vec());
                let consensus_state = GpConsensusState::new(root, timestamp);
                Ok(consensus_state)
            }
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(build_consensus_state(
                relay_rpc,
                Some(para_rpc),
                light_block,
            )),
            RpcClient::SubChainRpc { rpc } => {
                self.block_on(build_consensus_state(rpc, None, light_block))
            }
        }
    }

    fn build_header(
        &mut self,
        trusted_height: ICSHeight,
        target_height: ICSHeight,
        client_state: &AnyClientState,
    ) -> Result<(Self::Header, Vec<Self::Header>), Error> {
        crate::time!("build_header");

        async fn build_header(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            trusted_height: ICSHeight,
            target_height: ICSHeight,
            client_state: &AnyClientState,
        ) -> Result<(GpHeader, Vec<GpHeader>), Error> {
            use codec::Decode;
            use ibc_relayer_types::clients::ics10_grandpa::beefy_authority_set::BeefyAuthoritySet;
            use ibc_relayer_types::clients::ics10_grandpa::client_state::ChaninType;
            use ibc_relayer_types::Height;
            if let Some(para_rpc_client) = para_rpc_client {
                todo!()
            } else {
                assert!(trusted_height.revision_height() < target_height.revision_height());

                let grandpa_client_state = match client_state {
                    AnyClientState::Grandpa(state) => state,
                    _ => unimplemented!(),
                };

                assert!(
                    target_height.revision_height()
                        < grandpa_client_state.latest_beefy_height.revision_height()
                );
                // assert trust_height <= grandpa_client_state height
                // if trusted_height > grandpa_client_state.latest_chain_height {
                //     panic!("trust height miss match client state height");
                // }

                // build mmr proof for target height
                let target_heights = vec![(target_height.revision_height() + 1) as u32];
                let mmr_batch_proof = utils::build_mmr_proofs(
                    relay_rpc_client,
                    target_heights,
                    Some(grandpa_client_state.latest_beefy_height.revision_height() as u32),
                    None,
                )
                .await
                .unwrap();

                let mmr_leaves_proof = beefy_light_client::mmr::MmrLeavesProof::try_from(
                    mmr_batch_proof.clone().proof.0,
                )
                .unwrap();

                // build target height header
                let headers = utils::build_subchain_headers(
                    relay_rpc_client,
                    mmr_leaves_proof.leaf_indices,
                    grandpa_client_state.chain_id.to_string(),
                )
                .await
                .unwrap();

                let proof = Some(utils::convert_mmrproof(mmr_batch_proof).unwrap());
                let subchain_headers =
                    ibc_relayer_types::clients::ics10_grandpa::header::message::SubchainHeaders {
                        subchain_headers: headers,
                        mmr_leaves_and_batch_proof: proof,
                    };
                let result = GpHeader {
                    // leave beefy mmr None
                    beefy_mmr:None,
                    message: ibc_relayer_types::clients::ics10_grandpa::header::message::Message::SubchainHeaders(subchain_headers),
                };

                Ok((result, vec![]))
            }
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.rt.block_on(build_header(
                relay_rpc,
                Some(para_rpc),
                trusted_height,
                target_height,
                client_state,
            )),
            RpcClient::SubChainRpc { rpc } => self.rt.block_on(build_header(
                rpc,
                None,
                trusted_height,
                target_height,
                client_state,
            )),
        }
    }

    fn maybe_register_counterparty_payee(
        &mut self,
        channel_id: &ChannelId,
        port_id: &PortId,
        counterparty_payee: &Signer,
    ) -> Result<(), Error> {
        async fn maybe_register_counterparty_payee(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            channel_id: &ChannelId,
            port_id: &PortId,
            counterparty_payee: &Signer,
        ) -> Result<(), Error> {
            Ok(())
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(maybe_register_counterparty_payee(
                relay_rpc,
                Some(para_rpc),
                channel_id,
                port_id,
                counterparty_payee,
            )),
            RpcClient::SubChainRpc { rpc } => self.block_on(maybe_register_counterparty_payee(
                rpc,
                None,
                channel_id,
                port_id,
                counterparty_payee,
            )),
        }
    }

    fn cross_chain_query(
        &self,
        requests: Vec<CrossChainQueryRequest>,
    ) -> Result<Vec<CrossChainQueryResponse>, Error> {
        async fn cross_chain_query(
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            requests: Vec<CrossChainQueryRequest>,
        ) -> Result<Vec<CrossChainQueryResponse>, Error> {
            Ok(vec![])
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => self.block_on(cross_chain_query(relay_rpc, Some(para_rpc), requests)),
            RpcClient::SubChainRpc { rpc } => self.block_on(cross_chain_query(rpc, None, requests)),
        }
    }
}