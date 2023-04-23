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
use tracing::{error, instrument, trace, warn};

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
use crate::event::monitor::{EventMonitor, TxMonitorCmd};
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
use subxt::{tx::PairSigner, OnlineClient, PolkadotConfig, SubstrateConfig};

pub mod parachain;
pub mod relaychain;
pub mod utils;

use parachain::parachain_node;
use relaychain::relaychain_node;

/// An encoded signed commitment proving that the given header has been finalized.
/// The given bytes should be the SCALE-encoded representation of a
/// `beefy_primitives::SignedCommitment`.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignedCommitment(pub sp_core::Bytes);

/// Subscribe to beefy justifications.
pub async fn subscribe_beefy_justifications(
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

        todo!()
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
        name = "send_messages_and_wait_commit",
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
        crate::time!("send_messages_and_wait_commit");
        use ibc_relayer_types::events::IbcEvent;
        use sp_keyring::AccountKeyring;

        let proto_msgs = tracked_msgs.msgs;

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => todo!(),
            RpcClient::SubChainRpc { rpc } => {
                let msg: Vec<relaychain_node::runtime_types::ibc_proto::google::protobuf::Any> =
                    proto_msgs
                        .iter()
                        .map(
                            |m| relaychain_node::runtime_types::ibc_proto::google::protobuf::Any {
                                type_url: m.type_url.clone(),
                                value: m.value.clone(),
                            },
                        )
                        .collect();

                // use default signer
                let signer = PairSigner::new(AccountKeyring::Alice.pair());

                let binding = rpc.tx();
                let tx = relaychain_node::tx().ibc().deliver(msg);

                let runtime = self.rt.clone();
                let deliver = binding.sign_and_submit_then_watch_default(&tx, &signer);
                let result = runtime.block_on(deliver);

                let events = runtime.block_on(result.unwrap().wait_for_finalized_success());

                let ibc_events = events
                    .unwrap()
                    .find_first::<relaychain_node::ibc::events::IbcEvents>()
                    .unwrap()
                    .unwrap();
                let es: Vec<IbcEventWithHeight> = ibc_events
                    .events
                    .into_iter()
                    .map(|e| IbcEventWithHeight {
                        event: IbcEvent::from(e),
                        height: ICSHeight::new(0, 10).unwrap(),
                    })
                    .collect();

                Ok(es)
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
        })
    }

    fn shutdown(self) -> Result<(), Error> {
        Ok(())
    }

    fn keybase(&self) -> &KeyRing<Self::SigningKeyPair> {
        &self.keybase
    }

    fn keybase_mut(&mut self) -> &mut KeyRing<Self::SigningKeyPair> {
        &mut self.keybase
    }

    fn subscribe(&mut self) -> Result<Subscription, Error> {
        fn subscribe(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
        ) -> Result<Subscription, Error> {
            todo!()
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => subscribe(self.rt.clone(), relay_rpc, Some(para_rpc)),
            RpcClient::SubChainRpc { rpc } => subscribe(self.rt.clone(), rpc, None),
        }
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
        fn verify_header(
            rt: Arc<TokioRuntime>,
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
            } => verify_header(
                self.rt.clone(),
                relay_rpc,
                Some(para_rpc),
                trusted,
                target,
                client_state,
            ),
            RpcClient::SubChainRpc { rpc } => {
                verify_header(self.rt.clone(), rpc, None, trusted, target, client_state)
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
        fn check_misbehaviour(
            rt: Arc<TokioRuntime>,
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
            } => check_misbehaviour(
                self.rt.clone(),
                relay_rpc,
                Some(para_rpc),
                update,
                client_state,
            ),
            RpcClient::SubChainRpc { rpc } => {
                check_misbehaviour(self.rt.clone(), rpc, None, update, client_state)
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
        fn ibc_version(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
        ) -> Result<Option<semver::Version>, Error> {
            Ok(None)
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => ibc_version(self.rt.clone(), relay_rpc, Some(para_rpc)),
            RpcClient::SubChainRpc { rpc } => ibc_version(self.rt.clone(), rpc, None),
        }
    }

    fn query_balance(&self, key_name: Option<&str>, denom: Option<&str>) -> Result<Balance, Error> {
        fn query_balance(
            rt: Arc<TokioRuntime>,
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
            } => query_balance(self.rt.clone(), relay_rpc, Some(para_rpc), key_name, denom),
            RpcClient::SubChainRpc { rpc } => {
                query_balance(self.rt.clone(), rpc, None, key_name, denom)
            }
        }
    }

    fn query_all_balances(&self, key_name: Option<&str>) -> Result<Vec<Balance>, Error> {
        fn query_all_balances(
            rt: Arc<TokioRuntime>,
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
            } => query_all_balances(self.rt.clone(), relay_rpc, Some(para_rpc), key_name),
            RpcClient::SubChainRpc { rpc } => {
                query_all_balances(self.rt.clone(), rpc, None, key_name)
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

        fn query_application_state(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
        ) -> Result<ChainStatus, Error> {
            use subxt::config::Header;
            let finalized_head = rt
                .block_on(relay_rpc_client.rpc().finalized_head())
                .unwrap();

            let block = rt
                .block_on(relay_rpc_client.rpc().block(Some(finalized_head)))
                .unwrap();

            Ok(ChainStatus {
                height: ICSHeight::new(0, u64::from(block.unwrap().block.header.number())).unwrap(),
                timestamp: Timestamp::default(),
            })
        }

        match &self.rpc_client {
            RpcClient::SubChainRpc { rpc } => query_application_state(self.rt.clone(), rpc, None),
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_application_state(self.rt.clone(), relay_rpc, Some(para_rpc)),
        }
    }

    fn query_clients(
        &self,
        request: QueryClientStatesRequest,
    ) -> Result<Vec<IdentifiedAnyClientState>, Error> {
        crate::time!("query_clients");
        crate::telemetry!(query, self.id(), "query_clients");

        fn query_clients(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryClientStatesRequest,
        ) -> Result<Vec<IdentifiedAnyClientState>, Error> {
            todo!()
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_clients(self.rt.clone(), relay_rpc, Some(para_rpc), request),
            RpcClient::SubChainRpc { rpc } => query_clients(self.rt.clone(), rpc, None, request),
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

        fn query_client_state(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryClientStateRequest,
            include_proof: IncludeProof,
        ) -> Result<(AnyClientState, Option<MerkleProof>), Error> {
            let client_id =
                relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ClientId(
                    request.client_id.to_string(),
                );
            let storage = relaychain_node::storage().ibc().client_states(client_id);

            let closure = async {
                relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
            };
            let states = rt.block_on(closure).unwrap();

            let client_state =
                AnyClientState::decode_vec(&states.unwrap()).map_err(Error::decode)?;

            println!("states: {:?}", client_state);
            Ok((client_state, None))
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_client_state(
                self.rt.clone(),
                relay_rpc,
                Some(para_rpc),
                request,
                include_proof,
            ),
            RpcClient::SubChainRpc { rpc } => {
                query_client_state(self.rt.clone(), rpc, None, request, include_proof)
            }
        }
    }

    fn query_upgraded_client_state(
        &self,
        request: QueryUpgradedClientStateRequest,
    ) -> Result<(AnyClientState, MerkleProof), Error> {
        crate::time!("query_upgraded_client_state");
        crate::telemetry!(query, self.id(), "query_upgraded_client_state");
        fn query_upgraded_client_state(
            rt: Arc<TokioRuntime>,
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
            } => query_upgraded_client_state(self.rt.clone(), relay_rpc, Some(para_rpc), request),
            RpcClient::SubChainRpc { rpc } => {
                query_upgraded_client_state(self.rt.clone(), rpc, None, request)
            }
        }
    }

    fn query_upgraded_consensus_state(
        &self,
        request: QueryUpgradedConsensusStateRequest,
    ) -> Result<(AnyConsensusState, MerkleProof), Error> {
        crate::time!("query_upgraded_consensus_state");
        crate::telemetry!(query, self.id(), "query_upgraded_consensus_state");
        fn query_upgraded_consensus_state(
            rt: Arc<TokioRuntime>,
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
            } => {
                query_upgraded_consensus_state(self.rt.clone(), relay_rpc, Some(para_rpc), request)
            }
            RpcClient::SubChainRpc { rpc } => {
                query_upgraded_consensus_state(self.rt.clone(), rpc, None, request)
            }
        }
    }

    fn query_consensus_state_heights(
        &self,
        request: QueryConsensusStateHeightsRequest,
    ) -> Result<Vec<ICSHeight>, Error> {
        fn query_consensus_state_heights(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryConsensusStateHeightsRequest,
        ) -> Result<Vec<ICSHeight>, Error> {
            todo!()
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_consensus_state_heights(self.rt.clone(), relay_rpc, Some(para_rpc), request),
            RpcClient::SubChainRpc { rpc } => {
                query_consensus_state_heights(self.rt.clone(), rpc, None, request)
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

        fn query_consensus_state(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryConsensusStateRequest,
            include_proof: IncludeProof,
        ) -> Result<(AnyConsensusState, Option<MerkleProof>), Error> {
            let client_id =
                relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ClientId(
                    request.client_id.to_string(),
                );

            let height = relaychain_node::runtime_types::ibc::core::ics02_client::height::Height {
                revision_number: request.consensus_height.revision_number(),
                revision_height: request.consensus_height.revision_height(),
            };
            let storage = relaychain_node::storage()
                .ibc()
                .consensus_states(client_id, height);

            let closure = async {
                relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
            };
            let consensus_states = rt.block_on(closure).unwrap();

            let consensus_state =
                AnyConsensusState::decode_vec(&consensus_states.unwrap()).map_err(Error::decode)?;

            println!("consensus_state: {:?}", consensus_state);
            Ok((consensus_state, None))
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_consensus_state(
                self.rt.clone(),
                relay_rpc,
                Some(para_rpc),
                request,
                include_proof,
            ),
            RpcClient::SubChainRpc { rpc } => {
                query_consensus_state(self.rt.clone(), rpc, None, request, include_proof)
            }
        }
    }

    fn query_client_connections(
        &self,
        request: QueryClientConnectionsRequest,
    ) -> Result<Vec<ConnectionId>, Error> {
        crate::time!("query_client_connections");
        crate::telemetry!(query, self.id(), "query_client_connections");

        fn query_client_connections(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryClientConnectionsRequest,
        ) -> Result<Vec<ConnectionId>, Error> {
            todo!()
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_client_connections(self.rt.clone(), relay_rpc, Some(para_rpc), request),
            RpcClient::SubChainRpc { rpc } => {
                query_client_connections(self.rt.clone(), rpc, None, request)
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
                    let connection_id = ConnectionId::from(parachain_node::runtime_types::ibc::core::ics24_host::identifier::ConnectionId::decode(&mut &*raw_key).unwrap());
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
                    let connection_id = ConnectionId::from(relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ConnectionId::decode(&mut &*raw_key).unwrap());
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

        fn query_connection(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryConnectionRequest,
            include_proof: IncludeProof,
        ) -> Result<(ConnectionEnd, Option<MerkleProof>), Error> {
            let connection_id =
                relaychain_node::runtime_types::ibc::core::ics24_host::identifier::ConnectionId(
                    request.connection_id.to_string(),
                );
            let storage = relaychain_node::storage().ibc().connections(connection_id);

            let closure = async {
                relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
            };
            let connection = rt.block_on(closure).unwrap();

            let conn = connection.unwrap();

            println!("connection: {:?}", conn); // update ConnectionsPath key
            Ok((conn.into(), None))
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_connection(
                self.rt.clone(),
                relay_rpc,
                Some(para_rpc),
                request,
                include_proof,
            ),
            RpcClient::SubChainRpc { rpc } => {
                query_connection(self.rt.clone(), rpc, None, request, include_proof)
            }
        }
    }

    fn query_connection_channels(
        &self,
        request: QueryConnectionChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        crate::time!("query_connection_channels");
        crate::telemetry!(query, self.id(), "query_connection_channels");

        fn query_connection_channels(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryConnectionChannelsRequest,
        ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
            todo!()
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_connection_channels(self.rt.clone(), relay_rpc, Some(para_rpc), request),
            RpcClient::SubChainRpc { rpc } => {
                query_connection_channels(self.rt.clone(), rpc, None, request)
            }
        }
    }

    fn query_channels(
        &self,
        request: QueryChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        crate::time!("query_channels");
        crate::telemetry!(query, self.id(), "query_channels");

        fn query_channels(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryChannelsRequest,
        ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
            todo!()
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_channels(self.rt.clone(), relay_rpc, Some(para_rpc), request),
            RpcClient::SubChainRpc { rpc } => query_channels(self.rt.clone(), rpc, None, request),
        }
    }

    fn query_channel(
        &self,
        request: QueryChannelRequest,
        include_proof: IncludeProof,
    ) -> Result<(ChannelEnd, Option<MerkleProof>), Error> {
        crate::time!("query_channel");
        crate::telemetry!(query, self.id(), "query_channel");

        fn query_channel(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryChannelRequest,
            include_proof: IncludeProof,
        ) -> Result<(ChannelEnd, Option<MerkleProof>), Error> {
            todo!()
        }
        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_channel(
                self.rt.clone(),
                relay_rpc,
                Some(para_rpc),
                request,
                include_proof,
            ),
            RpcClient::SubChainRpc { rpc } => {
                query_channel(self.rt.clone(), rpc, None, request, include_proof)
            }
        }
    }

    fn query_channel_client_state(
        &self,
        request: QueryChannelClientStateRequest,
    ) -> Result<Option<IdentifiedAnyClientState>, Error> {
        crate::time!("query_channel_client_state");
        crate::telemetry!(query, self.id(), "query_channel_client_state");

        fn query_channel_client_state(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryChannelClientStateRequest,
        ) -> Result<Option<IdentifiedAnyClientState>, Error> {
            todo!()
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_channel_client_state(self.rt.clone(), relay_rpc, Some(para_rpc), request),
            RpcClient::SubChainRpc { rpc } => {
                query_channel_client_state(self.rt.clone(), rpc, None, request)
            }
        }
    }

    fn query_packet_commitment(
        &self,
        request: QueryPacketCommitmentRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        fn query_packet_commitment(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryPacketCommitmentRequest,
            include_proof: IncludeProof,
        ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
            todo!()
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_packet_commitment(
                self.rt.clone(),
                relay_rpc,
                Some(para_rpc),
                request,
                include_proof,
            ),
            RpcClient::SubChainRpc { rpc } => {
                query_packet_commitment(self.rt.clone(), rpc, None, request, include_proof)
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

        fn query_packet_commitments(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryPacketCommitmentsRequest,
        ) -> Result<(Vec<Sequence>, ICSHeight), Error> {
            todo!()
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_packet_commitments(self.rt.clone(), relay_rpc, Some(para_rpc), request),
            RpcClient::SubChainRpc { rpc } => {
                query_packet_commitments(self.rt.clone(), rpc, None, request)
            }
        }
    }

    fn query_packet_receipt(
        &self,
        request: QueryPacketReceiptRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        fn query_packet_receipt(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryPacketReceiptRequest,
            include_proof: IncludeProof,
        ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
            todo!()
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_packet_receipt(
                self.rt.clone(),
                relay_rpc,
                Some(para_rpc),
                request,
                include_proof,
            ),
            RpcClient::SubChainRpc { rpc } => {
                query_packet_receipt(self.rt.clone(), rpc, None, request, include_proof)
            }
        }
    }

    /// Queries the unreceived packet sequences associated with a channel.
    fn query_unreceived_packets(
        &self,
        request: QueryUnreceivedPacketsRequest,
    ) -> Result<Vec<Sequence>, Error> {
        crate::time!("query_unreceived_packets");
        crate::telemetry!(query, self.id(), "query_unreceived_packets");

        fn query_packet_receipt(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryUnreceivedPacketsRequest,
        ) -> Result<Vec<Sequence>, Error> {
            todo!()
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_packet_receipt(self.rt.clone(), relay_rpc, Some(para_rpc), request),
            RpcClient::SubChainRpc { rpc } => {
                query_packet_receipt(self.rt.clone(), rpc, None, request)
            }
        }
    }

    fn query_next_sequence_receive(
        &self,
        request: QueryNextSequenceReceiveRequest,
        include_proof: IncludeProof,
    ) -> Result<(Sequence, Option<MerkleProof>), Error> {
        fn query_next_sequence_receive(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryNextSequenceReceiveRequest,
            include_proof: IncludeProof,
        ) -> Result<(Sequence, Option<MerkleProof>), Error> {
            todo!()
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_next_sequence_receive(
                self.rt.clone(),
                relay_rpc,
                Some(para_rpc),
                request,
                include_proof,
            ),
            RpcClient::SubChainRpc { rpc } => {
                query_next_sequence_receive(self.rt.clone(), rpc, None, request, include_proof)
            }
        }
    }

    fn query_packet_acknowledgement(
        &self,
        request: QueryPacketAcknowledgementRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        fn query_packet_acknowledgement(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryPacketAcknowledgementRequest,
            include_proof: IncludeProof,
        ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
            todo!()
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_packet_acknowledgement(
                self.rt.clone(),
                relay_rpc,
                Some(para_rpc),
                request,
                include_proof,
            ),
            RpcClient::SubChainRpc { rpc } => {
                query_packet_acknowledgement(self.rt.clone(), rpc, None, request, include_proof)
            }
        }
    }

    /// Queries the packet acknowledgment hashes associated with a channel.
    fn query_packet_acknowledgements(
        &self,
        request: QueryPacketAcknowledgementsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), Error> {
        crate::time!("query_packet_acknowledgements");
        crate::telemetry!(query, self.id(), "query_packet_acknowledgements");

        fn query_packet_acknowledgements(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryPacketAcknowledgementsRequest,
        ) -> Result<(Vec<Sequence>, ICSHeight), Error> {
            todo!()
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_packet_acknowledgements(self.rt.clone(), relay_rpc, Some(para_rpc), request),
            RpcClient::SubChainRpc { rpc } => {
                query_packet_acknowledgements(self.rt.clone(), rpc, None, request)
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

        fn query_unreceived_acknowledgements(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryUnreceivedAcksRequest,
        ) -> Result<Vec<Sequence>, Error> {
            todo!()
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_unreceived_acknowledgements(
                self.rt.clone(),
                relay_rpc,
                Some(para_rpc),
                request,
            ),
            RpcClient::SubChainRpc { rpc } => {
                query_unreceived_acknowledgements(self.rt.clone(), rpc, None, request)
            }
        }
    }

    /// This function queries transactions for events matching certain criteria.
    /// 1. Client Update request - returns a vector with at most one update client event
    /// 2. Transaction event request - returns all IBC events resulted from a Tx execution
    fn query_txs(&self, request: QueryTxRequest) -> Result<Vec<IbcEventWithHeight>, Error> {
        crate::time!("query_txs");
        crate::telemetry!(query, self.id(), "query_txs");
        fn query_txs(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryTxRequest,
        ) -> Result<Vec<IbcEventWithHeight>, Error> {
            todo!()
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_txs(self.rt.clone(), relay_rpc, Some(para_rpc), request),
            RpcClient::SubChainRpc { rpc } => query_txs(self.rt.clone(), rpc, None, request),
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

        fn query_packet_events(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            mut request: QueryPacketEventDataRequest,
        ) -> Result<Vec<IbcEventWithHeight>, Error> {
            todo!()
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_packet_events(self.rt.clone(), relay_rpc, Some(para_rpc), request),
            RpcClient::SubChainRpc { rpc } => {
                query_packet_events(self.rt.clone(), rpc, None, request)
            }
        }
    }

    fn query_host_consensus_state(
        &self,
        request: QueryHostConsensusStateRequest,
    ) -> Result<Self::ConsensusState, Error> {
        fn query_host_consensus_state(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            request: QueryHostConsensusStateRequest,
        ) -> Result<GpConsensusState, Error> {
            todo!()
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => query_host_consensus_state(self.rt.clone(), relay_rpc, Some(para_rpc), request),
            RpcClient::SubChainRpc { rpc } => {
                query_host_consensus_state(self.rt.clone(), rpc, None, request)
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
            use codec::Decode;
            use ibc_relayer_types::clients::ics10_grandpa::beefy_authority_set::BeefyAuthoritySet;
            use ibc_relayer_types::clients::ics10_grandpa::client_state::ChaninType;
            use ibc_relayer_types::Height;

            let chain_id = config.id.clone();
            let beefy_activation_height = 9999; // todo(davirian) need use correct beefy activation height

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
            let latest_beefy_height = Height::new(reversion_number, block_number as u64).unwrap();
            let mmr_root_hash = mmr_root_hash.clone();
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
            let authority_set = rt.block_on(closure).unwrap().map(|v| BeefyAuthoritySet {
                id: v.id,
                len: v.len,
                root: v.root.as_bytes().to_vec(),
            });

            // get next authorith set
            let storage = relaychain_node::storage()
                .mmr_leaf()
                .beefy_next_authorities();

            let closure = async {
                relay_rpc_client
                    .storage()
                    .at(None)
                    .await
                    .unwrap()
                    .fetch(&storage)
                    .await
            };
            let next_authority_set = rt.block_on(closure).unwrap().map(|v| BeefyAuthoritySet {
                id: v.id,
                len: v.len,
                root: v.root.as_bytes().to_vec(),
            });

            let (chain_type, parachain_id, latest_chain_height) = if para_rpc_client.is_none() {
                (
                    ChaninType::Subchain,
                    0,
                    Height::new(reversion_number, block_number as u64).unwrap(),
                )
            } else {
                (
                    ChaninType::Parachian,
                    2000,
                    // todo(davirian)need query parachain height, maybey revision number neet to change
                    Height::new(reversion_number, block_number as u64).unwrap(),
                )
            };
            let client_state = GpClientState {
                chain_type,
                chain_id,
                parachain_id,
                beefy_activation_height,
                latest_beefy_height,
                mmr_root_hash,
                latest_chain_height,
                frozen_height: None,
                authority_set,
                next_authority_set,
            };
            Ok(client_state)
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
        fn build_consensus_state(
            rt: Arc<TokioRuntime>,
            relay_rpc_client: &OnlineClient<PolkadotConfig>,
            para_rpc_client: Option<&OnlineClient<SubstrateConfig>>,
            light_block: SubLightBlock,
        ) -> Result<GpConsensusState, Error> {
            use core::time::Duration;
            use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentRoot;
            use tendermint::time::Time;

            let timestamp = Time::now(); // todo(davirian) need to use correct time stamp
            let root = if let Some(rpc_client) = para_rpc_client {
                let last_finalized_head_hash =
                    rt.block_on(rpc_client.rpc().finalized_head()).unwrap();
                let finalized_head = rt
                    .block_on(rpc_client.rpc().header(Some(last_finalized_head_hash)))
                    .unwrap()
                    .unwrap();
                CommitmentRoot::from(finalized_head.state_root.as_bytes().to_vec())
            } else {
                let last_finalized_head_hash = rt
                    .block_on(relay_rpc_client.rpc().finalized_head())
                    .unwrap();
                let finalized_head = rt
                    .block_on(
                        relay_rpc_client
                            .rpc()
                            .header(Some(last_finalized_head_hash)),
                    )
                    .unwrap()
                    .unwrap();
                CommitmentRoot::from(finalized_head.state_root.as_bytes().to_vec())
            };
            let consensus_state = GpConsensusState::new(root, timestamp);
            Ok(consensus_state)
        }

        match &self.rpc_client {
            RpcClient::ParachainRpc {
                relay_rpc,
                para_rpc,
            } => build_consensus_state(self.rt.clone(), relay_rpc, Some(para_rpc), light_block),
            RpcClient::SubChainRpc { rpc } => {
                build_consensus_state(self.rt.clone(), rpc, None, light_block)
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

                // assert trust_height <= grandpa_client_state height
                if trusted_height > grandpa_client_state.latest_chain_height {
                    panic!("trust height miss match client state height");
                }

                let mmr_root_height = grandpa_client_state.latest_height();

                // build target height header

                let client = relay_rpc_client.clone();

                // subscribe beefy justification and get signed commitment
                let mut sub = subscribe_beefy_justifications(&*relay_rpc_client.rpc())
                    .await
                    .unwrap();

                let raw_signed_commitment = sub.next().await.unwrap().unwrap().0;

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
                } = signed_commitment.commitment.clone();

                let authority_proof = utils::build_validator_proof(relay_rpc_client, block_number)
                    .await
                    .unwrap();

                let target_heights = vec![block_number - 1];
                let mmr_batch_proof = utils::build_mmr_proofs(
                    relay_rpc_client,
                    target_heights,
                    Some(block_number),
                    None,
                )
                .await
                .unwrap();

                let beefy_mmr = utils::to_pb_beefy_mmr(
                    signed_commitment,
                    mmr_batch_proof.clone(),
                    authority_proof,
                );

                let mmr_leaves_proof =
                    beefy_light_client::mmr::MmrLeavesProof::try_from(mmr_batch_proof.proof.0)
                        .unwrap();

                let message = utils::build_subchain_header_map(
                    relay_rpc_client,
                    mmr_leaves_proof.leaf_indices,
                    "sub-0".to_string(), // todo
                )
                .await
                .unwrap();

                let result = GpHeader {
                    // the latest mmr data
                    beefy_mmr,
                    // only one header
                    message: ibc_relayer_types::clients::ics10_grandpa::header::message::Message::SubchainHeaderMap(message),
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
        fn maybe_register_counterparty_payee(
            rt: Arc<TokioRuntime>,
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
            } => maybe_register_counterparty_payee(
                self.rt.clone(),
                relay_rpc,
                Some(para_rpc),
                channel_id,
                port_id,
                counterparty_payee,
            ),
            RpcClient::SubChainRpc { rpc } => maybe_register_counterparty_payee(
                self.rt.clone(),
                rpc,
                None,
                channel_id,
                port_id,
                counterparty_payee,
            ),
        }
    }

    fn cross_chain_query(
        &self,
        requests: Vec<CrossChainQueryRequest>,
    ) -> Result<Vec<CrossChainQueryResponse>, Error> {
        fn cross_chain_query(
            rt: Arc<TokioRuntime>,
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
            } => cross_chain_query(self.rt.clone(), relay_rpc, Some(para_rpc), requests),
            RpcClient::SubChainRpc { rpc } => {
                cross_chain_query(self.rt.clone(), rpc, None, requests)
            }
        }
    }
}
