use alloc::sync::Arc;
// use bytes::{Buf, Bytes};
// use core::{
//     convert::{TryFrom, TryInto},
//     future::Future,
//     str::FromStr,
//     time::Duration,
// };
// use futures::future::join_all;
// use num_bigint::BigInt;
// use std::{cmp::Ordering, thread};

use tokio::runtime::Runtime as TokioRuntime;
use tonic::codegen::http::Uri;
// use tracing::{error, instrument, trace, warn};

// use ibc_proto::cosmos::base::node::v1beta1::ConfigResponse;
// use ibc_proto::cosmos::staking::v1beta1::Params as StakingParams;
use ibc_proto::protobuf::Protobuf;
use ibc_relayer_types::applications::ics31_icq::response::CrossChainQueryResponse;
use ibc_relayer_types::clients::ics07_tendermint::client_state::{
    AllowUpdate, ClientState as TmClientState,
};
use ibc_relayer_types::clients::ics07_tendermint::consensus_state::ConsensusState as TMConsensusState;
use ibc_relayer_types::clients::ics07_tendermint::header::Header as TmHeader;
// use ibc_relayer_types::core::ics02_client::client_type::ClientType;
use ibc_relayer_types::core::ics02_client::error::Error as ClientError;
use ibc_relayer_types::core::ics02_client::events::UpdateClient;
use ibc_relayer_types::core::ics03_connection::connection::{
    ConnectionEnd, IdentifiedConnectionEnd,
};
use ibc_relayer_types::core::ics04_channel::channel::{ChannelEnd, IdentifiedChannelEnd};
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentPrefix;
use ibc_relayer_types::core::ics23_commitment::merkle::MerkleProof;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ChannelId, ConnectionId, PortId};
// use ibc_relayer_types::core::ics24_host::path::{
//     AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath, CommitmentsPath,
//     ConnectionsPath, ReceiptsPath, SeqRecvsPath,
// };
// use ibc_relayer_types::core::ics24_host::{
//     ClientUpgradePath, Path, IBC_QUERY_PATH, SDK_UPGRADE_QUERY_PATH,
// };
use ibc_relayer_types::signer::Signer;
use ibc_relayer_types::Height as ICSHeight;

// use tendermint::block::Height as TmHeight;
// use tendermint::node::info::TxIndexStatus;
use tendermint_light_client_verifier::types::LightBlock as TmLightBlock;
use tendermint_rpc::endpoint::broadcast::tx_sync::Response;
// use tendermint_rpc::endpoint::status;
use tendermint_rpc::HttpClient;

use crate::account::Balance;
use crate::chain::client::ClientSettings;
// use crate::chain::cosmos::batch::{
//     send_batched_messages_and_wait_check_tx, send_batched_messages_and_wait_commit,
//     sequential_send_batched_messages_and_wait_commit,
// };
// use crate::chain::cosmos::encode::key_pair_to_signer;
// use crate::chain::cosmos::fee::maybe_register_counterparty_payee;
// use crate::chain::cosmos::gas::{calculate_fee, mul_ceil};
// use crate::chain::cosmos::query::account::get_or_fetch_account;
// use crate::chain::cosmos::query::balance::{query_all_balances, query_balance};
// use crate::chain::cosmos::query::custom::cross_chain_query_via_rpc;
// use crate::chain::cosmos::query::denom_trace::query_denom_trace;
// use crate::chain::cosmos::query::status::query_status;
// use crate::chain::cosmos::query::tx::{
//     filter_matching_event, query_packets_from_block, query_packets_from_txs, query_txs,
// };
// use crate::chain::cosmos::query::{abci_query, fetch_version_specs, packet_query, QueryResponse};
use crate::chain::cosmos::types::account::Account;
use crate::chain::cosmos::types::config::TxConfig;
// use crate::chain::cosmos::types::gas::{
//     default_gas_from_config, gas_multiplier_from_config, max_gas_from_config,
// };
use crate::chain::endpoint::{ChainEndpoint, ChainStatus, HealthCheck};
use crate::chain::handle::Subscription;
use crate::chain::requests::*;
use crate::chain::tracking::TrackedMsgs;
use crate::client_state::{AnyClientState, IdentifiedAnyClientState};
use crate::config::ChainConfig;
use crate::consensus_state::{AnyConsensusState, AnyConsensusStateWithHeight};
use crate::denom::DenomTrace;
use crate::error::Error;
use crate::event::monitor::{EventMonitor, TxMonitorCmd};
use crate::event::IbcEventWithHeight;
use ibc_relayer_types::events::IbcEvent;

use crate::keyring::{KeyRing, Secp256k1KeyPair, SigningKeyPair};
use crate::light_client::tendermint::LightClient as TmLightClient;
// use crate::light_client::{LightClient, Verified};
use crate::misbehaviour::MisbehaviourEvidence;
// use crate::util::pretty::{
//     PrettyConsensusStateWithHeight, PrettyIdentifiedChannel, PrettyIdentifiedClientState,
//     PrettyIdentifiedConnection,
// };

use core::str::FromStr;
// use ibc_proto::google::protobuf::Any;
use ibc_relayer_types::clients::ics06_solomachine::ClientState as SmClientState;
use ibc_relayer_types::clients::ics06_solomachine::ConsensusState as SmConsensusState;
use ibc_relayer_types::clients::ics06_solomachine::PublicKey;
use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentRoot;
use ibc_relayer_types::timestamp::Timestamp;
use sp_keyring::AccountKeyring;
use std::time::{Duration, SystemTime};
use subxt::{tx::PairSigner, OnlineClient, SubstrateConfig};
use tracing::info;
use ibc_relayer_types::clients::ics06_solomachine::{Header as SmHeader, SignBytes, HeaderData};
use ibc_relayer_types::clients::ics06_solomachine::HeaderData as SmHeaderData;
use hdpath::StandardHDPath;
use crate::config::AddressType;




// pub mod batch;
// pub mod client;
// pub mod compatibility;
// pub mod encode;
// pub mod estimate;
// pub mod fee;
// pub mod gas;
// pub mod query;
// pub mod retry;
// pub mod simulate;
// pub mod tx;
// pub mod types;
// pub mod version;
// pub mod wait;

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
// pub const BLOCK_MAX_BYTES_MAX_FRACTION: f64 = 0.9;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubLightBlock {}

#[subxt::subxt(runtime_metadata_path = "./metadata.scale")]
pub mod substrate {}

pub struct SubstrateChain {
    config: ChainConfig,
    // tx_config: TxConfig,
    rpc_client: OnlineClient<SubstrateConfig>,
    // grpc_addr: Uri,
    // light_client: TmLightClient,
    rt: Arc<TokioRuntime>,
    // keybase: KeyRing<Secp256k1KeyPair>,

    // account: Option<Account>,

    // tx_monitor_cmd: Option<TxMonitorCmd>,
}

impl ChainEndpoint for SubstrateChain {
    type LightBlock = SubLightBlock;
    type Header = SmHeader;
    type ConsensusState = SmConsensusState;
    type ClientState = SmClientState;
    type SigningKeyPair = Secp256k1KeyPair;

    fn bootstrap(config: ChainConfig, rt: Arc<TokioRuntime>) -> Result<Self, Error> {
        let rpc_client = rt.block_on(OnlineClient::<SubstrateConfig>::new()).unwrap();

        Ok(Self {
            config,
            rpc_client,
            rt,
        })
    }

    fn shutdown(self) -> Result<(), Error> {
        unimplemented!();
    }

    fn keybase(&self) -> &KeyRing<Self::SigningKeyPair> {
        unimplemented!();
    }

    fn keybase_mut(&mut self) -> &mut KeyRing<Self::SigningKeyPair> {
        unimplemented!();
    }

    fn subscribe(&mut self) -> Result<Subscription, Error> {
        unimplemented!();
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
        unimplemented!();
    }

    /// Fetch a header from the chain at the given height and verify it.
    fn verify_header(
        &mut self,
        trusted: ICSHeight,
        target: ICSHeight,
        client_state: &AnyClientState,
    ) -> Result<Self::LightBlock, Error> {
        println!(
            "trusted: {:?}, target: {:?}, client_state: {:?}",
            trusted, target, client_state
        );
        Ok(SubLightBlock {})
    }

    /// Given a client update event that includes the header used in a client update,
    /// look for misbehaviour by fetching a header at same or latest height.
    fn check_misbehaviour(
        &mut self,
        update: &UpdateClient,
        client_state: &AnyClientState,
    ) -> Result<Option<MisbehaviourEvidence>, Error> {
        unimplemented!();
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
        let proto_msgs = tracked_msgs.msgs;

        // let msg: Vec<pallet_ibc::Any> = proto_msgs.iter().map(|m| pallet_ibc::Any{type_url: m.type_url.as_bytes().to_vec(), value: m.value}).collect();
        let msg: Vec<substrate::runtime_types::ibc_proto::google::protobuf::Any> = proto_msgs
            .iter()
            .map(
                |m| substrate::runtime_types::ibc_proto::google::protobuf::Any {
                    type_url: m.type_url.clone(),
                    value: m.value.clone(),
                },
            )
            .collect();
        let signer: PairSigner<SubstrateConfig, subxt::ext::sp_core::sr25519::Pair> =
            PairSigner::new(AccountKeyring::Alice.pair());

        let binding = self.rpc_client.tx();
        let tx = substrate::tx().ibc().deliver(msg);

        let runtime = self.rt.clone();
        let deliver = binding.sign_and_submit_then_watch_default(&tx, &signer);
        let result = runtime.block_on(deliver);
        println!("send_messages_and_wait_commit result: {:?}", result);
        let events = runtime.block_on(result.unwrap().wait_for_finalized_success());

        let ibc_events = events.unwrap().find_first::<substrate::ibc::events::IbcEvents>().unwrap().unwrap();
        let es: Vec<IbcEventWithHeight> = ibc_events.events.iter().map(|e|{
            match e {
                _ => IbcEventWithHeight { event: IbcEvent::ChainError("".to_string()), height: ICSHeight::new(0, 10).unwrap() }
            }
        }).collect();


        Ok(es)
    }

    fn send_messages_and_wait_check_tx(
        &mut self,
        tracked_msgs: TrackedMsgs,
    ) -> Result<Vec<Response>, Error> {
        unimplemented!();
    }

    /// Get the account for the signer
    fn get_signer(&self) -> Result<Signer, Error> {
        // $ subkey inspect //Alice
        // Secret Key URI `//Alice` is account:
        // Network ID:        substrate
        // Secret seed:       0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a
        // Public key (hex):  0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
        // Account ID:        0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
        // Public key (SS58): 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
        // SS58 Address:      5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
        Ok(
            Signer::from_str("0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a")
                .unwrap(),
        )
    }

    /// Get the chain configuration
    fn config(&self) -> &ChainConfig {
        &self.config
    }

    fn ibc_version(&self) -> Result<Option<semver::Version>, Error> {
        unimplemented!();
    }

    fn query_balance(&self, key_name: Option<&str>, denom: Option<&str>) -> Result<Balance, Error> {
        unimplemented!();
    }

    fn query_all_balances(&self, key_name: Option<&str>) -> Result<Vec<Balance>, Error> {
        unimplemented!();
    }

    fn query_denom_trace(&self, hash: String) -> Result<DenomTrace, Error> {
        unimplemented!();
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

        let finalized_head = self
            .rt
            .block_on(self.rpc_client.rpc().finalized_head())
            .unwrap();
        info!("finalized_header '{}'", finalized_head);
        let block = self
            .rt
            .block_on(self.rpc_client.rpc().block(Some(finalized_head)))
            .unwrap();
        // info!("extrinsics[0] '{:?}'", block.unwrap().block.extrinsics[0]);
        // let extrinsic = block.unwrap().block.extrinsics[0];
        // let b = parity_scale_codec::Decode::decode::<substrate::timestamp::calls::TransactionApi>(&mut extrinsic.0[..]);
        // let b = parity_scale_codec::Decode::decode::<substrate::runtime_types::kitchensink_runtime::RuntimeCall>(&mut extrinsic.0[..]);

        // let a = substrate::runtime_types::kitchensink_runtime::RuntimeCall::Timestamp.decode(&mut extrinsic);
        // let a = parity_scale_codec::Decode::decode::<substrate::timestamp::calls::Set>(&mut block.unwrap().block.extrinsics[0]);

        Ok(ChainStatus {
           height: ICSHeight::new(0, u64::from(block.unwrap().block.header.number)).unwrap(),
           timestamp: Timestamp::default(),
        })
    }

    fn query_clients(
        &self,
        request: QueryClientStatesRequest,
    ) -> Result<Vec<IdentifiedAnyClientState>, Error> {
        unimplemented!();
    }

    fn query_client_state(
        &self,
        request: QueryClientStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyClientState, Option<MerkleProof>), Error> {
        println!("query_client_state: request: {:?}", request);
        println!("query_client_state: include_proof: {:?}", include_proof);


        let client_id = substrate::runtime_types::ibc::core::ics24_host::identifier::ClientId(request.client_id.to_string());
        let storage = substrate::storage().ibc().client_states(client_id);

        let states = self
            .rt
            .block_on(self.rpc_client.storage().fetch(&storage, None))
            .unwrap();

        let client_state = AnyClientState::decode_vec(&states.unwrap()).map_err(Error::decode)?;


        println!("states: {:?}", client_state);
        Ok((client_state, None))
    }

    fn query_upgraded_client_state(
        &self,
        request: QueryUpgradedClientStateRequest,
    ) -> Result<(AnyClientState, MerkleProof), Error> {
        unimplemented!();
    }

    fn query_upgraded_consensus_state(
        &self,
        request: QueryUpgradedConsensusStateRequest,
    ) -> Result<(AnyConsensusState, MerkleProof), Error> {
        unimplemented!();
    }

    fn query_consensus_state(
        &self,
        request: QueryConsensusStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyConsensusState, Option<MerkleProof>), Error> {
        unimplemented!();
    }

    /// Query the heights of every consensus state for a given client.
    fn query_consensus_state_heights(
        &self,
        request: QueryConsensusStateHeightsRequest,
    ) -> Result<Vec<ICSHeight>, Error> {
        unimplemented!();
    }

    fn query_client_connections(
        &self,
        request: QueryClientConnectionsRequest,
    ) -> Result<Vec<ConnectionId>, Error> {
        unimplemented!();
    }

    fn query_connections(
        &self,
        request: QueryConnectionsRequest,
    ) -> Result<Vec<IdentifiedConnectionEnd>, Error> {
        unimplemented!();
    }

    fn query_connection(
        &self,
        request: QueryConnectionRequest,
        include_proof: IncludeProof,
    ) -> Result<(ConnectionEnd, Option<MerkleProof>), Error> {
        unimplemented!();
    }

    fn query_connection_channels(
        &self,
        request: QueryConnectionChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        unimplemented!();
    }

    fn query_channels(
        &self,
        request: QueryChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        unimplemented!();
    }

    fn query_channel(
        &self,
        request: QueryChannelRequest,
        include_proof: IncludeProof,
    ) -> Result<(ChannelEnd, Option<MerkleProof>), Error> {
        unimplemented!();
    }

    fn query_channel_client_state(
        &self,
        request: QueryChannelClientStateRequest,
    ) -> Result<Option<IdentifiedAnyClientState>, Error> {
        unimplemented!();
    }

    fn query_packet_commitment(
        &self,
        request: QueryPacketCommitmentRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        unimplemented!();
    }

    /// Queries the packet commitment hashes associated with a channel.
    fn query_packet_commitments(
        &self,
        request: QueryPacketCommitmentsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), Error> {
        unimplemented!();
    }

    fn query_packet_receipt(
        &self,
        request: QueryPacketReceiptRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        unimplemented!();
    }

    /// Queries the unreceived packet sequences associated with a channel.
    fn query_unreceived_packets(
        &self,
        request: QueryUnreceivedPacketsRequest,
    ) -> Result<Vec<Sequence>, Error> {
        unimplemented!();
    }

    fn query_packet_acknowledgement(
        &self,
        request: QueryPacketAcknowledgementRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        unimplemented!();
    }

    /// Queries the packet acknowledgment hashes associated with a channel.
    fn query_packet_acknowledgements(
        &self,
        request: QueryPacketAcknowledgementsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), Error> {
        unimplemented!();
    }

    /// Queries the unreceived acknowledgements sequences associated with a channel.
    fn query_unreceived_acknowledgements(
        &self,
        request: QueryUnreceivedAcksRequest,
    ) -> Result<Vec<Sequence>, Error> {
        unimplemented!();
    }

    fn query_next_sequence_receive(
        &self,
        request: QueryNextSequenceReceiveRequest,
        include_proof: IncludeProof,
    ) -> Result<(Sequence, Option<MerkleProof>), Error> {
        unimplemented!();
    }

    /// This function queries transactions for events matching certain criteria.
    /// 1. Client Update request - returns a vector with at most one update client event
    /// 2. Transaction event request - returns all IBC events resulted from a Tx execution
    fn query_txs(&self, request: QueryTxRequest) -> Result<Vec<IbcEventWithHeight>, Error> {
        unimplemented!();
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
        unimplemented!();
    }

    fn query_host_consensus_state(
        &self,
        request: QueryHostConsensusStateRequest,
    ) -> Result<Self::ConsensusState, Error> {
        unimplemented!();
    }

    fn build_client_state(
        &self,
        height: ICSHeight,
        settings: ClientSettings,
    ) -> Result<Self::ClientState, Error> {
        // Build the client state.
        let pk = PublicKey(tendermint::PublicKey::from_raw_secp256k1(&hex_literal::hex!("02c88aca653727db28e0ade87497c1f03b551143dedfd4db8de71689ad5e38421c")).unwrap());
        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let timestamp_nanos = duration_since_epoch.as_nanos(); // u128
        Ok(SmClientState {
            sequence: height.revision_height(),
            frozen_sequence: u64::max_value(),
            consensus_state: SmConsensusState {
                public_key: pk,
                diversifier: "oct".to_string(),
                // timestamp: timestamp_nanos as u64,
                timestamp: 9999,
                root: CommitmentRoot::from_bytes(&pk.to_bytes()),
            },
            allow_update_after_proposal: false,
        })
    }

    fn build_consensus_state(
        &self,
        light_block: Self::LightBlock,
    ) -> Result<Self::ConsensusState, Error> {
        let pk = PublicKey(tendermint::PublicKey::from_raw_secp256k1(&hex_literal::hex!("02c88aca653727db28e0ade87497c1f03b551143dedfd4db8de71689ad5e38421c")).unwrap());
        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let timestamp_nanos = duration_since_epoch.as_nanos(); // u128

        Ok(SmConsensusState {
            public_key: pk,
            diversifier: "oct".to_string(),
            // timestamp: timestamp_nanos as u64,
            timestamp: 9999,
            root: CommitmentRoot::from_bytes(&pk.to_bytes()),
        })
    }

    fn build_header(
        &mut self,
        trusted_height: ICSHeight,
        target_height: ICSHeight,
        client_state: &AnyClientState,
    ) -> Result<(Self::Header, Vec<Self::Header>), Error> {
        println!("trusted_height: {:?}, target_height: {:?}, client_state: {:?}", trusted_height, target_height, client_state);
        let pk = PublicKey(tendermint::PublicKey::from_raw_secp256k1(&hex_literal::hex!("02c88aca653727db28e0ade87497c1f03b551143dedfd4db8de71689ad5e38421c")).unwrap());
        println!("pk: {:?}", pk);
        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let timestamp_nanos = duration_since_epoch.checked_sub(Duration::from_secs(5)).unwrap().as_nanos() as u64; // u128
        let data = HeaderData{
            new_pub_key: Some(pk),
            new_diversifier: "oct".to_string(),
        };

        let bytes = SignBytes{
            sequence: client_state.latest_height().revision_height(),
            timestamp: timestamp_nanos,
            diversifier: "oct".to_string(),
            data_type: 9,
            // data: Protobuf::<SmHeaderData>::encode_vec(&data).expect("encoding to `Any` from `HeaderData`"),
            data: data.encode_vec().unwrap(),
        };

        let encoded_bytes = bytes.encode_vec().unwrap();
        println!("encoded_bytes: {:?}", encoded_bytes);
        let standard = StandardHDPath::from_str("m/44'/60'/0'/0/0").unwrap();
        println!("standard: {:?}", standard);



        // m/44'/60'/0'/0/0
        // 0xd73E35f53b8180b241E70C0e9040173dd8D0e2A0
        // 0x02c88aca653727db28e0ade87497c1f03b551143dedfd4db8de71689ad5e38421c
        // 0x281afd44d50ffd0bab6502cbb9bc58a7f9b53813c862db01836d46a27b51168c


        let key_pair = Secp256k1KeyPair::from_mnemonic("captain walk infant web eye return ahead once face sunny usage devote cotton car old check symbol antique derive wire kid solve forest fish", &standard, &AddressType::Cosmos, "oct").unwrap();
        println!("key_pair: {:?}", key_pair.public_key.to_string());
        let signature = key_pair.sign(&encoded_bytes).unwrap();
        println!("signature: {:?}", signature);


        let header = SmHeader{
            sequence: client_state.latest_height().revision_height(),
            timestamp: timestamp_nanos,
            signature,
            new_public_key: Some(pk),
            new_diversifier: "oct".to_string(),
        };

        Ok((header, vec![]))
    }

    fn maybe_register_counterparty_payee(
        &mut self,
        channel_id: &ChannelId,
        port_id: &PortId,
        counterparty_payee: &Signer,
    ) -> Result<(), Error> {
        unimplemented!();
    }

    fn cross_chain_query(
        &self,
        requests: Vec<CrossChainQueryRequest>,
    ) -> Result<Vec<CrossChainQueryResponse>, Error> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {}
