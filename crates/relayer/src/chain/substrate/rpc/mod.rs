pub mod beefy;
pub mod channel;
pub mod client;
pub mod connection;
pub mod event;
pub use channel::*;
pub use client::*;
pub use connection::*;

use crate::chain::substrate::config::ibc_node;
use crate::chain::substrate::rpc::ibc_node::runtime_types::ibc_support::Any as RuntimeAny;
use crate::chain::substrate::MyConfig;
use anyhow::Result;
use beefy::subscribe_beefy_justifications;
use beefy_merkle_tree::Hash;
use ibc_proto::google::protobuf::Any;
use sp_core::H256;
use sp_keyring::AccountKeyring;
use subxt::rpc::BlockNumber;
use subxt::rpc_params;
use subxt::tx::PairSigner;
use subxt::OnlineClient;

/// Subscribe beefy justifications
pub async fn subscribe_beefy(
    client: OnlineClient<MyConfig>,
) -> Result<beefy::SignedCommitment, subxt::error::Error> {
    tracing::info!("In call_ibc: [subscribe_beefy_justifications]");

    let mut sub = subscribe_beefy_justifications(client).await?;

    let raw = sub.next().await.unwrap().unwrap();

    Ok(raw)
}

/// get latest height used by subscribe_blocks
pub async fn get_latest_height(client: OnlineClient<MyConfig>) -> Result<u64, subxt::error::Error> {
    tracing::info!("In call_ibc: [get_latest_height]");

    let mut blocks = client.rpc().subscribe_finalized_blocks().await?;

    let height = match blocks.next().await {
        Some(Ok(header)) => header.number as u64,
        Some(Err(_)) => 0,
        None => 0,
    };

    Ok(height)
}

/// ibc protocol core function, ics26 deliver function
/// this function will dispatch msg to process
/// return block_hash, extrinsic_hash, and event
pub async fn deliver(
    msg: Vec<Any>,
    client: OnlineClient<MyConfig>,
) -> Result<H256, subxt::error::Error> {
    tracing::info!("in call_ibc: [deliver]");

    let msg: Vec<RuntimeAny> = msg
        .into_iter()
        .map(|value| RuntimeAny {
            type_url: value.type_url.as_bytes().to_vec(),
            value: value.value,
        })
        .collect();

    let signer = PairSigner::new(AccountKeyring::Bob.pair());

    // Create a transaction to submit:
    let tx = ibc_node::tx().ibc().deliver(msg);

    // Submit the transaction with default params:
    let hash = client.tx().sign_and_submit_default(&tx, &signer).await?;

    Ok(hash)
}

// process ibc transfer
pub async fn raw_transfer(
    msg: Vec<Any>,
    client: OnlineClient<MyConfig>,
) -> Result<H256, subxt::error::Error> {
    tracing::info!("in call_ibc: [deliver]");

    let msg: Vec<RuntimeAny> = msg
        .into_iter()
        .map(|value| RuntimeAny {
            type_url: value.type_url.as_bytes().to_vec(),
            value: value.value,
        })
        .collect();

    let signer = PairSigner::new(AccountKeyring::Bob.pair());

    // Create a transaction to submit:
    let tx = ibc_node::tx().ics20().raw_transfer(msg);

    // Submit the transaction with default params:
    let hash = client.tx().sign_and_submit_default(&tx, &signer).await?;

    Ok(hash)
}

pub async fn get_mmr_leaf_and_mmr_proof(
    block_number: Option<BlockNumber>,
    block_hash: Option<H256>,
    client: OnlineClient<MyConfig>,
) -> Result<(String, Vec<u8>, Vec<u8>), subxt::error::Error> {
    tracing::info!("in call_ibc [get_mmr_leaf_and_mmr_proof]");

    let params = rpc_params![block_number, block_hash];

    let generate_proof: pallet_mmr_rpc::LeafProof<String> =
        client.rpc().request("mmr_generateProof", params).await?;

    Ok((
        generate_proof.block_hash,
        generate_proof.leaf.0,
        generate_proof.proof.0,
    ))
}

/// get header by block hash
pub async fn get_header_by_block_hash(
    block_hash: Option<H256>,
    client: OnlineClient<MyConfig>,
) -> Result<ibc_relayer_types::clients::ics10_grandpa::help::BlockHeader, subxt::error::Error> {
    let header = client.rpc().header(block_hash).await?.unwrap();

    let header = convert_substrate_header_to_ibc_header(header);

    Ok(header.into())
}

/// get header by block number
pub async fn get_header_by_block_number(
    block_number: Option<BlockNumber>,
    client: OnlineClient<MyConfig>,
) -> Result<ibc_relayer_types::clients::ics10_grandpa::help::BlockHeader, subxt::error::Error> {
    let block_hash = client.rpc().block_hash(block_number).await?;

    let header = client.rpc().header(block_hash).await?.unwrap();

    let header = convert_substrate_header_to_ibc_header(header);

    Ok(header.into())
}

pub async fn get_timestamp(
    block_number: Option<BlockNumber>,
    client: OnlineClient<MyConfig>,
) -> Result<u64, subxt::error::Error> {
    let block_hash = client.rpc().block_hash(block_number).await?;

    let address = ibc_node::storage().timestamp().now();

    let timestamp: u64 = client
        .storage()
        .fetch(&address, block_hash)
        .await?
        .ok_or(subxt::error::Error::Other("timestamp is empty".to_string()))?;

    tracing::info!("in get_timestamp timestamp = {:?}", timestamp);

    Ok(timestamp)
}

/// convert substrate Header to Ibc Header
fn convert_substrate_header_to_ibc_header(
    header: sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>,
) -> beefy_light_client::header::Header {
    beefy_light_client::header::Header {
        parent_hash: Hash::from(header.parent_hash),
        number: header.number,
        state_root: Hash::from(header.state_root),
        extrinsics_root: Hash::from(header.extrinsics_root),
        digest: convert_substrate_digest_to_beefy_light_client_digest(header.digest),
    }
}

fn convert_substrate_digest_to_beefy_light_client_digest(
    digest: sp_runtime::Digest,
) -> beefy_light_client::header::Digest {
    beefy_light_client::header::Digest {
        logs: digest
            .logs
            .into_iter()
            .map(convert_substrate_digest_item_to_beefy_light_client_digest_item)
            .collect(),
    }
}

fn convert_substrate_digest_item_to_beefy_light_client_digest_item(
    digest_item: sp_runtime::DigestItem,
) -> beefy_light_client::header::DigestItem {
    match digest_item {
        sp_runtime::DigestItem::PreRuntime(consensus_engine_id, value) => {
            beefy_light_client::header::DigestItem::PreRuntime(consensus_engine_id, value)
        }
        sp_runtime::DigestItem::Consensus(consensus_engine_id, value) => {
            beefy_light_client::header::DigestItem::Consensus(consensus_engine_id, value)
        }
        sp_runtime::DigestItem::Seal(consensus_engine_id, value) => {
            beefy_light_client::header::DigestItem::Seal(consensus_engine_id, value)
        }
        sp_runtime::DigestItem::Other(value) => {
            beefy_light_client::header::DigestItem::Other(value)
        }
        sp_runtime::DigestItem::RuntimeEnvironmentUpdated => {
            beefy_light_client::header::DigestItem::RuntimeEnvironmentUpdated
        }
    }
}
