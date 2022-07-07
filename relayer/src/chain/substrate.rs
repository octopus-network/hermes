use crate::config::ChainConfig;
use crate::denom::DenomTrace;
use crate::error::Error as RelayerError;
// use crate::event::monitor::{EventMonitor, EventReceiver, TxMonitorCmd};
use crate::event::substrate_mointor::{EventMonitor, EventReceiver, TxMonitorCmd};
use crate::keyring::{KeyEntry, KeyRing};
use crate::light_client::grandpa::LightClient as GPLightClient;
use crate::util::retry::{retry_with_index, RetryResult};

use alloc::sync::Arc;
use codec::{Decode, Encode};
use core::fmt::Debug;
use core::{future::Future, str::FromStr, time::Duration};
use subxt::rpc::ClientT;
use tracing::{debug, error, info, trace, warn};

use super::client::ClientSettings;
use crate::chain::endpoint::{ChainEndpoint, ChainStatus, HealthCheck};
use crate::chain::tracking::TrackedMsgs;
use ibc::core::ics23_commitment::merkle::MerkleProof;
use ibc::{
    clients::{
        ics07_tendermint::header::Header as tHeader,
        ics10_grandpa::{
            client_state::ClientState as GPClientState,
            consensus_state::ConsensusState as GPConsensusState,
            header::Header as GPHeader,
            help::{BlockHeader, MmrLeaf, MmrLeafProof},
        },
    },
    core::{
        ics02_client::{
            client_consensus::{AnyConsensusState, AnyConsensusStateWithHeight},
            client_state::{AnyClientState, IdentifiedAnyClientState},
            client_type::ClientType,
        },
        ics03_connection::connection::{ConnectionEnd, Counterparty, IdentifiedConnectionEnd},
        ics04_channel::{
            channel::{ChannelEnd, IdentifiedChannelEnd},
            packet::{Packet, PacketMsgType, Receipt, Sequence},
        },
        ics23_commitment::commitment::CommitmentPrefix,
        ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
    },
    events::{IbcEvent, WithBlockDataType},
    signer::Signer,
    Height as ICSHeight,
};
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::channel::v1::PacketState;

use jsonrpsee::rpc_params;
use octopusxt::{
    ChannelRpc, ClientRpc, ConnectionRpc, MyConfig, OctopusxtClient, PacketRpc, Router,
};
use retry::delay::Fixed;
use semver::Version;
use sp_core::ByteArray;
use std::thread::{self, sleep};
use subxt::{
    rpc::NumberOrHex, storage::StorageEntry, storage::StorageKeyPrefix, BlockNumber, Client,
    ClientBuilder,
};

use tendermint::abci::{Code, Log};

use crate::chain::requests::QueryHeight;
use anyhow::Result;
use beefy_light_client::{commitment, mmr};
use ibc::clients::ics10_grandpa::help::Commitment;
use ibc::core::ics04_channel::events::SendPacket;
use octopusxt::ibc_node::ibc::storage;
use octopusxt::ibc_node::RuntimeApi;
use octopusxt::update_client_state::update_client_state;
use octopusxt::SubstrateNodeTemplateExtrinsicParams;
use serde::{Deserialize, Serialize};
use sp_core::sr25519;
use sp_core::{hexdisplay::HexDisplay, Bytes, Pair, H256};
use sp_runtime::{traits::IdentifyAccount, AccountId32, MultiSigner};
use tendermint::abci::transaction;
use tendermint_proto::Protobuf;
use tendermint_rpc::endpoint::broadcast::tx_sync::Response as TxResponse;
use tokio::runtime::Runtime as TokioRuntime;

use super::requests::{
    IncludeProof, QueryBlockRequest, QueryChannelClientStateRequest, QueryChannelRequest,
    QueryChannelsRequest, QueryClientConnectionsRequest, QueryClientStateRequest,
    QueryClientStatesRequest, QueryConnectionChannelsRequest, QueryConnectionRequest,
    QueryConnectionsRequest, QueryConsensusStateRequest, QueryConsensusStatesRequest,
    QueryHostConsensusStateRequest, QueryNextSequenceReceiveRequest,
    QueryPacketAcknowledgementRequest, QueryPacketAcknowledgementsRequest,
    QueryPacketCommitmentRequest, QueryPacketCommitmentsRequest, QueryPacketEventDataRequest,
    QueryPacketReceiptRequest, QueryTxRequest, QueryUnreceivedAcksRequest,
    QueryUnreceivedPacketsRequest, QueryUpgradedClientStateRequest,
    QueryUpgradedConsensusStateRequest,
};

const MAX_QUERY_TIMES: u64 = 100;

/// A struct used to start a Substrate chain instance in relayer
#[derive(Debug)]
pub struct SubstrateChain {
    config: ChainConfig,
    websocket_url: String,
    keybase: KeyRing,
    rt: Arc<TokioRuntime>,
    client: OctopusxtClient,
}

impl SubstrateChain {
    pub fn config(&self) -> &ChainConfig {
        &self.config
    }

    /// Run a future to completion on the Tokio runtime.
    fn block_on<F: Future>(&self, f: F) -> F::Output {
        self.rt.block_on(f)
    }

    fn retry_wapper<O, Op>(&self, operation: Op) -> Result<O, retry::Error<String>>
    where
        Op: FnOnce() -> Result<O> + Copy,
    {
        retry_with_index(Fixed::from_millis(200), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries".to_string());
            }

            let result = operation();

            match result {
                Ok(v) => RetryResult::Ok(v),
                Err(_) => RetryResult::Retry("Fail to retry".to_string()),
            }
        })
    }

    /// Subscribe Events
    fn subscribe_ibc_events(&self) -> Result<Vec<IbcEvent>> {
        trace!("in substrate: [subscribe_ibc_events]");

        self.block_on(self.client.subscribe_ibc_event())
    }

    /// get latest block height
    fn get_latest_height(&self) -> Result<u64> {
        trace!("in substrate: [get_latest_height]");

        self.block_on(self.client.query_latest_height())
    }

    /// The function to submit IBC request to a Substrate chain
    /// This function handles most of the IBC reqeusts, except the MMR root update
    fn deliever(&self, msgs: Vec<Any>) -> Result<H256> {
        trace!("in substrate: [deliever]");

        let result = self.block_on(self.client.deliver(msgs))?;

        Ok(result)
    }

    /// Retrieve the storage proof according to storage keys
    /// And convert the proof to IBC compatible type
    fn generate_storage_proof<F: StorageEntry>(
        &self,
        storage_entry: &F,
        height: &ICSHeight,
        storage_name: &str,
    ) -> Result<MerkleProof, RelayerError>
    where
        <F as StorageEntry>::Value: Serialize + Debug,
    {
        trace!("in substrate: [generate_storage_proof]");

        let generate_storage_proof = async {
            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<MyConfig>()
                .await
                .map_err(|_| RelayerError::substrate_client_builder_error())?;

            let height = NumberOrHex::Number(height.revision_height);

            let block_hash: H256 = client
                .rpc()
                .block_hash(Some(BlockNumber::from(height)))
                .await
                .map_err(|_| RelayerError::get_block_hash_error())?
                .ok_or_else(RelayerError::empty_hash)?;

            let storage_key = storage_entry.key().final_key(StorageKeyPrefix::new::<F>());
            trace!("in substrate: [generate_storage_proof] >> height: {:?}, block_hash: {:?}, storage key: {:?}, storage_name = {:?}",
                height, block_hash, storage_key, storage_name);

            let params = rpc_params![vec![storage_key], block_hash];

            #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            pub struct ReadProof_ {
                pub at: String,
                pub proof: Vec<Bytes>,
            }

            let storage_proof: ReadProof_ = client
                .rpc()
                .client
                .request("state_getReadProof", params)
                .await
                .map_err(|_| RelayerError::get_read_proof_error())?;

            trace!(
                "in substrate: [generate_storage_proof] >> storage_proof : {:?}",
                storage_proof
            );

            #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
            #[serde(rename_all = "camelCase")]
            pub struct ReadProofU8 {
                pub at: String,
                pub proof: Vec<Vec<u8>>,
            }
            let storage_proof_ = ReadProofU8 {
                at: storage_proof.at,
                proof: storage_proof
                    .proof
                    .iter()
                    .map(|val| val.clone().0)
                    .collect::<Vec<Vec<u8>>>(),
            };
            trace!(
                "in substrate: [generate_storage_proof] >> storage_proof_ : {:?}",
                storage_proof_
            );

            let storage_proof_str = serde_json::to_string(&storage_proof_)
                .map_err(RelayerError::invalid_serde_json_error)?;
            trace!(
                "in substrate: [generate_storage_proof] >> storage_proof_str: {:?}",
                storage_proof_str
            );

            Ok(storage_proof_str)
        };

        let storage_proof = self.block_on(generate_storage_proof)?;

        Ok(compose_ibc_merkle_proof(storage_proof))
    }

    fn key(&self) -> Result<KeyEntry, RelayerError> {
        trace!("in substrate: [key]");
        self.keybase()
            .get_key(&self.config.key_name)
            .map_err(RelayerError::key_base)
    }
}

impl ChainEndpoint for SubstrateChain {
    type LightBlock = GPHeader;
    type Header = GPHeader;
    type ConsensusState = AnyConsensusState;
    type ClientState = AnyClientState;
    type LightClient = GPLightClient;

    fn bootstrap(config: ChainConfig, rt: Arc<TokioRuntime>) -> Result<Self, RelayerError> {
        trace!("in Substrate: [bootstrap function]");

        let websocket_url = format!("{}", config.websocket_addr);

        // Initialize key store and load key
        let keybase = KeyRing::new(config.key_store_type, &config.account_prefix, &config.id)
            .map_err(RelayerError::key_base)?;

        let client = async {
            ClientBuilder::new()
                .set_url(websocket_url.clone())
                .build::<MyConfig>()
                .await
                .map_err(|_| {
                    anyhow::anyhow!(format!(
                        "{:?}",
                        RelayerError::substrate_client_builder_error()
                    ))
                })
        };

        let client = rt
            .block_on(client)
            .map_err(|_| RelayerError::substrate_client_builder_error())?;

        let chain = Self {
            config,
            websocket_url,
            rt,
            keybase,
            client: OctopusxtClient::new(client),
        };

        Ok(chain)
    }

    fn init_light_client(&self) -> Result<Self::LightClient, RelayerError> {
        trace!("in substrate: [init_light_client]");

        let config = self.config.clone();

        let public_key = async {
            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<MyConfig>()
                .await
                .map_err(|_| RelayerError::substrate_client_builder_error())?;

            let api =
                client.to_runtime_api::<RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

            let authorities = api
                .storage()
                .beefy()
                .authorities(None)
                .await
                .map_err(|_| RelayerError::authorities())?;

            info!("authorities length : {:?}", authorities.len());

            let result: Vec<String> = authorities
                .into_iter()
                .map(|val| format!("0x{}", HexDisplay::from(&val.to_raw_vec())))
                .collect();

            info!("authorities member: {:?}", result);
            Ok(result)
        };
        let public_key = self.block_on(public_key)?;

        let initial_public_keys = public_key;
        let light_client = GPLightClient::from_config(
            &config,
            self.websocket_url.clone(),
            self.rt.clone(),
            initial_public_keys,
        );
        Ok(light_client)
    }

    fn init_event_monitor(
        &self,
        rt: Arc<TokioRuntime>,
    ) -> Result<(EventReceiver, TxMonitorCmd), RelayerError> {
        trace!(
            "in substrate: [init_event_mointor] >> websocket addr: {:?}",
            self.config.websocket_addr.clone()
        );

        let (mut event_monitor, event_receiver, monitor_tx) = EventMonitor::new(
            self.config.id.clone(),
            self.config.websocket_addr.clone(),
            rt,
        )
        .map_err(RelayerError::event_monitor)?;

        event_monitor
            .subscribe()
            .map_err(RelayerError::event_monitor)?;

        thread::spawn(move || event_monitor.run());

        Ok((event_receiver, monitor_tx))
    }

    // TODO(davirain)
    fn shutdown(self) -> Result<(), RelayerError> {
        trace!("in substrate: [shutdown]");

        Ok(())
    }

    // TODO(davirain)
    fn health_check(&self) -> Result<HealthCheck, RelayerError> {
        trace!("in substrate: [health_check]");

        Ok(HealthCheck::Healthy)
    }

    fn id(&self) -> &ChainId {
        trace!("in substrate: [id]");

        &self.config().id
    }

    fn keybase(&self) -> &KeyRing {
        trace!("in substrate: [keybase]");

        &self.keybase
    }

    fn keybase_mut(&mut self) -> &mut KeyRing {
        trace!("in substrate: [keybase_mut]");
        &mut self.keybase
    }

    fn send_messages_and_wait_commit(
        &mut self,
        proto_msgs: TrackedMsgs,
    ) -> Result<Vec<IbcEvent>, RelayerError> {
        trace!(
            "in substrate: [send_messages_and_wait_commit], proto_msgs={:?}",
            proto_msgs.tracking_id
        );

        sleep(Duration::from_secs(4));
        let result = self
            .deliever(proto_msgs.messages().to_vec())
            .map_err(|e| RelayerError::deliver_error(e))?;

        debug!(
            "in substrate: [send_messages_and_wait_commit] >> extrics_hash  : {:?}",
            result
        );

        let ibc_event = self
            .subscribe_ibc_events()
            .map_err(|_| RelayerError::subscribe_ibc_events())?;

        Ok(ibc_event)
    }

    fn send_messages_and_wait_check_tx(
        &mut self,
        proto_msgs: TrackedMsgs,
    ) -> Result<Vec<TxResponse>, RelayerError> {
        debug!(
            "in substrate: [send_messages_and_wait_check_tx], proto_msgs={:?}",
            proto_msgs.tracking_id
        );

        sleep(Duration::from_secs(4));
        let result = self
            .deliever(proto_msgs.messages().to_vec())
            .map_err(|e| RelayerError::deliver_error(e))?;

        debug!(
            "in substrate: [send_messages_and_wait_check_tx] >> extrics_hash : {:?}",
            result
        );

        let json = "\"ChYKFGNvbm5lY3Rpb25fb3Blbl9pbml0\"";
        // TODO(davirain) this tx_response is correct???
        let tx_re = TxResponse {
            code: Code::default(),
            data: serde_json::from_str(json).map_err(RelayerError::invalid_serde_json_error)?,
            log: Log::from("testtest"),
            hash: transaction::Hash::new(*result.as_fixed_bytes()),
        };

        Ok(vec![tx_re])
    }

    fn get_signer(&mut self) -> Result<Signer, RelayerError> {
        trace!("In Substrate: [get signer]");
        crate::time!("get_signer");

        /// Public key type for Runtime
        pub type PublicFor<P> = <P as Pair>::Public;

        /// formats public key as accountId as hex
        fn format_account_id<P: Pair>(public_key: PublicFor<P>) -> String
        where
            PublicFor<P>: Into<MultiSigner>,
        {
            format!(
                "0x{}",
                HexDisplay::from(&public_key.into().into_account().as_ref())
            )
        }

        // Get the key from key seed file
        let key = self
            .keybase()
            .get_key(&self.config.key_name)
            .map_err(|e| RelayerError::key_not_found(self.config.key_name.clone(), e))?;

        let private_seed = key.mnemonic;
        let (pair, _seed) = sr25519::Pair::from_phrase(&private_seed, None).unwrap();
        let public_key = pair.public();

        let account_id = format_account_id::<sr25519::Pair>(public_key);
        let account = AccountId32::from_str(&account_id).unwrap();
        let encode_account = AccountId32::encode(&account);
        let hex_account = hex::encode(encode_account);

        Ok(Signer::from_str(&hex_account).unwrap())
    }

    fn get_key(&mut self) -> Result<KeyEntry, RelayerError> {
        trace!("in substrate: [get_key]");
        crate::time!("get_key");

        // Get the key from key seed file
        let key = self
            .keybase()
            .get_key(&self.config.key_name)
            .map_err(|e| RelayerError::key_not_found(self.config.key_name.clone(), e))?;

        Ok(key)
    }

    // TODO(davirain)
    fn query_commitment_prefix(&self) -> Result<CommitmentPrefix, RelayerError> {
        trace!("in substrate: [query_commitment_prefix]");

        // TODO - do a real chain query
        CommitmentPrefix::try_from(self.config().store_prefix.as_bytes().to_vec())
            .map_err(|_| RelayerError::invalid_commitment_prefix())
    }

    /// Query the latest height and timestamp the application is at
    fn query_application_status(&self) -> Result<ChainStatus, RelayerError> {
        trace!("in substrate: [query_status]");

        let chain_status = self
            .retry_wapper(|| self.client.query_application_status())
            .map_err(RelayerError::retry_error)?;

        Ok(ChainStatus {
            height: chain_status.height,
            timestamp: chain_status.timestamp,
        })
    }

    fn query_clients(
        &self,
        request: QueryClientStatesRequest,
    ) -> Result<Vec<IdentifiedAnyClientState>, RelayerError> {
        trace!("in substrate: [query_clients]");
        let QueryClientStatesRequest { pagination } = request;

        let request = octopusxt::requests::QueryClientStatesRequest { pagination: None };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let identified_any_client_states =
            self.block_on(self.client.query_clients(request)).unwrap();

        Ok(identified_any_client_states)
    }

    /// Performs a query to retrieve the state of the specified light client. A
    /// proof can optionally be returned along with the result.
    fn query_client_state(
        &self,
        request: QueryClientStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyClientState, Option<MerkleProof>), RelayerError> {
        trace!("in substrate: [query_client_state]");
        let QueryClientStateRequest { client_id, height } = request;

        let request = octopusxt::requests::QueryClientStateRequest {
            client_id,
            height: match height {
                QueryHeight::Latest => octopusxt::requests::QueryHeight::Latest,
                QueryHeight::Specific(value) => octopusxt::requests::QueryHeight::Specific(value),
            },
        };

        let include_proof = match include_proof {
            IncludeProof::Yes => octopusxt::requests::IncludeProof::Yes,
            IncludeProof::No => octopusxt::requests::IncludeProof::No,
        };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let any_client_state = self
            .block_on(self.client.query_client_state(request, include_proof))
            .unwrap();

        Ok(any_client_state)
    }

    /// Performs a query to retrieve all the consensus states that the specified
    /// light client stores.
    fn query_consensus_states(
        &self,
        request: QueryConsensusStatesRequest,
    ) -> Result<Vec<AnyConsensusStateWithHeight>, RelayerError> {
        trace!("in substrate: [query_consensus_states]");
        let QueryConsensusStatesRequest {
            client_id,
            pagination,
        } = request;

        let request = octopusxt::requests::QueryConsensusStatesRequest {
            client_id,
            pagination: None,
        };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let any_consensus_state_with_height = self
            .block_on(self.client.query_consensus_states(request))
            .unwrap();

        Ok(any_consensus_state_with_height)
    }

    /// Performs a query to retrieve the consensus state for a specified height
    /// `consensus_height` that the specified light client stores.
    fn query_consensus_state(
        &self,
        request: QueryConsensusStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyConsensusState, Option<MerkleProof>), RelayerError> {
        trace!("in substrate: [query_consensus_state]");
        let QueryConsensusStateRequest {
            client_id,
            consensus_height,
            query_height,
        } = request;

        let request = octopusxt::requests::QueryConsensusStateRequest {
            client_id,
            consensus_height,
            query_height: match query_height {
                QueryHeight::Latest => octopusxt::requests::QueryHeight::Latest,
                QueryHeight::Specific(value) => octopusxt::requests::QueryHeight::Specific(value),
            },
        };

        let include_proof = match include_proof {
            IncludeProof::Yes => octopusxt::requests::IncludeProof::Yes,
            IncludeProof::No => octopusxt::requests::IncludeProof::No,
        };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let any_consensus_state = self
            .block_on(self.client.query_consensus_state(request, include_proof))
            .unwrap();

        Ok(any_consensus_state)
    }

    fn query_upgraded_client_state(
        &self,
        request: QueryUpgradedClientStateRequest,
    ) -> Result<(AnyClientState, MerkleProof), RelayerError> {
        trace!("in substrate: [query_upgraded_client_state]");
        let QueryUpgradedClientStateRequest { upgrade_height } = request;

        let request = octopusxt::requests::QueryUpgradedClientStateRequest { upgrade_height };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let any_client_state = self
            .block_on(self.client.query_upgraded_client_state(request))
            .unwrap();

        Ok(any_client_state)
    }

    fn query_upgraded_consensus_state(
        &self,
        request: QueryUpgradedConsensusStateRequest,
    ) -> Result<(AnyConsensusState, MerkleProof), RelayerError> {
        trace!("in substrate: [query_upgraded_consensus_state]");
        let QueryUpgradedConsensusStateRequest { upgrade_height } = request;

        let request = octopusxt::requests::QueryUpgradedConsensusStateRequest { upgrade_height };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let any_consensus_state = self
            .block_on(self.client.query_upgraded_consensus_state(request))
            .unwrap();

        Ok(any_consensus_state)
    }

    /// Performs a query to retrieve the identifiers of all connections.
    fn query_connections(
        &self,
        request: QueryConnectionsRequest,
    ) -> Result<Vec<IdentifiedConnectionEnd>, RelayerError> {
        trace!("in substrate: [query_connections]");
        let QueryConnectionsRequest { pagination } = request;

        let request = octopusxt::requests::QueryConnectionsRequest { pagination: None };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let identified_connection_end = self
            .block_on(self.client.query_connections(request))
            .unwrap();

        Ok(identified_connection_end)
    }

    /// Performs a query to retrieve the identifiers of all connections.
    fn query_client_connections(
        &self,
        request: QueryClientConnectionsRequest,
    ) -> Result<Vec<ConnectionId>, RelayerError> {
        trace!("in substrate: [query_client_connections]");
        let QueryClientConnectionsRequest { client_id } = request;

        let request = octopusxt::requests::QueryClientConnectionsRequest { client_id };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let connection_ids = self
            .block_on(self.client.query_client_connections(request))
            .unwrap();

        Ok(connection_ids)
    }

    /// Performs a query to retrieve the connection associated with a given
    /// connection identifier. A proof can optionally be returned along with the
    /// result.
    fn query_connection(
        &self,
        request: QueryConnectionRequest,
        include_proof: IncludeProof,
    ) -> Result<(ConnectionEnd, Option<MerkleProof>), RelayerError> {
        trace!("in substrate: [query_connection]");
        let QueryConnectionRequest {
            connection_id,
            height,
        } = request;

        let request = octopusxt::requests::QueryConnectionRequest {
            connection_id,
            height: match height {
                QueryHeight::Latest => octopusxt::requests::QueryHeight::Latest,
                QueryHeight::Specific(value) => octopusxt::requests::QueryHeight::Specific(value),
            },
        };

        let include_proof = match include_proof {
            IncludeProof::Yes => octopusxt::requests::IncludeProof::Yes,
            IncludeProof::No => octopusxt::requests::IncludeProof::No,
        };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let connection_end = self
            .block_on(self.client.query_connection(request, include_proof))
            .unwrap();

        Ok(connection_end)
    }

    /// Performs a query to retrieve all channels associated with a connection.
    fn query_connection_channels(
        &self,
        request: QueryConnectionChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, RelayerError> {
        trace!("in substrate: [query_connection_channels] ");
        let QueryConnectionChannelsRequest {
            connection_id,
            pagination,
        } = request;

        let request = octopusxt::requests::QueryConnectionChannelsRequest {
            connection_id,
            pagination: None,
        };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let identified_channeld_end = self
            .block_on(self.client.query_connection_channels(request))
            .unwrap();

        Ok(identified_channeld_end)
    }

    /// Performs a query to retrieve all the channels of a chain.
    fn query_channels(
        &self,
        request: QueryChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, RelayerError> {
        trace!("in substrate: [query_channels]");
        let QueryChannelsRequest { pagination } = request;

        let request = octopusxt::requests::QueryChannelsRequest { pagination: None };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let identified_channel_ends = self.block_on(self.client.query_channels(request)).unwrap();

        Ok(identified_channel_ends)
    }

    /// Performs a query to retrieve the channel associated with a given channel
    /// identifier. A proof can optionally be returned along with the result.
    fn query_channel(
        &self,
        request: QueryChannelRequest,
        include_proof: IncludeProof,
    ) -> Result<(ChannelEnd, Option<MerkleProof>), RelayerError> {
        trace!("in substrate: [query_channel]");
        let QueryChannelRequest {
            port_id,
            channel_id,
            height,
        } = request;

        let request = octopusxt::requests::QueryChannelRequest {
            port_id,
            channel_id,
            height: match height {
                QueryHeight::Latest => octopusxt::requests::QueryHeight::Latest,
                QueryHeight::Specific(value) => octopusxt::requests::QueryHeight::Specific(value),
            },
        };

        let include_proof = match include_proof {
            IncludeProof::Yes => octopusxt::requests::IncludeProof::Yes,
            IncludeProof::No => octopusxt::requests::IncludeProof::No,
        };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let channel_end = self
            .block_on(self.client.query_channel(request, include_proof))
            .unwrap();

        Ok(channel_end)
    }

    // TODO(davirain)
    /// Performs a query to retrieve the client state for the channel associated
    /// with a given channel identifier.
    fn query_channel_client_state(
        &self,
        request: QueryChannelClientStateRequest,
    ) -> Result<Option<IdentifiedAnyClientState>, RelayerError> {
        trace!("in substrate: [query_channel_client_state]");
        let QueryChannelClientStateRequest {
            port_id,
            channel_id,
        } = request;

        let request = octopusxt::requests::QueryChannelClientStateRequest {
            port_id,
            channel_id,
        };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let identified_any_client_state = self
            .block_on(self.client.query_channel_client_state(request))
            .unwrap();

        Ok(identified_any_client_state)
    }

    /// Performs a query to retrieve all the packet commitments hashes
    /// associated with a channel. Returns the corresponding packet sequence
    /// numbers and the height at which they were retrieved.
    fn query_packet_commitments(
        &self,
        request: QueryPacketCommitmentsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), RelayerError> {
        trace!("in substrate: [query_packet_commitments]");
        let QueryPacketCommitmentsRequest {
            port_id,
            channel_id,
            pagination,
        } = request;

        let request = octopusxt::requests::QueryPacketCommitmentsRequest {
            port_id,
            channel_id,
            pagination: None,
        };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let packet_commitments = self
            .block_on(self.client.query_packet_commitments(request))
            .unwrap();

        Ok(packet_commitments)
    }

    /// Performs a query about which IBC packets in the specified list has not
    /// been received. Returns the sequence numbers of the packets that were not
    /// received.
    ///
    /// For example, given a request with the sequence numbers `[5,6,7,8]`, a
    /// response of `[7,8]` would indicate that packets 5 & 6 were received,
    /// while packets 7, 8 were not.
    fn query_unreceived_packets(
        &self,
        request: QueryUnreceivedPacketsRequest,
    ) -> Result<Vec<Sequence>, RelayerError> {
        trace!("in substrate: [query_unreceived_packets]");
        let QueryUnreceivedPacketsRequest {
            port_id,
            channel_id,
            packet_commitment_sequences,
        } = request;

        let request = octopusxt::requests::QueryUnreceivedPacketsRequest {
            port_id,
            channel_id,
            packet_commitment_sequences,
        };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let sequences = self
            .block_on(self.client.query_unreceived_packets(request))
            .unwrap();

        Ok(sequences)
    }

    /// Performs a query to retrieve all the packet acknowledgements associated
    /// with a channel. Returns the corresponding packet sequence numbers and
    /// the height at which they were retrieved.
    fn query_packet_acknowledgements(
        &self,
        request: QueryPacketAcknowledgementsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), RelayerError> {
        trace!("in substrate: [query_packet_acknowledgements]");

        let QueryPacketAcknowledgementsRequest {
            port_id,
            channel_id,
            pagination,
            packet_commitment_sequences,
        } = request;

        let request = octopusxt::requests::QueryPacketAcknowledgementsRequest {
            port_id,
            channel_id,
            pagination: None,
            packet_commitment_sequences,
        };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let sequences = self
            .block_on(self.client.query_packet_acknowledgements(request))
            .unwrap();

        Ok(sequences)
    }

    /// Performs a query about which IBC packets in the specified list has not
    /// been acknowledged. Returns the sequence numbers of the packets that were not
    /// acknowledged.
    ///
    /// For example, given a request with the sequence numbers `[5,6,7,8]`, a
    /// response of `[7,8]` would indicate that packets 5 & 6 were acknowledged,
    /// while packets 7, 8 were not.
    fn query_unreceived_acknowledgements(
        &self,
        request: QueryUnreceivedAcksRequest,
    ) -> Result<Vec<Sequence>, RelayerError> {
        trace!("in substrate: [query_unreceived_acknowledgements] ");
        let QueryUnreceivedAcksRequest {
            port_id,
            channel_id,
            packet_ack_sequences,
        } = request;

        let request = octopusxt::requests::QueryUnreceivedAcksRequest {
            port_id,
            channel_id,
            packet_ack_sequences,
        };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let sequences = self
            .block_on(self.client.query_unreceived_acknowledgements(request))
            .unwrap();

        Ok(sequences)
    }

    /// Performs a query to retrieve `nextSequenceRecv` stored at path
    /// `path::SeqRecvsPath` as defined in ICS-4. A proof can optionally be
    /// returned along with the result.
    fn query_next_sequence_receive(
        &self,
        request: QueryNextSequenceReceiveRequest,
        include_proof: IncludeProof,
    ) -> Result<(Sequence, Option<MerkleProof>), RelayerError> {
        trace!("in substrate: [query_next_sequence_receive] ");
        let QueryNextSequenceReceiveRequest {
            port_id,
            channel_id,
            height,
        } = request;

        let request = octopusxt::requests::QueryNextSequenceReceiveRequest {
            port_id,
            channel_id,
            height: match height {
                QueryHeight::Latest => octopusxt::requests::QueryHeight::Latest,
                QueryHeight::Specific(value) => octopusxt::requests::QueryHeight::Specific(value),
            },
        };

        let include_proof = match include_proof {
            IncludeProof::Yes => octopusxt::requests::IncludeProof::Yes,
            IncludeProof::No => octopusxt::requests::IncludeProof::No,
        };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let sequences = self
            .block_on(
                self.client
                    .query_next_sequence_receive(request, include_proof),
            )
            .unwrap();

        Ok(sequences)
    }

    // TODO(davirain)
    fn query_txs(&self, request: QueryTxRequest) -> Result<Vec<IbcEvent>, RelayerError> {
        trace!("in substrate: [query_txs]");

        match request {
            // Todo: Related to https://github.com/octopus-network/ibc-rs/issues/88
            QueryTxRequest::Packet(request) => {
                let result: Vec<IbcEvent> = vec![];
                if request.sequences.is_empty() {
                    return Ok(result);
                }

                match request.event_id {
                    WithBlockDataType::SendPacket => {
                        // let mut send_packet_event = self.get_ibc_send_packet_event(request)?;
                        // result.append(&mut send_packet_event);
                    }

                    WithBlockDataType::WriteAck => {
                        // let mut ack_event = self.get_ibc_write_acknowledgement_event(request)?;
                        // result.append(&mut ack_event);
                    }

                    _ => unimplemented!(),
                }

                Ok(result)
            }

            QueryTxRequest::Client(request) => {
                use ibc::core::ics02_client::events::Attributes;

                // Todo: the client event below is mock
                // replace it with real client event replied from a Substrate chain
                let result: Vec<IbcEvent> = vec![IbcEvent::UpdateClient(
                    ibc::core::ics02_client::events::UpdateClient::from(Attributes {
                        height: ICSHeight::new(0, self.get_latest_height().unwrap()),
                        client_id: request.client_id,
                        client_type: ClientType::Grandpa,
                        consensus_height: request.consensus_height,
                    }),
                )];

                Ok(result)
            }

            QueryTxRequest::Transaction(_tx) => {
                // tracing::trace!("in substrate: [query_txs]: Transaction: {:?}", tx);
                // Todo: https://github.com/octopus-network/ibc-rs/issues/98
                let result: Vec<IbcEvent> = vec![];
                Ok(result)
            }
        }
    }

    fn build_client_state(
        &self,
        height: ICSHeight,
        _dst_config: ClientSettings,
    ) -> Result<Self::ClientState, RelayerError> {
        trace!("in substrate: [build_client_state]");

        let public_key = async {
            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<MyConfig>()
                .await
                .map_err(|_| RelayerError::substrate_client_builder_error())?;

            let api =
                client.to_runtime_api::<RuntimeApi<MyConfig, SubstrateNodeTemplateExtrinsicParams<MyConfig>>>();

            let authorities = api
                .storage()
                .beefy()
                .authorities(None)
                .await
                .map_err(|_| RelayerError::authorities())?;

            let result: Vec<String> = authorities
                .into_iter()
                .map(|val| format!("0x{}", HexDisplay::from(&val.to_raw_vec())))
                .collect();

            Ok(result)
        };
        let public_key = self.block_on(public_key)?;

        let beefy_light_client = beefy_light_client::new(public_key);

        // Build client state
        let client_state = GPClientState::new(
            self.id().clone(),
            height.revision_height as u32,
            BlockHeader::default(),
            Commitment::default(),
            beefy_light_client.validator_set.into(),
        )
        .map_err(RelayerError::ics10)?;

        Ok(AnyClientState::Grandpa(client_state))
    }

    // TODO(davirain) because this consensus_state return is default value
    fn build_consensus_state(
        &self,
        _light_block: Self::LightBlock,
    ) -> Result<Self::ConsensusState, RelayerError> {
        trace!("in substrate: [build_consensus_state]");

        Ok(AnyConsensusState::Grandpa(GPConsensusState::default()))
    }

    fn build_header(
        &self,
        trusted_height: ICSHeight,
        target_height: ICSHeight,
        client_state: &AnyClientState,
        _light_client: &mut Self::LightClient,
    ) -> Result<(Self::Header, Vec<Self::Header>), RelayerError> {
        trace!("in substrate: [build_header]");

        assert!(trusted_height.revision_height < target_height.revision_height);

        let grandpa_client_state = match client_state {
            AnyClientState::Grandpa(state) => state,
            _ => unimplemented!(),
        };

        // assert trust_height <= grandpa_client_state height
        if trusted_height.revision_height > grandpa_client_state.block_number as u64 {
            return Err(RelayerError::trust_height_miss_match_client_state_height(
                trusted_height.revision_height,
                grandpa_client_state.block_number as u64,
            ));
        }

        // build target height header
        let result = async {
            let commitment::Commitment {
                payload: _payload,
                block_number,
                validator_set_id: _validator_set_id,
            } = grandpa_client_state.latest_commitment.clone().into();

            let mmr_root_height = block_number;
            assert!((target_height.revision_height as u32) <= mmr_root_height);

            // get block header

            let block_header = self
                .client
                .query_header_by_block_number(Some(BlockNumber::from(
                    target_height.revision_height as u32,
                )))
                .await
                .map_err(|_| RelayerError::get_header_by_block_number_error())?;

            let api = self.client.to_runtime_api();

            assert_eq!(
                block_header.block_number,
                target_height.revision_height as u32
            );

            let block_hash: Option<H256> = api
                .client
                .rpc()
                .block_hash(Some(BlockNumber::from(mmr_root_height)))
                .await
                .map_err(|_| RelayerError::get_block_hash_error())?;

            let mmr_leaf_and_mmr_leaf_proof = self
                .client
                .query_mmr_leaf_and_mmr_proof(
                    Some(BlockNumber::from(
                        (target_height.revision_height - 1) as u32,
                    )),
                    block_hash,
                )
                .await
                .map_err(|_| RelayerError::get_mmr_leaf_and_mmr_proof_error())?;

            Ok((block_header, mmr_leaf_and_mmr_leaf_proof))
        };

        let result = self.block_on(result)?;

        let encoded_mmr_leaf = result.1 .1;
        let encoded_mmr_leaf_proof = result.1 .2;

        let leaf: Vec<u8> = Decode::decode(&mut &encoded_mmr_leaf[..])
            .map_err(RelayerError::invalid_codec_decode)?;
        let mmr_leaf: mmr::MmrLeaf =
            Decode::decode(&mut &*leaf).map_err(RelayerError::invalid_codec_decode)?;
        let mmr_leaf_proof = mmr::MmrLeafProof::decode(&mut &encoded_mmr_leaf_proof[..])
            .map_err(RelayerError::invalid_codec_decode)?;

        let grandpa_header = GPHeader {
            block_header: result.0,
            mmr_leaf: MmrLeaf::from(mmr_leaf),
            mmr_leaf_proof: MmrLeafProof::from(mmr_leaf_proof),
        };

        Ok((grandpa_header, vec![]))
    }

    fn config(&self) -> ChainConfig {
        trace!("in substrate: [config]");
        self.config.clone()
    }

    fn add_key(&mut self, key_name: &str, key: KeyEntry) -> Result<(), RelayerError> {
        trace!("in substrate: [add_key]");
        self.keybase_mut()
            .add_key(key_name, key)
            .map_err(RelayerError::key_base)?;

        Ok(())
    }

    // TODO(davirain)
    fn ibc_version(&self) -> Result<Option<Version>, RelayerError> {
        trace!("in substrate: [ibc_version]");

        Ok(None)
    }

    fn query_blocks(
        &self,
        _request: QueryBlockRequest,
    ) -> Result<(Vec<IbcEvent>, Vec<IbcEvent>), RelayerError> {
        trace!("in substrate: [query_block]");
        let QueryBlockRequest::Packet(value) = _request;
        let request = octopusxt::requests::QueryBlockRequest::Packet(
            octopusxt::requests::QueryPacketEventDataRequest {
                event_id: value.event_id,
                source_channel_id: value.source_channel_id,
                source_port_id: value.source_port_id,
                destination_channel_id: value.destination_channel_id,
                destination_port_id: value.destination_port_id,
                sequences: value.sequences,
                height: match value.height {
                    QueryHeight::Latest => octopusxt::requests::QueryHeight::Latest,
                    QueryHeight::Specific(value) => {
                        octopusxt::requests::QueryHeight::Specific(value)
                    }
                },
            },
        );

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let result = self.client.query_blocks(request).unwrap();

        Ok(result)
    }

    // TODO(davirain)
    fn query_host_consensus_state(
        &self,
        request: QueryHostConsensusStateRequest,
    ) -> Result<Self::ConsensusState, RelayerError> {
        trace!("in substrate: [query_host_consensus_state]");

        let QueryHostConsensusStateRequest { height } = request;

        Ok(AnyConsensusState::Grandpa(GPConsensusState::default()))
    }

    fn query_balance(
        &self,
        key_name: Option<String>,
    ) -> Result<crate::account::Balance, RelayerError> {
        trace!("in substrate: [query_balance]");

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let balance = self.client.query_balance(key_name).unwrap();

        Ok(crate::account::Balance {
            amount: balance.amount,
            denom: balance.denom,
        })
    }

    /// Query the denomination trace given a trace hash.
    fn query_denom_trace(&self, hash: String) -> Result<DenomTrace, RelayerError> {
        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let denom_trace = self.client.query_denom_trace(hash).unwrap();

        Ok(DenomTrace {
            path: denom_trace.path,
            base_denom: denom_trace.base_denom,
        })
    }

    fn query_packet_commitment(
        &self,
        request: QueryPacketCommitmentRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), RelayerError> {
        trace!("in substrate: [query_packet_commitment]");

        let QueryPacketCommitmentRequest {
            port_id,
            channel_id,
            sequence,
            height,
        } = request;

        let request = octopusxt::requests::QueryPacketCommitmentRequest {
            port_id,
            channel_id,
            sequence,
            height: match height {
                QueryHeight::Latest => octopusxt::requests::QueryHeight::Latest,
                QueryHeight::Specific(value) => octopusxt::requests::QueryHeight::Specific(value),
            },
        };

        let include_proof = match include_proof {
            IncludeProof::Yes => octopusxt::requests::IncludeProof::Yes,
            IncludeProof::No => octopusxt::requests::IncludeProof::No,
        };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let packet_commitment = self
            .block_on(self.client.query_packet_commitment(request, include_proof))
            .unwrap();

        Ok(packet_commitment)
    }

    fn query_packet_receipt(
        &self,
        request: QueryPacketReceiptRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), RelayerError> {
        trace!("in substrate: [query_packet_receipt]");
        let QueryPacketReceiptRequest {
            port_id,
            channel_id,
            sequence,
            height,
        } = request;

        let request = octopusxt::requests::QueryPacketReceiptRequest {
            port_id,
            channel_id,
            sequence,
            height: match height {
                QueryHeight::Latest => octopusxt::requests::QueryHeight::Latest,
                QueryHeight::Specific(value) => octopusxt::requests::QueryHeight::Specific(value),
            },
        };

        let include_proof = match include_proof {
            IncludeProof::Yes => octopusxt::requests::IncludeProof::Yes,
            IncludeProof::No => octopusxt::requests::IncludeProof::No,
        };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirain): REMOVE unwrap()
        let packet_receipt = self
            .block_on(self.client.query_packet_receipt(request, include_proof))
            .unwrap();

        Ok(packet_receipt)
    }

    fn query_packet_acknowledgement(
        &self,
        request: QueryPacketAcknowledgementRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), RelayerError> {
        trace!("in substrate: [query_packet_acknowledgement]");
        let QueryPacketAcknowledgementRequest {
            port_id,
            channel_id,
            sequence,
            height,
        } = request;

        let request = octopusxt::requests::QueryPacketAcknowledgementRequest {
            port_id,
            channel_id,
            sequence,
            height: match height {
                QueryHeight::Latest => octopusxt::requests::QueryHeight::Latest,
                QueryHeight::Specific(value) => octopusxt::requests::QueryHeight::Specific(value),
            },
        };

        let include_proof = match include_proof {
            IncludeProof::Yes => octopusxt::requests::IncludeProof::Yes,
            IncludeProof::No => octopusxt::requests::IncludeProof::No,
        };

        // FIXED: Now there we not use retry_wapper
        // TODO(davirian): REMOVE unwrap()
        let packet_acknowledgement = self
            .block_on(
                self.client
                    .query_packet_acknowledgement(request, include_proof),
            )
            .unwrap();

        Ok(packet_acknowledgement)
    }
}

// to create a new type in `commitment_proof::Proof`
/// Compose merkle proof according to ibc proto
/// this is only for test
pub fn compose_ibc_merkle_proof(proof: String) -> MerkleProof {
    use ics23::{commitment_proof, ExistenceProof, InnerOp};
    trace!("in substrate: [compose_ibc_merkle_proof]");

    let _inner_op = InnerOp {
        hash: 0,
        prefix: vec![0],
        suffix: vec![0],
    };

    let _proof = commitment_proof::Proof::Exist(ExistenceProof {
        key: vec![0],
        value: proof.as_bytes().to_vec(),
        leaf: None,
        path: vec![_inner_op],
    });

    let parsed = ics23::CommitmentProof {
        proof: Some(_proof),
    };
    let proofs: Vec<ics23::CommitmentProof> = vec![parsed];
    MerkleProof { proofs }
}

/// this is only for test
pub fn get_dummy_merkle_proof() -> MerkleProof {
    let parsed = ics23::CommitmentProof { proof: None };
    let proofs: Vec<ics23::CommitmentProof> = vec![parsed];
    MerkleProof { proofs }
}

/// this is only for test
pub fn get_dummy_ics07_header() -> tHeader {
    use tendermint::block::signed_header::SignedHeader;
    // Build a SignedHeader from a JSON file.
    let shdr = serde_json::from_str::<SignedHeader>(include_str!(
        "../../../modules/tests/support/signed_header.json"
    ))
    .unwrap();

    // Build a set of validators.
    // Below are test values inspired form `test_validator_set()` in tendermint-rs.
    use subtle_encoding::hex;
    use tendermint::validator::Info as ValidatorInfo;
    use tendermint::PublicKey;

    let v1: ValidatorInfo = ValidatorInfo::new(
        PublicKey::from_raw_ed25519(
            &hex::decode_upper("F349539C7E5EF7C49549B09C4BFC2335318AB0FE51FBFAA2433B4F13E816F4A7")
                .unwrap(),
        )
        .unwrap(),
        281_815_u64.try_into().unwrap(),
    );

    use tendermint::validator::Set as ValidatorSet;
    let vs = ValidatorSet::new(vec![v1.clone()], Some(v1));

    tHeader {
        signed_header: shdr,
        validator_set: vs.clone(),
        trusted_height: ICSHeight::new(0, 1),
        trusted_validator_set: vs,
    }
}

#[test]
fn test_compose_ibc_merkle_proof() {
    use core::convert::TryFrom;
    use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
    use ibc_proto::ics23::commitment_proof::Proof::Exist;
    use serde::{Deserialize, Serialize};

    let ibc_proof = compose_ibc_merkle_proof("proof".to_string());
    let merkel_proof = RawMerkleProof::try_from(ibc_proof).unwrap();
    let _merkel_proof = merkel_proof.proofs[0].proof.clone().unwrap();

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ReadProofU8 {
        pub at: String,
        pub proof: Vec<Vec<u8>>,
    }

    match _merkel_proof {
        Exist(_exist_proof) => {
            let _proof_str = String::from_utf8(_exist_proof.value).unwrap();
            assert_eq!(_proof_str, "proof".to_string());
        }
        _ => unimplemented!(),
    };
}
