use super::Chain;
use crate::config::ChainConfig;
use crate::error::Error;
use crate::event::monitor::{EventMonitor, EventReceiver, TxMonitorCmd};
use crate::keyring::{KeyEntry, KeyRing, Store};
use crate::light_client::LightClient;
use ibc::events::IbcEvent;
use ibc::ics02_client::client_consensus::{AnyConsensusState, AnyConsensusStateWithHeight};
use ibc::ics02_client::client_state::{AnyClientState, IdentifiedAnyClientState};
use ibc::ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd};
use ibc::ics04_channel::channel::{ChannelEnd, IdentifiedChannelEnd};
use ibc::ics04_channel::packet::{PacketMsgType, Sequence};
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
use prost_types::Any;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::runtime::Runtime as TokioRuntime;
use crate::light_client::grandpa::LightClient as PGLightClient;
use std::thread;
use tendermint::account::Id as AccountId;
use bitcoin::hashes::hex::ToHex;
use std::str::FromStr;
use bech32::{ToBase32, Variant};
use std::future::Future;
use substrate_subxt::{ClientBuilder, PairSigner, Client, EventSubscription, system::ExtrinsicSuccessEvent};
use calls::{ibc::DeliverCallExt, NodeRuntime};
use sp_keyring::AccountKeyring;
use calls::ibc::{CreateClientEvent, OpenInitConnectionEvent, UpdateClientEvent, OpenTryConnectionEvent};
use calls::ibc::ClientStatesStoreExt;
use calls::ibc::ConnectionsStoreExt;
use calls::ibc::ConsensusStatesStoreExt;
use codec::{Decode, Encode};
use substrate_subxt::sp_runtime::traits::BlakeTwo256;
use substrate_subxt::sp_runtime::generic::Header;
use tendermint_proto::Protobuf;
use std::thread::sleep;
use std::time::Duration;

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

    async fn subscribe_events(
        &self,
        client: Client<NodeRuntime>,
    ) -> Result<Vec<IbcEvent>, Box<dyn std::error::Error>> {
        const COUNTER_SYSTEM_EVENT: i32 = 5;
        tracing::info!("In substrate: [subscribe_events]");

        let sub = client.subscribe_events().await?;
        let decoder = client.events_decoder();
        let mut sub = EventSubscription::<NodeRuntime>::new(sub, decoder);

        let mut events = Vec::new();
        let mut counter_system_event = 0;
        while let Some(raw_event) = sub.next().await {
            if let Err(err) = raw_event {
                println!("In substrate: [subscribe_events] >> raw_event error: {:?}", err);
                continue;
            }
            let raw_event = raw_event.unwrap();
            tracing::info!("In substrate: [subscribe_events] >> raw Event: {:?}", raw_event);
            let variant = raw_event.variant;
            tracing::info!("In substrate: [subscribe_events] >> variant: {:?}", variant);
            match variant.as_str() {
                "CreateClient" => {
                    let event = CreateClientEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
                    tracing::info!("In substrate: [subscribe_events] >> Create Client Event");

                    let height = event.height;
                    let client_id = event.client_id;
                    let client_type = event.client_type;
                    let consensus_height = event.consensus_height;
                    use ibc::ics02_client::events::Attributes;
                    events.push(IbcEvent::CreateClient(ibc::ics02_client::events::CreateClient(Attributes {
                        height: height.to_ibc_height(),
                        client_id: client_id.to_ibc_client_id(),
                        client_type: client_type.to_ibc_client_type(),
                        consensus_height: consensus_height.to_ibc_height()
                    })));
                    break;
                },
                "UpdateClient" => {
                    let event = UpdateClientEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
                    tracing::info!("In substrate: [subscribe_events] >> Update Client Event");

                    let height = event.height;
                    let client_id = event.client_id;
                    let client_type = event.client_type;
                    let consensus_height = event.consensus_height;
                    use ibc::ics02_client::events::Attributes;
                    events.push(IbcEvent::UpdateClient(ibc::ics02_client::events::UpdateClient{
                        common: Attributes {
                            height: height.to_ibc_height(),
                            client_id: client_id.to_ibc_client_id(),
                            client_type: client_type.to_ibc_client_type(),
                            consensus_height: consensus_height.to_ibc_height(),
                        },
                        header: None,
                    }));
                    // break;
                },
                "OpenInitConnection" => {
                    let event = OpenInitConnectionEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
                    tracing::info!("In substrate: [subscribe_events] >> OpenInitConnection Event");

                    let height = event.height;
                    let connection_id = event.connection_id.map(|val| val.to_ibc_connection_id());
                    let client_id = event.client_id;
                    let counterparty_connection_id = event.counterparty_connection_id.map(|val| val.to_ibc_connection_id());
                    let counterparty_client_id = event.counterparty_client_id;
                    use ibc::ics03_connection::events::Attributes;
                    events.push(IbcEvent::OpenInitConnection(ibc::ics03_connection::events::OpenInit(Attributes {
                        height: height.to_ibc_height(),
                        connection_id,
                        client_id: client_id.to_ibc_client_id(),
                        counterparty_connection_id,
                        counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
                    })));
                    sleep(Duration::from_secs(10));
                    break;
                },
                "OpenTryConnection" => {
                    let event = OpenTryConnectionEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
                    tracing::info!("In substrate: [subscribe_events] >> OpenTryConnection Event");

                    let height = event.height;
                    let connection_id = event.connection_id.map(|val| val.to_ibc_connection_id());
                    let client_id = event.client_id;
                    let counterparty_connection_id = event.counterparty_connection_id.map(|val| val.to_ibc_connection_id());
                    let counterparty_client_id = event.counterparty_client_id;
                    use ibc::ics03_connection::events::Attributes;
                    events.push(IbcEvent::OpenTryConnection(ibc::ics03_connection::events::OpenTry(Attributes {
                        height: height.to_ibc_height(),
                        connection_id,
                        client_id: client_id.to_ibc_client_id(),
                        counterparty_connection_id,
                        counterparty_client_id: counterparty_client_id.to_ibc_client_id(),
                    })));
                    sleep(Duration::from_secs(10));
                    break;
                },
                "ExtrinsicSuccess" => {
                    let event = ExtrinsicSuccessEvent::<NodeRuntime>::decode(&mut &raw_event.data[..]).unwrap();
                    tracing::info!("In substrate: [subscribe_events] >> SystemEvent: {:?}", event);
                    if counter_system_event < COUNTER_SYSTEM_EVENT {
                        tracing::info!("In substrate: [subscribe_events] >> counter_system_event: {}", counter_system_event);
                        counter_system_event += 1;
                    } else {
                        tracing::info!("In substrate: [subscribe_events] >> counter_system_event: {}", counter_system_event);
                        break;
                    }
                }
                _ =>  {
                    tracing::info!("In substrate: [subscribe_events] >> Unknown event");
                }
            }
        }
        Ok(events)
    }

    async fn get_latest_height(&self, client: Client<NodeRuntime>) -> Result<u64, Box<dyn std::error::Error>> {
        tracing::info!("In Substrate: [get_latest_height]");
        // let mut blocks = client.subscribe_finalized_blocks().await?;
        let mut blocks = client.subscribe_blocks().await?;
        let height= match blocks.next().await {
            Ok(Some(header)) => {
                header.number as u64
            },
            Ok(None) => {
                tracing::info!("In Substrate: [get_latest_height] >> None");
                0
            },
            Err(err) =>  {
              tracing::info!(" In substrate: [get_latest_height] >> error: {:?} ", err);
                0
            },
        };
        tracing::info!("In substrate: [get_latest_height] >> height: {}", height);
        Ok(height)
    }

    async fn get_connectionend(&self, connection_identifier: &ConnectionId, client: Client<NodeRuntime>) -> Result<ConnectionEnd, Box<dyn std::error::Error>> {
        let mut block = client.subscribe_finalized_blocks().await?;
        let block_header = block.next().await.unwrap().unwrap();

        let block_hash = block_header.hash();
        tracing::info!("In substrate: [get_connectionend] >> block_hash: {:?}", block_hash);

        let data = client.connections(connection_identifier.as_bytes().to_vec(), Some(block_hash)).await?;
        let connection_end = ConnectionEnd::decode_vec(&*data).unwrap();

        Ok(connection_end)
    }

    async fn get_client_state(&self, client_id:  &ClientId, client: Client<NodeRuntime>) -> Result<GPClientState, Box<dyn std::error::Error>> {

        let mut block = client.subscribe_finalized_blocks().await?;
        let block_header = block.next().await.unwrap().unwrap();

        let block_hash = block_header.hash();

        let data = client
            .client_states(
                client_id.as_bytes().to_vec(),
                Some(block_hash),
            )
            .await?;
        let client_state = AnyClientState::decode_vec(&*data).unwrap();
        let client_state = match client_state {
            AnyClientState::Grandpa(client_state) => client_state,
            _ => panic!("wrong client state type"),
        };

        Ok(client_state)
    }

    async fn get_client_consensus(&self, client_id:  &ClientId, height: ICSHeight, client: Client<NodeRuntime>) -> Result<GPConsensusState, Box<dyn std::error::Error>> {

        let mut block = client.subscribe_finalized_blocks().await?;
        let block_header = block.next().await.unwrap().unwrap();

        let block_hash = block_header.hash();
        tracing::info!("In substrate: [get_client_consensus] >> block_hash: {:?}", block_hash);

        let data = client
            .consensus_states(
                (
                    client_id.as_bytes().to_vec(),
                    height.encode_vec().unwrap()
                ),
                Some(block_hash),
            )
            .await?;
        let consensus_state = AnyConsensusState::decode_vec(&*data).unwrap();
        let consensus_state = match consensus_state {
            AnyConsensusState::Grandpa(consensus_state) => consensus_state,
            _ => panic!("wrong consensus_state type"),
        };

        Ok(consensus_state)
    }

    async fn get_consensus_state_with_height(&self, client_id: &ClientId, client: Client<NodeRuntime>)
        -> Result<Vec<(Height, GPConsensusState)>, Box<dyn std::error::Error>> {
        use jsonrpsee_types::to_json_value;
        use substrate_subxt::RpcClient;

        let client_id = client_id.as_bytes().to_vec();
        let param = &[to_json_value(client_id)?];
        let rpc_client = client.rpc_client();
        let ret: Vec<(Vec<u8>, Vec<u8>)> = rpc_client.request("get_consensus_state_with_height", param).await?;

        let mut result = vec![];
        for (height, consensus_state) in ret.iter() {
            let height = Height::decode_vec(&*height).unwrap();
            let consensus_state = AnyConsensusState::decode_vec(&*consensus_state).unwrap();
            let consensus_state = match consensus_state {
                AnyConsensusState::Grandpa(consensus_state) => consensus_state,
                _ => panic!("wrong consensus_state type"),
            };
            result.push((height, consensus_state));
        }

        Ok(result)
    }



}

impl Chain for SubstrateChain {
    type LightBlock = ();
    type Header = GPHeader;
    type ConsensusState = GPConsensusState;
    type ClientState = GPClientState;

    fn bootstrap(config: ChainConfig, rt: Arc<TokioRuntime>) -> Result<Self, Error> {
        tracing::info!("in Substrate: [bootstrap function]");

        let websocket_url = config.substrate_websocket_addr.clone();

        let chain = Self {
            config,
            websocket_url,
            rt,
        };

        Ok(chain)
    }

    fn init_light_client(&self) -> Result<Box<dyn LightClient<Self>>, Error> {
        tracing::info!("In Substrate: [init_light_client]");

        let light_client = PGLightClient::new();

        Ok(Box::new(light_client))
    }

    fn init_event_monitor(
        &self,
        rt: Arc<TokioRuntime>,
    ) -> Result<(EventReceiver, TxMonitorCmd), Error> {
        tracing::info!("in Substrate: [init_event_mointor]");

        tracing::info!("In Substrate: [init_event_mointor] >> websocket addr: {}", self.config.websocket_addr.clone());

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

    fn send_msgs(&mut self, proto_msgs: Vec<Any>) -> Result<Vec<IbcEvent>, Error> {
        tracing::info!("in Substrate: [send_msg]");
        use tokio::task;
        use std::time::Duration;

        let msg : Vec<pallet_ibc::Any> = proto_msgs.into_iter().map(|val| val.into()).collect();

        let signer = PairSigner::new(AccountKeyring::Bob.pair());


        let client = async {
                sleep(Duration::from_secs(3));

                let client = ClientBuilder::<NodeRuntime>::new().set_url(&self.websocket_url.clone())
                    .build().await.unwrap();

                let result = client.deliver(&signer, msg, 0).await;

                tracing::info!("in Substrate: [send_msg] >> result no unwrap: {:?}", result);

                let result = result.unwrap();
                tracing::info!("in Substrate: [send_msg] >> result : {:?}", result);

                result
        };

        let _ = self.block_on(client);

        let get_ibc_event = async {
            let client = ClientBuilder::<NodeRuntime>::new().set_url(&self.websocket_url.clone())
                .build().await.unwrap();
            let result = self.subscribe_events(client.clone()).await.unwrap();
            tracing::info!("In Substrate: [send_msg] >> get_ibc_event: {:?}", result);

            result
        };

        let ret = self.block_on(get_ibc_event);


        Ok(ret)
    }

    fn get_signer(&mut self) -> Result<Signer, Error> {
        tracing::info!("in Substrate: [get_signer]");
        tracing::info!("In Substraet: [get signer] >> key_name: {}", self.config.key_name.clone());

        fn get_dummy_account_id_raw() -> String {
            "0CDA3F47EF3C4906693B170EF650EB968C5F4B2C".to_string()
        }

        pub fn get_dummy_account_id() -> AccountId {
            AccountId::from_str(&get_dummy_account_id_raw()).unwrap()
        }

        let signer = Signer::new(get_dummy_account_id().to_string());

        tracing::info!("in Substrate: [get_signer] >>  signer {}", signer);

        Ok(signer)
    }

    fn get_key(&mut self) -> Result<KeyEntry, Error> {
        tracing::info!("in Substraet: [get_key]");

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

        let latest_height = async {
            let client = ClientBuilder::<NodeRuntime>::new().set_url(&self.websocket_url.clone())
                .build().await.unwrap();
            let height = self.get_latest_height(client.clone()).await.unwrap();
            tracing::info!("In Substrate: [query_latest_height] >> height: {:?}", height);

            height
        };

        let revision_height =  self.block_on(latest_height);
        let latest_height = Height::new(0, revision_height);
        Ok(latest_height)
    }

    fn query_clients(
        &self,
        request: QueryClientStatesRequest,
    ) -> Result<Vec<IdentifiedAnyClientState>, Error> {
        tracing::info!("in Substrate: [query_clients]");

        todo!()
    }

    fn query_client_state(
        &self,
        client_id: &ClientId,
        height: ICSHeight,
    ) -> Result<Self::ClientState, Error> {
        tracing::info!("in Substrate: [query_client_state]");
        tracing::info!("in Substrate: [query_client_state] >> height: {:?}", height);

        let client_state = async {
            let client = ClientBuilder::<NodeRuntime>::new().set_url(&self.websocket_url.clone())
                .build().await.unwrap();
            let client_state = self.get_client_state(client_id, client.clone()).await.unwrap();

            client_state
        };

        let client_state =  self.block_on(client_state);
        tracing::info!("in Substrate: [query_client_state] >> client_state: {:?}", client_state);

        Ok(client_state)
    }

    fn query_consensus_states(
        &self,
        request: QueryConsensusStatesRequest,
    ) -> Result<Vec<AnyConsensusStateWithHeight>, Error> {
        tracing::info!("in Substrate: [query_consensus_states]");
        let request_client_id = ClientId::from_str(request.client_id.as_str()).unwrap();

        let consensus_state = async {
            let client = ClientBuilder::<NodeRuntime>::new().set_url(&self.websocket_url.clone())
                .build().await.unwrap();
            let consensus_state = self.get_consensus_state_with_height(&request_client_id, client.clone()).await.unwrap();

            consensus_state
        };

        let consensus_state: Vec<(Height, GPConsensusState)> =  self.block_on(consensus_state);

        let mut any_consensus_state_with_height = vec![];
        for (height, consensus_state) in consensus_state.into_iter() {
            let consensus_state = AnyConsensusState::Grandpa(consensus_state);
            let tmp = AnyConsensusStateWithHeight {
                height: height,
                consensus_state,
            };
            any_consensus_state_with_height.push(tmp.clone());

            tracing::info!("In Substrate: [query_consensus_state] >> any_consensus_state_with_height: {:?}", tmp);
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

        todo!()
    }

    fn query_upgraded_client_state(
        &self,
        height: ICSHeight,
    ) -> Result<(Self::ClientState, MerkleProof), Error> {
        tracing::info!("in Substrate: [query_upgraded_client_state]");

        todo!()
    }

    fn query_upgraded_consensus_state(
        &self,
        height: ICSHeight,
    ) -> Result<(Self::ConsensusState, MerkleProof), Error> {
        tracing::info!("in Substrate: [query_upgraded_consensus_state]");

        todo!()
    }

    fn query_connections(
        &self,
        request: QueryConnectionsRequest,
    ) -> Result<Vec<IdentifiedConnectionEnd>, Error> {
        tracing::info!("in Substrate: [query_connections]");

        todo!()
    }

    fn query_client_connections(
        &self,
        request: QueryClientConnectionsRequest,
    ) -> Result<Vec<ConnectionId>, Error> {
        tracing::info!("in Substrate: [query_client_connections]");

        todo!()
    }

    fn query_connection(
        &self,
        connection_id: &ConnectionId,
        height: ICSHeight,
    ) -> Result<ConnectionEnd, Error> {
        tracing::info!("in Substrate: [query_connection]");

        let connection_end = async {
            let client = ClientBuilder::<NodeRuntime>::new().set_url(&self.websocket_url.clone())
                .build().await.unwrap();
            let connection_end = self.get_connectionend(connection_id, client.clone()).await.unwrap();
            tracing::info!("In Substrate: [query_connection] >> connection_id: {:?}, connection_end: {:?}", connection_id, connection_end);

            connection_end
        };

        let connection_end =  self.block_on(connection_end);

        Ok(connection_end)
    }

    fn query_connection_channels(
        &self,
        request: QueryConnectionChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        tracing::info!("in Substrate: [query_connection_channels]");

        todo!()
    }

    fn query_channels(
        &self,
        request: QueryChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        tracing::info!("in Substrate: [query_channels]");

        todo!()
    }

    fn query_channel(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        height: ICSHeight,
    ) -> Result<ChannelEnd, Error> {
        tracing::info!("in Substrate: [query_channel]");

        todo!()
    }

    fn query_channel_client_state(
        &self,
        request: QueryChannelClientStateRequest,
    ) -> Result<Option<IdentifiedAnyClientState>, Error> {
        tracing::info!("in Substrate: [query_channel_client_state]");

        todo!()
    }

    fn query_packet_commitments(
        &self,
        request: QueryPacketCommitmentsRequest,
    ) -> Result<(Vec<PacketState>, ICSHeight), Error> {
        tracing::info!("in Substrate: [query_packet_commitments]");

        todo!()
    }

    fn query_unreceived_packets(
        &self,
        request: QueryUnreceivedPacketsRequest,
    ) -> Result<Vec<u64>, Error> {
        tracing::info!("in Substrate: [query_unreceived_packets]");

        todo!()
    }

    fn query_packet_acknowledgements(
        &self,
        request: QueryPacketAcknowledgementsRequest,
    ) -> Result<(Vec<PacketState>, ICSHeight), Error> {
        tracing::info!("in Substrate: [query_packet_acknowledegements]");
        todo!()
    }

    fn query_unreceived_acknowledgements(
        &self,
        request: QueryUnreceivedAcksRequest,
    ) -> Result<Vec<u64>, Error> {
        tracing::info!("in Substraete: [query_unreceived_acknowledegements]");

        todo!()
    }

    fn query_next_sequence_receive(
        &self,
        request: QueryNextSequenceReceiveRequest,
    ) -> Result<Sequence, Error> {
        tracing::info!("in Substrate: [query_next_sequence_receiven]");

        todo!()
    }

    fn query_txs(&self, request: QueryTxRequest) -> Result<Vec<IbcEvent>, Error> {
        tracing::info!("in Substrate: [query_txs]");

        todo!()
    }

    fn proven_client_state(
        &self,
        client_id: &ClientId,
        height: ICSHeight,
    ) -> Result<(Self::ClientState, MerkleProof), Error> {
        tracing::info!("in Substrate: [proven_client_state]");

        let client_state = async {
            let client = ClientBuilder::<NodeRuntime>::new().set_url(&self.websocket_url.clone())
                .build().await.unwrap();
            let client_state = self.get_client_state(client_id, client.clone()).await.unwrap();
            tracing::info!("In Substrate: [proven_client_state] >> client_state : {:?}", client_state);

            client_state
        };

        let client_state =  self.block_on(client_state);
        tracing::info!("in Substrate: [prove_client_state] >> before modify height client_state : {:?}", client_state);
        // client_state.latest_height = height;
        tracing::info!("in Substrate: [prove_client_state] >> after modify height client_state : {:?}", client_state);
        // assert_eq!(height, client_state.latest_height, "prove_client_state are not equal client_state");

        Ok((client_state, get_dummy_merkle_proof()))
    }

    fn proven_connection(
        &self,
        connection_id: &ConnectionId,
        height: ICSHeight,
    ) -> Result<(ConnectionEnd, MerkleProof), Error> {
        tracing::info !("in Substrate: [proven_connection]");

        let connection_end = async {
            let client = ClientBuilder::<NodeRuntime>::new().set_url(&self.websocket_url.clone())
                .build().await.unwrap();
            let connection_end = self.get_connectionend(connection_id, client.clone()).await.unwrap();
            tracing::info!("In Substrate: [proven_connection] >> connection_end: {:?}", connection_end);

            connection_end
        };

        let connection_end =  self.block_on(connection_end);

        Ok((connection_end, get_dummy_merkle_proof()))
    }

    fn proven_client_consensus(
        &self,
        client_id: &ClientId,
        consensus_height: ICSHeight,
        height: ICSHeight,
    ) -> Result<(Self::ConsensusState, MerkleProof), Error> {
        tracing::info!("in Substrate: [proven_client_consensus]");

        let consensus_state = async {
            let client = ClientBuilder::<NodeRuntime>::new().set_url(&self.websocket_url.clone())
                .build().await.unwrap();
            let consensus_state = self.get_client_consensus(client_id, consensus_height, client.clone()).await.unwrap();
            tracing::info!("In Substrate: [proven_client_consensus] >> consensus_state : {:?}", consensus_state);

            consensus_state
        };

        let consensus_state =  self.block_on(consensus_state);

        Ok((consensus_state, get_dummy_merkle_proof()))
    }

    fn proven_channel(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        height: ICSHeight,
    ) -> Result<(ChannelEnd, MerkleProof), Error> {
        tracing::info!("in Substrate: [proven_channel]");

        todo!()
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

        todo!()
    }

    fn build_client_state(&self, height: ICSHeight) -> Result<Self::ClientState, Error> {
        tracing::info!("in Substrate: [build_client_state]");

        let chain_id = self.id().clone();
        tracing::info!("in Substrate: [build_client_state] >> chain_id = {:?}", chain_id);

        let frozen_height = Height::zero();
        tracing::info!("in Substrate: [build_client_state] >> frozen_height = {:?}", frozen_height);

        use ibc::ics02_client::client_state::AnyClientState;
        use ibc::ics10_grandpa::client_state::ClientState as GRANDPAClientState;

        // Create mock grandpa client state
        let client_state = GRANDPAClientState::new(chain_id, height, frozen_height).unwrap();

        tracing::info!("in Substrate: [build_client_state] >> client_state: {:?}", client_state);

        Ok(client_state)
    }

    fn build_consensus_state(
        &self,
        light_block: Self::LightBlock,
    ) -> Result<Self::ConsensusState, Error> {
        tracing::info!("in Substrate: [build_consensus_state]");

        // Create mock grandpa consensus state
        use ibc::ics10_grandpa::consensus_state::ConsensusState as GRANDPAConsensusState;

        let consensus_state = GRANDPAConsensusState::new(CommitmentRoot::from(vec![1, 2, 3, 4]));

        Ok(consensus_state)
    }

    fn build_header(
        &self,
        trusted_height: ICSHeight,
        target_height: ICSHeight,
        client_state: &AnyClientState,
        light_client: &mut dyn LightClient<Self>,
    ) -> Result<(Self::Header, Vec<Self::Header>), Error> {
        tracing::info!("in Substrate: [build_header]");
        tracing::info!("in Substrate: [build_header] >> Trusted_height: {}, Target_height: {}, client_state: {:?}",
            trusted_height, target_height, client_state);
        tracing::info!("in Substrate: [build_header] >> GPHEADER: {:?}", GPHeader::new(target_height.revision_height));

        Ok((GPHeader::new(target_height.revision_height), vec![GPHeader::new(trusted_height.revision_height)]))
    }
}

/// Returns a dummy `MerkleProof`, for testing only!
pub fn get_dummy_merkle_proof() -> MerkleProof {
    let parsed = ibc_proto::ics23::CommitmentProof { proof: None };
    let mproofs: Vec<ibc_proto::ics23::CommitmentProof> = vec![parsed];
    MerkleProof { proofs: mproofs }
}