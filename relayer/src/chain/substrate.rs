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
use codec::{Decode, Encode};
use core::future::Future;
use core::str::FromStr;
use core::time::Duration;
use ibc::events::{IbcEvent, WithBlockDataType};

use super::tx::TrackedMsgs;
use crate::chain::{ChainStatus, QueryResponse};
use crate::connection::ConnectionMsgType;
use crate::light_client::Verified;
use ibc::clients::ics07_tendermint::header::Header as tHeader;
use ibc::clients::ics10_grandpa::client_state::ClientState as GPClientState;
use ibc::clients::ics10_grandpa::consensus_state::ConsensusState as GPConsensusState;
use ibc::clients::ics10_grandpa::header::Header as GPHeader;
use ibc::clients::ics10_grandpa::help::{
    BlockHeader, MmrLeaf, MmrLeafProof, SignedCommitment, ValidatorMerkleProof, ValidatorSet,
};
use ibc::core::ics02_client::client_consensus::{AnyConsensusState, AnyConsensusStateWithHeight};
use ibc::core::ics02_client::client_state::{AnyClientState, IdentifiedAnyClientState};
use ibc::core::ics02_client::client_type::ClientType;
use ibc::core::ics03_connection::connection::{
    ConnectionEnd, Counterparty, IdentifiedConnectionEnd,
};
use ibc::core::ics04_channel::channel::{ChannelEnd, IdentifiedChannelEnd};
use ibc::core::ics04_channel::error::Error as Ics04Error;
use ibc::core::ics04_channel::packet::{Packet, PacketMsgType, Receipt, Sequence};
use ibc::core::ics23_commitment::commitment::{CommitmentPrefix, CommitmentRoot};
use ibc::core::ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId};
use ibc::proofs::Proofs;
use ibc::query::{QueryBlockRequest, QueryTxRequest};
use ibc::signer::Signer;
use ibc::timestamp::Timestamp;
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
// use prost_types::Any;
use super::client::ClientSettings;
use ibc_proto::google::protobuf::Any;
use retry::delay::Fixed;
use retry::OperationResult;
use semver::Version;
use std::thread;
use std::thread::sleep;
use subxt::sp_core::storage::StorageKey;
use subxt::sp_runtime::generic::Header;
use subxt::sp_runtime::traits::BlakeTwo256;
use subxt::storage::StorageEntry;
use subxt::{BlockNumber, Client, ClientBuilder};
use tendermint::abci::transaction::Hash;
use tendermint::abci::{Code, Log};
use tendermint::account::Id as AccountId;
use tendermint_light_client::types::Validator;
use tendermint_proto::Protobuf;
use tendermint_rpc::endpoint::broadcast::tx_sync::Response as TxResponse;
use tokio::runtime::Runtime;
use tokio::runtime::Runtime as TokioRuntime;
use tokio::task;

const MAX_QUERY_TIMES: u64 = 100;

/// A struct used to start a Substrate chain instance in relayer
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
        self.rt.block_on(f)
    }

    fn get_client(&self) -> Result<Client<ibc_node::DefaultConfig>, Error> {
        let client = async {
            ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<ibc_node::DefaultConfig>()
                .await
                .map_err(|_| Error::substrate_client_builder_error())
        };

        self.block_on(client)
    }

    fn retry_wapper<O, Op>(&self, operation: Op) -> Result<O, retry::Error<String>>
    where
        Op: FnOnce() -> Result<O, Box<dyn std::error::Error>> + Copy,
    {
        retry_with_index(Fixed::from_millis(200), |current_try| {
            if current_try > MAX_QUERY_TIMES {
                return RetryResult::Err("did not succeed within tries".to_string());
            }

            let result = operation();

            match result {
                Ok(v) => RetryResult::Ok(v),
                Err(e) => RetryResult::Retry("Fail to retry".to_string()),
            }
        })
    }

    /// Subscribe Events
    fn subscribe_ibc_events(&self) -> Result<Vec<IbcEvent>, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [subscribe_ibc_events]");
        let client = self.get_client()?;

        self.block_on(octopusxt::subscribe_ibc_event(client))
    }

    /// get latest block height
    fn get_latest_height(&self) -> Result<u64, Box<dyn std::error::Error>> {
        tracing::trace!("in substrate: [get_latest_height]");
        let client = self.get_client()?;
        self.block_on(octopusxt::get_latest_height(client))
    }

    /// get connectionEnd by connection_identifier
    fn get_connection_end(
        &self,
        connection_identifier: &ConnectionId,
    ) -> Result<ConnectionEnd, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_connection_end]");

        let client = self.get_client()?;
        self.block_on(octopusxt::get_connection_end(connection_identifier, client))
    }

    /// get channelEnd  by port_identifier, and channel_identifier
    fn get_channel_end(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<ChannelEnd, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_channel_end]");
        let client = self.get_client()?;
        self.block_on(octopusxt::get_channel_end(port_id, channel_id, client))
    }

    /// get packet receipt by port_id, channel_id and sequence
    fn get_packet_receipt(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
    ) -> Result<Receipt, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_packet_receipt]");

        let client = self.get_client()?;
        self.block_on(octopusxt::get_packet_receipt(
            port_id, channel_id, seq, client,
        ))
    }

    /// get send packet event by port_id, channel_id and sequence
    fn get_send_packet_event(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
    ) -> Result<Packet, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_send_packet_event]");

        let client = self.get_client()?;
        self.block_on(octopusxt::get_send_packet_event(
            &port_id,
            &channel_id,
            &seq,
            client,
        ))
    }

    /// get client_state by client_id
    fn get_client_state(
        &self,
        client_id: &ClientId,
    ) -> Result<AnyClientState, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_client_state]");

        let client = self.get_client()?;
        self.block_on(octopusxt::get_client_state(&client_id, client))
    }

    /// get consensus_state by client_identifier and height
    fn get_client_consensus(
        &self,
        client_id: &ClientId,
        height: &ICSHeight,
    ) -> Result<AnyConsensusState, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_client_consensus] client_id: {:?}, height: {:?}", client_id, height);

        let client = self.get_client()?;

        let cs = self
            .block_on(octopusxt::get_client_consensus(&client_id, &height, client))
            .unwrap();
        tracing::trace!(target:"ibc-rs","in substrate: [get_client_consensus] ConsensusState: {:?}", cs);
        Ok(cs)
    }

    fn get_consensus_state_with_height(
        &self,
        client_id: &ClientId,
    ) -> Result<Vec<(Height, AnyConsensusState)>, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_consensus_state_with_height]");

        let client = self.get_client()?;

        self.block_on(octopusxt::get_consensus_state_with_height(
            &client_id, client,
        ))
    }

    fn get_unreceipt_packet(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequences: &[Sequence],
    ) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_unreceipt_packet]");

        let client = self.get_client()?;

        self.block_on(octopusxt::get_unreceipt_packet(
            port_id,
            channel_id,
            sequences.to_vec(),
            client,
        ))
    }

    fn get_clients(&self) -> Result<Vec<IdentifiedAnyClientState>, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_clients]");
        let client = self.get_client()?;
        self.block_on(octopusxt::get_clients(client))
    }

    fn get_connections(&self) -> Result<Vec<IdentifiedConnectionEnd>, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_connections]");
        let client = self.get_client()?;
        self.block_on(octopusxt::get_connections(client))
    }

    fn get_channels(&self) -> Result<Vec<IdentifiedChannelEnd>, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_channels]");
        let client = self.get_client()?;
        self.block_on(octopusxt::get_channels(client))
    }

    fn get_commitment_packet_state(&self) -> Result<Vec<PacketState>, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_commitment_packet_state]");
        let client = self.get_client()?;
        self.block_on(octopusxt::get_commitment_packet_state(client))
    }

    /// get packet commitment by port_id, channel_id and sequence
    fn get_packet_commitment(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: &Sequence,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_packet_commitment]");

        let client = self.get_client()?;
        self.block_on(octopusxt::get_packet_commitment(
            port_id, channel_id, sequence, client,
        ))
    }

    fn get_acknowledge_packet_state(&self) -> Result<Vec<PacketState>, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_acknowledge_packet_state]");
        let client = self.get_client()?;
        self.block_on(octopusxt::get_acknowledge_packet_state(client))
    }

    /// get connection_identifier vector by client_identifier
    fn get_client_connections(
        &self,
        client_id: &ClientId,
    ) -> Result<Vec<ConnectionId>, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_client_connections]");

        let client = self.get_client()?;

        self.block_on(octopusxt::get_client_connections(client_id, client))
    }

    fn get_connection_channels(
        &self,
        connection_id: &ConnectionId,
    ) -> Result<Vec<IdentifiedChannelEnd>, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_connection_channels]");

        let client = self.get_client()?;
        self.block_on(octopusxt::get_connection_channels(connection_id, client))
    }

    /// The function to submit IBC request to a Substrate chain
    /// This function handles most of the IBC reqeusts, except the MMR root update
    fn deliever(&self, msgs: Vec<Any>) -> Result<subxt::sp_core::H256, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [deliever]");

        let client = self.get_client()?;

        let result = self.block_on(octopusxt::deliver(msgs, client))?;

        Ok(result)
    }

    fn get_write_ack_packet_event(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: &Sequence,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_send_packet_event]");

        let client = self.get_client()?;

        self.block_on(octopusxt::ibc_rpc::get_write_ack_packet_event(
            port_id, channel_id, sequence, client,
        ))
    }

    fn get_ibc_send_packet_event(
        &self,
        request: ibc::core::ics04_channel::channel::QueryPacketEventDataRequest,
    ) -> Result<Vec<IbcEvent>, Error> {
        let mut result_event = vec![];
        for sequence in &request.sequences {
            let packet = self
                .get_send_packet_event(
                    &request.source_port_id,
                    &request.source_channel_id,
                    sequence,
                )
                .map_err(|_| Error::get_send_packet_event_error())?;

            result_event.push(IbcEvent::SendPacket(
                ibc::core::ics04_channel::events::SendPacket {
                    height: request.height,
                    packet,
                },
            ));
        }
        Ok(result_event)
    }

    fn get_ibc_write_acknowledgement_event(
        &self,
        request: ibc::core::ics04_channel::channel::QueryPacketEventDataRequest,
    ) -> Result<Vec<IbcEvent>, Error> {
        use ibc::core::ics04_channel::events::WriteAcknowledgement;
        let mut result_event = vec![];
        for sequence in &request.sequences {
            let write_ack = self
                .get_write_ack_packet_event(
                    &request.source_port_id,
                    &request.source_channel_id,
                    sequence,
                )
                .map_err(|_| Error::get_write_ack_packet_event_error())?;

            let write_ack = WriteAcknowledgement::decode(&*write_ack).map_err(Error::decode)?;
            result_event.push(IbcEvent::WriteAcknowledgement(write_ack));
        }

        Ok(result_event)
    }

    /// Retrieve the storage proof according to storage keys
    /// And convert the proof to IBC compatible type
    fn generate_storage_proof<F: StorageEntry>(
        &self,
        storage_entry: &F,
        height: &Height,
        storage_name: &str,
    ) -> Result<MerkleProof, Error>
    where
        <F as StorageEntry>::Value: serde::Serialize + core::fmt::Debug,
    {
        let generate_storage_proof = async {
            use serde::{Deserialize, Serialize};
            use sp_core::{storage::StorageKey, Bytes};
            use subxt::{rpc::NumberOrHex, sp_core::H256, storage::StorageKeyPrefix, BlockNumber};

            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<ibc_node::DefaultConfig>()
                .await
                .map_err(|_| Error::substrate_client_builder_error())?;

            let _height = NumberOrHex::Number(height.revision_height);

            let block_hash: H256 = client
                .rpc()
                .block_hash(Some(BlockNumber::from(_height)))
                .await
                .map_err(|_| Error::get_block_hash_error())?
                .ok_or_else(Error::empty_hash)?;

            let storage_key = storage_entry.key().final_key(StorageKeyPrefix::new::<F>());
            // tracing::trace!("in substrate: [generate_storage_proof] >> height: {:?}, block_hash: {:?}, storage key: {:?}, storage_name = {:?}",
            //     height, block_hash, storage_key, storage_name);

            use jsonrpsee::types::to_json_value;

            let params = &[
                to_json_value(vec![storage_key]).map_err(Error::invalid_serde_json_error)?,
                to_json_value(block_hash).map_err(Error::invalid_serde_json_error)?,
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
                .map_err(|_| Error::get_read_proof_error())?;

            // tracing::trace!(
            //     "in substrate: [generate_storage_proof] >> storage_proof : {:?}",
            //     storage_proof
            // );

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
            // tracing::trace!(
            //     "in substrate: [generate_storage_proof] >> storage_proof_ : {:?}",
            //     storage_proof_
            // );

            let storage_proof_str =
                serde_json::to_string(&storage_proof_).map_err(Error::invalid_serde_json_error)?;
            // tracing::trace!(
            //     "in substrate: [generate_storage_proof] >> storage_proof_str: {:?}",
            //     storage_proof_str
            // );

            Ok(storage_proof_str)
        };

        let storage_proof = self.block_on(generate_storage_proof)?;

        Ok(compose_ibc_merkle_proof(storage_proof))
    }
}

impl ChainEndpoint for SubstrateChain {
    type LightBlock = GPHeader;
    type Header = GPHeader;
    type ConsensusState = AnyConsensusState;
    type ClientState = AnyClientState;
    type LightClient = GPLightClient;

    fn bootstrap(config: ChainConfig, rt: Arc<TokioRuntime>) -> Result<Self, Error> {
        tracing::info!(target:"ibc-rs","in Substrate: [bootstrap function]");

        let websocket_url = format!("{}", config.websocket_addr);

        let chain = Self {
            config,
            websocket_url,
            rt,
        };

        Ok(chain)
    }

    fn init_light_client(&self) -> Result<Self::LightClient, Error> {
        tracing::debug!(target:"ibc-rs","in substrate: [init_light_client]");
        use subxt::sp_core::Public;

        let config = self.config.clone();

        let public_key = async {
            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<ibc_node::DefaultConfig>()
                .await
                .map_err(|e| Error::substrate_client_builder_error())?;

            let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

            let authorities = api
                .storage()
                .beefy()
                .authorities(None)
                .await
                .map_err(|_| Error::authorities())?;
            tracing::info!(target:"ibc-rs","authorities length : {:?}", authorities.len());
            let result: Vec<String> = authorities
                .into_iter()
                .map(|val| {
                    format!(
                        "0x{}",
                        subxt::sp_core::hexdisplay::HexDisplay::from(&val.to_raw_vec())
                    )
                })
                .collect();
            tracing::info!(target:"ibc-rs","authorities member: {:?}", result);
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
    ) -> Result<(EventReceiver, TxMonitorCmd), Error> {
        tracing::debug!(target:"ibc-rs",
            "in substrate: [init_event_mointor] >> websocket addr: {:?}",
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
        tracing::info!(target:"ibc-rs","in substrate: [shutdown]");

        Ok(())
    }

    fn health_check(&self) -> Result<HealthCheck, Error> {
        Ok(HealthCheck::Healthy)
    }

    fn id(&self) -> &ChainId {
        tracing::trace!(target:"ibc-rs","in substrate: [id]");

        &self.config().id
    }

    fn keybase(&self) -> &KeyRing {
        tracing::trace!(target:"ibc-rs","in substrate: [keybase]");

        todo!()
    }

    fn keybase_mut(&mut self) -> &mut KeyRing {
        tracing::trace!(target:"ibc-rs","in substrate: [keybase_mut]");

        todo!()
    }

    fn send_messages_and_wait_commit(
        &mut self,
        proto_msgs: TrackedMsgs,
    ) -> Result<Vec<IbcEvent>, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [send_messages_and_wait_commit]");

        sleep(Duration::from_secs(4));
        let result = self
            .deliever(proto_msgs.messages().to_vec())
            .map_err(|e| Error::deliver_error(e))?;

        tracing::debug!(target:"ibc-rs",
            "in substrate: [send_messages_and_wait_commit] >> extrics_hash  : {:?}",
            result
        );

        let ibc_event = self
            .subscribe_ibc_events()
            .map_err(|_| Error::subscribe_ibc_events())?;

        Ok(ibc_event)
    }

    fn send_messages_and_wait_check_tx(
        &mut self,
        proto_msgs: TrackedMsgs,
    ) -> Result<Vec<TxResponse>, Error> {
        tracing::debug!(target:"ibc-rs","in substrate: [send_messages_and_wait_check_tx]");

        sleep(Duration::from_secs(4));
        let result = self
            .deliever(proto_msgs.messages().to_vec())
            .map_err(|e| Error::deliver_error(e))?;

        // if result.is_err() {
        //     let err_str = result.err().ok_or_else(Error::deliver_error)?.to_string();
        //     if err_str.contains("Priority is too low") {
        //         // Todo: to catch the error by error type? maybe related to the repeated submission issue in github
        //         tracing::error!(
        //             "in substrate: [send_messages_and_wait_check_tx] >> error : {:?}",
        //             err_str
        //         );

        //         return Ok(vec![]);
        //     } else {
        //         return Err(Error::sub_tx_error(err_str));
        //     }
        // }

        // let result = result.map_err(|_| Error::deliver_error())?;
        tracing::debug!(target:"ibc-rs",
            "in substrate: [send_messages_and_wait_check_tx] >> extrics_hash : {:?}",
            result
        );

        use tendermint::abci::transaction; // Todo: apply with real responses
        let json = "\"ChYKFGNvbm5lY3Rpb25fb3Blbl9pbml0\"";
        let tx_re = TxResponse {
            code: Code::default(),
            data: serde_json::from_str(json).map_err(Error::invalid_serde_json_error)?,
            log: Log::from("testtest"),
            hash: transaction::Hash::new(
                *result
                    // .as_slice()
                    // .last()
                    // .ok_or_else(Error::empty_element)?
                    .as_fixed_bytes(),
            ),
        };

        Ok(vec![tx_re])
    }

    fn get_signer(&mut self) -> Result<Signer, Error> {
        // Todo: Get signer from config
        tracing::trace!(target:"ibc-rs","In Substraet: [get signer]");

        fn get_dummy_account_id_raw() -> String {
            "0CDA3F47EF3C4906693B170EF650EB968C5F4B2C".to_string()
        }

        pub fn get_dummy_account_id() -> Result<AccountId, Error> {
            AccountId::from_str(&get_dummy_account_id_raw())
                .map_err(|e| Error::unknown_account_type(get_dummy_account_id_raw()))
        }

        let signer = Signer::new(get_dummy_account_id()?.to_string());
        // tracing::trace!("in substrate: [get_signer] >>  signer {:?}", signer);

        Ok(signer)
    }

    fn get_key(&mut self) -> Result<KeyEntry, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [get_key]");

        todo!()
    }

    fn query_commitment_prefix(&self) -> Result<CommitmentPrefix, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_commitment_prefix]");

        // TODO - do a real chain query
        CommitmentPrefix::try_from(self.config().store_prefix.as_bytes().to_vec())
            .map_err(|_| Error::invalid_commitment_prefix())
    }

    fn query_clients(
        &self,
        request: QueryClientStatesRequest,
    ) -> Result<Vec<IdentifiedAnyClientState>, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_clients]");

        let result = self
            .retry_wapper(|| self.get_clients())
            .map_err(Error::retry_error)?;

        // tracing::trace!("in substrate: [query_clients] >> clients: {:?}", result);

        Ok(result)
    }

    fn query_client_state(
        &self,
        client_id: &ClientId,
        height: ICSHeight,
    ) -> Result<Self::ClientState, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_client_state]");

        let result = self
            .retry_wapper(|| self.get_client_state(client_id))
            .map_err(Error::retry_error)?;

        tracing::trace!(target:"ibc-rs",
            "in substrate: [query_client_state] >> client_state: {:?}",
            result
        );

        Ok(result)
    }

    fn query_consensus_states(
        &self,
        request: QueryConsensusStatesRequest,
    ) -> Result<Vec<AnyConsensusStateWithHeight>, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_consensus_states]");

        let request_client_id =
            ClientId::from_str(request.client_id.as_str()).map_err(Error::identifier)?;

        let result = self
            .retry_wapper(|| self.get_consensus_state_with_height(&request_client_id))
            .map_err(Error::retry_error)?;

        let consensus_state: Vec<(Height, AnyConsensusState)> = result;

        let mut any_consensus_state_with_height = vec![];
        for (height, consensus_state) in consensus_state.into_iter() {
            let tmp = AnyConsensusStateWithHeight {
                height,
                consensus_state,
            };
            any_consensus_state_with_height.push(tmp.clone());

            tracing::trace!(target:"ibc-rs",
                "in substrate: [query_consensus_state] >> any_consensus_state_with_height: {:?}",
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
        tracing::trace!(target:"ibc-rs","in substrate: [query_consensus_state] client_id: {:?} ", client_id);
        tracing::trace!(target:"ibc-rs","in substrate: [query_consensus_state] consensus_height: {:?} ", consensus_height);
        tracing::trace!(target:"ibc-rs","in substrate: [query_consensus_state] query_height: {:?} ", query_height);

        let result = self
            .retry_wapper(|| self.get_client_consensus(&client_id, &consensus_height))
            .map_err(Error::retry_error)?;

        // let consensus_state = self
        //     .proven_client_consensus(&client_id, consensus_height, query_height)?
        //     .0;
        tracing::trace!(target:"ibc-rs","in substrate: [query_consensus_state] result {:?}", result);
        Ok(result)
    }

    fn query_upgraded_client_state(
        &self,
        height: ICSHeight,
    ) -> Result<(Self::ClientState, MerkleProof), Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_upgraded_client_state]");

        todo!()
    }

    fn query_upgraded_consensus_state(
        &self,
        height: ICSHeight,
    ) -> Result<(Self::ConsensusState, MerkleProof), Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_upgraded_consensus_state]");

        todo!()
    }

    fn query_connections(
        &self,
        request: QueryConnectionsRequest,
    ) -> Result<Vec<IdentifiedConnectionEnd>, Error> {
        tracing::trace!("in substrate: [query_connections]");

        let result = self
            .retry_wapper(|| self.get_connections())
            .map_err(Error::retry_error)?;

        // tracing::trace!("in substrate: [query_connections] >> clients: {:?}", result);

        Ok(result)
    }

    fn query_client_connections(
        &self,
        request: QueryClientConnectionsRequest,
    ) -> Result<Vec<ConnectionId>, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_client_connections]");

        let client_id =
            ClientId::from_str(request.client_id.as_str()).map_err(Error::identifier)?;

        let result = self
            .retry_wapper(|| self.get_client_connections(&client_id))
            .map_err(Error::retry_error)?;

        // tracing::trace!(
        //     "in substrate: [query_client_connections] >> client_connections: {:#?}",
        //     result
        // );

        Ok(result)
    }

    // TODO fo substrate
    fn query_connection(
        &self,
        connection_id: &ConnectionId,
        height: ICSHeight,
    ) -> Result<ConnectionEnd, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_connection]");

        let connection_end = self
            .retry_wapper(|| self.get_connection_end(connection_id))
            .map_err(Error::retry_error)?;
        // tracing::trace!(
        //     "in substrate: [query_connection] >> connection_end: {:#?}",
        //     connection_end
        // );

        Ok(connection_end)
    }

    fn query_connection_channels(
        &self,
        request: QueryConnectionChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_connection_channels] ");

        let connection_id =
            ConnectionId::from_str(&request.connection).map_err(Error::identifier)?;
        let result = self
            .retry_wapper(|| self.get_connection_channels(&connection_id))
            .map_err(Error::retry_error)?;

        // tracing::trace!(
        //     "in substrate: [query_connection_channels] >> connection_channels: {:?}",
        //     result
        // );

        Ok(result)
    }

    fn query_channels(
        &self,
        request: QueryChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_channels]");

        let result = self
            .retry_wapper(|| self.get_channels())
            .map_err(Error::retry_error)?;

        // tracing::trace!("in substrate: [query_connections] >> clients: {:?}", result);

        Ok(result)
    }

    // todo for substrate
    fn query_channel(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        height: ICSHeight,
    ) -> Result<ChannelEnd, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_channel]");

        let channel_end = self
            .retry_wapper(|| self.get_channel_end(port_id, channel_id))
            .map_err(Error::retry_error)?;

        // tracing::trace!(
        //     "in substrate: [query_channel] >> channel_end: {:#?}",
        //     channel_end
        // );

        Ok(channel_end)
    }

    fn query_channel_client_state(
        &self,
        request: QueryChannelClientStateRequest,
    ) -> Result<Option<IdentifiedAnyClientState>, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_channel_client_state]");

        todo!()
    }

    fn query_packet_commitments(
        &self,
        request: QueryPacketCommitmentsRequest,
    ) -> Result<(Vec<PacketState>, ICSHeight), Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_packet_commitments]");

        let packet_commitments = self
            .retry_wapper(|| self.get_commitment_packet_state())
            .map_err(Error::retry_error)?;

        let height = self
            .retry_wapper(|| self.get_latest_height())
            .map_err(Error::retry_error)?;

        let latest_height = Height::new(0, height);

        Ok((packet_commitments, latest_height))
    }

    fn query_unreceived_packets(
        &self,
        request: QueryUnreceivedPacketsRequest,
    ) -> Result<Vec<u64>, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_unreceived_packets]");

        let port_id = PortId::from_str(request.port_id.as_str()).map_err(Error::identifier)?;
        let channel_id =
            ChannelId::from_str(request.channel_id.as_str()).map_err(Error::identifier)?;
        let sequences = request
            .packet_commitment_sequences
            .into_iter()
            .map(Sequence::from)
            .collect::<Vec<_>>();

        let result = self
            .retry_wapper(|| self.get_unreceipt_packet(&port_id, &channel_id, &sequences))
            .map_err(Error::retry_error)?;

        Ok(result)
    }

    fn query_packet_acknowledgements(
        &self,
        request: QueryPacketAcknowledgementsRequest,
    ) -> Result<(Vec<PacketState>, ICSHeight), Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_packet_acknowledgements]");

        let packet_acknowledgements = self
            .retry_wapper(|| self.get_acknowledge_packet_state())
            .map_err(Error::retry_error)?;

        let height = self
            .retry_wapper(|| self.get_latest_height())
            .map_err(Error::retry_error)?;

        let latest_height = Height::new(0, height);

        Ok((packet_acknowledgements, latest_height))
    }

    fn query_unreceived_acknowledgements(
        &self,
        request: QueryUnreceivedAcksRequest,
    ) -> Result<Vec<u64>, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_unreceived_acknowledgements] ");

        let port_id = PortId::from_str(request.port_id.as_str()).map_err(Error::identifier)?;
        let channel_id =
            ChannelId::from_str(request.channel_id.as_str()).map_err(Error::identifier)?;
        let sequences = request
            .packet_ack_sequences
            .into_iter()
            .map(Sequence::from)
            .collect::<Vec<_>>();

        let mut unreceived_seqs: Vec<u64> = vec![];

        for seq in sequences {
            let cmt = self.retry_wapper(|| self.get_packet_commitment(&port_id, &channel_id, &seq));

            // if packet commitment still exists on the original sending chain, then packet ack is unreceived
            // since processing the ack will delete the packet commitment
            if let Ok(ret) = cmt {
                unreceived_seqs.push(u64::from(seq));
            }
        }

        Ok(unreceived_seqs)
    }

    fn query_next_sequence_receive(
        &self,
        request: QueryNextSequenceReceiveRequest,
    ) -> Result<Sequence, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_next_sequence_receive] ");

        todo!()
    }

    fn query_txs(&self, request: QueryTxRequest) -> Result<Vec<IbcEvent>, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_txs]");

        match request {
            // Todo: Related to https://github.com/octopus-network/ibc-rs/issues/88
            QueryTxRequest::Packet(request) => {
                let mut result: Vec<IbcEvent> = vec![];
                if request.sequences.is_empty() {
                    return Ok(result);
                }

                // tracing::trace!(
                //     "in substrate: [query_txs]: query packet events request: {:?}",
                //     request
                // );

                match request.event_id {
                    WithBlockDataType::SendPacket => {
                        let mut send_packet_event = self.get_ibc_send_packet_event(request)?;
                        result.append(&mut send_packet_event);
                    }

                    WithBlockDataType::WriteAck => {
                        let mut ack_event = self.get_ibc_write_acknowledgement_event(request)?;
                        result.append(&mut ack_event);
                    }

                    _ => unimplemented!(),
                }

                Ok(result)
            }

            QueryTxRequest::Client(request) => {
                // tracing::trace!(
                //     "in substrate: [query_txs]: single client update event: request:{:?}",
                //     request
                // );

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
                use ibc::core::ics02_client::events::Attributes;
                use ibc::core::ics02_client::header::AnyHeader;

                // Todo: the client event below is mock
                // replace it with real client event replied from a Substrate chain
                let result: Vec<IbcEvent> = vec![IbcEvent::UpdateClient(
                    ibc::core::ics02_client::events::UpdateClient::from(Attributes {
                        height: request.height,
                        client_id: request.client_id,
                        client_type: ClientType::Grandpa,
                        consensus_height: request.consensus_height,
                    }),
                )];

                Ok(result)
                // Ok(event.into_iter().collect())
            }

            QueryTxRequest::Transaction(tx) => {
                // tracing::trace!("in substrate: [query_txs]: Transaction: {:?}", tx);
                // Todo: https://github.com/octopus-network/ibc-rs/issues/98
                let result: Vec<IbcEvent> = vec![];
                Ok(result)
            }
        }
    }

    fn proven_client_state(
        &self,
        client_id: &ClientId,
        height: ICSHeight,
    ) -> Result<(Self::ClientState, MerkleProof), Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [proven_client_state]");

        let result = self
            .retry_wapper(|| self.get_client_state(client_id))
            .map_err(Error::retry_error)?;

        tracing::trace!(
            "in substrate: [proven_client_state] >> client_state : {:?}",
            result
        );

        let storage_entry = ibc_node::ibc::storage::ClientStates(client_id.as_bytes().to_vec());
        Ok((
            result,
            self.generate_storage_proof(&storage_entry, &height, "ClientStates")?,
        ))
    }

    fn proven_connection(
        &self,
        connection_id: &ConnectionId,
        height: ICSHeight,
    ) -> Result<(ConnectionEnd, MerkleProof), Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [proven_connection]");

        let result = self
            .retry_wapper(|| self.get_connection_end(connection_id))
            .map_err(Error::retry_error)?;

        tracing::trace!(target:"ibc-rs",
            "in substrate: [proven_connection] >> connection_end: {:?}",
            result
        );

        let connection_end = result;

        let new_connection_end = if connection_end
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
            let versions = connection_end.versions();
            let delay_period = connection_end.delay_period();

            ConnectionEnd::new(
                state,
                client_id,
                counterparty,
                versions.to_vec(),
                delay_period,
            )
        } else {
            connection_end
        };

        let storage_entry = ibc_node::ibc::storage::Connections(connection_id.as_bytes().to_vec());
        Ok((
            new_connection_end,
            self.generate_storage_proof(&storage_entry, &height, "Connections")?,
        ))
    }

    fn proven_client_consensus(
        &self,
        client_id: &ClientId,
        consensus_height: ICSHeight,
        height: ICSHeight,
    ) -> Result<(Self::ConsensusState, MerkleProof), Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [proven_client_consensus] ");

        let result = self
            .retry_wapper(|| self.get_client_consensus(client_id, &consensus_height))
            .map_err(Error::retry_error)?;

        tracing::trace!(
            "in substrate: [proven_client_consensus] >> consensus_state : {:?}",
            result
        );

        let storage_entry = ibc_node::ibc::storage::ConsensusStates(client_id.as_bytes().to_vec());
        Ok((
            result,
            self.generate_storage_proof(&storage_entry, &height, "ConsensusStates")?,
        ))
    }

    fn proven_channel(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        height: ICSHeight,
    ) -> Result<(ChannelEnd, MerkleProof), Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [proven_channel]");

        let result = self
            .retry_wapper(|| self.get_channel_end(port_id, channel_id))
            .map_err(Error::retry_error)?;

        // tracing::trace!(
        //     "in substrate: [query_channel] >> port_id: {:?}, channel_id: {:?}, channel_end: {:?}",
        //     port_id,
        //     channel_id,
        //     result
        // );

        // let storage_entry = ibc_node::ibc::storage::Channels(
        //     port_id.as_bytes().to_vec(),
        //     channel_id.as_bytes().to_vec(),
        // );
        // Ok((
        //     result,
        //     self.generate_storage_proof(&storage_entry, &height, "Channels")?,
        // ))
        Ok((result, get_dummy_merkle_proof()))
    }

    fn proven_packet(
        &self,
        packet_type: PacketMsgType,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
        height: ICSHeight,
    ) -> Result<(Vec<u8>, MerkleProof), Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [proven_packet]");

        // let result = retry_with_index(Fixed::from_millis(200), |current_try| {
        //     if current_try > MAX_QUERY_TIMES {
        //         return RetryResult::Err("did not succeed within tries".to_string());
        //     }

        //     let result = async {
        //         let client = ClientBuilder::new()
        //             .set_url(&self.websocket_url.clone())
        //             .build::<ibc_node::DefaultConfig>()
        //             .await
        //             .map_err(|_| Error::substrate_client_builder_error())?;

        //         match packet_type {
        //             PacketMsgType::Recv => {
        //                 // PacketCommitment
        //                 octopusxt::ibc_rpc::get_packet_commitment(
        //                     &port_id,
        //                     &channel_id,
        //                     &sequence,
        //                     client,
        //                 )
        //                 .await
        //             }
        //             PacketMsgType::Ack => {
        //                 // Acknowledgements
        //                 octopusxt::ibc_rpc::get_packet_ack(&port_id, &channel_id, &sequence, client)
        //                     .await
        //             }
        //             PacketMsgType::TimeoutOnClose => {
        //                 // PacketReceipt
        //                 octopusxt::ibc_rpc::get_packet_receipt_vec(
        //                     &port_id,
        //                     &channel_id,
        //                     &sequence,
        //                     client,
        //                 )
        //                 .await
        //             }
        //             PacketMsgType::TimeoutUnordered => {
        //                 // PacketReceipt
        //                 octopusxt::ibc_rpc::get_packet_receipt_vec(
        //                     &port_id,
        //                     &channel_id,
        //                     &sequence,
        //                     client,
        //                 )
        //                 .await
        //             }
        //             PacketMsgType::TimeoutOrdered => {
        //                 // NextSequenceRecv
        //                 octopusxt::ibc_rpc::get_next_sequence_recv(&port_id, &channel_id, client)
        //                     .await
        //             }
        //         }
        //     };

        //     match self.block_on(result) {
        //         Ok(v) => RetryResult::Ok(v),
        //         Err(e) => RetryResult::Retry("Fail to retry".to_string()),
        //     }
        // })
        // .map_err(Error::retry_error)?;

        // tracing::trace!(
        //     "in substrate: [proven_packet] >> result: {:?}, packet_type = {:?}",
        //     result,
        //     packet_type
        // );

        // match packet_type {
        //     PacketMsgType::Recv => {
        //         let storage_entry = ibc_node::ibc::storage::PacketCommitment(
        //             port_id.as_bytes().to_vec(),
        //             channel_id.as_bytes().to_vec(),
        //             u64::from(sequence),
        //         );
        //         Ok((
        //             result,
        //             self.generate_storage_proof(&storage_entry, &height, "PacketCommitment")?,
        //         ))
        //     }
        //     PacketMsgType::Ack => {
        //         let storage_entry = ibc_node::ibc::storage::Acknowledgements(
        //             port_id.as_bytes().to_vec(),
        //             channel_id.as_bytes().to_vec(),
        //             u64::from(sequence),
        //         );
        //         Ok((
        //             result,
        //             self.generate_storage_proof(&storage_entry, &height, "Acknowledgements")?,
        //         ))
        //     }
        //     PacketMsgType::TimeoutOnClose => {
        //         Ok((vec![], compose_ibc_merkle_proof("12345678".to_string())))
        //     } // Todo: https://github.com/cosmos/ibc/issues/620
        //     PacketMsgType::TimeoutUnordered => {
        //         Ok((vec![], compose_ibc_merkle_proof("12345678".to_string())))
        //     } // Todo: https://github.com/cosmos/ibc/issues/620
        //     PacketMsgType::TimeoutOrdered => {
        //         let storage_entry = ibc_node::ibc::storage::NextSequenceRecv(
        //             port_id.as_bytes().to_vec(),
        //             channel_id.as_bytes().to_vec(),
        //         );
        //         Ok((
        //             result,
        //             self.generate_storage_proof(&storage_entry, &height, "NextSequenceRecv")?,
        //         ))
        //     } // Todo: Ordered channel not supported in ibc-rs. https://github.com/octopus-network/ibc-rs/blob/b98094a57620d0b3d9f8d2caced09abfc14ab00f/relayer/src/link.rs#L135
        // }
        Ok((vec![1, 2, 3], get_dummy_merkle_proof()))
    }

    fn build_client_state(
        &self,
        height: ICSHeight,
        dst_config: ClientSettings,
    ) -> Result<Self::ClientState, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [build_client_state]");

        use ibc::clients::ics10_grandpa::help::Commitment;
        use sp_core::Public;

        let public_key = async {
            let client = ClientBuilder::new()
                .set_url(&self.websocket_url.clone())
                .build::<ibc_node::DefaultConfig>()
                .await
                .map_err(|_| Error::substrate_client_builder_error())?;

            let api = client.to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

            let authorities = api
                .storage()
                .beefy()
                .authorities(None)
                .await
                .map_err(|_| Error::authorities())?;
            // tracing::trace!("authorities length : {:?}", authorities.len());
            let result: Vec<String> = authorities
                .into_iter()
                .map(|val| {
                    format!(
                        "0x{}",
                        subxt::sp_core::hexdisplay::HexDisplay::from(&val.to_raw_vec())
                    )
                })
                .collect();
            // tracing::trace!("authorities member: {:?}", result);

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
        .map_err(Error::ics10)?;

        // tracing::trace!(
        //     "in substrate: [build_client_state] >> client_state: {:?}",
        //     client_state
        // );

        Ok(AnyClientState::Grandpa(client_state))
    }

    fn build_consensus_state(
        &self,
        light_block: Self::LightBlock,
    ) -> Result<Self::ConsensusState, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [build_consensus_state]");

        Ok(AnyConsensusState::Grandpa(GPConsensusState::default()))
    }

    fn build_header(
        &self,
        trusted_height: ICSHeight,
        target_height: ICSHeight,
        client_state: &AnyClientState,
        light_client: &mut Self::LightClient,
    ) -> Result<(Self::Header, Vec<Self::Header>), Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [build_header]");

        assert!(trusted_height.revision_height < target_height.revision_height);

        let grandpa_client_state = match client_state {
            AnyClientState::Grandpa(state) => state,
            _ => unimplemented!(),
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
                .map_err(|_| Error::substrate_client_builder_error())?;

            let beefy_light_client::commitment::Commitment {
                payload,
                block_number,
                validator_set_id,
            } = grandpa_client_state.latest_commitment.clone().into();

            let mmr_root_height = block_number;

            //TODO: remove comment after test
            // assert!((target_height.revision_height as u32) <= mmr_root_height);

            // get block header

            let block_header = octopusxt::ibc_rpc::get_header_by_block_number(
                Some(BlockNumber::from(target_height.revision_height as u32)),
                client.clone(),
            )
            .await
            .map_err(|_| Error::get_header_by_block_number_error())?;

            let api = client
                .clone()
                .to_runtime_api::<ibc_node::RuntimeApi<ibc_node::DefaultConfig>>();

            assert_eq!(
                block_header.block_number,
                target_height.revision_height as u32
            );

            tracing::trace!(target:"ibc-rs",
                "in substrate: [build_header] >> mmr_root_height = {:?}, target_height = {:?}",
                mmr_root_height,
                target_height
            );

            //TODO: remove comment after test
            let mmr_root_height = target_height.revision_height as u32;

            let block_hash: Option<sp_core::H256> = api
                .client
                .rpc()
                .block_hash(Some(BlockNumber::from(mmr_root_height)))
                .await
                .map_err(|_| Error::get_block_hash_error())?;

            tracing::trace!(target:"ibc-rs",
                "in substrate: [build_header] >> block_hash = {:?}",
                block_hash
            );
            let mmr_leaf_and_mmr_leaf_proof = octopusxt::ibc_rpc::get_mmr_leaf_and_mmr_proof(
                Some(BlockNumber::from(
                    (target_height.revision_height - 1) as u32,
                )),
                block_hash,
                client,
            )
            .await
            .map_err(|_| Error::get_mmr_leaf_and_mmr_proof_error())?;

            Ok((block_header, mmr_leaf_and_mmr_leaf_proof))
        };

        let result = self.block_on(result)?;

        let encoded_mmr_leaf = result.1 .1;
        let encoded_mmr_leaf_proof = result.1 .2;

        let leaf: Vec<u8> =
            Decode::decode(&mut &encoded_mmr_leaf[..]).map_err(Error::invalid_codec_decode)?;
        let mmr_leaf: beefy_light_client::mmr::MmrLeaf =
            Decode::decode(&mut &*leaf).map_err(Error::invalid_codec_decode)?;
        let mmr_leaf_proof =
            beefy_light_client::mmr::MmrLeafProof::decode(&mut &encoded_mmr_leaf_proof[..])
                .map_err(Error::invalid_codec_decode)?;

        let grandpa_header = GPHeader {
            block_header: result.0,
            mmr_leaf: MmrLeaf::from(mmr_leaf),
            mmr_leaf_proof: MmrLeafProof::from(mmr_leaf_proof),
        };
        // tracing::trace!(
        //     "in substrate [build header] >> grandpa_header = {:?}",
        //     grandpa_header
        // );

        Ok((grandpa_header, vec![]))
    }

    fn websocket_url(&self) -> Result<String, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [websocket_url]");

        Ok(self.websocket_url.clone())
    }

    fn update_mmr_root(
        &self,
        src_chain_websocket_url: String,
        dst_chain_websocket_url: String,
    ) -> Result<(), Error> {
        tracing::info!(target:"ibc-rs","in substrate: [update_mmr_root]");

        let result = async {
            let chain_a = ClientBuilder::new()
                .set_url(src_chain_websocket_url)
                .build::<ibc_node::DefaultConfig>()
                .await
                .map_err(|_| Error::substrate_client_builder_error())?;

            let chain_b = ClientBuilder::new()
                .set_url(dst_chain_websocket_url)
                .build::<ibc_node::DefaultConfig>()
                .await
                .map_err(|_| Error::substrate_client_builder_error())?;

            octopusxt::update_client_state::update_client_state(chain_a.clone(), chain_b.clone())
                .await
                .map_err(|_| Error::update_client_state_error())?;

            octopusxt::update_client_state::update_client_state(chain_b.clone(), chain_a.clone())
                .await
                .map_err(|_| Error::update_client_state_error())
        };

        self.block_on(result)
    }

    fn config(&self) -> ChainConfig {
        self.config.clone()
    }

    fn add_key(&mut self, key_name: &str, key: KeyEntry) -> Result<(), Error> {
        self.keybase_mut()
            .add_key(key_name, key)
            .map_err(Error::key_base)?;

        Ok(())
    }

    fn ibc_version(&self) -> Result<Option<Version>, Error> {
        todo!()
    }

    fn query_application_status(&self) -> Result<ChainStatus, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_status]");

        let height = self
            .retry_wapper(|| self.get_latest_height())
            .map_err(Error::retry_error)?;

        let latest_height = Height::new(0, height);

        // tracing::trace!(
        //     "in substrate: [query_status] >> latest_height = {:?}",
        //     latest_height
        // );

        Ok(ChainStatus {
            height: latest_height,
            timestamp: Default::default(),
        })
    }

    fn query_blocks(
        &self,
        request: QueryBlockRequest,
    ) -> Result<(Vec<IbcEvent>, Vec<IbcEvent>), Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_block]");

        // let result = match request {
        //     QueryBlockRequest::Packet(request) => {
        //         let send_packet_event = self.get_ibc_send_packet_event(request.clone()).unwrap();
        //         let ack_event = self.get_ibc_write_acknowledgement_event(request).unwrap();
        //         (send_packet_event, ack_event)
        //     }
        // };

        // Ok(result)
        Ok((vec![], vec![]))
    }

    fn query_host_consensus_state(&self, height: ICSHeight) -> Result<Self::ConsensusState, Error> {
        tracing::trace!(target:"ibc-rs","in substrate: [query_host_consensus_state]");

        Ok(AnyConsensusState::Grandpa(GPConsensusState::default()))
    }
}

// Todo: to create a new type in `commitment_proof::Proof`
/// Compose merkle proof according to ibc proto
pub fn compose_ibc_merkle_proof(proof: String) -> MerkleProof {
    use ibc_proto::ics23::{commitment_proof, ExistenceProof, InnerOp};
    tracing::trace!(target:"ibc-rs","in substrate: [compose_ibc_merkle_proof]");

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
pub fn get_dummy_merkle_proof() -> MerkleProof {
    let parsed = ibc_proto::ics23::CommitmentProof { proof: None };
    let mproofs: Vec<ibc_proto::ics23::CommitmentProof> = vec![parsed];
    MerkleProof { proofs: mproofs }
}

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
