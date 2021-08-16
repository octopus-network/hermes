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
use ibc::ics23_commitment::commitment::CommitmentPrefix;
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
use substrate_subxt::{ClientBuilder, PairSigner, Client};
use calls::{ibc::DeliverCallExt, NodeRuntime};
use sp_keyring::AccountKeyring;

// use tendermint_light_client::types::LightBlock as TMLightBlock;

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

    // async fn subscribe_events(
    //     client: Client<runtime::AppchainRuntime>,
    // ) -> Result<(), Box<dyn std::error::Error>> {
    //     let sub = client.subscribe_events().await?;
    //     let decoder = client.events_decoder();
    //     let mut sub = EventSubscription::<runtime::AppchainRuntime>::new(sub, decoder);
    //     sub.filter_event::<BurnedEvent<_>>();
    //     while let Some(raw_event) = sub.next().await {
    //         if let Err(err) = raw_event {
    //             println!("raw_event error: {:?}", err);
    //             continue;
    //         }
    //         let raw_event = raw_event.unwrap();
    //         let event = BurnedEvent::<runtime::AppchainRuntime>::decode(&mut &raw_event.data[..]);
    //         if let Ok(e) = event {
    //             println!("Burned success: value: {:?}", e.amount);
    //         } else {
    //             println!("Failed to decode OctopusAppchain Event");
    //         }
    //     }
    //     Ok(())
    // }

}

impl Chain for SubstrateChain {
    type LightBlock = ();
    type Header = GPHeader;
    type ConsensusState = GPConsensusState;
    type ClientState = GPClientState;

    fn bootstrap(config: ChainConfig, rt: Arc<TokioRuntime>) -> Result<Self, Error> {
        tracing::info!("in bootstrap");

        let websocket_url = config.substrate_websocket_addr.clone();
        // tracing::info!("websocket url : {}", websocket_url);

        let chain = Self {
            config,
            websocket_url,
            rt,
        };

        Ok(chain)
    }

    fn init_light_client(&self) -> Result<Box<dyn LightClient<Self>>, Error> {
        tracing::info!("in init_light_client");

        let light_client = PGLightClient::new();

        Ok(Box::new(light_client))
    }

    fn init_event_monitor(
        &self,
        rt: Arc<TokioRuntime>,
    ) -> Result<(EventReceiver, TxMonitorCmd), Error> {
        tracing::info!("in init event mointor");
        tracing::info!("websocket addr: {}", self.config.websocket_addr.clone());

        let (mut event_monitor, event_receiver, monitor_tx) = EventMonitor::new(
            self.config.id.clone(),
            self.config.websocket_addr.clone(),
            rt,
        )
            .map_err(Error::event_monitor)?;
        tracing::info!("in event mointor");

        event_monitor.subscribe().map_err(Error::event_monitor)?;

        thread::spawn(move || event_monitor.run());

        Ok((event_receiver, monitor_tx))
    }

    fn shutdown(self) -> Result<(), Error> {
        tracing::info!("in shutdown");

        Ok(())
    }

    fn id(&self) -> &ChainId {
        tracing::info!("in id");

        &self.config().id
    }

    fn keybase(&self) -> &KeyRing {
        tracing::info!("in keybase");

        todo!()
    }

    fn keybase_mut(&mut self) -> &mut KeyRing {
        tracing::info!("in keybase mut");

        todo!()
    }

    fn send_msgs(&mut self, proto_msgs: Vec<Any>) -> Result<Vec<IbcEvent>, Error> {
        tracing::info!("in send msg");

        let msg : Vec<pallet_ibc::Any> = proto_msgs.into_iter().map(|val| val.into()).collect();


        let signer = PairSigner::new(AccountKeyring::Bob.pair());

        let client = async {
                let client = ClientBuilder::<NodeRuntime>::new().set_url(&self.websocket_url.clone())
                    .build().await.unwrap();
                let result = client.deliver(&signer, msg, 0).await.unwrap();
                tracing::info!("result: {:?}", result);

                result
        };

        let ret = self.block_on(client);

        Ok(Vec::new())
    }

    fn get_signer(&mut self) -> Result<Signer, Error> {
        tracing::info!("in get signer");
        tracing::info!("key_name: {}", self.config.key_name.clone());

        fn get_dummy_account_id_raw() -> String {
            "0CDA3F47EF3C4906693B170EF650EB968C5F4B2C".to_string()
        }

        pub fn get_dummy_account_id() -> AccountId {
            AccountId::from_str(&get_dummy_account_id_raw()).unwrap()
        }

        let signer = Signer::new(get_dummy_account_id().to_string());

        tracing::info!("in build create client: signer {}", signer);

        Ok(signer)
    }

    fn get_key(&mut self) -> Result<KeyEntry, Error> {
        tracing::info!("in get key");

        todo!()
    }

    fn query_commitment_prefix(&self) -> Result<CommitmentPrefix, Error> {
        tracing::info!("in query commitment prefix");

        todo!()
    }

    fn query_latest_height(&self) -> Result<ICSHeight, Error> {
        tracing::info!("in query latest height");
        let latest_height = Height::new(0, 26);
        Ok(latest_height)
    }

    fn query_clients(
        &self,
        request: QueryClientStatesRequest,
    ) -> Result<Vec<IdentifiedAnyClientState>, Error> {
        tracing::info!("in query clients");

        todo!()
    }

    fn query_client_state(
        &self,
        client_id: &ClientId,
        height: ICSHeight,
    ) -> Result<Self::ClientState, Error> {
        tracing::info!("in query client state");

        let chain_id = ChainId::new("ibc".to_string(), 0);
        tracing::info!("chain_id = {:?}", chain_id);

        let frozen_height = Height::new(0, 0);
        tracing::info!("frozen_height = {:?}", frozen_height);

        use ibc::ics02_client::client_state::AnyClientState;
        use ibc::ics10_grandpa::client_state::ClientState as GRANDPAClientState;

        // Create mock grandpa client state
        let client_state = GRANDPAClientState::new(chain_id, height, frozen_height).unwrap();

        tracing::info!("client_state: {:?}", client_state);

        Ok(client_state)
    }

    fn query_consensus_states(
        &self,
        request: QueryConsensusStatesRequest,
    ) -> Result<Vec<AnyConsensusStateWithHeight>, Error> {
        tracing::info!("in query consensus states");

        // Create mock grandpa consensus state
        use ibc::ics10_grandpa::consensus_state::ConsensusState as GRANDPAConsensusState;

        let consensus_state = AnyConsensusState::Grandpa(GRANDPAConsensusState::new());
        let any_consensus_state_with_height = AnyConsensusStateWithHeight {
            height: Height::new(0, 0),
            consensus_state,
        };
        tracing::info!("Any consensus state with height: {:?}", any_consensus_state_with_height);


        Ok(vec![any_consensus_state_with_height])
    }

    fn query_consensus_state(
        &self,
        client_id: ClientId,
        consensus_height: ICSHeight,
        query_height: ICSHeight,
    ) -> Result<AnyConsensusState, Error> {
        tracing::info!("in query consensus state");

        todo!()
    }

    fn query_upgraded_client_state(
        &self,
        height: ICSHeight,
    ) -> Result<(Self::ClientState, MerkleProof), Error> {
        tracing::info!("in query upgraded client state");

        todo!()
    }

    fn query_upgraded_consensus_state(
        &self,
        height: ICSHeight,
    ) -> Result<(Self::ConsensusState, MerkleProof), Error> {
        tracing::info!("in query upgraded consensus state");

        todo!()
    }

    fn query_connections(
        &self,
        request: QueryConnectionsRequest,
    ) -> Result<Vec<IdentifiedConnectionEnd>, Error> {
        tracing::info!("in query connections");

        todo!()
    }

    fn query_client_connections(
        &self,
        request: QueryClientConnectionsRequest,
    ) -> Result<Vec<ConnectionId>, Error> {
        tracing::info!("in query client connections");

        todo!()
    }

    fn query_connection(
        &self,
        connection_id: &ConnectionId,
        height: ICSHeight,
    ) -> Result<ConnectionEnd, Error> {
        tracing::info!("in query connection");

        todo!()
    }

    fn query_connection_channels(
        &self,
        request: QueryConnectionChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        tracing::info!("in query connection channels");

        todo!()
    }

    fn query_channels(
        &self,
        request: QueryChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        tracing::info!("in query channels");

        todo!()
    }

    fn query_channel(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        height: ICSHeight,
    ) -> Result<ChannelEnd, Error> {
        tracing::info!("in query channel");

        todo!()
    }

    fn query_channel_client_state(
        &self,
        request: QueryChannelClientStateRequest,
    ) -> Result<Option<IdentifiedAnyClientState>, Error> {
        tracing::info!("in query channel client state");

        todo!()
    }

    fn query_packet_commitments(
        &self,
        request: QueryPacketCommitmentsRequest,
    ) -> Result<(Vec<PacketState>, ICSHeight), Error> {
        tracing::info!("in query packet commitments");

        todo!()
    }

    fn query_unreceived_packets(
        &self,
        request: QueryUnreceivedPacketsRequest,
    ) -> Result<Vec<u64>, Error> {
        tracing::info!("in query unreceived packets");

        todo!()
    }

    fn query_packet_acknowledgements(
        &self,
        request: QueryPacketAcknowledgementsRequest,
    ) -> Result<(Vec<PacketState>, ICSHeight), Error> {
        tracing::info!("in query packet acknowledegements");
        todo!()
    }

    fn query_unreceived_acknowledgements(
        &self,
        request: QueryUnreceivedAcksRequest,
    ) -> Result<Vec<u64>, Error> {
        tracing::info!("in query unreceived acknowledegements");

        todo!()
    }

    fn query_next_sequence_receive(
        &self,
        request: QueryNextSequenceReceiveRequest,
    ) -> Result<Sequence, Error> {
        tracing::info!("in query next sequence receiven");

        todo!()
    }

    fn query_txs(&self, request: QueryTxRequest) -> Result<Vec<IbcEvent>, Error> {
        tracing::info!("in query txs");

        todo!()
    }

    fn proven_client_state(
        &self,
        client_id: &ClientId,
        height: ICSHeight,
    ) -> Result<(Self::ClientState, MerkleProof), Error> {
        tracing::info!("in proven client state");

        todo!()
    }

    fn proven_connection(
        &self,
        connection_id: &ConnectionId,
        height: ICSHeight,
    ) -> Result<(ConnectionEnd, MerkleProof), Error> {
        tracing::info!("in proven connection");

        todo!()
    }

    fn proven_client_consensus(
        &self,
        client_id: &ClientId,
        consensus_height: ICSHeight,
        height: ICSHeight,
    ) -> Result<(Self::ConsensusState, MerkleProof), Error> {
        tracing::info!("in proven client consensus");

        todo!()
    }

    fn proven_channel(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        height: ICSHeight,
    ) -> Result<(ChannelEnd, MerkleProof), Error> {
        tracing::info!("in proven channel");

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
        tracing::info!("in proven packet");

        todo!()
    }

    fn build_client_state(&self, height: ICSHeight) -> Result<Self::ClientState, Error> {
        tracing::info!("in build client state");

        let chain_id = ChainId::new("ibc".to_string(), 2);
        tracing::info!("chain_id = {:?}", chain_id);

        let frozen_height = Height::new(0, 0);
        tracing::info!("frozen_height = {:?}", frozen_height);

        use ibc::ics02_client::client_state::AnyClientState;
        use ibc::ics10_grandpa::client_state::ClientState as GRANDPAClientState;

        // Create mock grandpa client state
        let client_state = GRANDPAClientState::new(chain_id, height, frozen_height).unwrap();

        tracing::info!("client_state: {:?}", client_state);

        Ok(client_state)
    }

    fn build_consensus_state(
        &self,
        light_block: Self::LightBlock,
    ) -> Result<Self::ConsensusState, Error> {
        tracing::info!("in build consensus state");

        // Create mock grandpa consensus state
        use ibc::ics10_grandpa::consensus_state::ConsensusState as GRANDPAConsensusState;

        let consensus_state = GRANDPAConsensusState::new();

        Ok(consensus_state)
    }

    fn build_header(
        &self,
        trusted_height: ICSHeight,
        target_height: ICSHeight,
        client_state: &AnyClientState,
        light_client: &mut dyn LightClient<Self>,
    ) -> Result<(Self::Header, Vec<Self::Header>), Error> {
        tracing::info!("in build header");

        Ok((GPHeader::new(), vec![GPHeader::new()]))
    }
}

fn encode_to_bech32(address: &str, account_prefix: &str) -> Result<String, Error> {
    let account = AccountId::from_str(address)
        .map_err(|e| Error::invalid_key_address(address.to_string(), e))?;

    let encoded = bech32::encode(account_prefix, account.to_base32(), Variant::Bech32)
        .map_err(Error::bech32_encoding)?;

    Ok(encoded)
}
