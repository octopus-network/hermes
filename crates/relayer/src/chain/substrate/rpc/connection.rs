use super::channel::query_channel_end;
use super::super::config::{MyConfig, ibc_node};
use crate::chain::substrate::rpc::storage_iter;
use anyhow::Result;
use core::str::FromStr;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::core::ics24_host::path::{ChannelEndsPath, ConnectionsPath};
use ibc_relayer_types::core::ics24_host::Path;
use ibc_relayer_types::core::{
    ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd},
    ics04_channel::channel::IdentifiedChannelEnd,
    ics24_host::identifier::ConnectionId,
};
use sp_core::H256;
use subxt::OnlineClient;
use ibc_proto::protobuf::Protobuf;

/// get connectionEnd according by connection_identifier and read Connections StorageMaps
pub async fn query_connection_end(
        connection_identifier: &ConnectionId,
        client: OnlineClient<MyConfig>,
        ) -> Result<ConnectionEnd> {
    tracing::info!("in call_ibc: [get_connection_end]");

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let connections_path = ConnectionsPath(connection_identifier.clone())
    .to_string()
    .as_bytes()
    .to_vec();

    let data: Vec<u8> = client
    .storage()
    .ibc()
    .connections(&connections_path, Some(block_hash))
    .await?;

    if data.is_empty() {
        return Err(anyhow::anyhow!(
                "get_connection_end is empty! by connection_identifier = ({})",
        connection_identifier
        ));
    }

    let connection_end = ConnectionEnd::decode_vec(&*data).unwrap();

    Ok(connection_end)
}

/// get key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
pub async fn get_connections(client: OnlineClient<MyConfig>) -> Result<Vec<IdentifiedConnectionEnd>> {
    tracing::info!("in call_ibc: [get_connections]");

    let callback = Box::new(
            |path: Path,
            result: &mut Vec<IdentifiedConnectionEnd>,
            value: Vec<u8>,
            _client_id: ClientId| {
                match path {
                    Path::Connections(connections_path) => {
                        let ConnectionsPath(connection_id) = connections_path;
                        // store key-value
                        let connection_end = ConnectionEnd::decode_vec(&*value).unwrap();

                        result.push(IdentifiedConnectionEnd::new(connection_id, connection_end));
                    }
                    _ => unimplemented!(),
                }
            },
    );

    let mut result = vec![];

    let _ret = storage_iter::<IdentifiedConnectionEnd, ibc_node::ibc::storage::Connections>(
            client.clone(),
    &mut result,
    ClientId::default(),
    callback,
    )
    .await?;

    Ok(result)
}

pub async fn get_connection_channels(
        connection_id: &ConnectionId,
        client: OnlineClient<MyConfig>,
        ) -> Result<Vec<IdentifiedChannelEnd>> {
    tracing::info!("in call_ibc: [get_connection_channels]");

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let connections_path = ConnectionsPath(connection_id.clone())
    .to_string()
    .as_bytes()
    .to_vec();

    // ConnectionsPath(connection_id) <-> Vec<ChannelEndsPath(port_id, channel_id)>
    let connections_paths: Vec<Vec<u8>> = client
    .storage()
    .ibc()
    .channels_connection(&connections_path, Some(block_hash))
    .await?;

    if connections_paths.is_empty() {
        return Err(anyhow::anyhow!(
                "get_connection_channels is empty! by connection_id = ({})",
        connection_id
        ));
    }

    let mut result = vec![];

    for connections_path in connections_paths.into_iter() {
        let raw_path = String::from_utf8(connections_path)?;
        // decode key
        let path = Path::from_str(&raw_path).map_err(|_| anyhow::anyhow!("decode path error"))?;

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