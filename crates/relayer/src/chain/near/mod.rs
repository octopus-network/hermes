use super::client::ClientSettings;
use crate::config::ChainConfig;
use crate::error::Error;
use crate::event::substrate_mointor::{EventMonitor, EventReceiver, TxMonitorCmd};
use crate::keyring::{KeyEntry, KeyRing};
use tracing::{debug, info, trace};

use crate::chain::endpoint::ChainEndpoint;
use crate::chain::endpoint::ChainStatus;
use crate::chain::endpoint::HealthCheck;

use crate::account::Balance;
use crate::chain::requests::QueryChannelClientStateRequest;
use crate::chain::requests::QueryChannelRequest;
use crate::chain::requests::QueryChannelsRequest;
use crate::chain::requests::QueryClientConnectionsRequest;
use crate::chain::requests::QueryClientStatesRequest;
use crate::chain::requests::QueryConnectionChannelsRequest;
use crate::chain::requests::QueryConnectionRequest;
use crate::chain::requests::QueryConnectionsRequest;
use crate::chain::requests::QueryConsensusStateRequest;
use crate::chain::requests::QueryConsensusStatesRequest;
use crate::chain::requests::QueryNextSequenceReceiveRequest;
use crate::chain::requests::QueryPacketAcknowledgementsRequest;
use crate::chain::requests::QueryPacketCommitmentsRequest;
use crate::chain::requests::QueryPacketEventDataRequest;
use crate::chain::requests::QueryTxRequest;
use crate::chain::requests::QueryUnreceivedAcksRequest;
use crate::chain::requests::QueryUnreceivedPacketsRequest;
use crate::chain::requests::{
    IncludeProof, QueryClientStateRequest, QueryHeight, QueryHostConsensusStateRequest,
    QueryPacketAcknowledgementRequest, QueryPacketCommitmentRequest, QueryPacketReceiptRequest,
    QueryUpgradedClientStateRequest, QueryUpgradedConsensusStateRequest,
};
use crate::chain::substrate::rpc::get_header_by_block_number;
use crate::chain::substrate::rpc::get_mmr_leaf_and_mmr_proof;
use crate::chain::substrate::rpc::subscribe_beefy;
use crate::chain::substrate::update_client_state::build_validator_proof;
use crate::chain::tracking::{TrackedMsgs, TrackingId};
use crate::client_state::{AnyClientState, IdentifiedAnyClientState};
use crate::consensus_state::{AnyConsensusState, AnyConsensusStateWithHeight};
use crate::denom::DenomTrace;
use crate::event::IbcEventWithHeight;
use crate::misbehaviour::MisbehaviourEvidence;
use alloc::sync::Arc;
use anyhow::Result;
use codec::Decode;
use core::fmt::Debug;
use core::{future::Future, str::FromStr};
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::channel::v1::PacketState;
use ibc_relayer_types::clients::ics10_grandpa::help::Commitment;
use ibc_relayer_types::clients::ics10_grandpa::help::MmrRoot;
use ibc_relayer_types::clients::ics10_grandpa::help::SignedCommitment;
use ibc_relayer_types::clients::ics10_grandpa::help::ValidatorMerkleProof;
use ibc_relayer_types::core::ics02_client::events::UpdateClient;
use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentRoot;
use ibc_relayer_types::core::ics23_commitment::merkle::MerkleProof;
use ibc_relayer_types::core::ics24_host::path::{
    AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath, CommitmentsPath,
    ConnectionsPath, ReceiptsPath, SeqRecvsPath,
};
use ibc_relayer_types::{
    clients::ics10_grandpa::{
        client_state::ClientState as GPClientState,
        consensus_state::ConsensusState as GPConsensusState, header::Header as GPHeader,
    },
    core::{
        ics02_client::client_type::ClientType,
        ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd},
        ics04_channel::{
            channel::{ChannelEnd, IdentifiedChannelEnd},
            packet::{Receipt, Sequence},
        },
        ics23_commitment::commitment::CommitmentPrefix,
        ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
    },
    events::IbcEvent,
    signer::Signer,
    Height, Height as ICSHeight,
};
use semver::Version;
use serde::{Deserialize, Serialize};
use sp_core::sr25519;
use sp_core::{hexdisplay::HexDisplay, Pair, H256};
use sp_runtime::{traits::IdentifyAccount, MultiSigner};
use std::thread;
use subxt::{self, rpc::BlockNumber, rpc::NumberOrHex, OnlineClient};
use subxt::rpc::RpcClient;
use tendermint::abci::transaction;
use tendermint::abci::{Code, Log};
use tendermint::time::Time;
use tendermint_rpc::endpoint::broadcast::tx_sync::Response as TxResponse;
use tokio::runtime::Runtime as TokioRuntime;

pub const REVISION_NUMBER: u64 = 0;

/// A struct used to start a Near chain instance in relayer
#[derive(Debug)]
pub struct NearChain {
    client: RpcClient, //todo!()//Bob,
    config: ChainConfig,
    keybase: KeyRing,
    rt: Arc<TokioRuntime>,
}

impl NearChain {
    pub fn config(&self) -> &ChainConfig {
        &self.config
    }

    /// Run a future to completion on the Tokio runtime.
    fn block_on<F: Future>(&self, f: F) -> F::Output {
        self.rt.block_on(f)
    }

    /// Subscribe Events
    fn subscribe_ibc_events(&self) -> Result<Vec<IbcEvent>, subxt::error::Error> {
        tracing::trace!("in near: [subscribe_ibc_events]");
        todo!()//Bob
    }

    /// get latest block height
    fn get_latest_height(&self) -> Result<Height, subxt::error::Error> {
        tracing::trace!("in near: [get_latest_height]");

        // let height_number = self.block_on(rpc::get_latest_height(self.client.clone()))?;
        // Ok(Height::new(0, height_number).expect("never failed"))
        todo!()//Bob
    }

    /// get connectionEnd by connection_identifier
    fn query_connection_end(
        &self,
        connection_identifier: &ConnectionId,
    ) -> Result<ConnectionEnd, subxt::error::Error> {
        tracing::trace!("in near: [query_connection_end]");
        // self.block_on(rpc::query_connection_end(
        //     connection_identifier,
        //     self.client.clone(),
        // ))
        todo!()//Bob
    }

    /// query channelEnd  by port_identifier, and channel_identifier
    fn query_channel_end(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<ChannelEnd, subxt::error::Error> {
        tracing::trace!("in near: [query_channel_end]");
        todo!()//Bob
    }

    /// get packet receipt by port_id, channel_id and sequence
    fn _get_packet_receipt(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
    ) -> Result<Receipt, subxt::error::Error> {
        tracing::trace!("in near: [get_packet_receipt]");
        todo!()//Bob
    }

    // TODO(davirain) need add query height
    /// get client_state by client_id
    fn query_client_state(
        &self,
        client_id: &ClientId,
    ) -> Result<AnyClientState, subxt::error::Error> {
        tracing::trace!("in near: [query_client_state]");
        todo!()//Bob
    }

    // TODO(davirain) need add query height
    /// Performs a query to retrieve the consensus state for a specified height
    /// `consensus_height` that the specified light client stores.
    fn query_client_consensus(
        &self,
        client_id: &ClientId,
        consensus_height: &ICSHeight,
    ) -> Result<AnyConsensusState, subxt::error::Error> {
        tracing::trace!("in near: [query_client_consensus]");
        todo!()//Bob
    }

    fn get_consensus_state_with_height(
        &self,
        client_id: &ClientId,
    ) -> Result<Vec<(Height, AnyConsensusState)>, subxt::error::Error> {
        tracing::trace!("in near: [get_consensus_state_with_height]");
        todo!()//Bob
    }

    fn get_unreceipt_packet(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequences: &[Sequence],
    ) -> Result<Vec<u64>, subxt::error::Error> {
        tracing::trace!("in near: [get_unreceipt_packet]");
        todo!()//Bob
    }

    fn get_clients(&self) -> Result<Vec<IdentifiedAnyClientState>, subxt::error::Error> {
        tracing::trace!("in near: [get_clients]");
        todo!()//Bob
    }

    fn get_connections(&self) -> Result<Vec<IdentifiedConnectionEnd>, subxt::error::Error> {
        tracing::trace!("in near: [get_connections]");
        todo!()//Bob
    }

    fn get_channels(&self) -> Result<Vec<IdentifiedChannelEnd>, subxt::error::Error> {
        tracing::trace!("in near: [get_channels]");
        todo!()//Bob
    }

    fn get_commitment_packet_state(&self) -> Result<Vec<PacketState>, subxt::error::Error> {
        tracing::trace!("in near: [get_commitment_packet_state]");
        todo!()//Bob
    }

    /// get packet commitment by port_id, channel_id and sequence
    fn get_packet_commitment(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: &Sequence,
    ) -> Result<Vec<u8>, subxt::error::Error> {
        tracing::trace!("in near: [get_packet_commitment]");
        todo!()//Bob
    }

    fn get_acknowledge_packet_state(&self) -> Result<Vec<PacketState>, subxt::error::Error> {
        tracing::trace!("in near: [get_acknowledge_packet_state]");
        todo!()//Bob
    }

    /// get connection_identifier vector by client_identifier
    fn get_client_connections(
        &self,
        client_id: &ClientId,
    ) -> Result<Vec<ConnectionId>, subxt::error::Error> {
        tracing::trace!("in near: [get_client_connections]");
        todo!()//Bob
    }

    fn get_connection_channels(
        &self,
        connection_id: &ConnectionId,
    ) -> Result<Vec<IdentifiedChannelEnd>, subxt::error::Error> {
        tracing::trace!("in near: [get_connection_channels]");
        todo!()//Bob
    }

    /// The function to submit IBC request to a Near chain
    /// This function handles most of the IBC reqeusts to Near, except the MMR root update
    fn deliever(&self, msgs: Vec<Any>) -> Result<H256, subxt::error::Error> {
        info!("in near: [deliever]");
        todo!()//Bob
    }

    fn raw_transfer(&self, msgs: Vec<Any>) -> Result<H256, subxt::error::Error> {
        tracing::trace!("in near: [raw_transfer]");
        todo!()//Bob
    }

    /// Retrieve the storage proof according to storage keys
    /// And convert the proof to IBC compatible type
    fn generate_storage_proof<'a>(
        &self,
        storage_entry: impl IntoIterator<Item = &'a [u8]>,
        height: &Height,
        _storage_name: &str,
    ) -> Result<MerkleProof, Error> {
        todo!()//Bob
    }

    fn query_packet_commitment(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: &Sequence,
    ) -> Result<Vec<u8>, subxt::error::Error> {
        todo!()//Bob
    }

    fn query_packet_receipt(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: &Sequence,
    ) -> Result<Vec<u8>, subxt::error::Error> {
        todo!()//Bob
    }

    fn query_next_sequence_receive(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<Sequence, subxt::error::Error> {
        todo!()//Bob
    }

    fn query_packet_acknowledgement(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: &Sequence,
    ) -> Result<Vec<u8>, subxt::error::Error> {
        todo!()//Bob
    }
}

impl ChainEndpoint for NearChain {
    type LightBlock = GPHeader; // Todo: Import from Near light client //CS
    type Header = GPHeader; // Todo: Import from Near light client //CS
    type ConsensusState = GPConsensusState; // Todo: Import from Near light client //CS
    type ClientState = GPClientState;  // Todo: Import from Near light client //CS

    fn id(&self) -> &ChainId {
        &self.config().id
    }

    fn config(&self) -> ChainConfig {
        self.config().clone()
    }

    fn bootstrap(config: ChainConfig, rt: Arc<TokioRuntime>) -> Result<Self, Error> {
        tracing::info!("in near: [bootstrap function]");
        todo!()//Bob
    }

    fn init_event_monitor(
        &self,
        rt: Arc<TokioRuntime>,
    ) -> Result<(EventReceiver, TxMonitorCmd), Error> {
        debug!(
            "in near: [init_event_mointor] >> websocket addr: {:?}",
            self.config.websocket_addr.clone()
        );
        todo!()//Bob
    }

    fn shutdown(self) -> Result<(), Error> {
        Ok(())
    }

    fn health_check(&self) -> Result<HealthCheck, Error> {
        Ok(HealthCheck::Healthy)
    }

    // keyring

    fn keybase(&self) -> &KeyRing {
        &self.keybase
    }

    fn keybase_mut(&mut self) -> &mut KeyRing {
        &mut self.keybase
    }

    fn get_signer(&self) -> Result<Signer, Error> {
        trace!("In near: [get signer]");
        crate::time!("get_signer");
        // Todo: get Near Signer //CS
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
            .map_err(|e| Error::key_not_found(self.config.key_name.clone(), e))?;

        let private_seed = key.private_key.private_key;

        let pair = sr25519::Pair::from_seed_slice(private_seed.as_ref())
        .map_err(|e| Error::report_error(format!("{:?}", e)))?;
        let public_key = pair.public();

        let account_id = format_account_id::<sr25519::Pair>(public_key);

        Ok(Signer::from_str(&account_id).unwrap())
    }

    fn get_key(&mut self) -> Result<KeyEntry, Error> {
        tracing::trace!("in near: [get_key]");
        crate::time!("get_key");

        // Get the key from key seed file
        let key = self
            .keybase()
            .get_key(&self.config.key_name)
            .map_err(|e| Error::key_not_found(self.config.key_name.clone(), e))?;

        Ok(key)
    }

    fn add_key(&mut self, key_name: &str, key: KeyEntry) -> Result<(), Error> {
        self.keybase_mut()
            .add_key(key_name, key)
            .map_err(Error::key_base)?;

        Ok(())
    }

    // versioning
    fn ibc_version(&self) -> Result<Option<Version>, Error> {
        // todo(davirian)
        Ok(None)
    }

    // send transactions

    fn send_messages_and_wait_commit(
        &mut self,
        proto_msgs: TrackedMsgs,
    ) -> Result<Vec<IbcEventWithHeight>, Error> {
        info!(
            "in near: [send_messages_and_wait_commit], proto_msgs={:?}",
            proto_msgs.tracking_id
        );

        match proto_msgs.tracking_id {
            TrackingId::Uuid(_) => {
                let result = self.deliever(proto_msgs.messages().to_vec()).map_err(|e| {
                    Error::report_error(format!("deliever error ({:?})", e.to_string()))
                })?;
                debug!(
                    "in near: [send_messages_and_wait_commit] >> extrics_hash  : {:?}",
                    result
                );
            }
            TrackingId::Static(value) => match value {
                "ft-transfer" => {
                    let result = self
                        .raw_transfer(proto_msgs.messages().to_vec())
                        .map_err(|_| Error::report_error("ics20_transfer".to_string()))?;
                    debug!(
                        "in near: [send_messages_and_wait_commit] >> extrics_hash  : {:?}",
                        result
                    );
                }
                _ => {
                    let result = self.deliever(proto_msgs.messages().to_vec()).map_err(|e| {
                        Error::report_error(format!("deliever error ({:?})", e.to_string()))
                    })?;
                    debug!(
                        "in near: [send_messages_and_wait_commit] >> extrics_hash  : {:?}",
                        result
                    );
                }
            },
            TrackingId::ClearedUuid(_) => {}
        }

        let ibc_event = self
            .subscribe_ibc_events()
            .map_err(|_| Error::report_error("subscribe_ibc_events".to_string()))?;

        let height = self
            .get_latest_height()
            .map_err(|e| Error::report_error(format!("get_latest_height ({:?})", e.to_string())))?;

        let ibc_event_with_height = ibc_event
            .into_iter()
            .map(|value| IbcEventWithHeight {
                event: value,
                height: height.clone(),
            })
            .collect();

        Ok(ibc_event_with_height)
    }

    fn send_messages_and_wait_check_tx(
        &mut self,
        proto_msgs: TrackedMsgs,
    ) -> Result<Vec<TxResponse>, Error> {
        debug!(
            "in near: [send_messages_and_wait_check_tx], proto_msgs={:?}",
            proto_msgs.tracking_id
        );

        match proto_msgs.tracking_id {
            TrackingId::Uuid(_) => {
                let result = self.deliever(proto_msgs.messages().to_vec()).map_err(|e| {
                    Error::report_error(format!("deliever error ({:?})", e.to_string()))
                })?;
                debug!(
                    "in near: [send_messages_and_wait_commit] >> extrics_hash  : {:?}",
                    result
                );
            }
            TrackingId::Static(value) => match value {
                "ft-transfer" => {
                    let result = self
                        .raw_transfer(proto_msgs.messages().to_vec())
                        .map_err(|_| Error::report_error("ics20_transfer".to_string()))?;

                    debug!(
                        "in near: [send_messages_and_wait_commit] >> extrics_hash  : {:?}",
                        result
                    );
                }
                _ => {
                    let result = self.deliever(proto_msgs.messages().to_vec()).map_err(|e| {
                        Error::report_error(format!("deliever error ({:?})", e.to_string()))
                    })?;
                    debug!(
                        "in near: [send_messages_and_wait_commit] >> extrics_hash  : {:?}",
                        result
                    );
                }
            },
            TrackingId::ClearedUuid(_) => {}
        }

        let json = "\"ChYKFGNvbm5lY3Rpb25fb3Blbl9pbml0\"";
        let tx_re = TxResponse {
            code: Code::default(),
            data: serde_json::from_str(json)
                .map_err(|_| Error::report_error("invalid_serde_json_error".to_string()))?,
            log: Log::from("test_test"),
            hash: transaction::Hash::new([0u8; 32]),
        };

        Ok(vec![tx_re])
    }

    // Light client

    /// Fetch a header from the chain at the given height and verify it.
    fn verify_header(
            &mut self,
            _trusted: ICSHeight,
            _target: ICSHeight,
            _client_state: &AnyClientState,
    ) -> Result<Self::LightBlock, Error> {
        todo!()
    }

    /// Given a client update event that includes the header used in a client update,
    /// look for misbehaviour by fetching a header at same or latest height.
    fn check_misbehaviour(
        &mut self,
        _update: &UpdateClient,
        _client_state: &AnyClientState,
    ) -> Result<Option<MisbehaviourEvidence>, Error> {
        todo!()
    }

    // Queries

    fn query_balance(
            &self,
            _key_name: Option<&str>,
            _denom: Option<&str>,
    ) -> std::result::Result<Balance, Error> {
        // todo(davirain) add mock balance
        Ok(Balance {
            amount: String::default(),
            denom: String::default(),
        })
    }

    fn query_denom_trace(&self, _hash: String) -> std::result::Result<DenomTrace, Error> {
        // todo(daviarin) add mock denom trace
        Ok(DenomTrace {
            /// The chain of port/channel identifiers used for tracing the source of the coin.
            path: String::default(),
            /// The base denomination for that coin
            base_denom: String::default(),
        })
    }

    fn query_commitment_prefix(&self) -> Result<CommitmentPrefix, Error> {
        tracing::trace!("in near: [query_commitment_prefix]");

        // TODO - do a real chain query
        CommitmentPrefix::try_from(self.config().store_prefix.as_bytes().to_vec())
            .map_err(|_| Error::report_error("invalid_commitment_prefix".to_string()))
    }

    fn query_application_status(&self) -> Result<ChainStatus, Error> {
        tracing::trace!("in near: [query_status]");

        let latest_height = self
            .get_latest_height()
            .map_err(|_| Error::report_error("get_latest_height".to_string()))?;

        Ok(ChainStatus {
            height: latest_height,
            timestamp: Default::default(),
        })
    }

    fn query_clients(
        &self,
        _request: QueryClientStatesRequest,
    ) -> Result<Vec<IdentifiedAnyClientState>, Error> {
        tracing::trace!("in near: [query_clients]");

        let result = self
            .get_clients()
            .map_err(|_| Error::report_error("get_clients".to_string()))?;

        Ok(result)
    }

    fn query_client_state(
        &self,
        request: QueryClientStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyClientState, Option<MerkleProof>), Error> {
        tracing::trace!("in near: [query_client_state]");

        let QueryClientStateRequest { client_id, height } = request;

        let query_height = match height {
            QueryHeight::Latest => {
                let height = self
                    .get_latest_height()
                    .map_err(|_| Error::report_error("query_latest_height".to_string()))?;
                height
            }
            QueryHeight::Specific(value) => value,
        };

        let result = self
            .query_client_state(&client_id)
            .map_err(|_| Error::report_error("query_client_state".to_string()))?;

        let client_state_path = ClientStatePath(client_id.clone()).to_string();

        match include_proof {
            IncludeProof::Yes => Ok((
                result,
                Some(self.generate_storage_proof(
                    vec![client_state_path.as_bytes()],
                    &query_height,
                    "ClientStates",
                )?),
            )),
            IncludeProof::No => Ok((result, None)),
        }
    }

    fn query_consensus_state(
        &self,
        request: QueryConsensusStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyConsensusState, Option<MerkleProof>), Error> {
        tracing::trace!("in near: [query_consensus_state]");

        // query_height to amit to search chain height
        let QueryConsensusStateRequest {
            client_id,
            consensus_height,
            query_height: _,
        } = request;

        let result = self
            .query_client_consensus(&client_id, &consensus_height)
            .map_err(|_| Error::report_error("query_client_consensus".to_string()))?;

        // search key
        let client_consensus_state_path = ClientConsensusStatePath {
            client_id: client_id.clone(),
            epoch: consensus_height.revision_number(),
            height: consensus_height.revision_height(),
        }
        .to_string();

        match include_proof {
            IncludeProof::Yes => Ok((
                result,
                Some(self.generate_storage_proof(
                    vec![client_consensus_state_path.as_bytes()],
                    &consensus_height,
                    "ConsensusStates",
                )?),
            )),
            IncludeProof::No => Ok((result, None)),
        }
    }

    fn query_consensus_states(
        &self,
        request: QueryConsensusStatesRequest,
    ) -> Result<Vec<AnyConsensusStateWithHeight>, Error> {
        tracing::trace!("in near: [query_consensus_states]");

        let request_client_id = ClientId::from_str(request.client_id.as_str())
            .map_err(|_| Error::report_error("identifier".to_string()))?;

        let result = self
            .get_consensus_state_with_height(&request_client_id)
            .map_err(|_| Error::report_error("get_consensus_state_with_height".to_string()))?;

        let consensus_state: Vec<(Height, AnyConsensusState)> = result;

        let mut any_consensus_state_with_height = vec![];
        for (height, consensus_state) in consensus_state.into_iter() {
            let tmp = AnyConsensusStateWithHeight {
                height,
                consensus_state,
            };
            any_consensus_state_with_height.push(tmp.clone());

            tracing::trace!(
                "in near: [query_consensus_state] >> any_consensus_state_with_height: {:?}",
                tmp
            );
        }

        any_consensus_state_with_height.sort_by(|a, b| a.height.cmp(&b.height));

        Ok(any_consensus_state_with_height)
    }

    fn query_upgraded_client_state(
        &self,
        _request: QueryUpgradedClientStateRequest,
    ) -> Result<(AnyClientState, MerkleProof), Error> {
        tracing::trace!("in near: [query_upgraded_client_state]");

        todo!()
    }

    fn query_upgraded_consensus_state(
        &self,
        _request: QueryUpgradedConsensusStateRequest,
    ) -> Result<(AnyConsensusState, MerkleProof), Error> {
        tracing::trace!("in near: [query_upgraded_consensus_state]");

        todo!()
    }

    fn query_connections(
        &self,
        _request: QueryConnectionsRequest,
    ) -> Result<Vec<IdentifiedConnectionEnd>, Error> {
        tracing::trace!("in near: [query_connections]");

        let result = self
            .get_connections()
            .map_err(|_| Error::report_error("get_connections".to_string()))?;

        Ok(result)
    }

    fn query_client_connections(
        &self,
        request: QueryClientConnectionsRequest,
    ) -> Result<Vec<ConnectionId>, Error> {
        tracing::trace!("in near: [query_client_connections]");
        todo!()//Bob
    }

    fn query_connection(
        &self,
        request: QueryConnectionRequest,
        include_proof: IncludeProof,
    ) -> Result<(ConnectionEnd, Option<MerkleProof>), Error> {
        tracing::trace!("in near: [query_connection]");

        let QueryConnectionRequest {
            connection_id,
            height,
        } = request;

        let connection_end = self
            .query_connection_end(&connection_id)
            .map_err(|_| Error::report_error("query_connection_end".to_string()))?;

        // update ConnectionsPath key
        let connections_path = ConnectionsPath(connection_id.clone())
            .to_string();

        match include_proof {
            IncludeProof::Yes => {
                let query_height = match height {
                    QueryHeight::Latest => {
                        let height = self
                            .get_latest_height()
                            .map_err(|_| Error::report_error("query_latest_height".to_string()))?;
                        height
                    }
                    QueryHeight::Specific(value) => value,
                };

                Ok((
                    connection_end,
                    Some(self.generate_storage_proof(
                            vec![connections_path.as_bytes()],
                        &query_height,
                        "Connections",
                    )?),
                ))
            }
            IncludeProof::No => Ok((connection_end, None)),
        }
    }

    fn query_connection_channels(
        &self,
        request: QueryConnectionChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        tracing::trace!("in near: [query_connection_channels] ");

        let result = self
            .get_connection_channels(&request.connection_id)
            .map_err(|_| Error::report_error("get_connection_channels".to_string()))?;

        Ok(result)
    }

    fn query_channels(
        &self,
        _request: QueryChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        tracing::trace!("in near: [query_channels]");

        let result = self
            .get_channels()
            .map_err(|_| Error::report_error("get_channels".to_string()))?;

        Ok(result)
    }

    fn query_channel(
        &self,
        request: QueryChannelRequest,
        include_proof: IncludeProof,
    ) -> Result<(ChannelEnd, Option<MerkleProof>), Error> {
        tracing::trace!("in near: [query_channel]");

        let QueryChannelRequest {
            port_id,
            channel_id,
            height,
        } = request;

        let channel_end = self
            .query_channel_end(&port_id, &channel_id)
            .map_err(|_| Error::report_error("query_channel_end".to_string()))?;

        // use channel_end path as key
        let channel_end_path = ChannelEndsPath(port_id.clone(), channel_id.clone())
            .to_string();

        match include_proof {
            IncludeProof::Yes => {
                let query_height = match height {
                    QueryHeight::Latest => {
                        let height = self
                            .get_latest_height()
                            .map_err(|_| Error::report_error("query_latest_height".to_string()))?;
                        height
                    }
                    QueryHeight::Specific(value) => value,
                };

                Ok((
                    channel_end,
                    Some(self.generate_storage_proof(
                            vec![channel_end_path.as_bytes()],
                        &query_height,
                        "Channels",
                    )?),
                ))
            }
            IncludeProof::No => Ok((channel_end, None)),
        }
    }

    fn query_channel_client_state(
        &self,
        _request: QueryChannelClientStateRequest,
    ) -> Result<Option<IdentifiedAnyClientState>, Error> {
        tracing::trace!("in near: [query_channel_client_state]");

        todo!()
    }

    fn query_packet_commitment(
        &self,
        request: QueryPacketCommitmentRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        let QueryPacketCommitmentRequest {
            port_id,
            channel_id,
            sequence,
            height,
        } = request;

        let packet_commit = self
            .query_packet_commitment(&port_id, &channel_id, &sequence)
            .map_err(|_| Error::report_error("query_packet_commitment".to_string()))?;

        let packet_commits_path = CommitmentsPath {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            sequence: sequence.clone(),
        }
        .to_string();

        match include_proof {
            IncludeProof::Yes => {
                let query_height = match height {
                    QueryHeight::Latest => {
                        let height = self
                            .get_latest_height()
                            .map_err(|_| Error::report_error("query_latest_height".to_string()))?;
                        height
                    }
                    QueryHeight::Specific(value) => value,
                };
                Ok((
                    packet_commit,
                    Some(self.generate_storage_proof(
                            vec![packet_commits_path.as_bytes()],
                        &query_height,
                        "PacketCommitment",
                    )?),
                ))
            }
            IncludeProof::No => Ok((packet_commit, None)),
        }
    }

    fn query_packet_commitments(
        &self,
        _request: QueryPacketCommitmentsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), Error> {
        tracing::trace!("in near: [query_packet_commitments]");

        let packet_commitments = self
            .get_commitment_packet_state()
            .map_err(|_| Error::report_error("get_commitment_packet_state".to_string()))?
            .into_iter()
            .map(|value| Sequence::from(value.sequence))
            .collect();

        let latest_height = self
            .get_latest_height()
            .map_err(|_| Error::report_error("get_latest_height_error".to_string()))?;

        Ok((packet_commitments, latest_height))
    }

    fn query_packet_receipt(
        &self,
        request: QueryPacketReceiptRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        let QueryPacketReceiptRequest {
            port_id,
            channel_id,
            sequence,
            height,
        } = request;

        let packet_receipt = self
            .query_packet_receipt(&port_id, &channel_id, &sequence)
            .map_err(|_| Error::report_error("query_packet_receipt".to_string()))?;

        let packet_receipt_path = ReceiptsPath {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            sequence: sequence.clone(),
        }
        .to_string();

        match include_proof {
            IncludeProof::Yes => {
                let query_height = match height {
                    QueryHeight::Latest => {
                        let height = self
                            .get_latest_height()
                            .map_err(|_| Error::report_error("query_latest_height".to_string()))?;
                        height
                    }
                    QueryHeight::Specific(value) => value,
                };
                Ok((
                    packet_receipt,
                    Some(self.generate_storage_proof(
                            vec![packet_receipt_path.as_bytes()],
                        &query_height,
                        "PacketReceipt",
                    )?),
                ))
            }
            IncludeProof::No => Ok((packet_receipt, None)),
        }
    }

    fn query_unreceived_packets(
        &self,
        request: QueryUnreceivedPacketsRequest,
    ) -> Result<Vec<Sequence>, Error> {
        tracing::trace!("in near: [query_unreceived_packets]");

        let port_id = PortId::from_str(request.port_id.as_str())
            .map_err(|_| Error::report_error("identifier".to_string()))?;
        let channel_id = ChannelId::from_str(request.channel_id.as_str())
            .map_err(|_| Error::report_error("identifier".to_string()))?;
        let sequences = request
            .packet_commitment_sequences
            .into_iter()
            .map(Sequence::from)
            .collect::<Vec<_>>();

        let result = self
            .get_unreceipt_packet(&port_id, &channel_id, &sequences)
            .map_err(|_| Error::report_error("get_unreceipt_packet".to_string()))?
            .into_iter()
            .map(|value| Sequence::from(value))
            .collect();

        Ok(result)
    }

    fn query_packet_acknowledgement(
        &self,
        request: QueryPacketAcknowledgementRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        let QueryPacketAcknowledgementRequest {
            port_id,
            channel_id,
            sequence,
            height,
        } = request;

        let packet_acknowledgement = self
            .query_packet_acknowledgement(&port_id, &channel_id, &sequence)
            .map_err(|_| Error::report_error("query_packet_acknowledgement".to_string()))?;

        let packet_acknowledgement_path = AcksPath {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            sequence: sequence.clone(),
        }
        .to_string();

        match include_proof {
            IncludeProof::Yes => {
                let query_height = match height {
                    QueryHeight::Latest => {
                        let height = self
                            .get_latest_height()
                            .map_err(|_| Error::report_error("query_latest_height".to_string()))?;
                        height
                    }
                    QueryHeight::Specific(value) => value,
                };
                Ok((
                    packet_acknowledgement,
                    Some(self.generate_storage_proof(
                            vec![packet_acknowledgement_path.as_bytes()],
                        &query_height,
                        "Acknowledgements",
                    )?),
                ))
            }
            IncludeProof::No => Ok((packet_acknowledgement, None)),
        }
    }

    fn query_packet_acknowledgements(
        &self,
        _request: QueryPacketAcknowledgementsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), Error> {
        tracing::trace!("in near: [query_packet_acknowledgements]");

        let packet_acknowledgements = self
            .get_acknowledge_packet_state()
            .map_err(|_| Error::report_error("get_acknowledge_packet_state".to_string()))?
            .into_iter()
            .map(|value| Sequence::from(value.sequence))
            .collect();

        let latest_height = self
            .get_latest_height()
            .map_err(|_| Error::report_error("get_latest_height".to_string()))?;

        Ok((packet_acknowledgements, latest_height))
    }

    fn query_unreceived_acknowledgements(
        &self,
        request: QueryUnreceivedAcksRequest,
    ) -> Result<Vec<Sequence>, Error> {
        tracing::trace!("in near: [query_unreceived_acknowledgements] ");

        let port_id = PortId::from_str(request.port_id.as_str())
            .map_err(|_| Error::report_error("identifier".to_string()))?;
        let channel_id = ChannelId::from_str(request.channel_id.as_str())
            .map_err(|_| Error::report_error("identifier".to_string()))?;
        let sequences = request
            .packet_ack_sequences
            .into_iter()
            .map(Sequence::from)
            .collect::<Vec<_>>();

        let mut unreceived_seqs = vec![];

        for seq in sequences {
            let cmt = self.get_packet_commitment(&port_id, &channel_id, &seq);

            // if packet commitment still exists on the original sending chain, then packet ack is unreceived
            // since processing the ack will delete the packet commitment
            if let Ok(_) = cmt {
                unreceived_seqs.push(seq);
            }
        }

        Ok(unreceived_seqs)
    }

    fn query_next_sequence_receive(
        &self,
        request: QueryNextSequenceReceiveRequest,
        include_proof: IncludeProof,
    ) -> Result<(Sequence, Option<MerkleProof>), Error> {
        tracing::trace!("in near: [query_next_sequence_receive] ");

        let QueryNextSequenceReceiveRequest {
            port_id,
            channel_id,
            height,
        } = request;

        let next_sequence_receive = self
            .query_next_sequence_receive(&port_id, &channel_id)
            .map_err(|_| Error::report_error("query_next_sequence_receive".to_string()))?;

        let next_sequence_receive_path = SeqRecvsPath(port_id.clone(), channel_id.clone())
        .to_string();

        match include_proof {
            IncludeProof::Yes => {
                let query_height = match height {
                    QueryHeight::Latest => {
                        let height = self
                            .get_latest_height()
                            .map_err(|_| Error::report_error("query_latest_height".to_string()))?;
                        height
                    }
                    QueryHeight::Specific(value) => value,
                };
                Ok((
                    next_sequence_receive,
                    Some(self.generate_storage_proof(
                            vec![next_sequence_receive_path.as_bytes()],
                        &query_height,
                        "NextSequenceRecv",
                    )?),
                ))
            }
            IncludeProof::No => Ok((next_sequence_receive, None)),
        }
    }

    fn query_txs(&self, request: QueryTxRequest) -> Result<Vec<IbcEventWithHeight>, Error> {
        tracing::trace!("in near: [query_txs]");

        match request {
            QueryTxRequest::Client(request) => {
                use ibc_relayer_types::core::ics02_client::events::Attributes;
                // Todo: the client event below is mock
                // replace it with real client event replied from a near chain
                // todo(davirian)
                let result: Vec<IbcEventWithHeight> = vec![IbcEventWithHeight {
                    event: IbcEvent::UpdateClient(
                        ibc_relayer_types::core::ics02_client::events::UpdateClient::from(
                            Attributes {
                                client_id: request.client_id,
                                client_type: ClientType::Grandpa,
                                consensus_height: request.consensus_height,
                            },
                        ),
                    ),
                    height: Height::new(0, 9).unwrap(),
                }];

                Ok(result)
            }

            QueryTxRequest::Transaction(_tx) => {
                // Todo: https://github.com/octopus-network/ibc-rs/issues/98
                let result: Vec<IbcEventWithHeight> = vec![];
                Ok(result)
            }
        }
    }

    fn query_host_consensus_state(
        &self,
        _request: QueryHostConsensusStateRequest,
    ) -> Result<Self::ConsensusState, Error> {
        tracing::trace!("in near: [query_host_consensus_state]");

        Ok(GPConsensusState::default())
    }

    fn build_client_state(
        &self,
        height: ICSHeight,
        _dst_config: ClientSettings,
    ) -> Result<Self::ClientState, Error> {
        tracing::trace!("in near: [build_client_state]");
        // let public_key = async {
        //     // Address to a storage entry we'd like to access.
        //     let address = ibc_node::storage().beefy().authorities();
        //
        //     let authorities = self
        //         .client
        //         .storage()
        //         .fetch(&address, None)
        //         .await
        //         .map_err(|_| Error::report_error("get authorities error".to_string()))?
        //         .ok_or(Error::report_error("empty authorities".into()))?;
        //
        //     // covert authorities to strings
        //
        //     let result: Vec<String> = authorities
        //         .into_inner()
        //         .into_iter()
        //         .map(|val| format!("0x{}", HexDisplay::from(&format!("{:?}", val).into_bytes()))) // todo(davirain) HexDisplay beefy_primitives::crypto::Public
        //         .collect();
        //
        //     Ok(result)
        // };
        // let public_key = self.block_on(public_key)?;
        //
        // let beefy_light_client = beefy_light_client::new(public_key);
        //
        // // Build client state
        // let client_state = GPClientState::new(
        //     self.id().clone(),
        //     height.revision_height() as u32,
        //     Commitment::default(),
        //     beefy_light_client.validator_set.into(),
        // )
        //     .map_err(|_| Error::report_error("ics10 grandpa client state create error".to_string()))?;
        //
        // Ok(client_state)
        todo!()//Bob
    }

    fn build_consensus_state(
        &self,
        light_block: Self::LightBlock,
    ) -> Result<Self::ConsensusState, Error> {
        tracing::trace!(
            "in near: [build_consensus_state] light_block:{:?}",
            light_block
        );
        //build consensus state from header
        // let commitment = light_block.mmr_root.signed_commitment.commitment.unwrap();
        // let state_root = CommitmentRoot::from_bytes(&light_block.block_header.state_root);
        // let consensue_state = GPConsensusState::new(commitment, state_root, light_block.timestamp);
        // Ok(consensue_state)
        todo!()//Bob
    }

    fn build_header(
        &mut self,
        trusted_height: ICSHeight,
        target_height: ICSHeight,
        client_state: &AnyClientState,
    ) -> Result<(Self::Header, Vec<Self::Header>), Error> {
        tracing::trace!("in near [build_header]");
        todo!()//Bob
    }

    fn query_all_balances(&self, _key_name: Option<&str>) -> Result<Vec<Balance>, Error> {
        todo!()
    }

    fn query_packet_events(
        &self,
        _request: QueryPacketEventDataRequest,
    ) -> Result<Vec<IbcEventWithHeight>, Error> {
        todo!()
    }

    fn maybe_register_counterparty_payee(
        &mut self,
        _channel_id: &ChannelId,
        _port_id: &PortId,
        _counterparty_payee: &Signer,
    ) -> Result<(), Error> {
        todo!()
    }
}

// Todo: to create a new type in `commitment_proof::Proof`
/// Compose merkle proof according to ibc proto
pub fn compose_ibc_merkle_proof(proof: String) -> MerkleProof {
    use ics23::{commitment_proof, ExistenceProof, InnerOp};
    tracing::trace!("in near: [compose_ibc_merkle_proof]");

    let _inner_op = InnerOp {
        hash: 0,
        prefix: vec![0],
        suffix: vec![0],
    };

    let proof = commitment_proof::Proof::Exist(ExistenceProof {
        key: vec![0],
        value: proof.as_bytes().to_vec(),
        leaf: None,
        path: vec![_inner_op],
    });

    let parsed = ics23::CommitmentProof { proof: Some(proof) };
    let mproofs: Vec<ics23::CommitmentProof> = vec![parsed];
    MerkleProof { proofs: mproofs }
}
pub fn get_dummy_merkle_proof() -> MerkleProof {
    let parsed = ics23::CommitmentProof { proof: None };
    let mproofs: Vec<ics23::CommitmentProof> = vec![parsed];
    MerkleProof { proofs: mproofs }
}
