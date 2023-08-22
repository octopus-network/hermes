use super::client::ClientSettings;
use crate::{
    account::Balance,
    chain::endpoint::{ChainEndpoint, ChainStatus, HealthCheck},
    chain::handle::Subscription,
    chain::near::{
        contract::NearIbcContract,
        rpc::{client::NearRpcClient, tool::convert_ibc_event_to_hermes_ibc_event},
    },
    chain::requests::{
        CrossChainQueryRequest, QueryChannelClientStateRequest, QueryChannelRequest,
        QueryChannelsRequest, QueryClientConnectionsRequest, QueryClientStatesRequest,
        QueryConnectionChannelsRequest, QueryConnectionRequest, QueryConnectionsRequest,
        QueryConsensusStateHeightsRequest, QueryConsensusStateRequest,
        QueryNextSequenceReceiveRequest, QueryPacketAcknowledgementsRequest,
        QueryPacketCommitmentsRequest, QueryPacketEventDataRequest, QueryTxRequest,
        QueryUnreceivedAcksRequest, QueryUnreceivedPacketsRequest,
    },
    chain::requests::{
        IncludeProof, QueryClientStateRequest, QueryHeight, QueryHostConsensusStateRequest,
        QueryPacketAcknowledgementRequest, QueryPacketCommitmentRequest, QueryPacketReceiptRequest,
        QueryUpgradedClientStateRequest, QueryUpgradedConsensusStateRequest,
    },
    chain::tracking::{TrackedMsgs, TrackingId},
    client_state::{AnyClientState, IdentifiedAnyClientState},
    config::ChainConfig,
    connection::ConnectionMsgType,
    consensus_state::AnyConsensusState,
    denom::DenomTrace,
    error::Error,
    event::{
        near_event_monitor::{NearEventMonitor, TxMonitorCmd},
        IbcEventWithHeight,
    },
    keyring::{KeyRing, Secp256k1KeyPair, SigningKeyPair},
    misbehaviour::MisbehaviourEvidence,
};
use alloc::{string::String, sync::Arc};
use anyhow::Result;
use bitcoin::block;
use core::{fmt::Debug, future::Future, str::FromStr};
use ibc_proto::{
    google::protobuf::Any,
    ibc::lightclients::solomachine::v2::{
        ChannelStateData, ClientStateData, ConnectionStateData, ConsensusStateData, DataType,
        SignBytes, TimestampedSignatureData,
    },
    protobuf::Protobuf,
};
use ibc_relayer_types::{
    applications::ics31_icq::response::CrossChainQueryResponse,
    clients::{
        ics06_solomachine::consensus_state::ConsensusState as SmConsensusState,
        ics06_solomachine::header::{Header as SmHeader, HeaderData as SmHeaderData},
    },
    core::ics02_client::events::UpdateClient,
    core::ics23_commitment::merkle::MerkleProof,
    core::ics24_host::path::{
        AcksPath, ChannelEndsPath, CommitmentsPath, ConnectionsPath, ReceiptsPath, SeqRecvsPath,
    },
    core::{
        ics02_client::{client_type::ClientType, error::Error as ClientError},
        ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd},
        ics04_channel::{
            channel::{ChannelEnd, IdentifiedChannelEnd},
            packet::{Receipt, Sequence},
        },
        ics23_commitment::commitment::CommitmentPrefix,
        ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId},
    },
    core::{
        ics03_connection::connection::State,
        ics23_commitment::commitment::{CommitmentProofBytes, CommitmentRoot},
    },
    events::IbcEvent,
    proofs::{ConsensusProof, Proofs},
    signer::Signer,
    timestamp::Timestamp,
    Height, Height as ICSHeight,
};
use ibc_relayer_types::{
    clients::ics12_near::{
        client_state::ClientState as NearClientState,
        consensus_state::ConsensusState as NearConsensusState,
        header::Header as NearHeader,
        near_types::{
            hash::CryptoHash,
            signature::{ED25519PublicKey, PublicKey, Signature},
            BlockHeaderInnerLite, EpochId, LightClientBlock, ValidatorStakeView,
            ValidatorStakeViewV1,
        },
    },
    core::ics02_client::header::Header,
};
use near_crypto::{InMemorySigner, KeyType};
use near_primitives::types::BlockId;
use near_primitives::{types::AccountId, views::FinalExecutionOutcomeView};
use prost::Message;
use semver::Version;
use serde_json::json;
use std::{thread, time::SystemTime};
use tendermint_rpc::endpoint::broadcast::tx_sync::Response as TxResponse;
use tokio::runtime::Runtime as TokioRuntime;
use tracing::{debug, info, trace};

pub mod constants;
pub mod contract;
pub mod error;
// pub mod light_client;
pub mod rpc;

use error::NearError;

pub const REVISION_NUMBER: u64 = 0;
pub const CLIENT_DIVERSIFIER: &str = "NEAR";
pub const CONTRACT_ACCOUNT_ID: &str = "v3.nearibc.testnet";
pub const SIGNER_ACCOUNT_TESTNET: &str = "juliansun.testnet";
const MINIMUM_ATTACHED_NEAR_FOR_DELEVER_MSG: u128 = 100_000_000_000_000_000_000_000;

/// A struct used to start a Near chain instance in relayer
#[derive(Debug)]
pub struct NearChain {
    client: NearRpcClient,
    config: ChainConfig,
    keybase: KeyRing<Secp256k1KeyPair>,
    near_ibc_contract: AccountId,
    rt: Arc<TokioRuntime>,
    tx_monitor_cmd: Option<TxMonitorCmd>,
    signing_key_pair: Option<Secp256k1KeyPair>,
}

impl NearIbcContract for NearChain {
    fn get_contract_id(&self) -> AccountId {
        self.near_ibc_contract.clone()
    }

    fn get_client(&self) -> &NearRpcClient {
        &self.client
    }

    fn get_rt(&self) -> &Arc<TokioRuntime> {
        &self.rt
    }
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
    /// todo near don't have events subscription
    pub fn subscribe_ibc_events(&self) -> Result<Vec<IbcEvent>> {
        info!("{}: [subscribe_ibc_events]", self.id());
        todo!() //Bob
    }

    /// get packet receipt by port_id, channel_id and sequence
    fn _get_packet_receipt(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: &Sequence,
    ) -> Result<Receipt> {
        info!(
            "{}: [get_packet_receipt] - port_id: {}, channel_id: {}, seq: {}",
            self.id(),
            port_id,
            channel_id,
            seq
        );

        let _port_id = serde_json::to_string(port_id).map_err(NearError::SerdeJsonError)?;
        let _channel_id = serde_json::to_string(channel_id).map_err(NearError::SerdeJsonError)?;
        let _seq = serde_json::to_string(seq).map_err(NearError::SerdeJsonError)?;

        // self.block_on(self.client.view(
        //     self.near_ibc_contract.clone(),
        //     "get_packet_receipt".to_string(),
        //     json!({"port_id": port_id, "channel_id": channel_id, "seq": seq}).to_string().into_bytes()
        // )).expect("Failed to get_packet_receipt.").json()

        todo!() // todo the receipt can't deserialize
    }

    /// The function to submit IBC request to a Near chain
    /// This function handles most of the IBC reqeusts to Near, except the MMR root update
    fn deliver(&self, messages: Vec<Any>) -> Result<FinalExecutionOutcomeView> {
        info!("{}: [deliver] - messages: {:?}", self.id(), messages);

        let mut home_dir = dirs::home_dir().expect("Impossible to get your home dir!");
        home_dir.push(format!(
            ".near-credentials/testnet/{}.json",
            self.get_signer()
                .map_err(NearError::GetSignerFailure)?
                .as_ref()
        ));
        let signer = InMemorySigner::from_file(home_dir.as_path())
            .map_err(NearError::ParserInMemorySignerFailure)?;

        self.block_on(self.client.call(
            &signer,
            &self.near_ibc_contract,
            "deliver".to_string(),
            json!({ "messages": messages }).to_string().into_bytes(),
            300000000000000,
            MINIMUM_ATTACHED_NEAR_FOR_DELEVER_MSG,
        ))
    }

    fn raw_transfer(&self, messages: Vec<Any>) -> Result<FinalExecutionOutcomeView> {
        info!("{}: [raw_transfer] - messages: {:?}", self.id(), messages);
        let _msg = serde_json::to_string(&messages).map_err(NearError::SerdeJsonError)?;

        let signer = InMemorySigner::from_random(
            "bob.testnet"
                .parse()
                .map_err(NearError::ParserNearAccountIdFailure)?,
            KeyType::ED25519,
        );
        self.block_on(self.client.call(
            &signer,
            &self.near_ibc_contract,
            "deliver".to_string(),
            json!({ "messages": messages }).to_string().into_bytes(),
            300000000000000,
            1,
        ))
    }

    /// Retrieve the storage proof according to storage keys
    /// And convert the proof to IBC compatible type
    fn generate_storage_proof<'a>(
        &self,
        _storage_entry: impl IntoIterator<Item = &'a [u8]>,
        _height: &Height,
        _storage_name: &str,
    ) -> Result<MerkleProof, Error> {
        Ok(MerkleProof::default())
    }

    fn init_signing_key_pair(&mut self) {
        let key_pair = self.get_key().expect("Failed to get key.");
        self.signing_key_pair = Some(key_pair);
    }

    fn sign_bytes_with_solomachine_pubkey(
        &self,
        sequence: u64,
        timestamp: u64,
        data_type: i32,
        data: Vec<u8>,
    ) -> Vec<u8> {
        use ibc_proto::cosmos::tx::signing::v1beta1::signature_descriptor::{
            data::{Single, Sum},
            Data,
        };

        debug!(
            "{}: [sign_bytes_with_solomachine_pubkey] - sequence {:?}, timestamp: {:?}, data_type: {:?}, data: {:?}",
            self.id(), sequence, timestamp, data_type, data
        );
        let bytes = SignBytes {
            sequence,
            timestamp,
            diversifier: CLIENT_DIVERSIFIER.to_string(),
            data_type,
            data,
        };
        let mut buf = Vec::new();
        Message::encode(&bytes, &mut buf).expect("encode sign bytes failed");
        debug!(
            "{}: [sign_bytes_with_solomachine_pubkey] - encoded_bytes: {:?}",
            self.id(),
            buf
        );

        let key_pair = self
            .signing_key_pair
            .as_ref()
            .expect("signing key pair is empty");
        let signature = key_pair.sign(&buf).expect("sign failed");
        debug!(
            "{}: [sign_bytes_with_solomachine_pubkey] - signature: {:?}",
            self.id(),
            signature
        );
        let sig = Data {
            sum: Some(Sum::Single(Single { mode: 1, signature })),
        };
        buf = Vec::new();
        Message::encode(&sig, &mut buf).expect("encode sign bytes failed");

        debug!(
            "{}: [sign_bytes_with_solomachine_pubkey] - sig_data: {:?}",
            self.id(),
            buf
        );
        buf
    }
}

impl ChainEndpoint for NearChain {
    type LightBlock = NearHeader;
    type Header = NearHeader;
    type ConsensusState = NearConsensusState;
    type ClientState = NearClientState;
    type SigningKeyPair = Secp256k1KeyPair;
    type Time = ibc::core::timestamp::Timestamp;

    fn id(&self) -> &ChainId {
        &self.config().id
    }

    fn config(&self) -> &ChainConfig {
        self.config()
    }

    // todo init NearChain
    fn bootstrap(config: ChainConfig, rt: Arc<TokioRuntime>) -> Result<Self, Error> {
        info!("{}: [bootstrap]", config.id);
        // Initialize key store and load key
        let keybase = KeyRing::new(
            config.key_store_type,
            &config.account_prefix,
            &config.id,
            &config.key_store_folder,
        )
        .map_err(Error::key_base)?;
        let mut new_instance = NearChain {
            client: NearRpcClient::new(config.rpc_addr.to_string().as_str()),
            config,
            keybase,
            near_ibc_contract: AccountId::from_str(CONTRACT_ACCOUNT_ID)
                .map_err(NearError::ParserNearAccountIdFailure)
                .map_err(|e| Error::custom_error(e.to_string()))?,
            rt,
            tx_monitor_cmd: None,
            signing_key_pair: None,
        };
        new_instance.init_signing_key_pair();
        Ok(new_instance)
    }

    fn shutdown(self) -> Result<(), Error> {
        Ok(())
    }

    fn health_check(&mut self) -> Result<HealthCheck, Error> {
        Ok(HealthCheck::Healthy)
    }

    fn keybase(&self) -> &KeyRing<Self::SigningKeyPair> {
        &self.keybase
    }

    fn keybase_mut(&mut self) -> &mut KeyRing<Self::SigningKeyPair> {
        &mut self.keybase
    }

    fn get_signer(&self) -> Result<Signer, Error> {
        trace!("In near: [get signer]");
        Signer::from_str(SIGNER_ACCOUNT_TESTNET).map_err(|e| Error::custom_error(e.to_string()))
    }

    // versioning
    fn ibc_version(&self) -> Result<Option<Version>, Error> {
        // todo(bob)
        Ok(None)
    }

    // send transactions
    fn send_messages_and_wait_commit(
        &mut self,
        proto_msgs: TrackedMsgs,
    ) -> Result<Vec<IbcEventWithHeight>, Error> {
        info!(
            "{}: [send_messages_and_wait_commit] - proto_msgs: {:?}",
            self.id(),
            proto_msgs
        );

        use crate::chain::ic::deliver;

        let runtime = self.rt.clone();

        let mut tracked_msgs = proto_msgs.clone();
        if tracked_msgs.tracking_id().to_string() != "ft-transfer" {
            let canister_id = self.config.canister_id.id.as_str();
            let mut msgs: Vec<Any> = Vec::new();
            for msg in tracked_msgs.messages() {
                let res = runtime
                    .block_on(deliver(canister_id, false, msg.encode_to_vec()))
                    .map_err(|e| Error::custom_error(e.to_string()))?;
                println!("ys-debug: near send_messages_and_wait_commit: {:?}", res);
                if !res.is_empty() {
                    msgs.push(
                        Any::decode(&res[..]).map_err(|e| Error::custom_error(e.to_string()))?,
                    );
                }
            }
            tracked_msgs.msgs = msgs;
        }

        let result = match tracked_msgs.tracking_id {
            TrackingId::Uuid(_) => {
                let result = self
                    .deliver(tracked_msgs.messages().to_vec())
                    .map_err(|e| {
                        Error::report_error(format!("deliever error ({:?})", e.to_string()))
                    })?;
                // result.transaction_outcome
                debug!(
                    "{}: [send_messages_and_wait_commit] - extrics_hash: {:?}",
                    self.id(),
                    result
                );
                result
            }
            TrackingId::Static(value) => match value {
                "ft-transfer" => {
                    todo!() // wait for near-ibc ics20
                }
                _ => {
                    let result = self
                        .deliver(tracked_msgs.messages().to_vec())
                        .map_err(|e| {
                            Error::report_error(format!("deliever error ({:?})", e.to_string()))
                        })?;

                    debug!(
                        "{}: [send_messages_and_wait_commit] - extrics_hash: {:?}",
                        self.id(),
                        result
                    );
                    result
                }
            },
            TrackingId::ClearedUuid(_) => {
                todo!()
            }
        };

        collect_ibc_event_by_outcome(result)
    }

    fn send_messages_and_wait_check_tx(
        &mut self,
        proto_msgs: TrackedMsgs,
    ) -> Result<Vec<TxResponse>, Error> {
        info!(
            "{}: [send_messages_and_wait_check_tx] - proto_msgs: {:?}",
            self.id(),
            proto_msgs
        );

        use crate::chain::ic::deliver;

        let runtime = self.rt.clone();

        let mut tracked_msgs = proto_msgs.clone();
        if tracked_msgs.tracking_id().to_string() != "ft-transfer" {
            let canister_id = self.config.canister_id.id.as_str();
            let mut msgs: Vec<Any> = Vec::new();
            for msg in tracked_msgs.messages() {
                let res = runtime
                    .block_on(deliver(canister_id, false, msg.encode_to_vec()))
                    .map_err(|e| Error::custom_error(e.to_string()))?;
                println!("ys-debug: near send_messages_and_wait_commit: {:?}", res);
                if !res.is_empty() {
                    msgs.push(
                        Any::decode(&res[..]).map_err(|e| Error::custom_error(e.to_string()))?,
                    );
                }
            }
            tracked_msgs.msgs = msgs;
        }

        match tracked_msgs.tracking_id {
            TrackingId::Uuid(_) => {
                let result = self
                    .deliver(tracked_msgs.messages().to_vec())
                    .map_err(|e| {
                        Error::report_error(format!("deliever error ({:?})", e.to_string()))
                    })?;
                debug!(
                    "{}: [send_messages_and_wait_commit] - extrics_hash: {:?}",
                    self.id(),
                    result
                );
            }
            TrackingId::Static(value) => match value {
                "ft-transfer" => {
                    let result = self
                        .raw_transfer(tracked_msgs.messages().to_vec())
                        .map_err(|_| Error::report_error("ics20_transfer".to_string()))?;

                    debug!(
                        "{}: [send_messages_and_wait_commit] - extrics_hash: {:?}",
                        self.id(),
                        result
                    );
                }
                _ => {
                    let result = self
                        .deliver(tracked_msgs.messages().to_vec())
                        .map_err(|e| {
                            Error::report_error(format!("deliever error ({:?})", e.to_string()))
                        })?;
                    debug!(
                        "{}: [send_messages_and_wait_commit] - extrics_hash: {:?}",
                        self.id(),
                        result
                    );
                }
            },
            TrackingId::ClearedUuid(_) => {}
        }

        Ok(vec![])
    }

    // Light client

    /// Fetch a header from the chain at the given height and verify it.
    fn verify_header(
        &mut self,
        trusted: ICSHeight,
        target: ICSHeight,
        client_state: &AnyClientState,
    ) -> Result<Self::LightBlock, Error> {
        info!(
            "{}: [verify_header] - trusted: {:?} target: {:?} client_state: {:?}",
            self.id(),
            trusted,
            target,
            client_state.latest_height()
        );
        let latest_block_view = self
            .block_on(self.client.view_block(Some(BlockId::Height(
                client_state.latest_height().revision_height(),
            ))))
            .unwrap();
        info!(
            "ys-debug: latest_block_view epoch_id: {:?}, next_epoch_id: {:?}",
            latest_block_view.header.epoch_id, latest_block_view.header.next_epoch_id
        );
        let prev_epoch_block_view = self
            .block_on(self.client.view_block(Some(BlockId::Height(
                client_state.latest_height().revision_height() - 43200 - 43200,
            ))))
            .unwrap();
        info!(
            "ys-debug: prev_epoch_block_view epoch_id: {:?}, next_epoch_id: {:?}",
            prev_epoch_block_view.header.epoch_id, prev_epoch_block_view.header.next_epoch_id
        );
        let light_client_block_view = self
            .block_on(
                self.client.query(
                    &near_jsonrpc_client::methods::next_light_client_block::RpcLightClientNextBlockRequest {
                        last_block_hash: prev_epoch_block_view.header.hash
                    },
                )
            )
            .unwrap().unwrap();
        let block_view = self
            .block_on(self.client.view_block(Some(BlockId::Height(
                light_client_block_view.inner_lite.height,
            ))))
            .unwrap();

        let header = produce_light_client_block(&light_client_block_view, &block_view);
        info!(
            "ys-debug: header epoch_id: {:?}, next_epoch_id: {:?}",
            header.light_client_block.inner_lite.epoch_id,
            header.light_client_block.inner_lite.next_epoch_id
        );
        assert!(
            header
                .light_client_block
                .inner_lite
                .next_epoch_id
                .0
                 .0
                .to_vec()
                == latest_block_view.header.epoch_id.0.to_vec()
        );

        Ok(header)
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
        Ok(Balance {
            amount: String::default(),
            denom: String::default(),
        })
    }

    fn query_all_balances(&self, _key_name: Option<&str>) -> Result<Vec<Balance>, Error> {
        todo!()
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
        info!("{}: [query_commitment_prefix]", self.id());
        self.get_commitment_prefix()
            .map_err(|_| Error::report_error("invalid_commitment_prefix".to_string()))

        // self.block_on(self.client.view(
        //     self.near_ibc_contract.clone(),
        //     "get_commitment_prefix".to_string(),
        //     json!({}).to_string().into_bytes()
        // )).and_then(|e| e.json())

        // TODO - do a real chain query
        // CommitmentPrefix::try_from(self.config().store_prefix.as_bytes().to_vec())
        //     .map_err(|_| Error::report_error("invalid_commitment_prefix".to_string()))
    }

    fn query_application_status(&self) -> Result<ChainStatus, Error> {
        info!("{}: [query_application_status]", self.id());

        let latest_height = self
            .get_latest_height()
            .map_err(|e| Error::report_error(format!("{}", e)))?;

        Ok(ChainStatus {
            height: latest_height,
            timestamp: Timestamp::now(),
        })
    }

    fn query_clients(
        &self,
        request: QueryClientStatesRequest,
    ) -> Result<Vec<IdentifiedAnyClientState>, Error> {
        info!("{}: [query_clients] - request: {:?}", self.id(), request);

        let clients = self
            .get_clients(request)
            .map_err(|_| Error::report_error("get_clients".to_string()))?;

        let result: Result<Vec<_>, _> = clients
            .into_iter()
            .map(|(client_id, client_state_bytes)| {
                let client_state = AnyClientState::decode_vec(client_state_bytes.as_ref())
                    .map_err(|e| Error::custom_error(e.to_string()))?;
                Ok(IdentifiedAnyClientState {
                    client_id,
                    client_state,
                })
            })
            .collect();

        result
    }

    fn query_client_state(
        &self,
        request: QueryClientStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyClientState, Option<MerkleProof>), Error> {
        use crate::chain::ic::query_client_state;
        info!(
            "{}: [query_client_state] - request: {:?} include_proof: {:?}",
            self.id(),
            request,
            include_proof
        );

        if matches!(include_proof, IncludeProof::No) {
            let runtime = self.rt.clone();

            let canister_id = self.config.canister_id.id.as_str();

            let res = runtime
                .block_on(query_client_state(canister_id, false, vec![]))
                .map_err(|e| Error::custom_error(e.to_string()))?;
            let client_state = AnyClientState::decode_vec(&res).map_err(Error::decode)?;
            return Ok((client_state, None));
        }

        let QueryClientStateRequest { client_id, height } = request;

        let _query_height = match height {
            QueryHeight::Latest => self
                .get_latest_height()
                .map_err(|_| Error::report_error("query_latest_height".to_string()))?,
            QueryHeight::Specific(value) => value,
        };

        let result = self
            .get_client_state(&client_id)
            .map_err(|_| Error::report_error("query_client_state".to_string()))?;
        let client_state =
            AnyClientState::decode_vec(&result).map_err(|e| Error::custom_error(e.to_string()))?;

        match include_proof {
            IncludeProof::Yes => Ok((client_state, Some(MerkleProof::default()))),
            IncludeProof::No => Ok((client_state, None)),
        }
    }

    fn query_consensus_state(
        &self,
        request: QueryConsensusStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyConsensusState, Option<MerkleProof>), Error> {
        use crate::chain::ic::query_consensus_state;
        info!(
            "{}: [query_consensus_state] - request: {:?} include_proof: {:?}",
            self.id(),
            request,
            include_proof
        );
        assert!(matches!(include_proof, IncludeProof::No));
        assert!(request.client_id.to_string().starts_with("06-solomachine"));
        let runtime = self.rt.clone();
        let canister_id = self.config.canister_id.id.as_str();

        let mut buf = vec![];
        request
            .consensus_height
            .encode(&mut buf)
            .map_err(|e| Error::custom_error(e.to_string()))?;
        let res = runtime
            .block_on(query_consensus_state(canister_id, false, buf))
            .map_err(|e| Error::custom_error(e.to_string()))?;
        let consensus_state = AnyConsensusState::decode_vec(&res).map_err(Error::decode)?;
        Ok((consensus_state, None))

        // // query_height to amit to search chain height
        // let QueryConsensusStateRequest {
        //     client_id,
        //     consensus_height,
        //     query_height: _,
        // } = request;

        // let result = self
        //     .get_client_consensus(&client_id, &consensus_height)
        //     .map_err(|_| Error::report_error("query_client_consensus".to_string()))?;

        // if result.is_empty() {
        //     return Err(Error::report_error("query_client_consensus".to_string()));
        // }

        // let consensus_state = AnyConsensusState::decode_vec(&result)
        //     .map_err(|e| Error::custom_error(e.to_string()))?;

        // match include_proof {
        //     IncludeProof::Yes => Ok((consensus_state, Some(MerkleProof::default()))),
        //     IncludeProof::No => Ok((consensus_state, None)),
        // }
    }

    // fn query_consensus_states(
    //     &self,
    //     request: QueryConsensusStatesRequest,
    // ) -> Result<Vec<AnyConsensusStateWithHeight>, Error> {
    //     info!("{}: [query_consensus_states]");
    //
    //     let request_client_id = ClientId::from_str(request.client_id.as_str())
    //         .map_err(|_| Error::report_error("identifier".to_string()))?;
    //
    //     let result = self
    //         .get_consensus_state_with_height(&request_client_id)
    //         .map_err(|_| Error::report_error("get_consensus_state_with_height".to_string()))?;
    //
    //     let consensus_state: Vec<(Height, AnyConsensusState)> = result;
    //
    //     let mut any_consensus_state_with_height = vec![];
    //     for (height, consensus_state) in consensus_state.into_iter() {
    //         let tmp = AnyConsensusStateWithHeight {
    //             height,
    //             consensus_state,
    //         };
    //         any_consensus_state_with_height.push(tmp.clone());
    //
    //         info!(
    //             "{}: [query_consensus_state] >> any_consensus_state_with_height: {:?}",
    //             tmp
    //         );
    //     }
    //
    //     any_consensus_state_with_height.sort_by(|a, b| a.height.cmp(&b.height));
    //
    //     Ok(any_consensus_state_with_height)
    // }

    fn query_upgraded_client_state(
        &self,
        request: QueryUpgradedClientStateRequest,
    ) -> Result<(AnyClientState, MerkleProof), Error> {
        info!(
            "{}: [query_upgraded_client_state] - request: {:?}",
            self.id(),
            request
        );

        todo!()
    }

    fn query_upgraded_consensus_state(
        &self,
        request: QueryUpgradedConsensusStateRequest,
    ) -> Result<(AnyConsensusState, MerkleProof), Error> {
        info!(
            "{}: [query_upgraded_consensus_state] - request: {:?}",
            self.id(),
            request
        );

        todo!()
    }

    fn query_connections(
        &self,
        request: QueryConnectionsRequest,
    ) -> Result<Vec<IdentifiedConnectionEnd>, Error> {
        info!(
            "{}: [query_connections] - request: {:?}",
            self.id(),
            request
        );

        let result = self
            .get_connections(request)
            .map_err(|_| Error::report_error("get_connections".to_string()))?;

        Ok(result)
    }

    fn query_client_connections(
        &self,
        request: QueryClientConnectionsRequest,
    ) -> Result<Vec<ConnectionId>, Error> {
        info!(
            "{}: [query_client_connections] - request: {:?}",
            self.id(),
            request
        );
        let result = self
            .get_client_connections(&request)
            .map_err(|_| Error::report_error("get_client_connections".to_string()))?;
        Ok(result)
    }

    fn query_connection(
        &self,
        request: QueryConnectionRequest,
        include_proof: IncludeProof,
    ) -> Result<(ConnectionEnd, Option<MerkleProof>), Error> {
        info!(
            "{}: [query_connection] - request: {:?} include_proof: {:?}",
            self.id(),
            request,
            include_proof
        );

        let QueryConnectionRequest {
            connection_id,
            height: _,
        } = request;

        let connection_end = self
            .get_connection_end(&connection_id)
            .map_err(|_| Error::report_error("query_connection_end".to_string()))?;

        // update ConnectionsPath key
        let _connections_path = ConnectionsPath(connection_id).to_string();

        Ok((connection_end, Some(MerkleProof::default())))

        // match include_proof {
        //     IncludeProof::Yes => {
        //         let query_height = match height {
        //             QueryHeight::Latest => {
        //                 let height = self
        //                     .get_latest_height()
        //                     .map_err(|_| Error::report_error("query_latest_height".to_string()))?;
        //                 height
        //             }
        //             QueryHeight::Specific(value) => value,
        //         };
        //
        //         Ok((
        //             connection_end,
        //             Some(self.generate_storage_proof(
        //                     vec![connections_path.as_bytes()],
        //                 &query_height,
        //                 "Connections",
        //             )?),
        //         ))
        //     }
        //     IncludeProof::No => Ok((connection_end, None)),
        // }
    }

    fn query_connection_channels(
        &self,
        request: QueryConnectionChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        info!(
            "{}: [query_connection_channels] - request: {:?}",
            self.id(),
            request
        );

        let result = self
            .get_connection_channels(&request.connection_id)
            .map_err(|_| Error::report_error("get_connection_channels".to_string()))?;

        Ok(result)
    }

    fn query_channels(
        &self,
        request: QueryChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        info!("{}: [query_channels] - request: {:?}", self.id(), request);

        let result = self
            .get_channels(request)
            .map_err(|_| Error::report_error("get_channels".to_string()))?;

        Ok(result)
    }

    fn query_channel(
        &self,
        request: QueryChannelRequest,
        include_proof: IncludeProof,
    ) -> Result<(ChannelEnd, Option<MerkleProof>), Error> {
        info!(
            "{}: [query_channel] - request: {:?} include_proof: {:?}",
            self.id(),
            request,
            include_proof
        );

        let QueryChannelRequest {
            port_id,
            channel_id,
            height,
        } = request;

        let channel_end = self
            .get_channel_end(&port_id, &channel_id)
            .map_err(|_| Error::report_error("query_channel_end".to_string()))?;

        // use channel_end path as key
        let _channel_end_path = ChannelEndsPath(port_id, channel_id).to_string();

        match include_proof {
            IncludeProof::Yes => {
                let _query_height = match height {
                    QueryHeight::Latest => self
                        .get_latest_height()
                        .map_err(|_| Error::report_error("query_latest_height".to_string()))?,
                    QueryHeight::Specific(value) => value,
                };

                Ok((channel_end, Some(MerkleProof::default())))
            }
            IncludeProof::No => Ok((channel_end, None)),
        }
    }

    fn query_channel_client_state(
        &self,
        request: QueryChannelClientStateRequest,
    ) -> Result<Option<IdentifiedAnyClientState>, Error> {
        info!(
            "{}: [query_channel_client_state] - request: {:?}",
            self.id(),
            request
        );

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
            .get_packet_commitment(&port_id, &channel_id, &sequence)
            .map_err(|_| Error::report_error("query_packet_commitment".to_string()))?;

        let _packet_commits_path = CommitmentsPath {
            port_id,
            channel_id,
            sequence,
        }
        .to_string();

        match include_proof {
            IncludeProof::Yes => {
                let _query_height = match height {
                    QueryHeight::Latest => self
                        .get_latest_height()
                        .map_err(|_| Error::report_error("query_latest_height".to_string()))?,
                    QueryHeight::Specific(value) => value,
                };
                Ok((packet_commit, Some(MerkleProof::default())))
            }
            IncludeProof::No => Ok((packet_commit, None)),
        }
    }

    fn query_packet_commitments(
        &self,
        request: QueryPacketCommitmentsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), Error> {
        info!(
            "{}: [query_packet_commitments] - request: {:?}",
            self.id(),
            request
        );

        let sequences = self
            .get_packet_commitments(request)
            .map_err(|_| Error::report_error("get_packet_commitments".to_string()))?;

        let latest_height = self
            .get_latest_height()
            .map_err(|_| Error::report_error("get_latest_height_error".to_string()))?;

        Ok((sequences, latest_height))
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
            .get_packet_receipt(&port_id, &channel_id, &sequence)
            .map_err(|_| Error::report_error("query_packet_receipt".to_string()))?;

        let packet_receipt_path = ReceiptsPath {
            port_id,
            channel_id,
            sequence,
        }
        .to_string();

        match include_proof {
            IncludeProof::Yes => {
                let query_height = match height {
                    QueryHeight::Latest => self
                        .get_latest_height()
                        .map_err(|_| Error::report_error("query_latest_height".to_string()))?,
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
        info!(
            "{}: [query_unreceived_packets] - request: {:?}",
            self.id(),
            request
        );

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
            .map(Sequence::from)
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
            .get_packet_acknowledgement(&port_id, &channel_id, &sequence)
            .map_err(|_| Error::report_error("query_packet_acknowledgement".to_string()))?;

        let packet_acknowledgement_path = AcksPath {
            port_id,
            channel_id,
            sequence,
        }
        .to_string();

        match include_proof {
            IncludeProof::Yes => {
                let query_height = match height {
                    QueryHeight::Latest => self
                        .get_latest_height()
                        .map_err(|_| Error::report_error("query_latest_height".to_string()))?,
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
        request: QueryPacketAcknowledgementsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), Error> {
        info!(
            "{}: [query_packet_acknowledgements] - request: {:?}",
            self.id(),
            request
        );

        let sequences = self
            .get_packet_acknowledgements(request)
            .map_err(|_| Error::report_error("get_packet_acknowledgements".to_string()))?;

        let latest_height = self
            .get_latest_height()
            .map_err(|_| Error::report_error("get_latest_height".to_string()))?;

        Ok((sequences, latest_height))
    }

    fn query_unreceived_acknowledgements(
        &self,
        request: QueryUnreceivedAcksRequest,
    ) -> Result<Vec<Sequence>, Error> {
        info!(
            "{}: [query_unreceived_acknowledgements] - request: {:?}",
            self.id(),
            request
        );

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
            if cmt.is_ok() {
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
        info!(
            "{}: [query_next_sequence_receive] - request: {:?}",
            self.id(),
            request
        );

        let QueryNextSequenceReceiveRequest {
            port_id,
            channel_id,
            height,
        } = request;

        let next_sequence_receive = self
            .get_next_sequence_receive(&port_id, &channel_id)
            .map_err(|_| Error::report_error("query_next_sequence_receive".to_string()))?;

        let next_sequence_receive_path = SeqRecvsPath(port_id, channel_id).to_string();

        match include_proof {
            IncludeProof::Yes => {
                let query_height = match height {
                    QueryHeight::Latest => self
                        .get_latest_height()
                        .map_err(|_| Error::report_error("query_latest_height".to_string()))?,
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
        info!("{}: [query_txs] - request: {:?}", self.id(), request);

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
                                client_type: ClientType::Near,
                                consensus_height: request.consensus_height,
                            },
                        ),
                    ),
                    height: Height::new(0, 9).map_err(|e| Error::custom_error(e.to_string()))?,
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

    fn query_packet_events(
        &self,
        request: QueryPacketEventDataRequest,
    ) -> Result<Vec<IbcEventWithHeight>, Error> {
        info!(
            "{}: [query_packet_events] - request: {:?}",
            self.id(),
            request
        );
        let mut request = request;
        request.height = request.height.map(|height| match height {
            QueryHeight::Latest => QueryHeight::Latest,
            QueryHeight::Specific(value) => QueryHeight::Specific(
                Height::new(value.revision_number(), value.revision_height() + 10)
                    .expect("failed construct ibc height"),
            ),
            // todo(davirain) can improve this error handling
            // .map_err(|e| Error::custom_error(e.to_string()))
        });
        let original_result = self
            .get_packet_events(request)
            .map_err(|_| Error::report_error("get_packet_events".to_string()))?;
        let mut result: Vec<IbcEventWithHeight> = vec![];
        for (height, ibc_events) in original_result {
            for ibc_event in ibc_events.iter() {
                result.push(IbcEventWithHeight {
                    event: convert_ibc_event_to_hermes_ibc_event(ibc_event)
                        .map_err(|e| Error::custom_error(e.to_string()))?,
                    height,
                });
            }
        }
        Ok(result)
    }

    fn query_host_consensus_state(
        &self,
        request: QueryHostConsensusStateRequest,
    ) -> Result<Self::ConsensusState, Error> {
        info!(
            "{}: [query_host_consensus_state] - request: {:?}",
            self.id(),
            request
        );

        // Ok(self.get_sm_consensus_state())
        todo!()
    }

    fn build_client_state(
        &self,
        height: ICSHeight,
        dst_config: ClientSettings,
    ) -> Result<Self::ClientState, Error> {
        info!(
            "{}: [build_client_state] - height: {:?} dst_config: {:?}",
            self.id(),
            height,
            dst_config
        );

        let ClientSettings::Tendermint(settings) = dst_config;
        let trusting_period = settings.trusting_period.unwrap_or_default();

        let client_state = NearClientState {
            chain_id: self.id().clone(),
            trusting_period: trusting_period.as_nanos() as u64,
            latest_height: height.revision_height(),
            latest_timestamp: 100,
            frozen_height: None,
            upgrade_commitment_prefix: vec![],
            upgrade_key: vec![],
        };

        Ok(client_state)
    }

    fn build_consensus_state(
        &self,
        light_block: Self::LightBlock,
    ) -> Result<Self::ConsensusState, Error> {
        info!(
            "{}: [build_consensus_state] - light_block: {:?}",
            self.id(),
            light_block
        );
        let consensus_state = NearConsensusState {
            current_bps: vec![],
            header: light_block,
            commitment_root: CommitmentRoot::from(vec![]),
        };

        Ok(consensus_state)
    }

    fn build_header(
        &mut self,
        trusted_height: ICSHeight,
        target_height: ICSHeight,
        client_state: &AnyClientState,
    ) -> Result<(Self::Header, Vec<Self::Header>), Error> {
        info!(
            "{}: [build_header] - trusted_height: {:?} target_height: {:?} client_state: {:?}",
            self.id(),
            trusted_height,
            target_height,
            client_state
        );

        let mut headers = Vec::new();
        let mut height = trusted_height.revision_height();
        while height <= target_height.revision_height() {
            let bv = self
                .block_on(self.client.view_block(Some(BlockId::Height(height - 20))))
                .unwrap();
            info!("ys-debug: get block view: {:?}", bv);
            let light_client_block_view = self
            .block_on(
                self.client.query(
                    &near_jsonrpc_client::methods::next_light_client_block::RpcLightClientNextBlockRequest {
                        last_block_hash: bv.header.hash
                    },
                )
            )
            .unwrap().unwrap();

            let block_view = self
                .block_on(self.client.view_block(Some(BlockId::Height(
                    light_client_block_view.inner_lite.height,
                ))))
                .unwrap();

            let header = produce_light_client_block(&light_client_block_view, &block_view);
            info!("ys-debug: get new header: {:?}", header.height());
            height = header.height().revision_height();
            headers.push(header);
        }
        if !headers.is_empty() {
            let h = headers.pop().unwrap();
            Ok((h, headers))
        } else {
            Err(Error::report_error("failed to build header".to_string()))
        }
    }

    fn maybe_register_counterparty_payee(
        &mut self,
        _channel_id: &ChannelId,
        _port_id: &PortId,
        _counterparty_payee: &Signer,
    ) -> Result<(), Error> {
        todo!()
    }

    /// Builds the required proofs and the client state for connection handshake messages.
    /// The proofs and client state must be obtained from queries at same height.
    fn build_connection_proofs_and_client_state(
        &self,
        message_type: ConnectionMsgType,
        connection_id: &ConnectionId,
        client_id: &ClientId,
        height: ICSHeight,
    ) -> Result<(Option<AnyClientState>, Proofs), Error> {
        info!(
            "{}: [build_connection_proofs_and_client_state] - message_type: {:?} connection_id: {:?} client_id: {:?} height: {:?}",
            self.id(), message_type, connection_id, client_id, height
        );

        let (connection_end, _maybe_connection_proof) = self.query_connection(
            QueryConnectionRequest {
                connection_id: connection_id.clone(),
                height: QueryHeight::Specific(height),
            },
            IncludeProof::No,
        )?;

        let commitment_prefix = self
            .get_commitment_prefix()
            .map_err(|e| Error::custom_error(e.to_string()))?;

        let mut buf = Vec::new();
        let data = ConnectionStateData {
            path: format!(
                "/{}/connections%2F{}",
                String::from_utf8(commitment_prefix.clone().into_vec())
                    .map_err(|e| Error::custom_error(e.to_string()))?,
                connection_id.as_str()
            )
            .into(),
            connection: Some(connection_end.clone().into()),
        };
        debug!("{}: ConnectionStateData: {:?}", self.id(), data);
        Message::encode(&data, &mut buf).map_err(|e| Error::custom_error(e.to_string()))?;

        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| Error::custom_error(e.to_string()))?;
        let timestamp_nanos = duration_since_epoch.as_nanos() as u64; // u128

        let sig_data = self.sign_bytes_with_solomachine_pubkey(
            height.revision_height() + 1,
            timestamp_nanos,
            DataType::ConnectionState.into(),
            buf.to_vec(),
        );

        let timestamped = TimestampedSignatureData {
            signature_data: sig_data,
            timestamp: timestamp_nanos,
        };
        debug!(
            "{}: proof_init TimestampedSignatureData: {:?}",
            self.id(),
            timestamped
        );
        let mut proof_init = Vec::new();
        Message::encode(&timestamped, &mut proof_init)
            .map_err(|e| Error::custom_error(e.to_string()))?;

        // Check that the connection state is compatible with the message
        match message_type {
            ConnectionMsgType::OpenTry => {
                if !connection_end.state_matches(&State::Init)
                    && !connection_end.state_matches(&State::TryOpen)
                {
                    return Err(Error::bad_connection_state());
                }
            }
            ConnectionMsgType::OpenAck => {
                if !connection_end.state_matches(&State::TryOpen)
                    && !connection_end.state_matches(&State::Open)
                {
                    return Err(Error::bad_connection_state());
                }
            }
            ConnectionMsgType::OpenConfirm => {
                if !connection_end.state_matches(&State::Open) {
                    return Err(Error::bad_connection_state());
                }
            }
        }

        let mut client_state = None;
        let mut client_proof = None;
        let mut consensus_proof = None;

        match message_type {
            ConnectionMsgType::OpenTry | ConnectionMsgType::OpenAck => {
                let (client_state_value, _maybe_client_state_proof) = self.query_client_state(
                    QueryClientStateRequest {
                        client_id: client_id.clone(),
                        height: QueryHeight::Specific(height),
                    },
                    IncludeProof::No,
                )?;

                let mut buf = Vec::new();
                let data = ClientStateData {
                    path: format!(
                        "/{}/clients%2F{}%2FclientState",
                        String::from_utf8(commitment_prefix.clone().into_vec())
                            .map_err(|e| Error::custom_error(e.to_string()))?,
                        client_id.as_str()
                    )
                    .into(),
                    client_state: Some(client_state_value.clone().into()),
                };
                debug!("{}: ClientStateData: {:?}", self.id(), data);
                Message::encode(&data, &mut buf).map_err(|e| Error::custom_error(e.to_string()))?;

                // let duration_since_epoch = SystemTime::now()
                //     .duration_since(SystemTime::UNIX_EPOCH)
                //     .unwrap();
                // let timestamp_nanos = duration_since_epoch.as_nanos() as u64; // u128

                let sig_data = self.sign_bytes_with_solomachine_pubkey(
                    height.revision_height() + 2,
                    timestamp_nanos,
                    DataType::ClientState.into(),
                    buf.to_vec(),
                );

                let timestamped = TimestampedSignatureData {
                    signature_data: sig_data,
                    timestamp: timestamp_nanos,
                };
                debug!(
                    "{}: client_proof TimestampedSignatureData: {:?}",
                    self.id(),
                    timestamped
                );
                let mut proof_client = Vec::new();
                Message::encode(&timestamped, &mut proof_client)
                    .map_err(|e| Error::custom_error(e.to_string()))?;

                client_proof = Some(
                    CommitmentProofBytes::try_from(proof_client).map_err(Error::malformed_proof)?,
                );

                let (consensus_state_value, _maybe_consensus_state_proof) = self
                    .query_consensus_state(
                        QueryConsensusStateRequest {
                            client_id: client_id.clone(),
                            consensus_height: client_state_value.latest_height(),
                            query_height: QueryHeight::Specific(height),
                        },
                        IncludeProof::No,
                    )?;

                let mut buf = Vec::new();
                let data = ConsensusStateData {
                    path: format!(
                        "/{}/clients%2F{}%2FconsensusStates%2F0-{}",
                        String::from_utf8(commitment_prefix.into_vec())
                            .map_err(|e| Error::custom_error(e.to_string()))?,
                        client_id.as_str(),
                        client_state_value.latest_height().revision_height()
                    )
                    .into(),
                    consensus_state: Some(consensus_state_value.into()),
                };
                debug!("{}: ConsensusStateData: {:?}", self.id(), data);
                Message::encode(&data, &mut buf).map_err(|e| Error::custom_error(e.to_string()))?;

                // let duration_since_epoch = SystemTime::now()
                //     .duration_since(SystemTime::UNIX_EPOCH)
                //     .unwrap();
                // let timestamp_nanos = duration_since_epoch.as_nanos() as u64; // u128

                let sig_data = self.sign_bytes_with_solomachine_pubkey(
                    height.revision_height() + 3,
                    timestamp_nanos,
                    DataType::ConsensusState.into(),
                    buf.to_vec(),
                );

                let timestamped = TimestampedSignatureData {
                    signature_data: sig_data,
                    timestamp: timestamp_nanos,
                };
                debug!(
                    "{}: consensus_proof TimestampedSignatureData: {:?}",
                    self.id(),
                    timestamped
                );
                let mut consensus_state_proof = Vec::new();
                Message::encode(&timestamped, &mut consensus_state_proof)
                    .map_err(|e| Error::custom_error(e.to_string()))?;

                consensus_proof = Option::from(
                    ConsensusProof::new(
                        CommitmentProofBytes::try_from(consensus_state_proof)
                            .map_err(Error::malformed_proof)?,
                        client_state_value.latest_height(),
                    )
                    .map_err(Error::consensus_proof)?,
                );

                client_state = Some(client_state_value);
            }
            _ => {}
        }

        Ok((
            client_state,
            Proofs::new(
                CommitmentProofBytes::try_from(proof_init.to_vec())
                    .map_err(Error::malformed_proof)?,
                client_proof,
                consensus_proof,
                None,
                height.increment(),
            )
            .map_err(Error::malformed_proof)?,
        ))
    }

    /// Builds the proof for channel handshake messages.
    fn build_channel_proofs(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        height: ICSHeight,
    ) -> Result<Proofs, Error> {
        // Collect all proofs as required
        let (channel, _maybe_channel_proof) = self.query_channel(
            QueryChannelRequest {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
                height: QueryHeight::Specific(height),
            },
            IncludeProof::No,
        )?;

        let mut buf = Vec::new();
        let data = ChannelStateData {
            path: ("/ibc/channelEnds%2Fports%2F".to_string()
                + port_id.as_str()
                + "%2Fchannels%2F"
                + channel_id.as_str())
            .into(),
            channel: Some(channel.into()),
        };
        println!("ys-debug: ChannelStateData: {:?}", data);
        Message::encode(&data, &mut buf).map_err(|e| Error::custom_error(e.to_string()))?;

        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| Error::custom_error(e.to_string()))?;
        let timestamp_nanos = duration_since_epoch.as_nanos() as u64; // u128

        let sig_data = self.sign_bytes_with_solomachine_pubkey(
            height.revision_height() + 1,
            timestamp_nanos,
            DataType::ChannelState.into(),
            buf.to_vec(),
        );

        let timestamped = TimestampedSignatureData {
            signature_data: sig_data,
            timestamp: timestamp_nanos,
        };
        let mut channel_proof = Vec::new();
        Message::encode(&timestamped, &mut channel_proof)
            .map_err(|e| Error::custom_error(e.to_string()))?;

        let channel_proof_bytes =
            CommitmentProofBytes::try_from(channel_proof).map_err(Error::malformed_proof)?;

        Proofs::new(channel_proof_bytes, None, None, None, height.increment())
            .map_err(Error::malformed_proof)
    }

    fn subscribe(&mut self) -> std::result::Result<Subscription, Error> {
        info!("subscribing to events...");
        let tx_monitor_cmd = match &self.tx_monitor_cmd {
            Some(tx_monitor_cmd) => tx_monitor_cmd,
            None => {
                let tx_monitor_cmd = self.init_event_monitor()?;
                self.tx_monitor_cmd = Some(tx_monitor_cmd);
                self.tx_monitor_cmd.as_ref().unwrap()
            }
        };

        let subscription = tx_monitor_cmd.subscribe().map_err(Error::event_monitor)?;
        Ok(subscription)
    }

    fn query_consensus_state_heights(
        &self,
        request: QueryConsensusStateHeightsRequest,
    ) -> std::result::Result<Vec<Height>, Error> {
        info!(
            "{}: [query_consensus_state_heights] - request: {:?} ",
            self.id(),
            request,
        );

        let result = self
            .get_client_consensus_heights(&request.client_id)
            .map_err(|_| Error::report_error("query_consensus_state_heights".to_string()))?;

        info!(
            "{}: [query_consensus_state_heights] - result: {:?} ",
            self.id(),
            result,
        );

        Ok(result)
    }

    fn cross_chain_query(
        &self,
        _requests: Vec<CrossChainQueryRequest>,
    ) -> std::result::Result<Vec<CrossChainQueryResponse>, Error> {
        todo!()
    }

    fn query_incentivized_packet(
        &self,
        _request: ibc_proto::ibc::apps::fee::v1::QueryIncentivizedPacketRequest,
    ) -> std::result::Result<ibc_proto::ibc::apps::fee::v1::QueryIncentivizedPacketResponse, Error>
    {
        todo!()
    }
}

impl NearChain {
    fn init_event_monitor(&self) -> Result<TxMonitorCmd, Error> {
        info!("initializing event monitor");
        crate::time!("init_event_monitor");

        let (event_monitor, monitor_tx) = NearEventMonitor::new(
            self.config.id.clone(),
            self.config.rpc_addr.to_string(),
            self.rt.clone(),
        )
        .map_err(Error::event_monitor)?;

        thread::spawn(move || event_monitor.run());

        Ok(monitor_tx)
    }
}

pub fn collect_ibc_event_by_outcome(
    outcome: FinalExecutionOutcomeView,
) -> Result<Vec<IbcEventWithHeight>, Error> {
    let mut ibc_events = vec![];
    for receipt_outcome in outcome.receipts_outcome {
        for log in receipt_outcome.outcome.logs {
            if log.starts_with("EVENT_JSON:") {
                let event = log.replace("EVENT_JSON:", "");
                let event_value = serde_json::value::Value::from_str(event.as_str())
                    .map_err(|e| Error::custom_error(e.to_string()))?;
                if event_value["standard"].eq("near-ibc") {
                    let ibc_event: ibc::core::events::IbcEvent =
                        serde_json::from_value(event_value["raw-ibc-event"].clone())
                            .map_err(|e| Error::custom_error(e.to_string()))?;
                    let block_height = u64::from_str(
                        event_value["block_height"]
                            .as_str()
                            .expect("Failed to get block_height field."),
                    )
                    .expect("Failed to parse block_height field.");
                    match ibc_event {
                        ibc::core::events::IbcEvent::Message(_) => continue,
                        _ => ibc_events.push(IbcEventWithHeight {
                            event: convert_ibc_event_to_hermes_ibc_event(&ibc_event)
                                .map_err(|e| Error::custom_error(e.to_string()))?,
                            height: Height::new(0, block_height)
                                .map_err(|e| Error::custom_error(e.to_string()))?,
                        }),
                    }
                }
            }
        }
    }
    Ok(ibc_events)
}

/// Produce `BlockHeaderInnerLiteView` by its NEAR version
pub fn produce_block_header_inner_light(
    view: &near_primitives::views::BlockHeaderInnerLiteView,
) -> BlockHeaderInnerLite {
    BlockHeaderInnerLite {
        height: view.height,
        epoch_id: EpochId(CryptoHash(view.epoch_id.0)),
        next_epoch_id: EpochId(CryptoHash(view.next_epoch_id.0)),
        prev_state_root: CryptoHash(view.prev_state_root.0),
        outcome_root: CryptoHash(view.outcome_root.0),
        timestamp: view.timestamp,
        next_bp_hash: CryptoHash(view.next_bp_hash.0),
        block_merkle_root: CryptoHash(view.block_merkle_root.0),
    }
}

/// Produce `Header` by NEAR version of `LightClientBlockView` and `BlockView`.
pub fn produce_light_client_block(
    view: &near_primitives::views::LightClientBlockView,
    block_view: &near_primitives::views::BlockView,
) -> NearHeader {
    assert!(
        view.inner_lite.height == block_view.header.height,
        "Not same height of light client block view and block view. view: {}, block_view: {}",
        view.inner_lite.height,
        block_view.header.height
    );
    NearHeader {
        light_client_block: LightClientBlock {
            prev_block_hash: CryptoHash(view.prev_block_hash.0),
            next_block_inner_hash: CryptoHash(view.next_block_inner_hash.0),
            inner_lite: produce_block_header_inner_light(&view.inner_lite),
            inner_rest_hash: CryptoHash(view.inner_rest_hash.0),
            next_bps: Some(
                view.next_bps
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|f| match f {
                        near_primitives::views::validator_stake_view::ValidatorStakeView::V1(v) => {
                            ValidatorStakeView::V1(ValidatorStakeViewV1 {
                                account_id: v.account_id.to_string(),
                                public_key: match &v.public_key {
                                    near_crypto::PublicKey::ED25519(data) => {
                                        PublicKey::ED25519(ED25519PublicKey(data.clone().0))
                                    }
                                    _ => panic!("Unsupported publickey in next block producers."),
                                },
                                stake: v.stake,
                            })
                        }
                    })
                    .collect(),
            ),
            approvals_after_next: view
                .approvals_after_next
                .iter()
                .map(|f| {
                    f.as_ref().map(|s| match s {
                        near_crypto::Signature::ED25519(data) => Signature::ED25519(data.clone()),
                        _ => panic!("Unsupported signature in approvals after next."),
                    })
                })
                .collect(),
        },
        prev_state_root_of_chunks: block_view
            .chunks
            .iter()
            .map(|header| CryptoHash(header.prev_state_root.0))
            .collect(),
    }
}
