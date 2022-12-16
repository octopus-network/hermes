use near_account_id::AccountId;
use serde_json::json;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd};
use ibc_relayer_types::core::ics04_channel::channel::{ChannelEnd, IdentifiedChannelEnd};
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId};
use crate::chain::near::rpc::client::NearRpcClient;
use crate::client_state::{AnyClientState, IdentifiedAnyClientState};
use crate::consensus_state::AnyConsensusState;
use anyhow::Result;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::channel::v1::PacketState;
use near_crypto::{InMemorySigner, KeyType};
use near_primitives::views::FinalExecutionOutcomeView;
use tracing::info;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentPrefix;
use crate::chain::requests::{QueryClientStatesRequest, QueryConnectionsRequest, QueryPacketCommitmentsRequest, QueryUnreceivedAcksRequest};
use alloc::sync::Arc;
use tokio::runtime::Runtime as TokioRuntime;
use crate::chain::requests::QueryChannelsRequest;



pub trait NearIbcContract {
    fn get_contract_id(&self) -> AccountId;
    fn get_client(&self) -> &NearRpcClient;
    fn get_rt(&self) -> &Arc<TokioRuntime>;

    fn get_connection_end(
        &self,
        connection_identifier: &ConnectionId,
    ) -> Result<ConnectionEnd> {
        tracing::trace!("in near: [query_connection_end]");

        let connection_id = serde_json::to_string(connection_identifier).unwrap();

        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "query_connection_end".to_string(),
            json!({"connection_id": connection_id}).to_string().into_bytes()
        )).expect("Failed to query_connection_end").json()

        // self.get_client().view(
        //     self.get_contract_id().clone(),
        //     "query_connection_end".to_string(),
        //     json!({"connection_id": connection_id}).to_string().into_bytes()
        // ).await.expect("Failed to query_connection_end").json()
    }

    /// get channelEnd  by port_identifier, and channel_identifier
    fn get_channel_end(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<ChannelEnd> {
        tracing::trace!("in near: [query_channel_end]");

        let port_id = serde_json::to_string(port_id).unwrap();
        let channel_id = serde_json::to_string(channel_id).unwrap();

        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "query_channel_end".to_string(),
            json!({"port_id": port_id, "channel_id": channel_id}).to_string().into_bytes()
        )).expect("Failed to query_channel_end.").json()
    }

    // TODO(bob) need add query height
    /// get client_state by client_id
    fn get_client_state(
        &self,
        client_id: &ClientId,
    ) -> Result<AnyClientState> {
        tracing::trace!("in near: [query_client_state]");
        let client_id = serde_json::to_string(client_id).unwrap();

        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "query_client_state".to_string(),
            json!({"client_id": client_id}).to_string().into_bytes()
        )).expect("Failed to query_client_state.").json()
    }

    /// Performs a query to retrieve the consensus state for a specified height
    /// `consensus_height` that the specified light client stores.
    fn get_client_consensus(
        &self,
        client_id: &ClientId,
        consensus_height: &Height,
    ) -> Result<AnyConsensusState> {
        tracing::trace!("in near: [query_client_consensus]");
        let client_id = serde_json::to_string(client_id).unwrap();
        let consensus_height = serde_json::to_string(consensus_height).unwrap();

        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "query_client_consensus".to_string(),
            json!({"client_id": client_id, "consensus_height": consensus_height}).to_string().into_bytes()
        )).expect("Failed to query_client_consensus.").json()
    }

    fn get_consensus_state_with_height(
        &self,
        client_id: &ClientId,
    ) -> Result<Vec<(Height, AnyConsensusState)>> {
        tracing::trace!("in near: [get_consensus_state_with_height]");
        let client_id = serde_json::to_string(client_id).unwrap();

        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "get_consensus_state_with_height".to_string(),
            json!({"client_id": client_id}).to_string().into_bytes()
        )).expect("Failed to get_consensus_state_with_height.").json()
    }

    fn get_unreceipt_packet(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequences: &[Sequence],
    ) -> Result<Vec<u64>> {
        tracing::trace!("in near: [get_unreceipt_packet]");
        let port_id = serde_json::to_string(port_id).unwrap();
        let channel_id = serde_json::to_string(channel_id).unwrap();
        let sequences = serde_json::to_string(sequences).unwrap();

        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "get_unreceipt_packet".to_string(),
            json!({
                "port_id": port_id,
                "channel_id": channel_id,
                "sequences": sequences
            }).to_string().into_bytes()
        )).expect("Failed to get_unreceipt_packet.").json()
    }

    fn get_clients(&self, request: QueryClientStatesRequest) -> Result<Vec<IdentifiedAnyClientState>> {
        tracing::trace!("in near: [get_clients]");

        let request = serde_json::to_string(&request).unwrap();

        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "get_clients".to_string(),
            json!({"request": request}).to_string().into_bytes()
        )).expect("Failed to get_clients.").json()
    }

    fn get_connections(&self, request: QueryConnectionsRequest) -> Result<Vec<IdentifiedConnectionEnd>> {
        tracing::trace!("in near: [get_connections]");

        let request = serde_json::to_string(&request).unwrap();

        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "get_connections".to_string(),
            json!({"request": request}).to_string().into_bytes()
        )).expect("Failed to get_connections.").json()
    }

    fn get_channels(&self, request: QueryChannelsRequest) -> Result<Vec<IdentifiedChannelEnd>> {
        tracing::trace!("in near: [get_channels]");

        let request = serde_json::to_string(&request).unwrap();

        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "get_channels".to_string(),
            json!({"request": request}).to_string().into_bytes()
        )).expect("Failed to get_channels.").json()
    }

    fn get_commitment_packet_state(&self, request: QueryPacketCommitmentsRequest) -> Result<Vec<PacketState>> {
        tracing::trace!("in near: [get_commitment_packet_state]");
        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "get_commitment_packet_state".to_string(),
            json!({}).to_string().into_bytes()
        )).expect("Failed to get_commitment_packet_state.").json()
    }

    fn get_acknowledge_packet_state(&self) -> Result<Vec<PacketState>> {
        tracing::trace!("in near: [get_acknowledge_packet_state]");
        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "get_acknowledge_packet_state".to_string(),
            json!({}).to_string().into_bytes()
        )).expect("Failed to get_acknowledge_packet_state.").json()
    }

    /// get connection_identifier vector by client_identifier
    fn get_client_connections(
        &self,
        client_id: &ClientId,
    ) -> Result<Vec<ConnectionId>> {
        tracing::trace!("in near: [get_client_connections]");
        let client_id = serde_json::to_string(client_id).unwrap();
        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "get_client_connections".to_string(),
            json!({
                "client_id": client_id
            }).to_string().into_bytes()
        )).expect("Failed to get_client_connections.").json()
    }

    fn get_connection_channels(
        &self,
        connection_id: &ConnectionId,
    ) -> Result<Vec<IdentifiedChannelEnd>> {
        tracing::trace!("in near: [get_connection_channels]");
        let connection_id = serde_json::to_string(connection_id).unwrap();
        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "get_connection_channels".to_string(),
            json!({
                "connection_id": connection_id
            }).to_string().into_bytes()
        )).expect("Failed to get_connection_channels.").json()
    }

    /// The function to submit IBC request to a Near chain
    /// This function handles most of the IBC reqeusts to Near, except the MMR root update
    fn deliever(&self, msgs: Vec<Any>) -> Result<FinalExecutionOutcomeView> {
        tracing::trace!("in near: [deliever]");
        let msg = serde_json::to_string(&msgs).unwrap();

        let signer = InMemorySigner::from_random("bob.testnet".parse().unwrap(), KeyType::ED25519);
        self.get_rt().block_on(self.get_client().call(
            &signer,
            &self.get_contract_id(),
            "deliever".to_string(),
            json!({"messages": msg}).to_string().into_bytes(), 300000000000000, 1
        ))
    }

    fn raw_transfer(&self, msgs: Vec<Any>) -> Result<FinalExecutionOutcomeView> {
        tracing::trace!("in near: [raw_transfer]");
        let msg = serde_json::to_string(&msgs).unwrap();

        let signer = InMemorySigner::from_random("bob.testnet".parse().unwrap(), KeyType::ED25519);
        self.get_rt().block_on(self.get_client().call(
            &signer,
            &self.get_contract_id(),
            "raw_transfer".to_string(),
            json!({"messages": msg}).to_string().into_bytes(), 300000000000000,1
        ))
    }

    fn get_packet_commitment(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: &Sequence,
    ) -> Result<Vec<u8> > {
        tracing::trace!("in near: [get_packet_commitment]");
        let port_id = serde_json::to_string(port_id).unwrap();
        let channel_id = serde_json::to_string(channel_id).unwrap();
        let sequence = serde_json::to_string(sequence).unwrap();
        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "get_packet_commitment".to_string(),
            json!({
                "port_id": port_id,
                "channel_id": channel_id,
               "sequence": sequence
            }).to_string().into_bytes()
        )).expect("Failed to get_packet_commitment.").json()
    }

    fn get_commitment_prefix(&self) -> Result<CommitmentPrefix> {
        tracing::trace!("in near: [get_commitment_prefix]");
        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "get_commitment_prefix".to_string(),
            json!({}).to_string().into_bytes()
        )).expect("Failed to get_commitment_prefix.").json()
    }

    fn get_packet_receipt(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: &Sequence,
    ) -> Result<Vec<u8> > {
        info!("in near: [query_packet_receipt]");
        let port_id = serde_json::to_string(port_id).unwrap();
        let channel_id = serde_json::to_string(channel_id).unwrap();
        let sequence = serde_json::to_string(sequence).unwrap();
        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "query_packet_receipt".to_string(),
            json!({
                "port_id": port_id,
                "channel_id": channel_id,
               "sequence": sequence
            }).to_string().into_bytes()
        )).expect("Failed to query_packet_receipt.").json()
    }

    fn get_next_sequence_receive(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Result<Sequence > {
        info!("in near: [get_next_sequence_receive]");
        let port_id = serde_json::to_string(port_id).unwrap();
        let channel_id = serde_json::to_string(channel_id).unwrap();
        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "get_next_sequence_receive".to_string(),
            json!({
                "port_id": port_id,
                "channel_id": channel_id,
            }).to_string().into_bytes()
        )).expect("Failed to get_next_sequence_receive.").json()
    }

    fn get_packet_acknowledgement(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: &Sequence,
    ) -> Result<Vec<u8> > {
        info!("in near: [get_packet_acknowledgement]");
        let port_id = serde_json::to_string(port_id).unwrap();
        let channel_id = serde_json::to_string(channel_id).unwrap();
        let sequence = serde_json::to_string(sequence).unwrap();
        self.get_rt().block_on(self.get_client().view(
            self.get_contract_id().clone(),
            "get_packet_acknowledgement".to_string(),
            json!({
                "port_id": port_id,
                "channel_id": channel_id,
                "sequence": sequence
            }).to_string().into_bytes()
        )).expect("Failed to get_packet_acknowledgement.").json()
    }
}


#[derive(Debug)]
pub struct NearIbcContractInteractor<'s> {
    pub account_id: AccountId,
    pub client: &'s NearRpcClient,
}

impl<'s> NearIbcContractInteractor<'s>  {

    pub async fn query_connection_end(
        &self,
        connection_id: ConnectionId
    )->Result<ConnectionEnd>{
        let connection_id =  serde_json::to_string(&connection_id).unwrap();
        self.client.view(
            self.account_id.clone(),
            "query_connection_end".to_string(),
                json!({"connection_id": connection_id }).to_string().into_bytes()
        ).await.and_then(|e|e.json())
    }


    pub async fn query_channel_end(&self, port_id: PortId, channel_id: ChannelId)-> Result<ChannelEnd>{
        let port_id = serde_json::to_string(&port_id).unwrap();
        let channel_id = serde_json::to_string(&channel_id).unwrap();
        self.client.view(
            self.account_id.clone(),
            "query_channel_end".to_string(),
            json!({"port_id": port_id,"channel_id": channel_id}).to_string().into_bytes()
        ).await.and_then(|e|e.json())
    }

    pub async fn query_client_state(&self, client_id: ClientId)-> Result<AnyClientState>{
        let client_id = serde_json::to_string(&client_id).unwrap();
        self.client.view(
            self.account_id.clone(),
            "query_client_state".to_string(),
            json!({"client_id": client_id}).to_string().into_bytes()
        ).await.and_then(|e|e.json())
    }

    pub async fn query_client_consensus(&self, client_id: ClientId, consensus_height: Height)->Result<AnyConsensusState>{
        let client_id = serde_json::to_string(&client_id).unwrap();
        self.client.view(
            self.account_id.clone(),
            "query_client_state".to_string(),
            json!({"client_id": client_id}).to_string().into_bytes()
        ).await.and_then(|e|e.json())
    }

}
