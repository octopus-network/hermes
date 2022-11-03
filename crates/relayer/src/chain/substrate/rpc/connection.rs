use super::super::config::{ibc_node, MyConfig};
use super::channel::query_channel_end;
use anyhow::Result;
use codec::Decode;
use core::str::FromStr;
use ibc_proto::protobuf::Protobuf;
use ibc_relayer_types::core::ics24_host::path::{ChannelEndsPath, ConnectionsPath};
use ibc_relayer_types::core::ics24_host::Path;
use ibc_relayer_types::core::{
    ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd},
    ics04_channel::channel::IdentifiedChannelEnd,
    ics24_host::identifier::ConnectionId,
};
use sp_core::H256;
use subxt::OnlineClient;

/// get connectionEnd according by connection_identifier and read Connections StorageMaps
pub async fn query_connection_end(
    connection_identifier: &ConnectionId,
    client: OnlineClient<MyConfig>,
) -> Result<ConnectionEnd, subxt::error::Error> {
    tracing::info!("in call_ibc: [get_connection_end]");

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let connections_path = ConnectionsPath(connection_identifier.clone())
        .to_string()
        .as_bytes()
        .to_vec();

    // Address to a storage entry we'd like to access.
    let address = ibc_node::storage().ibc().connections(connections_path);

    let data: Option<Vec<u8>> = client.storage().fetch(&address, Some(block_hash)).await?;

    if let Some(data) = data {
        let connection_end = ConnectionEnd::decode_vec(&*data).unwrap();

        Ok(connection_end)
    } else {
        Err(subxt::error::Error::Other(format!(
            "get_connection_end is empty! by connection_identifier = ({})",
            connection_identifier
        )))
    }
}

/// get key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
pub async fn get_connections(
    client: OnlineClient<MyConfig>,
) -> Result<Vec<IdentifiedConnectionEnd>, subxt::error::Error> {
    tracing::info!("in call_ibc: [get_connections]");

    let mut result = vec![];

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let address = ibc_node::storage().ibc().connections_root();

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
            Path::Connections(connections_path) => {
                let ConnectionsPath(connection_id) = connections_path;
                // store key-value
                let connection_end = ConnectionEnd::decode_vec(&*value).unwrap();

                result.push(IdentifiedConnectionEnd::new(connection_id, connection_end));
            }
            _ => unimplemented!(),
        }
    }

    Ok(result)
}

pub async fn get_connection_channels(
    connection_id: &ConnectionId,
    client: OnlineClient<MyConfig>,
) -> Result<Vec<IdentifiedChannelEnd>, subxt::error::Error> {
    tracing::info!("in call_ibc: [get_connection_channels]");

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let connections_path = ConnectionsPath(connection_id.clone())
        .to_string()
        .as_bytes()
        .to_vec();

    // Address to a storage entry we'd like to access.
    let address = ibc_node::storage()
        .ibc()
        .channels_connection(connections_path);

    // ConnectionsPath(connection_id) <-> Vec<ChannelEndsPath(port_id, channel_id)>
    let connections_paths: Option<Vec<Vec<u8>>> =
        client.storage().fetch(&address, Some(block_hash)).await?;

    if connections_paths.is_none() {
        return Err(subxt::error::Error::Other(format!(
            "get_connection_channels is empty! by connection_id = ({})",
            connection_id
        )));
    }

    let mut result = vec![];

    for connections_path in connections_paths.unwrap().into_iter() {
        let raw_path = String::from_utf8(connections_path)
            .map_err(|e| subxt::error::Error::Other(format!("{}", e.to_string())))?;
        // decode key
        let path = Path::from_str(&raw_path)
            .map_err(|_| subxt::error::Error::Other(format!("decode path error")))?;

        match path {
            Path::ChannelEnds(channel_ends_path) => {
                let ChannelEndsPath(port_id, channel_id) = channel_ends_path;

                // get channel_end
                let channel_end = query_channel_end(&port_id, &channel_id, client.clone()).await?;

                result.push(IdentifiedChannelEnd::new(port_id, channel_id, channel_end));
            }
            _ => unimplemented!(),
        }
    }

    Ok(result)
}
