use super::super::config::{ibc_node, MyConfig};
use crate::chain::substrate::rpc::storage_iter;
use anyhow::Result;
use ibc_proto::ibc::core::channel::v1::PacketState;
use ibc_proto::protobuf::Protobuf;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
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
use subxt::OnlineClient;

/// get key-value pair (connection_id, connection_end) construct IdentifiedConnectionEnd
pub async fn get_channels(client: OnlineClient<MyConfig>) -> Result<Vec<IdentifiedChannelEnd>> {
    tracing::info!("in call_ibc: [get_channels]");

    let callback = Box::new(
        |path: Path,
         result: &mut Vec<IdentifiedChannelEnd>,
         value: Vec<u8>,
         _client_id: ClientId| {
            match path {
                Path::ChannelEnds(channel_ends_path) => {
                    let ChannelEndsPath(port_id, channel_id) = channel_ends_path;
                    let channel_end = ChannelEnd::decode_vec(&*value).unwrap();

                    result.push(IdentifiedChannelEnd::new(port_id, channel_id, channel_end));
                }
                _ => unimplemented!(),
            }
        },
    );

    let mut result = vec![];

    let _ret = storage_iter::<IdentifiedChannelEnd, ibc_node::ibc::storage::Channels>(
        client.clone(),
        &mut result,
        ClientId::default(),
        callback,
    )
    .await?;

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

    let data: Vec<u8> = client
        .storage()
        .ibc()
        .channels(&channel_end_path, Some(block_hash))
        .await?;

    if data.is_empty() {
        return Err(subxt::error::Error::Other(format!(
            "get_channel_end is empty by port_id = ({}), channel_id = ({})",
            port_id, channel_id
        )));
    }

    let channel_end = ChannelEnd::decode_vec(&*data).unwrap();

    Ok(channel_end)
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

    let data: Vec<u8> = client
        .storage()
        .ibc()
        .packet_receipt(&packet_receipt_path, Some(block_hash))
        .await?;

    if data.is_empty() {
        return Err(subxt::error::Error::Other(format!(
            "get_packet_receipt is empty! by port_id = ({}), channel_id = ({})",
            port_id, channel_id
        )));
    }

    let receipt = String::from_utf8(data)?;
    if receipt.eq("Ok") {
        Ok(Receipt::Ok)
    } else {
        Err(subxt::error::Error::Other(format!(
            "unrecognized packet receipt: {:?}",
            receipt
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

    let data: Vec<u8> = client
        .storage()
        .ibc()
        .packet_receipt(&packet_receipt_path, Some(block_hash))
        .await?;

    if data.is_empty() {
        return Err(subxt::error::Error::Other(format!(
            "get_packet_receipt is empty! by port_id = ({}), channel_id = ({})",
            port_id, channel_id
        )));
    }

    Ok(data)
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

        let data: Vec<u8> = client
            .storage()
            .ibc()
            .packet_receipt(&packet_receipt_path, Some(block_hash))
            .await?;
        if data.is_empty() {
            result.push(u64::from(sequence));
        }
    }

    Ok(result)
}

/// get get_commitment_packet_state
pub async fn get_commitment_packet_state(
    client: OnlineClient<MyConfig>,
) -> Result<Vec<PacketState>> {
    tracing::info!("in call_ibc: [get_commitment_packet_state]");

    let callback = Box::new(
        |path: Path, result: &mut Vec<PacketState>, value: Vec<u8>, _client_id: ClientId| match path
        {
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
        },
    );

    let mut result = vec![];

    let _ret = storage_iter::<PacketState, ibc_node::ibc::storage::PacketCommitment>(
        client.clone(),
        &mut result,
        ClientId::default(),
        callback,
    )
    .await?;

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

    let data: Vec<u8> = client
        .storage()
        .ibc()
        .packet_commitment(&packet_commits_path, Some(block_hash))
        .await?;

    if data.is_empty() {
        Err(subxt::error::Error::Other(format!(
            "get_packet_commitment is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id, channel_id, sequence
        )))
    } else {
        Ok(data)
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

    let data: Vec<u8> = client
        .storage()
        .ibc()
        .acknowledgements(&acks_path, Some(block_hash))
        .await?;

    if data.is_empty() {
        Err(subxt::error::Error::Other(format!(
            "get_packet_ack is empty! by port_id = ({}), channel_id = ({}), sequence = ({})",
            port_id, channel_id, sequence
        )))
    } else {
        Ok(data)
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

    let sequence: u64 = client
        .storage()
        .ibc()
        .next_sequence_recv(&seq_recvs_path, Some(block_hash))
        .await?;

    Ok(Sequence::from(sequence))
}

/// get get_commitment_packet_state
pub async fn get_acknowledge_packet_state(
    client: OnlineClient<MyConfig>,
) -> Result<Vec<PacketState>> {
    tracing::info!("in call_ibc: [get_acknowledge_packet_state]");

    let callback = Box::new(
        |path: Path, result: &mut Vec<PacketState>, value: Vec<u8>, _client_id: ClientId| match path
        {
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
        },
    );

    let mut result = vec![];

    let _ret = storage_iter::<PacketState, ibc_node::ibc::storage::Acknowledgements>(
        client.clone(),
        &mut result,
        ClientId::default(),
        callback,
    )
    .await?;

    Ok(result)
}
