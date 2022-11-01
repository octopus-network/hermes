use super::super::config::{ibc_node, MyConfig};
use crate::chain::substrate::rpc::storage_iter;
use crate::client_state::AnyClientState;
use crate::client_state::IdentifiedAnyClientState;
use crate::consensus_state::AnyConsensusState;
use anyhow::Result;
use core::str::FromStr;
use ibc_proto::protobuf::Protobuf;
use ibc_relayer_types::core::ics24_host::path::{ClientConnectionsPath, ClientConsensusStatePath};
use ibc_relayer_types::core::ics24_host::Path;
use ibc_relayer_types::{
    core::{
        ics24_host::identifier::{ClientId, ConnectionId},
        ics24_host::path::ClientStatePath,
    },
    Height as ICSHeight,
};
use sp_core::H256;
use subxt::OnlineClient;

/// query client_state according by client_id, and read ClientStates StorageMap
pub async fn query_client_state(
    client_id: &ClientId,
    client: OnlineClient<MyConfig>,
) -> Result<AnyClientState> {
    tracing::info!("in call_ibc : [get_client_state]");

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let client_state_path = ClientStatePath(client_id.clone())
        .to_string()
        .as_bytes()
        .to_vec();

    let data: Vec<u8> = client
        .storage()
        .ibc()
        .client_states(&client_state_path, Some(block_hash))
        .await?;

    if data.is_empty() {
        return Err(anyhow::anyhow!(
            "get_client_state is empty! by client_id = ({})",
            client_id
        ));
    }

    let client_state = AnyClientState::decode_vec(&*data).unwrap();

    Ok(client_state)
}

/// get appoint height consensus_state according by client_identifier and height
/// and read ConsensusStates StorageMap
/// Performs a query to retrieve the consensus state for a specified height
/// `consensus_height` that the specified light client stores.
pub async fn query_client_consensus(
    client_id: &ClientId,
    consensus_height: &ICSHeight,
    client: OnlineClient<MyConfig>,
) -> Result<AnyConsensusState> {
    tracing::info!("in call_ibc: [get_client_consensus]");

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    // search key
    let client_consensus_state_path = ClientConsensusStatePath {
        client_id: client_id.clone(),
        epoch: consensus_height.revision_number(),
        height: consensus_height.revision_height(),
    }
    .to_string()
    .as_bytes()
    .to_vec();

    let consensus_state: Vec<u8> = client
        .storage()
        .ibc()
        .consensus_states(&client_consensus_state_path, Some(block_hash))
        .await?;

    tracing::info!(
        "query_client_consensus is empty! by client_id = ({}), consensus_height = ({})",
        client_id,
        consensus_height
    );

    let consensus_state = if consensus_state.is_empty() {
        // TODO
        AnyConsensusState::Grandpa(
            ibc_relayer_types::clients::ics10_grandpa::consensus_state::ConsensusState::default(),
        )
    } else {
        AnyConsensusState::decode_vec(&*consensus_state).unwrap()
    };

    Ok(consensus_state)
}

/// get consensus state with height
pub async fn get_consensus_state_with_height(
    client_id: &ClientId,
    client: OnlineClient<MyConfig>,
) -> Result<Vec<(ICSHeight, AnyConsensusState)>> {
    tracing::info!("in call_ibc: [get_consensus_state_with_height]");

    let callback = Box::new(
        |path: Path,
         result: &mut Vec<(ICSHeight, AnyConsensusState)>,
         value: Vec<u8>,
         client_id: ClientId| {
            match path {
                Path::ClientConsensusState(client_consensus_state) => {
                    let ClientConsensusStatePath {
                        client_id: read_client_id,
                        epoch,
                        height,
                    } = client_consensus_state;

                    if read_client_id == client_id.clone() {
                        let height = ICSHeight::new(epoch, height).unwrap();
                        let consensus_state = AnyConsensusState::decode_vec(&*value).unwrap();
                        // store key-value
                        result.push((height, consensus_state));
                    }
                }
                _ => unimplemented!(),
            }
        },
    );

    let mut result = vec![];

    let _ret = storage_iter::<
        (ICSHeight, AnyConsensusState),
        ibc_node::ibc::storage::ConsensusStates,
    >(client.clone(), &mut result, client_id.clone(), callback)
    .await?;

    Ok(result)
}

/// get key-value pair (client_identifier, client_state) construct IdentifierAny Client state
pub async fn get_clients(client: OnlineClient<MyConfig>) -> Result<Vec<IdentifiedAnyClientState>> {
    tracing::info!("in call_ibc: [get_clients]");

    let callback = Box::new(
        |path: Path,
         result: &mut Vec<IdentifiedAnyClientState>,
         value: Vec<u8>,
         _client_id: ClientId| {
            match path {
                Path::ClientState(ClientStatePath(ibc_client_id)) => {
                    let client_state = AnyClientState::decode_vec(&*value).unwrap();

                    result.push(IdentifiedAnyClientState::new(ibc_client_id, client_state));
                }
                _ => unimplemented!(),
            }
        },
    );

    let mut result = vec![];

    let _ret = storage_iter::<IdentifiedAnyClientState, ibc_node::ibc::storage::ClientStates>(
        client.clone(),
        &mut result,
        ClientId::default(),
        callback,
    )
    .await?;

    Ok(result)
}

/// get connection_identifier vector according by client_identifier
pub async fn get_client_connections(
    client_id: &ClientId,
    client: OnlineClient<MyConfig>,
) -> Result<Vec<ConnectionId>> {
    tracing::info!("in call_ibc: [get_client_connections]");

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let client_connection_paths = ClientConnectionsPath(client_id.clone())
        .to_string()
        .as_bytes()
        .to_vec();

    // client_id <-> connection_id
    let connection_id: Vec<u8> = client
        .storage()
        .ibc()
        .connection_client(&client_connection_paths, Some(block_hash))
        .await?;

    if connection_id.is_empty() {
        return Err(anyhow::anyhow!(
            "get_client_connections is empty! by client_id = ({})",
            client_id
        ));
    }

    let mut result = vec![];

    let connection_id_str = String::from_utf8(connection_id).unwrap();
    let connection_id = ConnectionId::from_str(connection_id_str.as_str()).unwrap();

    result.push(connection_id);

    Ok(result)
}
