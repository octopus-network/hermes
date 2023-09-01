use crate::chain::near::error::NearError;
use crate::chain::near::rpc::client::NearRpcClient;
use crate::chain::requests::{
    QueryChannelsRequest, QueryPacketAcknowledgementsRequest, QueryPacketEventDataRequest,
};
use crate::chain::requests::{
    QueryClientConnectionsRequest, QueryClientStatesRequest, QueryConnectionsRequest,
    QueryPacketCommitmentsRequest,
};
use crate::consensus_state::AnyConsensusState;
use alloc::sync::Arc;
use ibc::core::events::IbcEvent;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics03_connection::connection::{
    ConnectionEnd, IdentifiedConnectionEnd,
};
use ibc_relayer_types::core::ics04_channel::channel::{ChannelEnd, IdentifiedChannelEnd};
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId};
use near_primitives::types::AccountId;
use serde_json::json;
use tokio::runtime::Runtime as TokioRuntime;
use tracing::trace;

pub trait NearIbcContract {
    fn get_contract_id(&self) -> AccountId;
    fn get_client(&self) -> &NearRpcClient;
    fn get_rt(&self) -> &Arc<TokioRuntime>;

    fn get_latest_height(&self) -> anyhow::Result<Height> {
        trace!("NearIbcContract: [get_latest_height]");

        let height: Height = self
            .get_rt()
            .block_on(self.get_client().view(
                self.get_contract_id(),
                "get_latest_height".into(),
                json!({}).to_string().into_bytes(),
            ))?
            .json()?;

        // As we use solomachine client, we set the revision number to 0
        // TODO: ibc height reversion number is chainid version
        Ok(Height::new(0, height.revision_height())?)
    }

    fn get_connection_end(
        &self,
        connection_identifier: &ConnectionId,
    ) -> anyhow::Result<ConnectionEnd> {
        trace!(
            "NearIbcContract: [get_connection_end] - connection_identifier: {:?}",
            connection_identifier
        );

        let connection_id = connection_identifier.to_string();
        // thread::sleep(Duration::from_secs(5));

        self.get_rt()
            .block_on(
                self.get_client().view(
                    self.get_contract_id(),
                    "get_connection_end".into(),
                    json!({ "connection_id": connection_id })
                        .to_string()
                        .into_bytes(),
                ),
            )?
            .json()

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
    ) -> anyhow::Result<ChannelEnd> {
        trace!(
            "NearIbcContract: [query_channel_end] - port_id: {:?} channel_id: {:?}",
            port_id,
            channel_id
        );

        self.get_rt()
            .block_on(
                self.get_client().view(
                    self.get_contract_id(),
                    "get_channel_end".into(),
                    json!({"port_id": port_id, "channel_id": channel_id})
                        .to_string()
                        .into_bytes(),
                ),
            )?
            .json()
    }

    /// get client_state by client_id
    fn get_client_state(&self, client_id: &ClientId) -> anyhow::Result<Vec<u8>> {
        trace!(
            "NearIbcContract: [get_client_state] - client_id: {:?}",
            client_id
        );

        self.get_rt()
            .block_on(
                self.get_client().view(
                    self.get_contract_id(),
                    "get_client_state".into(),
                    json!({"client_id": client_id.clone()})
                        .to_string()
                        .into_bytes(),
                ),
            )?
            .json()
    }

    fn get_client_consensus_heights(&self, client_id: &ClientId) -> anyhow::Result<Vec<Height>> {
        trace!(
            "NearIbcContract: [get_client_consensus_heights] - client_id: {:?}",
            client_id,
        );
        self.get_rt()
            .block_on(
                self.get_client().view(
                    self.get_contract_id(),
                    "get_client_consensus_heights".into(),
                    json!({"client_id": client_id.clone()})
                        .to_string()
                        .into_bytes(),
                ),
            )?
            .json()
    }

    /// Performs a query to retrieve the consensus state for a specified height
    /// `consensus_height` that the specified light client stores.
    fn get_client_consensus(
        &self,
        client_id: &ClientId,
        consensus_height: &Height,
    ) -> anyhow::Result<Vec<u8>> {
        trace!(
            "NearIbcContract: [get_client_consensus] - client_id: {:?} consensus_height: {:?}",
            client_id,
            consensus_height
        );
        self.get_rt()
            .block_on(
                self.get_client().view(
                    self.get_contract_id(),
                    "get_client_consensus".into(),
                    json!({"client_id": client_id.clone(), "consensus_height": consensus_height })
                        .to_string()
                        .into_bytes(),
                ),
            )?
            .json()
    }

    fn get_consensus_state_with_height(
        &self,
        client_id: &ClientId,
    ) -> anyhow::Result<Vec<(Height, AnyConsensusState)>> {
        trace!(
            "NearIbcContract: [get_consensus_state_with_height] - client_id: {:?}",
            client_id
        );
        let client_id = serde_json::to_string(client_id).map_err(NearError::SerdeJsonError)?;

        self.get_rt()
            .block_on(self.get_client().view(
                self.get_contract_id(),
                "get_consensus_state_with_height".into(),
                json!({ "client_id": client_id }).to_string().into_bytes(),
            ))?
            .json()
    }

    fn get_unreceipt_packet(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequences: &[Sequence],
    ) -> anyhow::Result<Vec<u64>> {
        trace!(
            "NearIbcContract: [get_unreceipt_packet] - port_id: {:?} channel_id: {:?} sequences: {:?}",
            port_id, channel_id, sequences
        );

        self.get_rt()
            .block_on(
                self.get_client().view(
                    self.get_contract_id(),
                    "get_unreceipt_packet".into(),
                    json!({
                        "port_id": port_id,
                        "channel_id": channel_id,
                        "sequences": sequences
                    })
                    .to_string()
                    .into_bytes(),
                ),
            )?
            .json()
    }

    fn get_clients(
        &self,
        request: QueryClientStatesRequest,
    ) -> anyhow::Result<Vec<(ClientId, Vec<u8>)>> {
        trace!("NearIbcContract: [get_clients] - request: {:?}", request);

        let request = serde_json::to_string(&request).map_err(NearError::SerdeJsonError)?;

        self.get_rt()
            .block_on(self.get_client().view(
                self.get_contract_id(),
                "get_clients".into(),
                json!({ "request": request }).to_string().into_bytes(),
            ))?
            .json()
    }

    fn get_connections(
        &self,
        request: QueryConnectionsRequest,
    ) -> anyhow::Result<Vec<IdentifiedConnectionEnd>> {
        trace!(
            "NearIbcContract: [get_connections] - request: {:?}",
            request
        );

        let request = serde_json::to_string(&request).map_err(NearError::SerdeJsonError)?;

        self.get_rt()
            .block_on(self.get_client().view(
                self.get_contract_id(),
                "get_connections".into(),
                json!({ "request": request }).to_string().into_bytes(),
            ))?
            .json()
    }

    fn get_channels(
        &self,
        request: QueryChannelsRequest,
    ) -> anyhow::Result<Vec<IdentifiedChannelEnd>> {
        trace!("NearIbcContract: [get_channels] - request: {:?}", request);

        let request = serde_json::to_string(&request).map_err(NearError::SerdeJsonError)?;

        self.get_rt()
            .block_on(self.get_client().view(
                self.get_contract_id(),
                "get_channels".into(),
                json!({ "request": request }).to_string().into_bytes(),
            ))?
            .json()
    }

    fn get_packet_commitments(
        &self,
        request: QueryPacketCommitmentsRequest,
    ) -> anyhow::Result<Vec<Sequence>> {
        trace!(
            "NearIbcContract: [get_packet_commitments] - request: {:?}",
            request
        );
        self.get_rt()
            .block_on(
                self.get_client().view(
                    self.get_contract_id(),
                    "get_packet_commitment_sequences".into(),
                    json!({
                        "port_id": request.port_id.to_string(),
                        "channel_id": request.channel_id.to_string()
                    })
                    .to_string()
                    .into_bytes(),
                ),
            )?
            .json()
    }

    fn get_packet_acknowledgements(
        &self,
        request: QueryPacketAcknowledgementsRequest,
    ) -> anyhow::Result<Vec<Sequence>> {
        trace!("NearIbcContract: [get_packet_acknowledgements]");
        self.get_rt()
            .block_on(
                self.get_client().view(
                    self.get_contract_id(),
                    "get_packet_acknowledgement_sequences".into(),
                    json!({
                        "port_id": request.port_id.to_string(),
                        "channel_id": request.channel_id.to_string()
                    })
                    .to_string()
                    .into_bytes(),
                ),
            )?
            .json()
    }

    /// get connection_identifier vector by client_identifier
    fn get_client_connections(
        &self,
        request: &QueryClientConnectionsRequest,
    ) -> anyhow::Result<Vec<ConnectionId>> {
        let client_id = request.client_id.to_string();
        trace!(
            "NearIbcContract: [get_client_connections] - client_id: {:?}",
            client_id
        );
        self.get_rt()
            .block_on(self.get_client().view(
                self.get_contract_id(),
                "get_client_connections".into(),
                json!({ "client_id": client_id }).to_string().into_bytes(),
            ))?
            .json()
    }

    fn get_connection_channels(
        &self,
        connection_id: &ConnectionId,
    ) -> anyhow::Result<Vec<IdentifiedChannelEnd>> {
        trace!(
            "NearIbcContract: [get_connection_channels] - connection_id: {:?}",
            connection_id
        );
        let connection_id = connection_id.to_string();
        self.get_rt()
            .block_on(
                self.get_client().view(
                    self.get_contract_id(),
                    "get_connection_channels".into(),
                    json!({ "connection_id": connection_id })
                        .to_string()
                        .into_bytes(),
                ),
            )?
            .json()
    }

    fn get_packet_commitment(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: &Sequence,
    ) -> anyhow::Result<Vec<u8>> {
        trace!(
            "NearIbcContract: [get_packet_commitment] - port_id: {:?}, channel_id: {:?}, sequence: {:?}",
            port_id, channel_id, sequence
        );

        self.get_rt()
            .block_on(
                self.get_client().view(
                    self.get_contract_id(),
                    "get_packet_commitment".into(),
                    json!({
                        "port_id": port_id,
                        "channel_id": channel_id,
                       "sequence": sequence
                    })
                    .to_string()
                    .into_bytes(),
                ),
            )?
            .json()
    }

    fn get_commitment_prefix(&self) -> anyhow::Result<Vec<u8>> {
        trace!("NearIbcContract: [get_commitment_prefix]");
        self.get_rt()
            .block_on(self.get_client().view(
                self.get_contract_id(),
                "get_commitment_prefix".into(),
                json!({}).to_string().into_bytes(),
            ))?
            .json()
    }

    fn get_packet_receipt(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: &Sequence,
    ) -> anyhow::Result<Vec<u8>> {
        trace!(
            "NearIbcContract: [get_packet_receipt] - port_id: {:?}, channel_id: {:?}, sequence: {:?}",
            port_id, channel_id, sequence
        );

        self.get_rt()
            .block_on(
                self.get_client().view(
                    self.get_contract_id(),
                    "get_packet_receipt".into(),
                    json!({
                        "port_id": port_id,
                        "channel_id": channel_id,
                        "sequence": sequence
                    })
                    .to_string()
                    .into_bytes(),
                ),
            )?
            .json()
    }

    fn get_next_sequence_receive(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> anyhow::Result<Sequence> {
        trace!(
            "NearIbcContract: [get_next_sequence_receive] - port_id: {:?}, channel_id: {:?}",
            port_id,
            channel_id,
        );

        self.get_rt()
            .block_on(
                self.get_client().view(
                    self.get_contract_id(),
                    "get_next_sequence_receive".into(),
                    json!({
                        "port_id": port_id,
                        "channel_id": channel_id,
                    })
                    .to_string()
                    .into_bytes(),
                ),
            )?
            .json()
    }

    fn get_packet_acknowledgement(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: &Sequence,
    ) -> anyhow::Result<Vec<u8>> {
        trace!(
            "NearIbcContract: [get_packet_acknowledgement] - port_id: {:?}, channel_id: {:?}, sequence: {:?}",
            port_id, channel_id, sequence
        );

        self.get_rt()
            .block_on(
                self.get_client().view(
                    self.get_contract_id(),
                    "get_packet_acknowledgement".into(),
                    json!({
                        "port_id": port_id,
                        "channel_id": channel_id,
                        "sequence": sequence
                    })
                    .to_string()
                    .into_bytes(),
                ),
            )?
            .json()
    }

    fn get_packet_events(
        &self,
        request: QueryPacketEventDataRequest,
    ) -> anyhow::Result<Vec<(Height, Vec<IbcEvent>)>> {
        trace!(
            "NearIbcContract: [get_packet_events] - request: {:?}",
            request
        );

        self.get_rt()
            .block_on(self.get_client().view(
                self.get_contract_id(),
                "get_packet_events".into(),
                json!({ "request": request }).to_string().into_bytes(),
            ))?
            .json::<Vec<(Height, Vec<IbcEvent>)>>()
    }
}

#[tokio::test]
pub async fn test123() -> anyhow::Result<()> {
    const CONTRACT_ACCOUNT_ID: &str = "v3.nearibc.testnet";
    let client =
        NearRpcClient::new("https://near-testnet.infura.io/v3/4f80a04e6eb2437a9ed20cb874e10d55");
    let connection_id = ConnectionId::new(5);
    let connection_id = serde_json::to_string(&connection_id).unwrap();
    dbg!(&connection_id);

    dbg!(&connection_id.eq("connection-5"));
    let result = client
        .view(
            CONTRACT_ACCOUNT_ID.parse()?,
            "get_connection_end".to_string(),
            json!({"connection_id": "connection-5".to_string()})
                .to_string()
                .into_bytes(),
        )
        .await
        .map_err(|e| NearError::CustomError(e.to_string()))?;

    // dbg!(&result);

    let result_raw: serde_json::value::Value = result.json().unwrap();
    dbg!(&result_raw);
    let connection_end: ConnectionEnd =
        serde_json::from_value(result_raw).map_err(NearError::SerdeJsonError)?;
    dbg!(&connection_end);
    Ok(())
}
