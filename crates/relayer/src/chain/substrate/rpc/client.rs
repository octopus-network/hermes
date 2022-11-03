use super::super::config::{ibc_node, MyConfig};
use crate::client_state::AnyClientState;
use crate::client_state::IdentifiedAnyClientState;
use crate::consensus_state::AnyConsensusState;
use anyhow::Result;
use codec::Decode;
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
) -> Result<AnyClientState, subxt::error::Error> {
    tracing::info!("in call_ibc : [get_client_state]");

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let client_state_path = ClientStatePath(client_id.clone())
        .to_string()
        .as_bytes()
        .to_vec();

    // Address to a storage entry we'd like to access.
    let address = ibc_node::storage().ibc().client_states(client_state_path);

    let value: Option<Vec<u8>> = client.storage().fetch(&address, Some(block_hash)).await?;

    if let Some(data) = value {
        let client_state = AnyClientState::decode_vec(&*data).unwrap();

        Ok(client_state)
    } else {
        Err(subxt::error::Error::Other(format!(
            "get_client_state is empty! by client_id = ({})",
            client_id
        )))
    }
}

/// get appoint height consensus_state according by client_identifier and height
/// and read ConsensusStates StorageMap
/// Performs a query to retrieve the consensus state for a specified height
/// `consensus_height` that the specified light client stores.
pub async fn query_client_consensus(
    client_id: &ClientId,
    consensus_height: &ICSHeight,
    client: OnlineClient<MyConfig>,
) -> Result<AnyConsensusState, subxt::error::Error> {
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

    // Address to a storage entry we'd like to access.
    let address = ibc_node::storage()
        .ibc()
        .consensus_states(client_consensus_state_path);

    let consensus_state: Option<Vec<u8>> =
        client.storage().fetch(&address, Some(block_hash)).await?;

    let consensus_state = if let Some(consensus_state) = consensus_state {
        AnyConsensusState::decode_vec(&*consensus_state).unwrap()
    } else {
        // TODO
        AnyConsensusState::Grandpa(
            ibc_relayer_types::clients::ics10_grandpa::consensus_state::ConsensusState::default(),
        )
    };

    Ok(consensus_state)
}

/// get consensus state with height
pub async fn get_consensus_state_with_height(
    client_id: &ClientId,
    client: OnlineClient<MyConfig>,
) -> Result<Vec<(ICSHeight, AnyConsensusState)>, subxt::error::Error> {
    tracing::info!("in call_ibc: [get_consensus_state_with_height]");

    let mut result = vec![];

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let address = ibc_node::storage().ibc().consensus_states_root();

    // Iterate over keys and values at that address.
    let mut iter = client.storage().iter(address, 10, None).await.unwrap();

    // prefix(32) + hash(data)(16) + data
    while let Some((key, value)) = iter.next().await? {
        let raw_key = key.0[48..].to_vec();
        let raw_key = Vec::<u8>::decode(&mut &*raw_key)
            .map_err(|_| subxt::error::Error::Other("decode vec<u8> error".to_string()))?;
        let client_state_path = String::from_utf8(raw_key)
            .map_err(|_| subxt::error::Error::Other("decode string error".to_string()))?;
        // decode key
        let path = Path::from_str(&client_state_path)
            .map_err(|_| subxt::error::Error::Other("decode path error".to_string()))?;

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
    }

    Ok(result)
}

/// get key-value pair (client_identifier, client_state) construct IdentifierAny Client state
pub async fn get_clients(
    client: OnlineClient<MyConfig>,
) -> Result<Vec<IdentifiedAnyClientState>, subxt::error::Error> {
    tracing::info!("in call_ibc: [get_clients]");

    let mut result = vec![];

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let address = ibc_node::storage().ibc().client_states_root();

    // Iterate over keys and values at that address.
    let mut iter = client.storage().iter(address, 10, None).await.unwrap();

    // prefix(32) + hash(data)(16) + data
    while let Some((key, value)) = iter.next().await? {
        let raw_key = key.0[48..].to_vec();
        let raw_key = Vec::<u8>::decode(&mut &*raw_key)
            .map_err(|_| subxt::error::Error::Other("decode vec<u8> error".to_string()))?;
        let client_state_path = String::from_utf8(raw_key)
            .map_err(|_| subxt::error::Error::Other("decode string error".to_string()))?;
        // decode key
        let path = Path::from_str(&client_state_path)
            .map_err(|_| subxt::error::Error::Other("decode path error".to_string()))?;

        match path {
            Path::ClientState(ClientStatePath(ibc_client_id)) => {
                let client_state = AnyClientState::decode_vec(&*value).unwrap();

                result.push(IdentifiedAnyClientState::new(ibc_client_id, client_state));
            }
            _ => unimplemented!(),
        }
    }

    Ok(result)
}

/// get connection_identifier vector according by client_identifier
pub async fn get_client_connections(
    client_id: &ClientId,
    client: OnlineClient<MyConfig>,
) -> Result<Vec<ConnectionId>, subxt::error::Error> {
    tracing::info!("in call_ibc: [get_client_connections]");

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let client_connection_paths = ClientConnectionsPath(client_id.clone())
        .to_string()
        .as_bytes()
        .to_vec();

    // Address to a storage entry we'd like to access.
    let address = ibc_node::storage()
        .ibc()
        .connection_client(client_connection_paths);

    let connection_id: Option<Vec<u8>> = client.storage().fetch(&address, Some(block_hash)).await?;

    if let Some(connection_id) = connection_id {
        let mut result = vec![];

        let connection_id_str = String::from_utf8(connection_id).unwrap();
        let connection_id = ConnectionId::from_str(connection_id_str.as_str()).unwrap();

        result.push(connection_id);

        Ok(result)
    } else {
        Err(subxt::error::Error::Other(format!(
            "get_client_connections is empty! by client_id = ({})",
            client_id
        )))
    }
}
