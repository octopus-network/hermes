use super::super::config::{ibc_node, MyConfig};
use anyhow::Result;
use codec::Decode;
use ibc_proto::ibc::core::channel::v1::PacketState;
use ibc_proto::protobuf::Protobuf;
use ibc_relayer_types::core::ics24_host::path::{
    AcksPath, ChannelEndsPath, CommitmentsPath, ReceiptsPath, SeqRecvsPath,
};
use ibc_relayer_types::core::ics24_host::Path;
use ibc_relayer_types::core::{
    ics04_channel::{
        channel::{ChannelEnd, IdentifiedChannelEnd},
        packet::{Receipt, Sequence},
    },
    ics24_host::identifier::{ChannelId, PortId},
};
use sp_core::H256;
use std::str::FromStr;
use subxt::OnlineClient;

/// get key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
pub async fn get_channels(
    client: OnlineClient<MyConfig>,
) -> Result<Vec<IdentifiedChannelEnd>, subxt::error::Error> {
    tracing::info!("in call_ibc: [get_channels]");

    let mut result = vec![];

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let address = ibc_node::storage().ibc().channels_root();

    // Iterate over keys and values at that address.
    let mut iter = client.storage().iter(address, 10, Some(block_hash)).await.unwrap();

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
            Path::ChannelEnds(channel_ends_path) => {
                let ChannelEndsPath(port_id, channel_id) = channel_ends_path;
                let channel_end = ChannelEnd::decode_vec(&*value).unwrap();

                result.push(IdentifiedChannelEnd::new(port_id, channel_id, channel_end));
            }
            _ => unimplemented!(),
        }
    }

    Ok(result)
}

/// query channelEnd according by port_identifier, channel_identifier and read Channel StorageMaps
pub async fn query_channel_end(
    port_id: &PortId,
    channel_id: &ChannelId,
    client: OnlineClient<MyConfig>,
) -> Result<ChannelEnd, subxt::error::Error> {
    tracing::info!("in call_ibc: [get_channel_end]");

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let channel_end_path = ChannelEndsPath(port_id.clone(), channel_id.clone())
        .to_string()
        .as_bytes()
        .to_vec();

    // Address to a storage entry we'd like to access.
    let address = ibc_node::storage().ibc().channels(channel_end_path);

    let value: Option<Vec<u8>> = client.storage().fetch(&address, Some(block_hash)).await?;

    if let Some(data) = value {
        let channel_end = ChannelEnd::decode_vec(&*data).unwrap();

        Ok(channel_end)
    } else {
        Err(subxt::error::Error::Other(format!(
            "get_channel_end is empty by port_id = ({}), channel_id = ({})",
            port_id, channel_id
        )))
    }
}

/// get packet receipt by port_id, channel_id and sequence
pub async fn get_packet_receipt(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: &Sequence,
    client: OnlineClient<MyConfig>,
) -> Result<Receipt, subxt::error::Error> {
    tracing::info!("in call_ibc : [get_packet_receipt]");

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let packet_receipt_path = ReceiptsPath {
        port_id: port_id.clone(),
        channel_id: channel_id.clone(),
        sequence: sequence.clone(),
    }
    .to_string()
    .as_bytes()
    .to_vec();

    // Address to a storage entry we'd like to access.
    let address = ibc_node::storage()
        .ibc()
        .packet_receipt(packet_receipt_path);

    let value: Option<Vec<u8>> = client.storage().fetch(&address, Some(block_hash)).await?;

    if let Some(data) = value {
        let receipt = String::from_utf8(data)
            .map_err(|_| subxt::error::Error::Other("decode receipt Error".to_string()))?;
        if receipt.eq("Ok") {
            Ok(Receipt::Ok)
        } else {
            Err(subxt::error::Error::Other(format!(
                "unrecognized packet receipt: {:?}",
                receipt
            )))
        }
    } else {
        Err(subxt::error::Error::Other(format!(
            "get_packet_receipt is empty! by port_id = ({}), channel_id = ({})",
            port_id, channel_id
        )))
    }
}

/// get packet receipt by port_id, channel_id and sequence
pub async fn get_packet_receipt_vec(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: &Sequence,
    client: OnlineClient<MyConfig>,
) -> Result<Vec<u8>, subxt::error::Error> {
    tracing::info!("in call_ibc : [get_packet_receipt]");

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let packet_receipt_path = ReceiptsPath {
        port_id: port_id.clone(),
        channel_id: channel_id.clone(),
        sequence: sequence.clone(),
    }
    .to_string()
    .as_bytes()
    .to_vec();

    // Address to a storage entry we'd like to access.
    let address = ibc_node::storage()
        .ibc()
        .packet_receipt(packet_receipt_path);

    let value: Option<Vec<u8>> = client.storage().fetch(&address, Some(block_hash)).await?;

    if let Some(data) = value {
        Ok(data)
    } else {
        Err(subxt::error::Error::Other(format!(
            "get_packet_receipt is empty! by port_id = ({}), channel_id = ({})",
            port_id, channel_id
        )))
    }
}

/// get  unreceipt packet
pub async fn get_unreceipt_packet(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequences: Vec<Sequence>,
    client: OnlineClient<MyConfig>,
) -> Result<Vec<u64>, subxt::error::Error> {
    tracing::info!("in call_ibc: [get_receipt_packet]");

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let mut result = Vec::new();

    let pair = sequences
        .into_iter()
        .map(|sequence| (port_id.clone(), channel_id.clone(), sequence.clone()));

    for (port_id, channel_id, sequence) in pair {
        let packet_receipt_path = ReceiptsPath {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            sequence: sequence.clone(),
        }
        .to_string()
        .as_bytes()
        .to_vec();

        // Address to a storage entry we'd like to access.
        let address = ibc_node::storage()
            .ibc()
            .packet_receipt(packet_receipt_path);

        let value: Option<Vec<u8>> = client.storage().fetch(&address, Some(block_hash)).await?;

        if value.is_none() {
            result.push(u64::from(sequence));
        }
    }

    Ok(result)
}

/// get get_commitment_packet_state
pub async fn get_commitment_packet_state(
    client: OnlineClient<MyConfig>,
) -> Result<Vec<PacketState>, subxt::error::Error> {
    tracing::info!("in call_ibc: [get_commitment_packet_state]");

    let mut result = vec![];

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let address = ibc_node::storage().ibc().packet_commitment_root();

    // Iterate over keys and values at that address.
    let mut iter = client.storage().iter(address, 10, Some(block_hash)).await.unwrap();

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
            Path::Commitments(commitments) => {
                let CommitmentsPath {
                    port_id,
                    channel_id,
                    sequence,
                } = commitments;

                let packet_state = PacketState {
                    port_id: port_id.to_string(),
                    channel_id: channel_id.to_string(),
                    sequence: u64::from(sequence),
                    data: value,
                };
                result.push(packet_state);
            }
            _ => unimplemented!(),
        }
    }

    Ok(result)
}

/// get packet commitment by port_id, channel_id and sequence to verify if the packet has been sent by the sending chain
pub async fn get_packet_commitment(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: &Sequence,
    client: OnlineClient<MyConfig>,
) -> Result<Vec<u8>, subxt::error::Error> {
    tracing::info!("in call_ibc: [get_packet_commitment]");

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let packet_commits_path = CommitmentsPath {
        port_id: port_id.clone(),
        channel_id: channel_id.clone(),
        sequence: sequence.clone(),
    }
    .to_string()
    .as_bytes()
    .to_vec();

    // Address to a storage entry we'd like to access.
    let address = ibc_node::storage()
        .ibc()
        .packet_commitment(packet_commits_path);

    let value: Option<Vec<u8>> = client.storage().fetch(&address, Some(block_hash)).await?;

    if let Some(data) = value {
        Ok(data)
    } else {
        Err(subxt::error::Error::Other(format!(
            "get_packet_commitment is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id, channel_id, sequence
        )))
    }
}

/// get packet acknowledgement by port_id, channel_id and sequence to verify if the packet has been received by the target chain
pub async fn get_packet_ack(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: &Sequence,
    client: OnlineClient<MyConfig>,
) -> Result<Vec<u8>, subxt::error::Error> {
    tracing::info!("in call_ibc: [get_packet_ack]");

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let acks_path = AcksPath {
        port_id: port_id.clone(),
        channel_id: channel_id.clone(),
        sequence: sequence.clone(),
    }
    .to_string()
    .as_bytes()
    .to_vec();

    // Address to a storage entry we'd like to access.
    let address = ibc_node::storage().ibc().acknowledgements(acks_path);

    let value: Option<Vec<u8>> = client.storage().fetch(&address, Some(block_hash)).await?;

    if let Some(data) = value {
        Ok(data)
    } else {
        Err(subxt::error::Error::Other(format!(
            "get_packet_ack is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id, channel_id, sequence
        )))
    }
}

/// get packet receipt by port_id, channel_id and sequence
pub async fn get_next_sequence_recv(
    port_id: &PortId,
    channel_id: &ChannelId,
    client: OnlineClient<MyConfig>,
) -> Result<Sequence, subxt::error::Error> {
    tracing::info!("in call_ibc: [get_next_sequence_recv]");

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let seq_recvs_path = SeqRecvsPath(port_id.clone(), channel_id.clone())
        .to_string()
        .as_bytes()
        .to_vec();

    // Address to a storage entry we'd like to access.
    let address = ibc_node::storage().ibc().next_sequence_recv(seq_recvs_path);

    let value: Option<u64> = client.storage().fetch(&address, Some(block_hash)).await?;

    if let Some(sequence) = value {
        Ok(Sequence::from(sequence))
    } else {
        Err(subxt::error::Error::Other(format!(
            "get_next_sequence_recv is empty!"
        )))
    }
}

/// get get_commitment_packet_state
pub async fn get_acknowledge_packet_state(
    client: OnlineClient<MyConfig>,
) -> Result<Vec<PacketState>, subxt::error::Error> {
    tracing::info!("in call_ibc: [get_acknowledge_packet_state]");

    let mut result = vec![];

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let address = ibc_node::storage().ibc().acknowledgements_root();

    // Iterate over keys and values at that address.
    let mut iter = client.storage().iter(address, 10, Some(block_hash)).await.unwrap();

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
            Path::Acks(acks_path) => {
                let AcksPath {
                    port_id,
                    channel_id,
                    sequence,
                } = acks_path;

                let packet_state = PacketState {
                    port_id: port_id.to_string(),
                    channel_id: channel_id.to_string(),
                    sequence: u64::from(sequence),
                    data: value,
                };
                result.push(packet_state);
            }
            _ => unimplemented!(),
        }
    }

    Ok(result)
}
