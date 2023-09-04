use super::client::ClientSettings;
use crate::util::retry::{retry_with_index, RetryResult};
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
    chain::tracking::TrackedMsgs,
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
    keyring::{KeyRing, NearKeyPair, SigningKeyPair},
    misbehaviour::MisbehaviourEvidence,
};
use alloc::{string::String, sync::Arc};
use anyhow::Result;
use core::{fmt::Debug, str::FromStr};
use ibc_proto::{google::protobuf::Any, protobuf::Protobuf};
use ibc_relayer_types::core::ics04_channel::packet::PacketMsgType;
use ibc_relayer_types::{
    applications::ics31_icq::response::CrossChainQueryResponse,
    core::ics02_client::events::UpdateClient,
    core::ics23_commitment::merkle::MerkleProof,
    core::ics24_host::path::{
        AcksPath, ChannelEndsPath, ClientStatePath, CommitmentsPath, ConnectionsPath,
    },
    core::{
        ics02_client::{client_type::ClientType, error::Error as ClientError},
        ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd},
        ics04_channel::{
            channel::{ChannelEnd, IdentifiedChannelEnd},
            packet::Sequence,
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
use near_primitives::types::BlockId;
use near_primitives::views::ViewStateResult;
use near_primitives::{types::AccountId, views::FinalExecutionOutcomeView};
use prost::Message;
use semver::Version;
use serde_json::json;
use std::path::PathBuf;
use std::thread;
use tendermint_rpc::endpoint::broadcast::tx_sync::Response as TxResponse;
use tokio::runtime::Runtime as TokioRuntime;
use tracing::{debug, info, trace, warn};

pub mod constants;
pub mod contract;
pub mod error;
pub mod rpc;

use borsh::{BorshDeserialize, BorshSerialize};

pub const REVISION_NUMBER: u64 = 0;
pub const CLIENT_DIVERSIFIER: &str = "NEAR";
const MINIMUM_ATTACHED_NEAR_FOR_DELEVER_MSG: u128 = 50_000_000_000_000_000_000_000;

#[derive(BorshSerialize, BorshDeserialize)]
struct NearProofs(Vec<Vec<u8>>);

/// A struct used to start a Near chain instance in relayer
#[derive(Debug)]
pub struct NearChain {
    client: NearRpcClient,
    lcb_client: NearRpcClient,
    config: ChainConfig,
    keybase: KeyRing<NearKeyPair>,
    near_ibc_contract: AccountId,
    rt: Arc<TokioRuntime>,
    tx_monitor_cmd: Option<TxMonitorCmd>,
    signing_key_pair: Option<NearKeyPair>,
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

    pub fn canister_id(&self) -> &str {
        &self.config.canister_id.id
    }

    pub fn canister_pem_file_path(&self) -> &PathBuf {
        &self.config.canister_pem
    }

    pub fn canister_endpoint(&self) -> &str {
        &self.config.ic_endpoint
    }

    /// Subscribe Events
    /// todo near don't have events subscription
    pub fn subscribe_ibc_events(&self) -> Result<Vec<IbcEvent>> {
        info!("{}: [subscribe_ibc_events]", self.id());
        todo!() //Bob
    }

    /// The function to submit IBC request to a Near chain
    /// This function handles most of the IBC reqeusts to Near, except the MMR root update
    fn deliver(&self, messages: Vec<Any>) -> Result<FinalExecutionOutcomeView> {
        trace!("[deliver] - messages: {:?}", messages);

        retry_with_index(retry_strategy::default_strategy(), |_index| {
            // get signer for this transaction
            let signer = self
                .keybase()
                .get_key(&self.config.key_name)
                .unwrap()
                .inner();

            let call_near_smart_contract_deliver = self.client.call(
                &signer,
                &self.near_ibc_contract,
                "deliver".into(),
                json!({ "messages": messages }).to_string().into_bytes(),
                300000000000000,
                MINIMUM_ATTACHED_NEAR_FOR_DELEVER_MSG * messages.len() as u128,
            );
            let result = self.block_on(call_near_smart_contract_deliver);

            match result {
                Ok(outcome) => RetryResult::Ok(outcome),
                Err(e) => {
                    warn!("ys-debug: retry deliver with error: {}", e);
                    RetryResult::Retry(())
                }
            }
        })
        .map_err(|e| {
            Error::report_error(format!("[Near chain deliver() failed] -> Error({:?})", e)).into()
        })
    }

    fn init_signing_key_pair(&mut self) {
        self.signing_key_pair = self.get_key().ok();
    }
}

impl ChainEndpoint for NearChain {
    type LightBlock = NearHeader;
    type Header = NearHeader;
    type ConsensusState = NearConsensusState;
    type ClientState = NearClientState;
    type SigningKeyPair = NearKeyPair;
    type Time = Timestamp;

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
        let keybase = KeyRing::new_near_keypair(
            config.key_store_type,
            &config.account_prefix,
            &config.id,
            &config.key_store_folder,
        )
        .map_err(Error::key_base)?;
        let mut new_instance = NearChain {
            client: NearRpcClient::new(config.rpc_addr.to_string().as_str()),
            lcb_client: NearRpcClient::new("https://rpc.testnet.near.org"),
            config: config.clone(),
            keybase,
            near_ibc_contract: config.near_ibc_address.into(),
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
        // Get the key from key seed file
        let key_pair = self
            .keybase()
            .get_key(&self.config.key_name)
            .map_err(Error::key_base)?;

        let signer = key_pair
            .account()
            .parse()
            .map_err(|e| Error::ics02(ClientError::signer(e)))?;

        Ok(signer)
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
            "[near - send_messages_and_wait_commit] - proto_msgs: {:?}, tracking_id: {:?}",
            proto_msgs
                .msgs
                .iter()
                .map(|msg| msg.type_url.clone())
                .collect::<Vec<_>>(),
            proto_msgs.tracking_id
        );

        use crate::chain::ic::deliver;

        let mut tracked_msgs = proto_msgs.clone();
        if tracked_msgs.tracking_id().to_string() != "ft-transfer" {
            let msgs_result: Result<Vec<_>, _> = tracked_msgs
                .messages()
                .iter()
                .map(|msg| {
                    self.block_on(deliver(
                        self.canister_id(),
                        self.canister_endpoint(),
                        msg.encode_to_vec(),
                        self.canister_pem_file_path(),
                    ))
                    .map_err(|e| Error::report_error(format!("[Near Chain send_messages_and_wait_commit call ic deliver] -> Error({})", e)))
                })
                .filter_map(|res| {
                    res.ok().and_then(|r| {
                        if r.is_empty() {
                            None
                        } else {
                            Some(r)
                        }
                    })
                })
                .map(|res| {
                    Any::decode(&res[..])
                        .map_err(|e| Error::report_error(format!("[Near Chain send_messages_and_wait_commit decode deliver result] -> Error({})", e)))
                })
                .collect();

            match msgs_result {
                Ok(msgs) => tracked_msgs.msgs = msgs,
                Err(e) => return Err(e),
            }

            info!(
                "[near - send_messages_and_wait_commit] - got proto_msgs from ic: {:?}",
                tracked_msgs
                    .msgs
                    .iter()
                    .map(|msg| msg.type_url.clone())
                    .collect::<Vec<_>>()
            );
        }

        let result = self
            .deliver(tracked_msgs.messages().to_vec())
            .map_err(|e| Error::report_error(format!("deliever error ({:?})", e.to_string())))?;
        debug!(
            "[send_messages_and_wait_commit] - extrics_hash: {:?}",
            result
        );

        collect_ibc_event_by_outcome(result)
    }

    fn send_messages_and_wait_check_tx(
        &mut self,
        proto_msgs: TrackedMsgs,
    ) -> Result<Vec<TxResponse>, Error> {
        info!(
            "[near - send_messages_and_wait_check_tx] - proto_msgs: {:?}, tracking_id: {:?}",
            proto_msgs
                .msgs
                .iter()
                .map(|msg| msg.type_url.clone())
                .collect::<Vec<_>>(),
            proto_msgs.tracking_id
        );

        use crate::chain::ic::deliver;

        let mut tracked_msgs = proto_msgs.clone();
        if tracked_msgs.tracking_id().to_string() != "ft-transfer" {
            let msgs_result: Result<Vec<_>, _> = tracked_msgs
                .messages()
                .iter()
                .map(|msg| {
                    self.block_on(deliver(
                        self.canister_id(),
                        self.canister_endpoint(),
                        msg.encode_to_vec(),
                        self.canister_pem_file_path(),
                    ))
                    .map_err(|e| Error::report_error(format!("[Near Chain send_messages_and_wait_commit_check_tx call ic deliver] -> Error({})", e)))
                })
                .filter_map(|res| {
                    res.ok().and_then(|r| {
                        if r.is_empty() {
                            None
                        } else {
                            Some(r)
                        }
                    })
                })
                .map(|res| {
                    Any::decode(&res[..])
                        .map_err(|e| Error::report_error(format!("[Near Chain send_messages_and_wait_commit_check_tx decode deliver result] -> Error({})", e)))
                })
                .collect();

            match msgs_result {
                Ok(msgs) => tracked_msgs.msgs = msgs,
                Err(e) => return Err(e),
            }

            info!(
                "[near - send_messages_and_wait_check_tx] - got proto_msgs from ic: {:?}",
                tracked_msgs
                    .msgs
                    .iter()
                    .map(|msg| msg.type_url.clone())
                    .collect::<Vec<_>>()
            );
        }

        let result = self
            .deliver(tracked_msgs.messages().to_vec())
            .map_err(|e| Error::report_error(format!("deliver error ({:?})", e.to_string())))?;

        debug!(
            "[send_messages_and_wait_check_tx] - extrics_hash: {:?}",
            result
        );

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
        let header = retry_with_index(retry_strategy::default_strategy(), |_index| {
            let result = self.block_on(self.client.view_block(Some(BlockId::Height(
                client_state.latest_height().revision_height(),
            ))));

            let latest_block_view = match result {
                Ok(bv) => bv,
                Err(e) => {
                    warn!("ys-debug: retry get latest_block_view with error: {}", e);
                    return RetryResult::Retry(());
                }
            };

            info!(
                "ys-debug: latest block height: {:?}, epoch: {:?}, next_epoch_id: {:?}",
                latest_block_view.header.height,
                latest_block_view.header.epoch_id,
                latest_block_view.header.next_epoch_id
            );

            let result = self.block_on(self.lcb_client.query(&near_jsonrpc_client::methods::next_light_client_block::RpcLightClientNextBlockRequest {
                last_block_hash: latest_block_view.header.epoch_id
            }));

            let light_client_block_view = match result {
                Ok(lcb) => lcb,
                Err(e) => {
                    warn!(
                        "ys-debug: retry get next_light_client_block with error: {}",
                        e
                    );
                    return RetryResult::Retry(());
                }
            }.expect("[Near Chain verify_header function light_client_block is empty]");

            let result = self.block_on(self.client.view_block(Some(BlockId::Height(
                light_client_block_view.inner_lite.height,
            ))));

            let block_view = match result {
                Ok(bv) => bv,
                Err(e) => {
                    warn!("ys-debug: retry get block_view with error: {}", e);
                    return RetryResult::Retry(());
                }
            };
            let header = produce_light_client_block(&light_client_block_view, &block_view);
            info!(
                "ys-debug: new header in verify_header: {:?}, {:?}, {:?}",
                header.height(),
                light_client_block_view.inner_lite.height,
                block_view.header.height
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
            RetryResult::Ok(header)
        })
        .map_err(|e| {
            Error::report_error(format!(
                "[Near chain verify_header get header failed] -> Error({:?})",
                e
            ))
        })?;

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
            amount: "0".to_string(),
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
        trace!("[query_commitment_prefix]");
        let prefix = self.get_commitment_prefix().map_err(|e| {
            Error::report_error(format!(
                "[Near chain query_commitment_prefix get_commitment_prefix failed] -> Error({})",
                e
            ))
        })?;

        CommitmentPrefix::try_from(prefix).map_err(|e| {
            Error::report_error(format!(
                "[Near Chain Convert Vec<u8> to CommitmentPrefix failed] -> Error({})",
                e
            ))
        })
    }

    fn query_application_status(&self) -> Result<ChainStatus, Error> {
        trace!("[query_application_status]");

        let latest_height = self.get_latest_height().map_err(|e| {
            Error::report_error(format!(
                "[Near Chain query_application_status call get_latest_height failed] -> Error({})",
                e
            ))
        })?;

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
                    .map_err(|e| {
                        Error::report_error(format!(
                            "[Near Chain query_cleints decode AnyClientState] -> Error({})",
                            e
                        ))
                    })?;
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
        trace!(
            "[query_client_state] - request: {:?}, include_proof: {:?}",
            request,
            include_proof
        );

        if matches!(include_proof, IncludeProof::No) {
            let res = self
                .block_on(query_client_state(
                    self.canister_id(),
                    self.canister_endpoint(),
                    vec![],
                ))
                .map_err(|e| Error::report_error(e.to_string()))?;

            return Ok((
                AnyClientState::decode_vec(&res).map_err(Error::decode)?,
                None,
            ));
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

        Ok((
            AnyClientState::decode_vec(&result).map_err(|e| {
                Error::report_error(format!(
                    "[Near Chain query_cleint_state decode AnyClientState] -> Error({})",
                    e
                ))
            })?,
            None,
        ))
    }

    fn query_consensus_state(
        &self,
        request: QueryConsensusStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyConsensusState, Option<MerkleProof>), Error> {
        use crate::chain::ic::query_consensus_state;
        trace!(
            "[query_consensus_state] - request: {:?}, include_proof: {:?}",
            request,
            include_proof
        );
        assert!(matches!(include_proof, IncludeProof::No));
        assert!(request.client_id.to_string().starts_with("06-solomachine"));

        let mut buf = vec![];
        request
            .consensus_height
            .encode(&mut buf)
            .map_err(|e| Error::report_error(e.to_string()))?;
        let res = self
            .block_on(query_consensus_state(
                self.canister_id(),
                self.canister_endpoint(),
                buf,
            ))
            .map_err(|e| Error::report_error(e.to_string()))?;

        Ok((
            AnyConsensusState::decode_vec(&res).map_err(Error::decode)?,
            None,
        ))
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
        //     .map_err(|e| Error::report_error(e.to_string()))?;

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

        self.get_client_connections(&request)
            .map_err(|_| Error::report_error("get_client_connections".to_string()))
    }

    fn query_connection(
        &self,
        request: QueryConnectionRequest,
        include_proof: IncludeProof,
    ) -> Result<(ConnectionEnd, Option<MerkleProof>), Error> {
        trace!(
            "[query_connection] - request: {:?}, include_proof: {:?}",
            request,
            include_proof
        );

        let QueryConnectionRequest {
            connection_id,
            height: _,
        } = request;

        Ok((
            self.get_connection_end(&connection_id)
                .map_err(|_| Error::report_error("query_connection_end".to_string()))?,
            None,
        ))
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

        self.get_connection_channels(&request.connection_id)
            .map_err(|_| Error::report_error("get_connection_channels".to_string()))
    }

    fn query_channels(
        &self,
        request: QueryChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        info!("{}: [query_channels] - request: {:?}", self.id(), request);

        self.get_channels(request)
            .map_err(|_| Error::report_error("get_channels".to_string()))
    }

    fn query_channel(
        &self,
        request: QueryChannelRequest,
        include_proof: IncludeProof,
    ) -> Result<(ChannelEnd, Option<MerkleProof>), Error> {
        trace!(
            "[query_channel] - request: {:?}, include_proof: {:?}",
            request,
            include_proof
        );

        let QueryChannelRequest {
            port_id,
            channel_id,
            height: _,
        } = request;

        Ok((
            self.get_channel_end(&port_id, &channel_id)
                .map_err(|_| Error::report_error("query_channel_end".to_string()))?,
            None,
        ))
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
        _include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        let QueryPacketCommitmentRequest {
            port_id,
            channel_id,
            sequence,
            height: _,
        } = request;

        Ok((
            self.get_packet_commitment(&port_id, &channel_id, &sequence)
                .map_err(|_| Error::report_error("query_packet_commitment".to_string()))?,
            None,
        ))
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

        Ok((
            self.get_packet_commitments(request)
                .map_err(|_| Error::report_error("get_packet_commitments".to_string()))?,
            self.get_latest_height()
                .map_err(|_| Error::report_error("get_latest_height_error".to_string()))?,
        ))
    }

    fn query_packet_receipt(
        &self,
        request: QueryPacketReceiptRequest,
        _include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        let QueryPacketReceiptRequest {
            port_id,
            channel_id,
            sequence,
            height: _,
        } = request;

        Ok((
            self.get_packet_receipt(&port_id, &channel_id, &sequence)
                .map_err(|_| Error::report_error("query_packet_receipt".to_string()))?,
            None,
        ))
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

        let QueryUnreceivedPacketsRequest {
            port_id,
            channel_id,
            packet_commitment_sequences: sequences,
        } = request;

        Ok(self
            .get_unreceipt_packet(&port_id, &channel_id, &sequences)
            .map_err(|_| Error::report_error("get_unreceipt_packet".to_string()))?
            .into_iter()
            .map(Sequence::from)
            .collect())
    }

    fn query_packet_acknowledgement(
        &self,
        request: QueryPacketAcknowledgementRequest,
        _include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        let QueryPacketAcknowledgementRequest {
            port_id,
            channel_id,
            sequence,
            height: _,
        } = request;

        Ok((
            self.get_packet_acknowledgement(&port_id, &channel_id, &sequence)
                .map_err(|_| Error::report_error("query_packet_acknowledgement".to_string()))?,
            None,
        ))
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

        let QueryUnreceivedAcksRequest {
            port_id,
            channel_id,
            packet_ack_sequences: sequences,
        } = request;

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
        _include_proof: IncludeProof,
    ) -> Result<(Sequence, Option<MerkleProof>), Error> {
        info!(
            "{}: [query_next_sequence_receive] - request: {:?}",
            self.id(),
            request
        );

        let QueryNextSequenceReceiveRequest {
            port_id,
            channel_id,
            height: _,
        } = request;

        Ok((
            self.get_next_sequence_receive(&port_id, &channel_id)
                .map_err(|_| Error::report_error("query_next_sequence_receive".to_string()))?,
            None,
        ))
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
                    height: Height::new(0, 9).map_err(|e| {
                        Error::report_error(format!(
                            "[Near Chain query_txs Contruct ibc Height] -> Error({})",
                            e
                        ))
                    })?,
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
        });
        let original_result = self
            .get_packet_events(request)
            .map_err(|_| Error::report_error("get_packet_events".to_string()))?;
        let mut result: Vec<IbcEventWithHeight> = vec![];
        for (height, ibc_events) in original_result {
            for ibc_event in ibc_events.iter() {
                result.push(IbcEventWithHeight {
                    event: convert_ibc_event_to_hermes_ibc_event(ibc_event)
                        .map_err(|e| Error::report_error(format!("[Near Chain query_packet_events call convert_ibc_event_to_hermes_ibc_event] -> Error({})", e)))?,
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
            "[build_client_state] - height: {:?} dst_config: {:?}",
            height, dst_config
        );

        let ClientSettings::Tendermint(settings) = dst_config;
        let trusting_period = settings.trusting_period.unwrap_or_default();

        Ok(NearClientState {
            chain_id: self.id().clone(),
            trusting_period: trusting_period.as_nanos() as u64,
            latest_height: height.revision_height(),
            latest_timestamp: 100,
            frozen_height: None,
            upgrade_commitment_prefix: vec![],
            upgrade_key: vec![],
        })
    }

    fn build_consensus_state(
        &self,
        light_block: Self::LightBlock,
    ) -> Result<Self::ConsensusState, Error> {
        info!(
            "[build_consensus_state] - light_block: height: {:?}, timestamp: {:?}, epoch_id: {:?}, next_epoch_id: {:?}",
            light_block.light_client_block.inner_lite.height,
            light_block.light_client_block.inner_lite.timestamp,
            light_block.light_client_block.inner_lite.epoch_id,
            light_block.light_client_block.inner_lite.next_epoch_id,
        );

        Ok(NearConsensusState {
            current_bps: vec![],
            header: light_block,
            commitment_root: CommitmentRoot::from(vec![]),
        })
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
            client_state.latest_height()
        );
        let trusted_block = self
            .block_on(
                self.client
                    .view_block(Some(BlockId::Height(trusted_height.revision_height()))),
            )
            .map_err(|e| {
                Error::report_error(format!(
                    "[Near Chain build_header call trusted_block] -> Error({})",
                    e
                ))
            })?;
        info!(
            "ys-debug: trusted block height: {:?}, epoch: {:?}, next_epoch_id: {:?}",
            trusted_block.header.height,
            trusted_block.header.epoch_id,
            trusted_block.header.next_epoch_id
        );

        // possible error: handler error: [Block not found: ]
        // handler error: [Block either has never been observed on the node or has been garbage collected: BlockId(Height(135888086))]
        let target_block = retry_with_index(retry_strategy::default_strategy(), |_index| {
            let result = self.block_on(
                self.client
                    .view_block(Some(BlockId::Height(target_height.revision_height()))),
            );

            match result {
                Ok(lcb) => RetryResult::Ok(lcb),
                Err(e) => {
                    warn!(
                        "ys-debug: retry get target_block_view(header) with error: {}",
                        e
                    );
                    RetryResult::Retry(())
                }
            }
        })
        .map_err(|e| {
            Error::report_error(format!(
                "[Near chain build_header get target_block failed] -> Error({:?})",
                e
            ))
        })?;

        info!(
            "ys-debug: target block height: {:?}, epoch: {:?}, next_epoch_id: {:?}",
            target_block.header.height,
            target_block.header.epoch_id,
            target_block.header.next_epoch_id
        );

        // TODO: julian, assert!(trusted_block.epoch == target_block.epoch || trusted_block_.next_epoch == target_block.epoch)
        let header = retry_with_index(retry_strategy::default_strategy(), |_index| {
            let result = self.block_on(self.lcb_client.query(&near_jsonrpc_client::methods::next_light_client_block::RpcLightClientNextBlockRequest {
                last_block_hash: target_block.header.hash
            }));

            let light_client_block_view = match result {
                Ok(lcb) => lcb,
                Err(e) => {
                    warn!(
                        "ys-debug: retry get next_light_client_block with error: {}",
                        e
                    );
                    return RetryResult::Retry(());
                }
            }
            .expect("[Near Chain build_header call light_client_block_view failed is empty]");

            let result = self.block_on(self.client.view_block(Some(BlockId::Height(
                light_client_block_view.inner_lite.height,
            ))));

            let block_view = match result {
                Ok(bv) => bv,
                Err(e) => {
                    warn!("ys-debug: retry get block_view with error: {}", e);
                    return RetryResult::Retry(());
                }
            };

            let proof_height = light_client_block_view.inner_lite.height - 1;

            let block_reference: near_primitives::types::BlockReference =
                BlockId::Height(proof_height).into();
            let prefix = near_primitives::types::StoreKey::from("version".as_bytes().to_vec());
            let result = self.block_on(self.client.query(
                &near_jsonrpc_client::methods::query::RpcQueryRequest {
                    block_reference,
                    request: near_primitives::views::QueryRequest::ViewState {
                        account_id: self.near_ibc_contract.clone(),
                        prefix,
                        include_proof: true,
                    },

                },
            ));

            let query_response = match result {
                Ok(resp) => resp,
                Err(e) => {
                    warn!("ys-debug: retry get proof_block_view with error: {}", e);
                    return RetryResult::Retry(());
                }
            };

            let state = match query_response.kind {
                near_jsonrpc_primitives::types::query::QueryResponseKind::ViewState(state) => {
                    Ok::<ViewStateResult, Error>(state)
                }
                _ => {
                    warn!("ys-debug: retry get view_state");

                    return RetryResult::Retry(());
                }
            }
            .expect("[Near chain build_header call state failed]");

            let proofs: Vec<Vec<u8>> = state.proof.iter().map(|proof| proof.to_vec()).collect();
            let root_hash = CryptoHash::hash_bytes(&proofs[0]);

            if block_view
                .chunks
                .iter()
                .any(|c| c.prev_state_root.0 == root_hash.0)
            {
                let header = produce_light_client_block(&light_client_block_view, &block_view);
                info!(
                    "ys-debug: new header: {:?}, {:?}, {:?}",
                    header.height(),
                    light_client_block_view.inner_lite.height,
                    block_view.header.height
                );
                RetryResult::Ok(header)
            } else {
                warn!(
                    "ys-debug: retry root_hash {:?} at {:?} does not in the lcb state at {:?}",
                    root_hash, proof_height, light_client_block_view.inner_lite.height
                );

                RetryResult::Retry(())
            }
        }).map_err(|e| {
            Error::report_error(format!(
                "[Near chain build_header call header failed] -> Error({:?})",
                e
            ))
        })?;

        // assert!(
        //     header
        //         .light_client_block
        //         .inner_lite
        //         .next_epoch_id
        //         .0
        //          .0
        //         .to_vec()
        //         == latest_block_view.header.epoch_id.0.to_vec()
        // );

        Ok((header, vec![]))
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

        // ServerError(HandlerError(UnknownBlock { block_reference: BlockId(Height(135705911)) }))
        let query_response = retry_with_index(retry_strategy::default_strategy(), |_index| {
            let connections_path = ConnectionsPath(connection_id.clone()).to_string();

            let block_reference: near_primitives::types::BlockReference =
                BlockId::Height(height.revision_height()).into();
            let prefix = near_primitives::types::StoreKey::from(connections_path.into_bytes());
            let result = self.block_on(self.client.query(
                &near_jsonrpc_client::methods::query::RpcQueryRequest {
                    block_reference,
                    request: near_primitives::views::QueryRequest::ViewState {
                        account_id: self.near_ibc_contract.clone(),
                        prefix,
                        include_proof: true,
                    },
                },
            ));

            match result {
                Ok(lcb) => RetryResult::Ok(lcb),
                Err(e) => {
                    warn!(
                        "ys-debug: retry get target_block_view(conn) with error: {}",
                        e
                    );
                    RetryResult::Retry(())
                }
            }
        })
        .map_err(|e| {
            Error::report_error(format!(
                "[Near chain build_connection_proofs_and_client_state get query_response failed ] -> Error({:?})",
                e
            ))
        })?;
        println!(
            "ys-debug: view state of connection result: {:?}",
            query_response.block_height
        );

        let state = match query_response.kind {
            near_jsonrpc_primitives::types::query::QueryResponseKind::ViewState(state) => Ok(state),
            _ => Err(Error::report_error(
                "failed to get connection proof".to_string(),
            )),
        }?;
        let proofs: Vec<Vec<u8>> = state.proof.iter().map(|proof| proof.to_vec()).collect();

        let _commitment_prefix = self
            .get_commitment_prefix()
            .map_err(|e| Error::report_error(format!("[Near Chain build_connection_proofs_and_client_state call get_commitment_prefix] -> Error({})", e)))?;

        let proof_init = NearProofs(proofs)
            .try_to_vec()
            .map_err(|e| Error::report_error(format!("[Near Chain build_connection_proofs_and_client_state NearProofs convert to Vec<u8> failed ] -> Error({})", e)))?;

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
                    IncludeProof::Yes,
                )?;

                let client_state_path = ClientStatePath(client_id.clone()).to_string();

                let block_reference: near_primitives::types::BlockReference =
                    BlockId::Height(height.revision_height()).into();
                let prefix = near_primitives::types::StoreKey::from(client_state_path.into_bytes());

                let query_response = self
                    .block_on(
                        self.client
                            .query(&near_jsonrpc_client::methods::query::RpcQueryRequest {
                                block_reference,
                                request: near_primitives::views::QueryRequest::ViewState {
                                    account_id: self.near_ibc_contract.clone(),
                                    prefix,
                                    include_proof: true,
                                },
                            }),
                    )
                    .map_err(|e| Error::report_error(format!("[Near Chain build_connection_proofs_and_client_state query_response failed] -> Error({})", e)))?;

                let state = match query_response.kind {
                    near_jsonrpc_primitives::types::query::QueryResponseKind::ViewState(state) => {
                        Ok(state)
                    }
                    _ => Err(Error::report_error(
                        "failed to get connection proof".to_string(),
                    )),
                }?;
                let proofs: Vec<Vec<u8>> = state.proof.iter().map(|proof| proof.to_vec()).collect();
                let proof_client = NearProofs(proofs).try_to_vec().map_err(|e| {
                    Error::report_error(format!(
                        "[Near chain build_connection_proofs_and_client_state BuildNearProofs failed] -> Error({:?})",
                        e
                    ))
                })?;

                client_proof = Some(
                    CommitmentProofBytes::try_from(proof_client).map_err(Error::malformed_proof)?,
                );

                let (near_client_state_value, _) = self.query_client_state(
                    QueryClientStateRequest {
                        client_id: client_id.clone(),
                        height: QueryHeight::Specific(height),
                    },
                    IncludeProof::No,
                )?;

                consensus_proof = Option::from(
                    ConsensusProof::new(
                        CommitmentProofBytes::try_from(vec![1]).map_err(Error::malformed_proof)?,
                        near_client_state_value.latest_height(),
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
        let (_channel, _maybe_channel_proof) = self.query_channel(
            QueryChannelRequest {
                port_id: port_id.clone(),
                channel_id: channel_id.clone(),
                height: QueryHeight::Specific(height),
            },
            IncludeProof::No,
        )?;

        // ServerError(HandlerError(UnknownBlock { block_reference: BlockId(Height(135705911)) }))
        let query_response = retry_with_index(retry_strategy::default_strategy(), |_index| {
            let channel_path = ChannelEndsPath(port_id.clone(), channel_id.clone()).to_string();

            let block_reference: near_primitives::types::BlockReference =
                BlockId::Height(height.revision_height()).into();
            let prefix = near_primitives::types::StoreKey::from(channel_path.into_bytes());
            let result = self.block_on(self.client.query(
                &near_jsonrpc_client::methods::query::RpcQueryRequest {
                    block_reference,
                    request: near_primitives::views::QueryRequest::ViewState {
                        account_id: self.near_ibc_contract.clone(),
                        prefix,
                        include_proof: true,
                    },
                },
            ));

            match result {
                Ok(lcb) => RetryResult::Ok(lcb),
                Err(e) => {
                    warn!(
                        "ys-debug: retry get target_block_view(chan) with error: {}",
                        e
                    );
                    RetryResult::Retry(())
                }
            }
        })
        .map_err(|e| {
            Error::report_error(format!(
                "[Near chain build_channel_proofs get query_response failed ] -> Error({:?})",
                e
            ))
        })?;

        println!(
            "ys-debug: view state of channel proof: result: {:?}",
            query_response.block_height
        );

        let state = match query_response.kind {
            near_jsonrpc_primitives::types::query::QueryResponseKind::ViewState(state) => Ok(state),
            _ => Err(Error::report_error(
                "failed to get channel proof".to_string(),
            )),
        }?;
        let proofs: Vec<Vec<u8>> = state.proof.iter().map(|proof| proof.to_vec()).collect();

        let _commitment_prefix = self
            .get_commitment_prefix()
            .map_err(|e| Error::report_error(e.to_string()))?;

        let channel_proof = NearProofs(proofs).try_to_vec().map_err(|e| {
            Error::report_error(format!(
                "[Near chain build_channel_proofs Build NearProofs failed] -> Error({:?})",
                e
            ))
        })?;

        let channel_proof_bytes =
            CommitmentProofBytes::try_from(channel_proof).map_err(Error::malformed_proof)?;

        Proofs::new(channel_proof_bytes, None, None, None, height.increment())
            .map_err(Error::malformed_proof)
    }

    /// Builds the proof for packet messages.
    fn build_packet_proofs(
        &self,
        packet_type: PacketMsgType,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
        height: ICSHeight,
    ) -> Result<Proofs, Error> {
        let (maybe_packet_proof, channel_proof) = match packet_type {
            PacketMsgType::Recv => {
                let (_, _maybe_packet_proof) = self.query_packet_commitment(
                    QueryPacketCommitmentRequest {
                        port_id: port_id.clone(),
                        channel_id: channel_id.clone(),
                        sequence,
                        height: QueryHeight::Specific(height),
                    },
                    IncludeProof::No,
                )?;

                let query_response = retry_with_index(retry_strategy::default_strategy(), |_index| {
                    let packet_commitments_path = CommitmentsPath {
                        port_id: port_id.clone(),
                        channel_id: channel_id.clone(),
                        sequence,
                    }
                    .to_string();

                    let block_reference: near_primitives::types::BlockReference =
                        BlockId::Height(height.revision_height()).into();
                    let prefix =
                        near_primitives::types::StoreKey::from(packet_commitments_path.into_bytes());
                    let result = self.block_on(self.client.query(
                        &near_jsonrpc_client::methods::query::RpcQueryRequest {
                            block_reference,
                            request: near_primitives::views::QueryRequest::ViewState {
                                account_id: self.near_ibc_contract.clone(),
                                prefix,
                                include_proof: true,
                            },
                        },
                    ));

                    match result {
                        Ok(lcb) => RetryResult::Ok(lcb),
                        Err(e) => {
                            warn!(
                                "ys-debug: retry get target_block_view(pkt) with error: {}",
                                e
                            );
                            RetryResult::Retry(())
                        }
                    }
                })
                .map_err(|e| {
                    Error::report_error(format!(
                        "[Near chain build_packet_proofs get query_response failed ] -> Error({:?})",
                        e
                    ))
                })?;

                println!(
                    "ys-debug: view state of packet proof: result: {:?}",
                    query_response.block_height
                );

                let state = match query_response.kind {
                    near_jsonrpc_primitives::types::query::QueryResponseKind::ViewState(state) => {
                        Ok(state)
                    }
                    _ => Err(Error::report_error(
                        "failed to get packet proof".to_string(),
                    )),
                }?;
                let proofs: Vec<Vec<u8>> = state.proof.iter().map(|proof| proof.to_vec()).collect();

                let packet_proof = NearProofs(proofs).try_to_vec().map_err(|e| {
                    Error::report_error(format!(
                        "[Near chain build_channel_proofs Build NearProofs failed] -> Error({:?})",
                        e
                    ))
                })?;

                (Some(packet_proof), None)
            }
            PacketMsgType::Ack => {
                let (_, _maybe_packet_proof) = self.query_packet_acknowledgement(
                    QueryPacketAcknowledgementRequest {
                        port_id: port_id.clone(),
                        channel_id: channel_id.clone(),
                        sequence,
                        height: QueryHeight::Specific(height),
                    },
                    IncludeProof::No,
                )?;

                let query_response =
                    retry_with_index(retry_strategy::default_strategy(), |_index| {
                        let packet_commitments_path = AcksPath {
                            port_id: port_id.clone(),
                            channel_id: channel_id.clone(),
                            sequence,
                        }
                        .to_string();

                        let block_reference: near_primitives::types::BlockReference =
                            BlockId::Height(height.revision_height()).into();
                        let prefix = near_primitives::types::StoreKey::from(
                            packet_commitments_path.into_bytes(),
                        );
                        let result = self.block_on(self.client.query(
                            &near_jsonrpc_client::methods::query::RpcQueryRequest {
                                block_reference,
                                request: near_primitives::views::QueryRequest::ViewState {
                                    account_id: self.near_ibc_contract.clone(),
                                    prefix,
                                    include_proof: true,
                                },
                            },
                        ));

                        match result {
                            Ok(lcb) => RetryResult::Ok(lcb),
                            Err(e) => {
                                warn!(
                                    "ys-debug: retry get target_block_view(pkt) with error: {}",
                                    e
                                );
                                RetryResult::Retry(())
                            }
                        }
                    })
                    .map_err(|e| {
                        Error::report_error(format!( "[Near chain build_packet_proofs get query_response failed ] -> Error({:?})", e))
                    })?;

                println!(
                    "ys-debug: view state of packet proof: result: {:?}",
                    query_response.block_height
                );

                let state = match query_response.kind {
                    near_jsonrpc_primitives::types::query::QueryResponseKind::ViewState(state) => {
                        Ok(state)
                    }
                    _ => Err(Error::report_error(
                        "failed to get packet proof".to_string(),
                    )),
                }?;
                let proofs: Vec<Vec<u8>> = state.proof.iter().map(|proof| proof.to_vec()).collect();

                let packet_proof = NearProofs(proofs).try_to_vec().map_err(|e| {
                    Error::report_error(format!(
                        "[Near chain build_channel_proofs Build NearProofs failed] -> Error({:?})",
                        e
                    ))
                })?;

                (Some(packet_proof), None)
            }
            PacketMsgType::TimeoutUnordered => {
                let (_, _maybe_packet_proof) = self.query_packet_receipt(
                    QueryPacketReceiptRequest {
                        port_id: port_id.clone(),
                        channel_id: channel_id.clone(),
                        sequence,
                        height: QueryHeight::Specific(height),
                    },
                    IncludeProof::Yes,
                )?;

                // (maybe_packet_proof, None)
                (None, None)
            }
            PacketMsgType::TimeoutOrdered => {
                let (_, _maybe_packet_proof) = self.query_next_sequence_receive(
                    QueryNextSequenceReceiveRequest {
                        port_id,
                        channel_id,
                        height: QueryHeight::Specific(height),
                    },
                    IncludeProof::Yes,
                )?;

                // (maybe_packet_proof, None)
                (None, None)
            }
            PacketMsgType::TimeoutOnCloseUnordered => {
                let _channel_proof = {
                    let (_, maybe_channel_proof) = self.query_channel(
                        QueryChannelRequest {
                            port_id: port_id.clone(),
                            channel_id: channel_id.clone(),
                            height: QueryHeight::Specific(height),
                        },
                        IncludeProof::Yes,
                    )?;

                    let Some(channel_merkle_proof) = maybe_channel_proof else {
                        return Err(Error::queried_proof_not_found());
                    };

                    Some(
                        CommitmentProofBytes::try_from(channel_merkle_proof)
                            .map_err(Error::malformed_proof)?,
                    )
                };

                let (_, _maybe_packet_proof) = self.query_packet_receipt(
                    QueryPacketReceiptRequest {
                        port_id,
                        channel_id,
                        sequence,
                        height: QueryHeight::Specific(height),
                    },
                    IncludeProof::Yes,
                )?;

                // (maybe_packet_proof, channel_proof)
                (None, None)
            }
            PacketMsgType::TimeoutOnCloseOrdered => {
                let _channel_proof = {
                    let (_, maybe_channel_proof) = self.query_channel(
                        QueryChannelRequest {
                            port_id: port_id.clone(),
                            channel_id: channel_id.clone(),
                            height: QueryHeight::Specific(height),
                        },
                        IncludeProof::Yes,
                    )?;

                    let Some(channel_merkle_proof) = maybe_channel_proof else {
                        return Err(Error::queried_proof_not_found());
                    };

                    Some(
                        CommitmentProofBytes::try_from(channel_merkle_proof)
                            .map_err(Error::malformed_proof)?,
                    )
                };
                let (_, _maybe_packet_proof) = self.query_next_sequence_receive(
                    QueryNextSequenceReceiveRequest {
                        port_id,
                        channel_id,
                        height: QueryHeight::Specific(height),
                    },
                    IncludeProof::Yes,
                )?;

                // (maybe_packet_proof, channel_proof)
                (None, None)
            }
        };

        let Some(packet_proof) = maybe_packet_proof else {
            return Err(Error::queried_proof_not_found());
        };

        let proofs = Proofs::new(
            CommitmentProofBytes::try_from(packet_proof).map_err(Error::malformed_proof)?,
            None,
            None,
            channel_proof,
            height.increment(),
        )
        .map_err(Error::malformed_proof)?;

        Ok(proofs)
    }

    fn subscribe(&mut self) -> std::result::Result<Subscription, Error> {
        info!("subscribing to events...");
        let tx_monitor_cmd = match &self.tx_monitor_cmd {
            Some(tx_monitor_cmd) => tx_monitor_cmd,
            None => {
                let tx_monitor_cmd = self.init_event_monitor()?;
                self.tx_monitor_cmd = Some(tx_monitor_cmd);
                self.tx_monitor_cmd.as_ref().ok_or(Error::report_error(
                    "[Near Chain subscribe tx_monitor_cmd is None]".to_string(),
                ))?
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
            self.config.near_ibc_address.clone().into(),
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
                    .map_err(|e| Error::report_error(format!(
                        "[Near Chain collect_ibc_event_by_outcome decode near event failed] -> Error({})", e
                    )))?;
                if event_value["standard"].eq("near-ibc") {
                    let ibc_event: ibc::core::events::IbcEvent =
                        serde_json::from_value(event_value["raw-ibc-event"].clone())
                            .map_err(|e| Error::report_error(format!(
                                "[Near Chain collect_ibc_event_by_outcome decode near event to ibc event failed] -> Error({})", e
                            )))?;
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
                                .map_err(|e| Error::report_error(format!(
                                    "[Near Chain  collect_ibc_event_by_outcome call convert_ibc_event_to_hermes_ibc_event failed] -> Error({})", e
                                )))?,
                            height: Height::new(0, block_height)
                                .map_err(|e| Error::report_error(format!(
                                    "[Near Chain  collect_ibc_event_by_outcome build ibc height failed] -> Error({})", e
                                )))?,
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
                    .expect("Failed to get next_bps field.")
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
                        near_crypto::Signature::ED25519(data) => Signature::ED25519(*data),
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

mod retry_strategy {
    use crate::util::retry::clamp_total;
    use core::time::Duration;
    use retry::delay::Fibonacci;

    // Default parameters for the retrying mechanism
    const MAX_DELAY: Duration = Duration::from_secs(10); // 10 seconds
    const MAX_TOTAL_DELAY: Duration = Duration::from_secs(60); // 1 minutes
    const INITIAL_DELAY: Duration = Duration::from_secs(2); // 2 seconds

    pub fn default_strategy() -> impl Iterator<Item = Duration> {
        clamp_total(Fibonacci::from(INITIAL_DELAY), MAX_DELAY, MAX_TOTAL_DELAY)
    }
}
