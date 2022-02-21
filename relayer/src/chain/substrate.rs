use super::{ChainEndpoint, HealthCheck};
use crate::config::ChainConfig;
use crate::error::Error;
// use crate::event::monitor::{EventMonitor, EventReceiver, TxMonitorCmd};
use crate::event::substrate_mointor::{EventMonitor, EventReceiver, TxMonitorCmd};
use crate::keyring::{KeyEntry, KeyRing, Store};
use crate::light_client::grandpa::LightClient as GPLightClient;
use crate::light_client::LightClient;
use crate::{
    util::retry::{retry_with_index, RetryResult},
    worker::retry_strategy,
};
use alloc::sync::Arc;
use bech32::{ToBase32, Variant};
use chrono::offset::Utc;
use codec::{Decode, Encode};
use core::future::Future;
use core::str::FromStr;
use core::time::Duration;
use ibc::events::IbcEvent;

use crate::light_client::Verified;
use ibc::ics02_client::client_consensus::{AnyConsensusState, AnyConsensusStateWithHeight};
use ibc::ics02_client::client_state::{AnyClientState, IdentifiedAnyClientState};
use ibc::ics02_client::client_type::ClientType;
use ibc::ics03_connection::connection::{ConnectionEnd, Counterparty, IdentifiedConnectionEnd};
use ibc::ics04_channel::channel::{ChannelEnd, IdentifiedChannelEnd};
use ibc::ics04_channel::error::Error as Ics04Error;
use ibc::ics04_channel::packet::{Packet, PacketMsgType, Receipt, Sequence};
use ibc::ics10_grandpa::client_state::ClientState as GPClientState;
use ibc::ics10_grandpa::consensus_state::ConsensusState as GPConsensusState;
use ibc::ics10_grandpa::header::Header as GPHeader;
use ibc::ics23_commitment::commitment::{CommitmentPrefix, CommitmentRoot};
use ibc::ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId};
use ibc::query::QueryTxRequest;
use ibc::signer::Signer;
use ibc::Height;
use ibc::Height as ICSHeight;
use ibc_proto::ibc::core::channel::v1::{
    PacketState, QueryChannelClientStateRequest, QueryChannelsRequest,
    QueryConnectionChannelsRequest, QueryNextSequenceReceiveRequest,
    QueryPacketAcknowledgementsRequest, QueryPacketCommitmentsRequest, QueryUnreceivedAcksRequest,
    QueryUnreceivedPacketsRequest,
};
use ibc_proto::ibc::core::client::v1::{QueryClientStatesRequest, QueryConsensusStatesRequest};
use ibc_proto::ibc::core::commitment::v1::MerkleProof;
use ibc_proto::ibc::core::connection::v1::{
    QueryClientConnectionsRequest, QueryConnectionsRequest,
};
use octopusxt::ibc_node;
use prost_types::Any;
use std::thread;
use subxt::sp_runtime::generic::Header;
use subxt::sp_runtime::traits::BlakeTwo256;
use subxt::storage::StorageEntry;
use subxt::{BlockNumber, Client, ClientBuilder};
use tendermint::abci::transaction::Hash;
use tendermint::abci::{Code, Log};
use tendermint::account::Id as AccountId;
use tendermint_proto::Protobuf;
use tendermint_rpc::endpoint::broadcast::tx_sync::Response as TxResponse;
use tokio::runtime::Runtime;
use tokio::runtime::Runtime as TokioRuntime;
use tokio::task;
use tokio::time::sleep;

const MAX_QUERY_TIMES: u64 = 100;

#[derive(Debug)]
pub struct SubstrateChain {
    config: ChainConfig,
    websocket_url: String,
    rt: Arc<TokioRuntime>,
}

impl SubstrateChain {
    pub fn config(&self) -> &ChainConfig {
        &self.config
    }

    /// Run a future to completion on the Tokio runtime.
    fn block_on<F: Future>(&self, f: F) -> F::Output {
        crate::time!("block_on");
        self.rt.block_on(f)
    }

    /// Subscribe Events
    async fn subscribe_ibc_events(
        &self,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<Vec<IbcEvent>, Box<dyn std::error::Error>> {
        tracing::info!("In Substrate: [subscribe_ibc_events]");

        octopusxt::subscribe_ibc_event(client).await
    }

    /// get latest height used by subscribe_blocks
    async fn get_latest_height(
        &self,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        tracing::info!("In Substrate: [get_latest_height]");

        octopusxt::get_latest_height(client).await
    }

    /// get connectionEnd according by connection_identifier and read Connections StorageMaps
    async fn get_connection_end(
        &self,
        connection_identifier: &ConnectionId,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<ConnectionEnd, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [get_connection_end]");
        tracing::info!(
            "in Substrate: [get_connection_end] >> connection_id = {:?}",
            connection_identifier
        );

        octopusxt::get_connection_end(connection_identifier, client).await
    }

    /// get channelEnd according by port_identifier, channel_identifier and read Channles StorageMaps
    async fn get_channel_end(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<ChannelEnd, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [get_channel_end]");
        tracing::info!("in Substrate: [get_channel_end] >> port_id = {:?}", port_id);
        tracing::info!(
            "in Substrate: [get_channel_end] >> channel_id = {:?}",
            channel_id
        );

        octopusxt::get_channel_end(port_id, channel_id, client).await
    }

    /// get packet receipt by port_id, channel_id and sequence
    async fn get_packet_receipt(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<Receipt, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [get_packet_receipt]");
        tracing::info!(
            "in Substrate: [get_packet_receipt] >> port_id = {:?}",
            port_id
        );
        tracing::info!(
            "in Substrate: [get_packet_receipt] >> channel_id = {:?}",
            channel_id
        );
        tracing::info!("in Substrate: [get_packet_receipt] >> seq = {:?}", seq);

        octopusxt::get_packet_receipt(port_id, channel_id, seq, client).await
    }

    /// get send packet event by port_id, channel_id and sequence
    async fn get_send_packet_event(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<Packet, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [get_send_packet_event]");
        tracing::info!(
            "in Substrate: [get_send_packet_event] >> port_id = {:?}",
            port_id
        );
        tracing::info!(
            "in Substrate: [get_send_packet_event] >> channel_id = {:?}",
            channel_id
        );
        tracing::info!("in Substrate: [get_send_packet_event] >> seq = {:?}", seq);

        octopusxt::get_send_packet_event(port_id, channel_id, seq, client).await
    }

    /// get client_state according by client_id, and read ClientStates StoraageMap
    async fn get_client_state(
        &self,
        client_id: &ClientId,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<AnyClientState, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [get_client_state]");
        tracing::info!(
            "in Substrate: [get_client_state] >> client_id = {:?}",
            client_id
        );

        octopusxt::get_client_state(client_id, client).await
    }

    /// get appoint height consensus_state according by client_identifier and height
    /// and read ConsensusStates StoreageMap
    async fn get_client_consensus(
        &self,
        client_id: &ClientId,
        height: ICSHeight,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<AnyConsensusState, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [get_client_consensus]");
        tracing::info!(
            "in Substrate: [get_client_consensus] >> client_id = {:?}",
            client_id
        );
        tracing::info!(
            "in Substrate: [get_client_consensus] >> height = {:?}",
            height
        );

        octopusxt::get_client_consensus(client_id, height, client).await
    }

    async fn get_consensus_state_with_height(
        &self,
        client_id: &ClientId,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<Vec<(Height, AnyConsensusState)>, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [get_consensus_state_with_height]");
        tracing::info!(
            "in Substrate: [get_consensus_state_with_height] >> client_id = {:?}",
            client_id
        );

        octopusxt::get_consensus_state_with_height(client_id, client).await
    }

    async fn get_unreceipt_packet(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        seqs: Vec<u64>,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [get_unreceipt_packet]");
        tracing::info!(
            "in Substrate: [get_unreceipt_packet] >> port_id = {:?}",
            port_id
        );
        tracing::info!(
            "in Substrate: [get_unreceipt_packet] >> channel_id = {:?}",
            channel_id
        );
        tracing::info!("in Substrate: [get_unreceipt_packet] >> seqs = {:?}", &seqs);

        octopusxt::get_unreceipt_packet(port_id, channel_id, seqs, client).await
    }

    /// get key-value pair (client_identifier, client_state) construct IdentifieredAnyClientstate
    async fn get_clients(
        &self,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<Vec<IdentifiedAnyClientState>, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [get_clients]");

        octopusxt::get_clients(client).await
    }

    /// get key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
    async fn get_connections(
        &self,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<Vec<IdentifiedConnectionEnd>, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [get_connections]");

        octopusxt::get_connections(client).await
    }

    /// get key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
    async fn get_channels(
        &self,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<Vec<IdentifiedChannelEnd>, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [get_channels]");

        octopusxt::get_channels(client).await
    }

    // get get_commitment_packet_state
    async fn get_commitment_packet_state(
        &self,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<Vec<PacketState>, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [get_commitment_packet_state]");

        octopusxt::get_commitment_packet_state(client).await
    }

    /// get packet commitment by port_id, channel_id and sequence to verify if the ack has been received by the sending chain
    async fn get_packet_commitment(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: u64,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [get_packet_commitment]");
        tracing::info!(
            "in Substrate: [get_packet_commitment] >> port_id = {:?}",
            port_id
        );
        tracing::info!(
            "in Substrate: [get_packet_commitment] >> channel_id = {:?}",
            channel_id
        );
        tracing::info!("in Substrate: [get_packet_commitment] >> seq = {:?}", seq);

        octopusxt::get_packet_commitment(port_id, channel_id, seq, client).await
    }

    // get get_commitment_packet_state
    async fn get_acknowledge_packet_state(
        &self,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<Vec<PacketState>, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [get_acknowledge_packet_state]");

        octopusxt::get_acknowledge_packet_state(client).await
    }

    /// get connection_identifier vector according by client_identifier
    async fn get_client_connections(
        &self,
        client_id: ClientId,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<Vec<ConnectionId>, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [get_client_connections]");
        tracing::info!(
            "in Substrate: [get_client_connections] >> client_id = {:?}",
            client_id
        );

        octopusxt::get_client_connections(client_id, client).await
    }

    async fn get_connection_channels(
        &self,
        connection_id: ConnectionId,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<Vec<IdentifiedChannelEnd>, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [get_connection_channels]");
        tracing::info!(
            "in Substrate: [get_connection_channels] >> connection_id = {:?}",
            connection_id
        );

        octopusxt::get_connection_channels(connection_id, client).await
    }

    async fn deliever(
        &self,
        msg: Vec<Any>,
        client: Client<ibc_node::DefaultConfig>,
    ) -> Result<subxt::sp_core::H256, Box<dyn std::error::Error>> {
        tracing::info!("in Substrate: [deliever]");
        tracing::info!("in Substrate: [deliever] = {:?}", msg);

        octopusxt::deliver(msg, client).await
    }

    /// Retrieve the storage proof according to storage keys
    /// And convert the proof to IBC compatible type
    fn generate_storage_proof<F: StorageEntry>(
        &self,
        storage_entry: &F,
        height: &Height,
    ) -> MerkleProof
    where
        <F as StorageEntry>::Value: serde::Serialize + core::fmt::Debug,
    {
        let generate_storage_proof = async {
            use serde::{Deserialize, Serialize};

            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();

            let _height = NumberOrHex::Number(height.revision_height);
            let block_hash: Option<H256> = client
                .rpc()
                .block_hash(Some(BlockNumber::from(_height)))
                .await
                .unwrap();
            let storage_key = storage_entry.key().final_key(StorageKeyPrefix::new::<F>());
            tracing::debug!("In substrate: [generate_storage_proof] >> height: {:?}, block_hash: {:?}, storage key: {:?}, storage_name = {:?}",
                height, block_hash, storage_key, storage_name);

            use jsonrpsee::types::to_json_value;
            let params = &[
                to_json_value(vec![storage_key]).unwrap(),
                to_json_value(block_hash.unwrap()).unwrap(),
            ];

            #[derive(Debug, PartialEq, Serialize, Deserialize)]
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
                .unwrap();
            tracing::trace!(
                "In Substrate: [generate_storage_proof] >> storage_proof : {:?}",
                storage_proof
            );

            #[derive(Debug, PartialEq, Serialize, Deserialize)]
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
            tracing::trace!(
                "In Substrate: [generate_storage_proof] >> storage_proof_ : {:?}",
                storage_proof_
            );

            let storage_proof_str = serde_json::to_string(&storage_proof_).unwrap();
            tracing::trace!(
                "In Substrate: [generate_storage_proof] >> storage_proof_str: {:?}",
                storage_proof_str
            );
            storage_proof_str
        };

        let storage_proof = self.block_on(generate_storage_proof);
        compose_ibc_merkle_proof(storage_proof)
    }
}

impl ChainEndpoint for SubstrateChain {
    type LightBlock = GPHeader;
    type Header = GPHeader;
    type ConsensusState = AnyConsensusState;
    type ClientState = AnyClientState;
    type LightClient = GPLightClient;

    fn bootstrap(config: ChainConfig, rt: Arc<TokioRuntime>) -> Result<Self, Error> {
        tracing::info!("in Substrate: [bootstrap function]");

        let websocket_url = format!("{}", config.websocket_addr.clone());
        tracing::info!(
            "in Substrate: [bootstrap] websocket_url = {:?}",
            websocket_url
        );

        let chain = Self {
            config,
            websocket_url,
            rt,
        };

        Ok(chain)
    }

    fn init_light_client(&self) -> Result<Self::LightClient, Error> {
        tracing::info!("In Substrate: [init_light_client]");
        use subxt::sp_core::Public;

        let config = self.config.clone();

        let public_key = async {
            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();

            let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

            let authorities = api.storage().beefy().authorities(None).await.unwrap();
            tracing::info!("authorities length : {:?}", authorities.len());
            let result: Vec<String> = authorities
                .into_iter()
                .map(|val| {
                    format!(
                        "0x{}",
                        subxt::sp_core::hexdisplay::HexDisplay::from(&val.to_raw_vec())
                    )
                })
                .collect();
            tracing::info!("authorities member: {:?}", result);
            result
        };
        let public_key = self.block_on(public_key);

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
    ) -> Result<(EventReceiver, TxMonitorCmd), Error> {
        tracing::info!("in Substrate: [init_event_mointor]");

        tracing::info!(
            "In Substrate: [init_event_mointor] >> websocket addr: {:?}",
            self.config.websocket_addr.clone()
        );

        let (mut event_monitor, event_receiver, monitor_tx) = EventMonitor::new(
            self.config.id.clone(),
            self.config.websocket_addr.clone(),
            rt,
        )
        .map_err(Error::event_monitor)?;

        event_monitor.subscribe().map_err(Error::event_monitor)?;

        thread::spawn(move || event_monitor.run());

        Ok((event_receiver, monitor_tx))
    }

    fn shutdown(self) -> Result<(), Error> {
        tracing::info!("in Substrate: [shutdown]");

        Ok(())
    }

    fn health_check(&self) -> Result<HealthCheck, Error> {
        Ok(HealthCheck::Healthy)
    }

    fn id(&self) -> &ChainId {
        tracing::info!("in Substrate: [id]");

        &self.config().id
    }

    fn keybase(&self) -> &KeyRing {
        tracing::info!("in Substrate: [keybase]");

        todo!()
    }

    fn keybase_mut(&mut self) -> &mut KeyRing {
        tracing::info!("in Substrate: [keybase_mut]");

        todo!()
    }

    fn send_messages_and_wait_commit(
        &mut self,
        proto_msgs: Vec<Any>,
    ) -> Result<Vec<IbcEvent>, Error> {
        tracing::info!("in Substrate: [send_messages_and_wait_commit]");
        tracing::info!(
            "in Substrate: proto_msg: {:?}",
            proto_msgs.first().unwrap().type_url
        );

        let client = async {
            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();

            // sleep(Duration::from_secs(4)).await;

            let result = self.deliever(proto_msgs, client).await.unwrap();

            tracing::info!(
                "in Substrate: [send_messages_and_wait_commit] >> result : {:?}",
                result
            );

            result
        };

        let _ = self.block_on(client);

        let get_ibc_event = async {
            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();

            let result = self.subscribe_ibc_events(client).await.unwrap();
            for event in result.iter() {
                tracing::info!(
                    "In Substrate: [send_messages_and_wait_commit] >> get_ibc_event: {:?}",
                    event
                );
            }

            result
        };

        let ret = self.block_on(get_ibc_event);
        Ok(ret)
    }

    fn send_messages_and_wait_check_tx(
        &mut self,
        proto_msgs: Vec<Any>,
    ) -> Result<Vec<TxResponse>, Error> {
        tracing::info!("in Substrate: [send_messages_and_wait_check_tx]");
        tracing::debug!(
            "in Substrate: [send_messages_and_wait_check_tx], raw msg to send {:?}",
            proto_msgs
        );

        let client = async {
            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();

            // sleep(Duration::from_secs(4)).await;

            let result = self.deliever(proto_msgs, client).await.unwrap();

            tracing::info!(
                "in Substrate: [send_messages_and_wait_check_tx] >> result : {:?}",
                result
            );

            result
        };

        let _result = self.block_on(client);

        use tendermint::abci::transaction; // Todo:
        let json = "\"ChYKFGNvbm5lY3Rpb25fb3Blbl9pbml0\"";
        let txRe = TxResponse {
            code: Code::default(),
            data: serde_json::from_str(json).unwrap(),
            log: Log::from("testtest"),
            hash: transaction::Hash::new(*_result.as_fixed_bytes()),
        };

        Ok(vec![txRe])
    }

    fn get_signer(&mut self) -> Result<Signer, Error> {
        tracing::info!("in Substrate: [get_signer]");
        tracing::info!(
            "In Substraet: [get signer] >> key_name: {:?}",
            self.config.key_name.clone()
        );

        fn get_dummy_account_id_raw() -> String {
            "0CDA3F47EF3C4906693B170EF650EB968C5F4B2C".to_string()
        }

        pub fn get_dummy_account_id() -> AccountId {
            AccountId::from_str(&get_dummy_account_id_raw()).unwrap()
        }

        let signer = Signer::new(get_dummy_account_id().to_string());
        tracing::info!("in Substrate: [get_signer] >>  signer {:?}", signer);

        Ok(signer)
    }

    fn get_key(&mut self) -> Result<KeyEntry, Error> {
        tracing::info!("in Substrate: [get_key]");

        todo!()
    }

    fn query_commitment_prefix(&self) -> Result<CommitmentPrefix, Error> {
        tracing::info!("in Substrate: [query_commitment_prefix]");

        // TODO - do a real chain query
        Ok(CommitmentPrefix::from(
            self.config().store_prefix.as_bytes().to_vec(),
        ))
    }

    fn query_latest_height(&self) -> Result<ICSHeight, Error> {
        tracing::info!("in Substrate: [query_latest_height]");

        let result = retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries");
            }

            let result = async {
                let client = ClientBuilder::new()
                    .set_url(&self.websocket_url.clone())
                    .build::<ibc_node::DefaultConfig>()
                    .await
                    .unwrap();
                self.get_latest_height(client).await
            };

            match self.block_on(result) {
                Ok(v) => RetryResult::Ok(v),
                Err(e) => RetryResult::Retry("Fail to retry"),
            }
        });

        let latest_height = Height::new(0, result.unwrap());
        Ok(latest_height)
    }

    fn query_clients(
        &self,
        request: QueryClientStatesRequest,
    ) -> Result<Vec<IdentifiedAnyClientState>, Error> {
        tracing::info!("in Substrate: [query_clients]");
        tracing::info!("in Substrate: [query_clients] >> request = {:?}", request);

        let result = retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries");
            }

            let result = async {
                let client = ClientBuilder::new()
                    .set_url(&self.websocket_url.clone())
                    .build::<ibc_node::DefaultConfig>()
                    .await
                    .unwrap();

                self.get_clients(client).await
            };

            match self.block_on(result) {
                Ok(v) => RetryResult::Ok(v),
                Err(e) => RetryResult::Retry("Fail to retry"),
            }
        });

        tracing::info!("in Substrate: [query_clients] >> clients: {:?}", result);

        Ok(result.unwrap())
    }

    fn query_client_state(
        &self,
        client_id: &ClientId,
        height: ICSHeight,
    ) -> Result<Self::ClientState, Error> {
        tracing::info!("in Substrate: [query_client_state]");
        tracing::info!(
            "in Substrate: [query_client_state] >> client_id = {:?}",
            client_id
        );
        tracing::info!(
            "in Substrate: [query_client_state] >> height = {:?}",
            height
        );

        let result = retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries");
            }

            let result = async {
                let client = ClientBuilder::new()
                    .set_url(&self.websocket_url.clone())
                    .build::<ibc_node::DefaultConfig>()
                    .await
                    .unwrap();
                self.get_client_state(client_id, client).await
            };

            match self.block_on(result) {
                Ok(v) => RetryResult::Ok(v),
                Err(e) => RetryResult::Retry("Fail to retry"),
            }
        });

        tracing::info!(
            "in Substrate: [query_client_state] >> client_state: {:?}",
            result
        );

        Ok(result.unwrap())
    }

    fn query_consensus_states(
        &self,
        request: QueryConsensusStatesRequest,
    ) -> Result<Vec<AnyConsensusStateWithHeight>, Error> {
        tracing::info!("in Substrate: [query_consensus_states]");
        tracing::info!(
            "in Substrate: [query_consensus_states] >> request = {:?}",
            request
        );
        let request_client_id = ClientId::from_str(request.client_id.as_str()).unwrap();

        let result = retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries");
            }

            let result = async {
                let client = ClientBuilder::new()
                    .set_url(&self.websocket_url.clone())
                    .build::<ibc_node::DefaultConfig>()
                    .await
                    .unwrap();

                self.get_consensus_state_with_height(&request_client_id, client)
                    .await
            };

            match self.block_on(result) {
                Ok(v) => RetryResult::Ok(v),
                Err(e) => RetryResult::Retry("Fail to retry"),
            }
        });

        let consensus_state: Vec<(Height, AnyConsensusState)> = result.unwrap();

        let mut any_consensus_state_with_height = vec![];
        for (height, consensus_state) in consensus_state.into_iter() {
            let tmp = AnyConsensusStateWithHeight {
                height: height,
                consensus_state,
            };
            any_consensus_state_with_height.push(tmp.clone());

            tracing::info!(
                "In Substrate: [query_consensus_state] >> any_consensus_state_with_height: {:?}",
                tmp
            );
        }

        any_consensus_state_with_height.sort_by(|a, b| a.height.cmp(&b.height));

        Ok(any_consensus_state_with_height)
    }

    fn query_consensus_state(
        &self,
        client_id: ClientId,
        consensus_height: ICSHeight,
        query_height: ICSHeight,
    ) -> Result<AnyConsensusState, Error> {
        tracing::info!("in Substrate: [query_consensus_state]");
        tracing::info!(
            "in Substrate: [query_consensus_state] >> client_id = {:?}",
            client_id
        );
        tracing::info!(
            "in Substrate: [query_consensus_state] >> consensus_height = {:?}",
            consensus_height
        );
        tracing::info!(
            "in Substrate: [query_consensus_state] >> query_height = {:?}",
            query_height
        );

        let consensus_state = self
            .proven_client_consensus(&client_id, consensus_height, query_height)?
            .0;

        Ok(consensus_state)
    }

    fn query_upgraded_client_state(
        &self,
        height: ICSHeight,
    ) -> Result<(Self::ClientState, MerkleProof), Error> {
        tracing::info!("in Substrate: [query_upgraded_client_state]");
        tracing::info!(
            "in Substrate: [query_upgraded_client_state] >> height = {:?}",
            height
        );

        todo!()
    }

    fn query_upgraded_consensus_state(
        &self,
        height: ICSHeight,
    ) -> Result<(Self::ConsensusState, MerkleProof), Error> {
        tracing::info!("in Substrate: [query_upgraded_consensus_state]");
        tracing::info!(
            "in Substrate: [query_upgraded_consensus_state] >> height = {:?}",
            height
        );

        todo!()
    }

    fn query_connections(
        &self,
        request: QueryConnectionsRequest,
    ) -> Result<Vec<IdentifiedConnectionEnd>, Error> {
        tracing::info!("in Substrate: [query_connections]");
        tracing::info!(
            "in Substrate: [query_connections] >> request = {:?}",
            request
        );

        let result = retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries");
            }

            let result = async {
                let client = ClientBuilder::new()
                    .set_url(&self.websocket_url.clone())
                    .build::<ibc_node::DefaultConfig>()
                    .await
                    .unwrap();

                let connections = self.get_connections(client).await;

                connections
            };

            match self.block_on(result) {
                Ok(v) => RetryResult::Ok(v),
                Err(e) => RetryResult::Retry("Fail to retry"),
            }
        });

        tracing::info!("in Substrate: [query_connections] >> clients: {:?}", result);

        Ok(result.unwrap())
    }

    fn query_client_connections(
        &self,
        request: QueryClientConnectionsRequest,
    ) -> Result<Vec<ConnectionId>, Error> {
        tracing::info!("in substrate: [query_client_connections]");
        tracing::info!(
            "in substrate: [query_client_connections] >> request = {:?}",
            request
        );

        let result = retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries");
            }

            let client_id = ClientId::from_str(request.client_id.as_str()).unwrap();

            let result = async {
                let client = ClientBuilder::new()
                    .set_url(&self.websocket_url.clone())
                    .build::<ibc_node::DefaultConfig>()
                    .await
                    .unwrap();

                self.get_client_connections(client_id, client).await
            };

            match self.block_on(result) {
                Ok(v) => RetryResult::Ok(v),
                Err(e) => RetryResult::Retry("Fail to retry"),
            }
        });

        tracing::info!(
            "In substrate: [query_client_connections] >> client_connections: {:#?}",
            result
        );

        Ok(result.unwrap())
    }

    // TODO fo substrate
    fn query_connection(
        &self,
        connection_id: &ConnectionId,
        height: ICSHeight,
    ) -> Result<ConnectionEnd, Error> {
        tracing::info!("in Substrate: [query_connection]");
        tracing::info!(
            "in Substrate: [query_connection] >> connection_id = {:?}",
            connection_id
        );
        tracing::info!("in Substrate: [query_connection] >> height = {:?}", height);

        let connection_end = async {
            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();

            sleep(Duration::from_secs(10)).await;

            let connection_end = self
                .get_connection_end(connection_id, client)
                .await
                .unwrap();

            connection_end
        };

        let connection_end = self.block_on(connection_end);

        tracing::info!(
            "In Substrate: [query_connection] \
                >> connection_id: {:?}, connection_end: {:?}",
            connection_id,
            connection_end
        );

        Ok(connection_end)
    }

    fn query_connection_channels(
        &self,
        request: QueryConnectionChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        tracing::info!("in substrate: [query_connection_channels]");
        tracing::info!(
            "in substrate: [query_connection_channels] >> request = {:?}",
            request
        );

        let result = retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries");
            }

            let connection_id = ConnectionId::from_str(&request.connection).unwrap();

            let result = async {
                let client = ClientBuilder::new()
                    .set_url(&self.websocket_url.clone())
                    .build::<ibc_node::DefaultConfig>()
                    .await
                    .unwrap();

                self.get_connection_channels(connection_id, client).await
            };

            match self.block_on(result) {
                Ok(v) => RetryResult::Ok(v),
                Err(e) => RetryResult::Retry("Fail to retry"),
            }
        });

        tracing::info!(
            "In substrate: [query_connection_channels] >> connection_channels: {:?}",
            result
        );

        Ok(result.unwrap())
    }

    fn query_channels(
        &self,
        request: QueryChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        tracing::info!("in Substrate: [query_channels]");
        tracing::info!("in Substrate: [query_channels] >> request = {:?}", request);

        let result = retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries");
            }
            let result = async {
                let client = ClientBuilder::new()
                    .set_url(&self.websocket_url.clone())
                    .build::<ibc_node::DefaultConfig>()
                    .await
                    .unwrap();

                let channels = self.get_channels(client).await;

                channels
            };

            match self.block_on(result) {
                Ok(v) => RetryResult::Ok(v),
                Err(e) => RetryResult::Retry("Fail to retry"),
            }
        });

        tracing::info!("in Substrate: [query_connections] >> clients: {:?}", result);

        Ok(result.unwrap())
    }

    // todo for substrate
    fn query_channel(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        height: ICSHeight,
    ) -> Result<ChannelEnd, Error> {
        tracing::info!("in Substrate: [query_channel]");
        tracing::info!("in Substrate: [query_channel] >> port_id = {:?}", port_id);
        tracing::info!(
            "in Substrate: [query_channel] >> channel_id = {:?}",
            channel_id
        );
        tracing::info!("in Substrate: [query_channel] >> height = {:?}", height);

        let channel_end = async {
            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();

            sleep(Duration::from_secs(10)).await;

            let channel_end = self
                .get_channel_end(port_id, channel_id, client)
                .await
                .unwrap();
            tracing::info!(
                "In Substrate: [query_channel] \
                >> port_id: {:?}, channel_id: {:?}, channel_end: {:?}",
                port_id,
                channel_id,
                channel_end
            );

            channel_end
        };

        let channel_end = self.block_on(channel_end);

        Ok(channel_end)
    }

    fn query_channel_client_state(
        &self,
        request: QueryChannelClientStateRequest,
    ) -> Result<Option<IdentifiedAnyClientState>, Error> {
        tracing::info!("in Substrate: [query_channel_client_state]");
        tracing::info!(
            "in Substrate: [query_channel_client_state] >> request = {:?}",
            request
        );

        todo!()
    }

    fn query_packet_commitments(
        &self,
        request: QueryPacketCommitmentsRequest,
    ) -> Result<(Vec<PacketState>, ICSHeight), Error> {
        tracing::info!("in Substrate: [query_packet_commitments]");
        tracing::info!(
            "in Substrate: [query_packet_commitments] >> request = {:?}",
            request
        );

        let result = retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries");
            }

            let result = async {
                let client = ClientBuilder::new()
                    .set_url(&self.websocket_url.clone())
                    .build::<ibc_node::DefaultConfig>()
                    .await
                    .unwrap();

                self.get_commitment_packet_state(client).await
            };

            match self.block_on(result) {
                Ok(v) => RetryResult::Ok(v),
                Err(e) => RetryResult::Retry("Fail to retry"),
            }
        });

        let last_height = self.query_latest_height().unwrap();

        Ok((result.unwrap(), last_height))
    }

    fn query_unreceived_packets(
        &self,
        request: QueryUnreceivedPacketsRequest,
    ) -> Result<Vec<u64>, Error> {
        tracing::info!("in Substrate: [query_unreceived_packets]");
        tracing::info!(
            "in Substrate: [query_unreceived_packets] >> request = {:?}",
            request
        );

        let result = retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries");
            }

            let port_id = PortId::from_str(request.port_id.as_str()).unwrap();
            let channel_id = ChannelId::from_str(request.channel_id.as_str()).unwrap();
            let seqs = request.packet_commitment_sequences.clone();

            let result = async {
                let client = ClientBuilder::new()
                    .set_url(&self.websocket_url.clone())
                    .build::<ibc_node::DefaultConfig>()
                    .await
                    .unwrap();

                self.get_unreceipt_packet(&port_id, &channel_id, seqs, client)
                    .await
            };

            match self.block_on(result) {
                Ok(v) => RetryResult::Ok(v),
                Err(e) => RetryResult::Retry("Fail to retry"),
            }
        });

        Ok(result.unwrap())
    }

    fn query_packet_acknowledgements(
        &self,
        request: QueryPacketAcknowledgementsRequest,
    ) -> Result<(Vec<PacketState>, ICSHeight), Error> {
        tracing::info!("in Substrate: [query_packet_acknowledgements]");
        tracing::info!(
            "in Substrate: [query_packet_acknowledgements] >> request = {:?}",
            request
        );

        let result = retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries");
            }

            let result = async {
                let client = ClientBuilder::new()
                    .set_url(&self.websocket_url.clone())
                    .build::<ibc_node::DefaultConfig>()
                    .await
                    .unwrap();

                self.get_acknowledge_packet_state(client).await
            };

            match self.block_on(result) {
                Ok(v) => RetryResult::Ok(v),
                Err(e) => RetryResult::Retry("Fail to retry"),
            }
        });

        let last_height = self.query_latest_height().unwrap();

        Ok((result.unwrap(), last_height))
    }

    fn query_unreceived_acknowledgements(
        &self,
        request: QueryUnreceivedAcksRequest,
    ) -> Result<Vec<u64>, Error> {
        tracing::info!("in Substrate: [query_unreceived_acknowledgements]");
        tracing::info!(
            "in Substrate: [query_unreceived_acknowledgements] >> request = {:?}",
            request
        );

        let port_id = PortId::from_str(request.port_id.as_str()).unwrap();
        let channel_id = ChannelId::from_str(request.channel_id.as_str()).unwrap();
        let seqs = request.packet_ack_sequences.clone();

        let unreceived_seqs = async {
            let mut unreceived_seqs: Vec<u64> = vec![];

            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();

            for _seq in seqs {
                let _cmt = self
                    .get_packet_commitment(&port_id, &channel_id, _seq, client.clone())
                    .await;

                // if packet commitment still exists on the original sending chain, then packet ack is unreceived
                // since processing the ack will delete the packet commitment
                match _cmt {
                    Ok(_) => {
                        unreceived_seqs.push(_seq);
                    }
                    Err(_) => {}
                }
            }

            unreceived_seqs
        };

        let result = self.block_on(unreceived_seqs);
        Ok(result)
    }

    fn query_next_sequence_receive(
        &self,
        request: QueryNextSequenceReceiveRequest,
    ) -> Result<Sequence, Error> {
        tracing::info!("in Substrate: [query_next_sequence_receive]");
        tracing::info!(
            "in Substrate: [query_next_sequence_receive] >> request = {:?}",
            request
        );

        todo!()
    }

    fn query_txs(&self, request: QueryTxRequest) -> Result<Vec<IbcEvent>, Error> {
        tracing::info!("in Substrate: [query_txs]");
        tracing::info!("in Substrate: [query_txs] >> request: {:?}", request);

        match request {
            QueryTxRequest::Packet(request) => {
                crate::time!("in Substrate: [query_txs]: query packet events");

                let mut result: Vec<IbcEvent> = vec![];
                if request.sequences.is_empty() {
                    return Ok(result);
                }

                tracing::info!(
                    "in Substrate: [query_txs]: query packet events request: {:?}",
                    request
                );
                tracing::debug!(
                    "in Substrate: [query_txs]: packet >> sequence :{:?}",
                    request.sequences
                );

                return Ok(result);

                // Todo: Related to https://github.com/octopus-network/ibc-rs/issues/88
                // To query to event by event type, sequence number and block height
                // use ibc::ics02_client::events::Attributes;
                /*                use ibc::ics04_channel::events::Attributes;
                use ibc::ics02_client::header::AnyHeader;
                use core::ops::Add;

                result.push(IbcEvent::SendPacket(ibc::ics04_channel::events::SendPacket{
                    height: request.height,
                    packet: Packet{
                        sequence: request.sequences[0],
                        source_port: request.source_port_id,
                        source_channel: request.source_channel_id,
                        destination_port: request.destination_port_id,
                        destination_channel: request.destination_channel_id,
                        data: vec![1,3,5],  //Todo
                        timeout_height: ibc::Height::zero().add( 9999999), //Todo
                        timeout_timestamp: ibc::timestamp::Timestamp::from_nanoseconds(Utc::now().timestamp_nanos() as u64)
                            .unwrap().add(Duration::from_secs(99999)).unwrap() //Todo
                    }
                }));

                Ok(result)*/
            }

            QueryTxRequest::Client(request) => {
                crate::time!("in Substrate: [query_txs]: single client update event");
                tracing::info!(
                    "in Substrate: [query_txs]: single client update event: request:{:?}",
                    request
                );

                // query the first Tx that includes the event matching the client request
                // Note: it is possible to have multiple Tx-es for same client and consensus height.
                // In this case it must be true that the client updates were performed with tha
                // same header as the first one, otherwise a subsequent transaction would have
                // failed on chain. Therefore only one Tx is of interest and current API returns
                // the first one.
                // let mut response = self
                //     .block_on(self.rpc_client.tx_search(
                //         header_query(&request),
                //         false,
                //         1,
                //         1, // get only the first Tx matching the query
                //         Order::Ascending,
                //     ))
                //     .map_err(|e| Error::rpc(self.config.rpc_addr.clone(), e))?;
                //
                // if response.txs.is_empty() {
                //     return Ok(vec![]);
                // }
                //
                // // the response must include a single Tx as specified in the query.
                // assert!(
                //     response.txs.len() <= 1,
                //     "packet_from_tx_search_response: unexpected number of txs"
                // );
                //
                // let tx = response.txs.remove(0);
                // let event = update_client_from_tx_search_response(self.id(), &request, tx);
                use ibc::ics02_client::events::Attributes;
                use ibc::ics02_client::header::AnyHeader;

                // Todo: the client event below is mock
                // replace it with real client event replied from a Substrate chain
                let mut result: Vec<IbcEvent> = vec![];
                result.push(IbcEvent::UpdateClient(
                    ibc::ics02_client::events::UpdateClient {
                        common: Attributes {
                            height: request.height,
                            client_id: request.client_id,
                            client_type: ClientType::Tendermint,
                            consensus_height: request.consensus_height,
                        },
                        header: Some(AnyHeader::Tendermint(get_dummy_ics07_header())),
                    },
                ));

                Ok(result)
                // Ok(event.into_iter().collect())
            }

            QueryTxRequest::Transaction(tx) => {
                crate::time!("in Substrate: [query_txs]: Transaction");
                tracing::info!("in Substrate: [query_txs]: Transaction: {:?}", tx);
                // Todo: https://github.com/octopus-network/ibc-rs/issues/98
                let mut result: Vec<IbcEvent> = vec![];
                Ok(result)
            }
        }
    }

    fn proven_client_state(
        &self,
        client_id: &ClientId,
        height: ICSHeight,
    ) -> Result<(Self::ClientState, MerkleProof), Error> {
        tracing::info!("in Substrate: [proven_client_state]");
        tracing::info!(
            "in Substrate: [proven_client_state] >> client_id = {:?}",
            client_id
        );
        tracing::info!(
            "in Substrate: [proven_client_state] >> height = {:?}",
            height
        );

        let result = retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries");
            }

            let result = async {
                let client = ClientBuilder::new()
                    .set_url(&self.websocket_url.clone())
                    .build::<ibc_node::DefaultConfig>()
                    .await
                    .unwrap();

                self.get_client_state(client_id, client).await
            };

            match self.block_on(result) {
                Ok(v) => RetryResult::Ok(v),
                Err(e) => RetryResult::Retry("Fail to retry"),
            }
        });

        tracing::info!(
            "In Substrate: [proven_client_state] \
                >> client_state : {:?}",
            result
        );

        let storage_entry = ibc_node::ibc::storage::ClientStates(client_id.as_bytes().to_vec());
        Ok((client_state, self.generate_storage_proof(&storage_entry, &height, "ClientStates")))
    }

    fn proven_connection(
        &self,
        connection_id: &ConnectionId,
        height: ICSHeight,
    ) -> Result<(ConnectionEnd, MerkleProof), Error> {
        tracing::info!("in Substrate: [proven_connection]");
        tracing::info!(
            "in Substrate: [proven_connection] >> connection_id = {:?}",
            connection_id
        );
        tracing::info!("in Substrate: [proven_connection] >> height = {:?}", height);

        let result = retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries");
            }

            let result = async {
                let client = ClientBuilder::new()
                    .set_url(&self.websocket_url.clone())
                    .build::<ibc_node::DefaultConfig>()
                    .await
                    .unwrap();

                self.get_connection_end(connection_id, client).await
            };
            match self.block_on(result) {
                Ok(v) => RetryResult::Ok(v),
                Err(e) => RetryResult::Retry("Fail to retry"),
            }
        });

        tracing::info!(
            "In Substrate: [proven_connection] \
                >> connection_end: {:?}",
            result
        );

        let connection_end = result.unwrap();

        let mut new_connection_end;

        if connection_end
            .counterparty()
            .clone()
            .connection_id
            .is_none()
        {
            // 构造 Counterparty
            let client_id = connection_end.counterparty().client_id().clone();
            let prefix = connection_end.counterparty().prefix().clone();
            let temp_connection_id = Some(connection_id.clone());

            let counterparty = Counterparty::new(client_id, temp_connection_id, prefix);
            let state = connection_end.state;
            let client_id = connection_end.client_id().clone();
            let versions = connection_end.versions().clone();
            let delay_period = connection_end.delay_period().clone();

            new_connection_end =
                ConnectionEnd::new(state, client_id, counterparty, versions, delay_period);
        } else {
            new_connection_end = connection_end;
        }

        let storage_entry = ibc_node::ibc::storage::Connections(connection_id.as_bytes().to_vec());

        Ok((new_connection_end, self.generate_storage_proof(&storage_entry, &height, "Connections")))
    }

    fn proven_client_consensus(
        &self,
        client_id: &ClientId,
        consensus_height: ICSHeight,
        height: ICSHeight,
    ) -> Result<(Self::ConsensusState, MerkleProof), Error> {
        tracing::info!("in Substrate: [proven_client_consensus]");
        tracing::info!(
            "in Substrate: [proven_client_consensus] >> client_id = {:?}",
            client_id
        );
        tracing::info!(
            "in Substrate: [proven_client_consensus] >> consensus_height = {:?}",
            consensus_height
        );
        tracing::info!(
            "in Substrate: [proven_client_consensus] >> height = {:?}",
            height
        );

        let result = retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries");
            }

            let result = async {
                let client = ClientBuilder::new()
                    .set_url(&self.websocket_url.clone())
                    .build::<ibc_node::DefaultConfig>()
                    .await
                    .unwrap();

                self.get_client_consensus(client_id, consensus_height, client)
                    .await
            };
            match self.block_on(result) {
                Ok(v) => RetryResult::Ok(v),
                Err(e) => RetryResult::Retry("Fail to retry"),
            }
        });

        tracing::info!(
            "In Substrate: [proven_client_consensus] \
                >> consensus_state : {:?}",
            result
        );

        let storage_entry = ibc_node::ibc::storage::ConsensusStates(client_id.as_bytes().to_vec());
        Ok((consensus_state, self.generate_storage_proof(&storage_entry, &consensus_height, "ConsensusStates")))
    }

    fn proven_channel(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        height: ICSHeight,
    ) -> Result<(ChannelEnd, MerkleProof), Error> {
        tracing::info!("in Substrate: [proven_channel]");
        tracing::info!("in Substrate: [proven_channel] >> port_id = {:?}", port_id);
        tracing::info!(
            "in Substrate: [proven_channel] >> channel_id = {:?}",
            channel_id
        );
        tracing::info!("in Substrate: [proven_channel] >> height = {:?}", height);

        let result = retry_with_index(Fixed::from_millis(100), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries");
            }

            let result = async {
                let client = ClientBuilder::new()
                    .set_url(&self.websocket_url.clone())
                    .build::<ibc_node::DefaultConfig>()
                    .await
                    .unwrap();

                self.get_channel_end(port_id, channel_id, client).await
            };

            match self.block_on(result) {
                Ok(v) => RetryResult::Ok(v),
                Err(e) => RetryResult::Retry("Fail to retry"),
            }
        });

        tracing::info!(
            "In Substrate: [query_channel] \
                >> port_id: {:?}, channel_id: {:?}, channel_end: {:?}",
            port_id,
            channel_id,
            result
        );
        
        let storage_entry = ibc_node::ibc::storage::Channels(port_id.as_bytes().to_vec(), channel_id.as_bytes().to_vec());
        Ok((channel_end, self.generate_storage_proof(&storage_entry, &height, "Channels")))
    }

    fn proven_packet(
        &self,
        packet_type: PacketMsgType,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
        height: ICSHeight,
    ) -> Result<(Vec<u8>, MerkleProof), Error> {
        tracing::info!("in Substrate: [proven_packet]");
        tracing::info!(
            "in Substrate: [proven_packet] >> paket_type = {:?}",
            packet_type
        );
        tracing::info!("in Substrate: [proven_packet] >> port_id = {:?}", port_id);
        tracing::info!(
            "in Substrate: [proven_packet] >> channel_id = {:?}",
            channel_id
        );
        tracing::info!("in Substrate: [proven_packet] >> sequence = {:?}", sequence);
        tracing::info!("in Substrate: [proven_packet] >> height = {:?}", height);

        let packet_result = async {
            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();

            sleep(Duration::from_secs(4)).await;

            let packet_result:Vec<u8> = match packet_type {
                PacketMsgType::Recv => {
                    let data = octopusxt::call_ibc::get_packet_commitment(&port_id, &channel_id, sequence.0, client).await.unwrap();
                    tracing::info!(
                        "In Substrate: [proven_packet] >> packet_commitment: {:?}", String::from_utf8(data.clone()).unwrap()
                    );
                    data
                }
                PacketMsgType::Ack => {
                    let data = octopusxt::call_ibc::get_packet_ack(&port_id, &channel_id, sequence.0, client).await.unwrap();
                    tracing::info!(
                        "In Substrate: [proven_packet] >> packet_ack: {:?}", String::from_utf8(data.clone()).unwrap()
                    );
                    data
                }
                _ => unimplemented!(),
            };

            packet_result
        };

        let packet_result = self.block_on(packet_result);

        match packet_type {
            PacketMsgType::Recv => {
                let storage_entry = ibc_node::ibc::storage::PacketCommitment(port_id.as_bytes().to_vec(), channel_id.as_bytes().to_vec(), u64::from(sequence.clone()).encode());
                Ok((packet_result, self.generate_storage_proof(&storage_entry, &height, "PacketCommitment")))
            }
            PacketMsgType::Ack => {
                let storage_entry = ibc_node::ibc::storage::Acknowledgements(port_id.as_bytes().to_vec(), channel_id.as_bytes().to_vec(), u64::from(sequence.clone()).encode());
                Ok((packet_result, self.generate_storage_proof(&storage_entry, &height, "Acknowledgements")))
            }
            _ => unimplemented!(),
        }
    }

    fn build_client_state(&self, height: ICSHeight) -> Result<Self::ClientState, Error> {
        tracing::info!("in Substrate: [build_client_state]");
        tracing::info!(
            "in Substrate: [build_client_state] >> height = {:?}",
            height
        );

        use ibc::ics10_grandpa::help::Commitment;
        use sp_core::Public;

        let public_key = async {
            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();

            let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

            let authorities = api.storage().beefy().authorities(None).await.unwrap();
            tracing::info!("authorities length : {:?}", authorities.len());
            let result: Vec<String> = authorities
                .into_iter()
                .map(|val| {
                    format!(
                        "0x{}",
                        subxt::sp_core::hexdisplay::HexDisplay::from(&val.to_raw_vec())
                    )
                })
                .collect();
            tracing::info!("authorities member: {:?}", result);
            result
        };
        let public_key = self.block_on(public_key);

        let beefy_light_client = beefy_light_client::new(public_key);

        // Build client state
        let client_state = GPClientState::new(
            self.id().clone(),
            height.revision_height as u32,
            Height::new(0, 0), // set frozen_height is zero height
            BlockHeader::default(),
            Commitment::default(),
            beefy_light_client.validator_set.into(),
        )
        .map_err(Error::ics10)?;

        tracing::info!(
            "in Substrate: [build_client_state] >> client_state: {:?}",
            client_state.clone()
        );

        Ok(AnyClientState::Grandpa(client_state))
    }

    fn build_consensus_state(
        &self,
        light_block: Self::LightBlock,
    ) -> Result<Self::ConsensusState, Error> {
        tracing::info!("in Substrate: [build_consensus_state]");

        tracing::info!(
            "in Substrate: [build_consensus_state] >> Any consensus state = {:?}",
            AnyConsensusState::Grandpa(GPConsensusState::from(light_block.clone()))
        );

        Ok(AnyConsensusState::Grandpa(GPConsensusState::from(
            light_block,
        )))
    }

    fn build_header(
        &self,
        trusted_height: ICSHeight,
        target_height: ICSHeight,
        client_state: &AnyClientState,
        light_client: &mut Self::LightClient,
    ) -> Result<(Self::Header, Vec<Self::Header>), Error> {
        tracing::info!("in Substrate: [build_header]");
        tracing::info!("in Substrate: [build_header] >> Trusted_height: {:?}, Target_height: {:?}, client_state: {:?}",
            trusted_height, target_height, client_state);

        assert!(trusted_height.revision_height < target_height.revision_height);

        let grandpa_client_state = match client_state {
            // 73
            AnyClientState::Grandpa(state) => state,
            _ => todo!(),
        };

        // assert trust_height <= grandpa_client_state height
        if trusted_height.revision_height > grandpa_client_state.block_number as u64 {
            return Err(Error::trust_height_miss_match_client_state_height(
                trusted_height.revision_height,
                grandpa_client_state.block_number as u64,
            ));
        }

        // build target height header
        let result = async {
            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();

            let beefy_light_client::commitment::Commitment {
                payload,
                block_number,
                validator_set_id,
            } = grandpa_client_state.latest_commitment.clone().into();

            // get commitment
            // let mut mmr_root_height = signed_commitment.commitment.block_number;
            let mut mmr_root_height = block_number; // 73

            // assert eq mmr_root_height target_height.reversion_height
            assert!((target_height.revision_height as u32) < mmr_root_height);

            // get block header
            let block_header = octopusxt::call_ibc::get_header_by_block_number(
                client.clone(),
                Some(BlockNumber::from(target_height.revision_height as u32)), // 66
            )
            .await
            .unwrap();

            let api = client
                .clone()
                .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

            // assert block_header.block_number == target_height
            assert_eq!(
                block_header.block_number,
                target_height.revision_height as u32
            );

            tracing::info!(
                "in Substrate: [build_header] >> mmr_root_height = {:?}, target_height = {:?}",
                mmr_root_height,
                target_height
            );

            // block hash by block number
            let block_hash: Option<sp_core::H256> = api // 73
                .client
                .rpc()
                .block_hash(Some(BlockNumber::from(
                    mmr_root_height, // 73
                )))
                .await
                .unwrap();

            // get mmr_leaf and mmr_leaf_proof
            let mmr_leaf_and_mmr_leaf_proof = octopusxt::call_ibc::get_mmr_leaf_and_mmr_proof(
                target_height.revision_height, // 66
                block_hash,                    // 73
                client,
            )
            .await
            .unwrap();
            // 65
            (block_header, mmr_leaf_and_mmr_leaf_proof)
        };

        let result = self.block_on(result);
        tracing::info!(
            "in substrate [build header] >> block header = {:?}",
            result.0
        );
        tracing::info!(
            "in substrate [build header] >> mmr_leaf = {:?}",
            result.1 .0
        );
        tracing::info!(
            "in substrate [build header] >> mmr_leaf_proof = {:?}",
            result.1 .1
        );

        let mut encoded_mmr_leaf = result.1 .1;
        let mut encoded_mmr_leaf_proof = result.1 .2;

        let leaf: Vec<u8> = Decode::decode(&mut &encoded_mmr_leaf[..]).unwrap();
        let mmr_leaf: beefy_light_client::mmr::MmrLeaf = Decode::decode(&mut &*leaf).unwrap();
        tracing::info!(
            "in substrate [build header] >> beef_light_client MmrLeaf = {:?}",
            mmr_leaf.clone()
        );

        let mmr_leaf_proof =
            beefy_light_client::mmr::MmrLeafProof::decode(&mut &encoded_mmr_leaf_proof[..])
                .unwrap();
        tracing::info!(
            "in substrate [build header] >> beef_light_client MmrLeafProof = {:?}",
            mmr_leaf_proof
        );

        let grandpa_header = GPHeader {
            block_header: result.0,
            mmr_leaf: MmrLeaf::from(mmr_leaf), // 66->73
            mmr_leaf_proof: MmrLeafProof::from(mmr_leaf_proof), // 66->73
        };
        tracing::info!(
            "in substrate [build header] >> grandpa_header = {:?}",
            grandpa_header.clone()
        );

        // // build support header
        // let mut support_header = vec![];
        //
        // // ensure target_height > trust_height
        // assert!(target_height > trusted_height);
        //
        // // for block_number in trusted_height.revision_height..target_height.revision_height {
        // let result = async {
        //     let client = ClientBuilder::new()
        //         .set_url(&self.websocket_url.clone())
        //         .build::<ibc_node::DefaultConfig>()
        //         .await
        //         .unwrap();
        //
        //     // get block header
        //     let block_header = octopusxt::call_ibc::get_header_by_block_number(
        //         client.clone(),
        //         Some(BlockNumber::from(trusted_height.revision_height as u32)),
        //     )
        //     .await
        //     .unwrap();
        //
        //     let api = client
        //         .clone()
        //         .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();
        //
        //     // assert block_header.block_number == target_height
        //     assert_eq!(
        //         block_header.block_number,
        //         trusted_height.revision_height as u32
        //     );
        //
        //     // block hash by block number
        //     let block_hash: Option<sp_core::H256> = api
        //         .client
        //         .rpc()
        //         .block_hash(Some(BlockNumber::from(
        //             mmr_root_height,
        //             // trusted_height.revision_height as u32,
        //         )))
        //         .await
        //         .unwrap();
        //
        //     // get mmr_leaf and mmr_leaf_proof
        //     let mmr_leaf_and_mmr_leaf_proof = octopusxt::call_ibc::get_mmr_leaf_and_mmr_proof(
        //         trusted_height.revision_height - 1,
        //         block_hash,
        //         client,
        //     )
        //     .await
        //     .unwrap();
        //
        //     (block_header, mmr_leaf_and_mmr_leaf_proof)
        // };
        //
        // let result = self.block_on(result);
        // tracing::info!("in substrate [build header] >> block header = {:?}, mmr_leaf = {:?}, mmr_leaf_proof = {:?}",
        //         result.0, result.1.0, result.1.1
        //     );
        //
        // let grandpa_client_state = match client_state {
        //     AnyClientState::Grandpa(state) => state,
        //     _ => todo!(),
        // };
        // // assert!(result.0.block_number <= grandpa_client_state.block_number);
        //
        // let mut encode_mmr_leaf = result.1.1;
        // let mut encode_mmr_leaf_proof = result.1.2;
        //
        // let leaf: Vec<u8> = Decode::decode(&mut &encoded_mmr_leaf[..]).unwrap();
        // let mmr_leaf: beefy_light_client::mmr::MmrLeaf = Decode::decode(&mut &*leaf).unwrap();
        //
        // let mmr_leaf_proof =
        //     beefy_light_client::mmr::MmrLeafProof::decode(&mut &encode_mmr_leaf_proof[..]).unwrap();
        //
        // let grandpa_header_temp = GPHeader {
        //     block_header: result.0,
        //     mmr_leaf: MmrLeaf::from(mmr_leaf),
        //     mmr_leaf_proof: MmrLeafProof::from(mmr_leaf_proof),
        // };
        //
        // support_header.push(grandpa_header_temp);
        // // }

        Ok((grandpa_header, vec![] /*support_header*/))
    }

    fn websocket_url(&self) -> Result<String, Error> {
        tracing::info!("in substrate: [websocket_url]");

        Ok(self.websocket_url.clone().to_string())
    }

    fn update_mmr_root(
        &self,
        src_chain_websocket_url: String,
        dst_chain_websocket_url: String,
    ) -> Result<(), Error> {
        tracing::info!("in substrate: [update_mmr_root]");
        let result = async {
            let chain_a = ClientBuilder::new()
                .set_url(src_chain_websocket_url)
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();
            let chain_b = ClientBuilder::new()
                .set_url(dst_chain_websocket_url)
                .build::<ibc_node::DefaultConfig>()
                .await
                .unwrap();

            octopusxt::update_client_state::update_client_state(chain_a.clone(), chain_b.clone())
                .await
                .unwrap();
            octopusxt::update_client_state::update_client_state(chain_b.clone(), chain_a.clone())
                .await
                .unwrap();

            ()
        };

        let ret = self.block_on(result);

        Ok(ret)
    }
}

/// Compose merkle proof according to ibc proto
pub fn compose_ibc_merkle_proof(proof: String) -> MerkleProof {
    use ibc_proto::ics23::{commitment_proof, ExistenceProof, InnerOp};
    tracing::info!("in substrate: [get_dummy_merk_proof]");

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

    let parsed = ibc_proto::ics23::CommitmentProof {
        proof: Some(_proof),
    };
    let mproofs: Vec<ibc_proto::ics23::CommitmentProof> = vec![parsed];
    MerkleProof { proofs: mproofs }
}

/// Returns a dummy `MerkleProof`, for testing only!
pub fn get_dummy_merkle_proof() -> MerkleProof {
    tracing::info!("in substrate: [get_dummy_merk_proof]");

    let parsed = ibc_proto::ics23::CommitmentProof { proof: None };
    let mproofs: Vec<ibc_proto::ics23::CommitmentProof> = vec![parsed];
    MerkleProof { proofs: mproofs }
}

use crate::connection::ConnectionMsgType;
use ibc::ics03_connection::version::Version;
use ibc::ics07_tendermint::header::Header as tHeader;
use ibc::ics10_grandpa::help::{
    BlockHeader, MmrLeaf, MmrLeafProof, SignedCommitment, ValidatorMerkleProof, ValidatorSet,
};
use ibc::proofs::Proofs;
use retry::delay::Fixed;
use subxt::sp_core::storage::StorageKey;
use tendermint_light_client::types::Validator;

pub fn get_dummy_ics07_header() -> tHeader {
    use tendermint::block::signed_header::SignedHeader;
    // Build a SignedHeader from a JSON file.
    let shdr = serde_json::from_str::<SignedHeader>(include_str!(
        "../../../modules/tests/support/signed_header.json"
    ))
    .unwrap();

    // Build a set of validators.
    // Below are test values inspired form `test_validator_set()` in tendermint-rs.
    use std::convert::TryInto;
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
        trusted_height: Height::new(0, 1),
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
