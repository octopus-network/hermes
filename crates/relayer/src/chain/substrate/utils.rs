use super::relaychain_node;
use crate::error::Error;
use alloc::sync::Arc;
use codec::{Decode, Encode};
use ibc_relayer_types::clients::ics10_grandpa::header::message::StateProof;
use ibc_relayer_types::clients::ics10_grandpa::header::message::{
    SubchainHeader, SubchainHeaderMap,
};
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use sp_core::{hexdisplay::HexDisplay, ByteArray, H256};
use subxt::rpc::types::BlockNumber;
use subxt::{tx::PairSigner, OnlineClient, PolkadotConfig, SubstrateConfig};
use tokio::runtime::Runtime as TokioRuntime;

pub async fn build_subchain_header_map(
    relay_rpc_client: &OnlineClient<PolkadotConfig>,
    leaf_indexes: Vec<u64>,
    chain_id: String,
) -> Result<SubchainHeaderMap, String> {
    let mut subchain_header_map = SubchainHeaderMap::new();
    for block_number in leaf_indexes {
        let block_hash = relay_rpc_client
            .rpc()
            .block_hash(Some(BlockNumber::from(block_number)))
            .await
            .unwrap()
            .unwrap();
        let block_header = relay_rpc_client
            .rpc()
            .header(Some(block_hash))
            .await
            .unwrap()
            .unwrap();
        let encode_header = codec::Encode::encode(&block_header);
        let timestamp = build_time_stamp_proof(relay_rpc_client, block_hash)
            .await
            .unwrap();
        subchain_header_map.subchain_header_map.insert(
            block_number as u32,
            SubchainHeader {
                chain_id: ChainId::from(chain_id.clone()),
                block_header: encode_header,
                timestamp,
            },
        );
    }
    Ok(subchain_header_map)
}

/// build merkle proof for validator
pub async fn build_validator_proof(
    relay_rpc_client: &OnlineClient<PolkadotConfig>,
    block_number: u32,
) -> Result<Vec<beefy_light_client::ValidatorMerkleProof>, Error> {
    // get block hash
    let block_hash = relay_rpc_client
        .rpc()
        .block_hash(Some(BlockNumber::from(block_number)))
        .await
        .unwrap();

    let storage = relaychain_node::storage().beefy().authorities();

    let authorities = relay_rpc_client
        .storage()
        .at(block_hash)
        .await
        .unwrap()
        .fetch(&storage)
        .await
        .unwrap()
        .unwrap();

    // covert authorities to strings
    let authority_strs: Vec<String> = authorities
        .0
        .into_iter()
        .map(|authority| format!("{}", HexDisplay::from(&authority.0 .0.as_ref())))
        .collect();

    // Convert BEEFY secp256k1 public keys into Ethereum addresses
    let validators: Vec<Vec<u8>> = authority_strs
        .into_iter()
        .map(|authority| {
            hex::decode(&authority)
                .map(|compressed_key| beefy_light_client::beefy_ecdsa_to_ethereum(&compressed_key))
                .unwrap_or_default()
        })
        .collect();

    let mut validator_merkle_proofs: Vec<beefy_light_client::ValidatorMerkleProof> = Vec::new();
    for l in 0..validators.len() {
        // when
        let proof = binary_merkle_tree::merkle_proof::<
            beefy_light_client::keccak256::Keccak256,
            _,
            _,
        >(validators.clone(), l);

        println!("get validator proof root = {}", hex::encode(&proof.root));

        let validator_merkle_proof = beefy_light_client::ValidatorMerkleProof {
            root: proof.root,
            proof: proof.proof,
            number_of_leaves: proof.number_of_leaves as u64,
            leaf_index: proof.leaf_index as u64,
            leaf: proof.leaf,
        };

        validator_merkle_proofs.push(validator_merkle_proof);
    }

    Ok(validator_merkle_proofs)
}

pub async fn build_time_stamp_proof(
    relay_rpc_client: &OnlineClient<PolkadotConfig>,
    block_hash: H256,
) -> Result<StateProof, Error> {
    let time_stamp_value = get_time_stamp_value(relay_rpc_client, block_hash).await?;
    let time_stamp_proof = get_time_stamp_proof(relay_rpc_client, block_hash).await?;

    let storage_key = relaychain_node::storage().timestamp().now();
    let time_stamp_proof = StateProof {
        key: storage_key.to_bytes(),
        value: time_stamp_value.encode(),
        proofs: time_stamp_proof.proof.into_iter().map(|v| v.0).collect(),
    };

    Ok(time_stamp_proof)
}

// ref: https://github.com/octopus-network/beefy-go/blob/2768424da33fecd82f3145f0e683820600b7f944/beefy/timestamp.go#L42
pub async fn get_time_stamp_value(
    relay_rpc_client: &OnlineClient<PolkadotConfig>,
    block_hash: H256,
) -> Result<u64, Error> {
    let storage = relaychain_node::storage().timestamp().now();

    let time_stamp = relay_rpc_client
        .storage()
        .at(Some(block_hash))
        .await
        .unwrap()
        .fetch(&storage)
        .await
        .unwrap()
        .unwrap();

    Ok(time_stamp)
}

pub async fn get_time_stamp_proof(
    relay_rpc_client: &OnlineClient<PolkadotConfig>,
    block_hash: H256,
) -> Result<subxt::rpc::types::ReadProof<H256>, Error> {
    let storage_key = relaychain_node::storage().timestamp().now();
    let result = relay_rpc_client
        .rpc()
        .read_proof(vec![storage_key.to_bytes().as_ref()], Some(block_hash))
        .await
        .unwrap();
    Ok(result)
}

// get mmr proofs for the given indexes without blockhash
pub async fn build_mmr_proofs(
    relay_rpc_client: &OnlineClient<PolkadotConfig>,
    block_numbers: Vec<u32>,
    best_known_block_number: Option<u32>,
    at: Option<H256>,
) -> Result<mmr_rpc::LeavesProof<H256>, Error> {
    if let Some(best_know_block_number) = best_known_block_number {
        // let call_closure = async {
        let mut block_number = block_numbers.clone();
        block_number.sort();
        let block_number: Vec<BlockNumber> = block_number.into_iter().map(|v| v.into()).collect();
        // best_known_block_number must ET all the blockNumbers
        if best_know_block_number < u32::try_from(block_numbers[block_numbers.len() - 1]).unwrap() {
            panic!("best_known_block_number must > all the blockNumbers")
        }
        let best_known_block_number: Option<BlockNumber> =
            Some(BlockNumber::from(best_know_block_number));

        let params = subxt::rpc_params![block_number, best_known_block_number, at];
        let leaves_proof_result: mmr_rpc::LeavesProof<H256> = relay_rpc_client
            .rpc()
            .request("mmr_generateProof", params)
            .await
            .unwrap();
        Ok(leaves_proof_result)
    } else {
        let block_numner: Vec<BlockNumber> = block_numbers.into_iter().map(|v| v.into()).collect();
        let best_known_block_number: Option<BlockNumber> = None;

        let params = subxt::rpc_params![block_numner, best_known_block_number, at];
        let leaves_proof_result: mmr_rpc::LeavesProof<H256> = relay_rpc_client
            .rpc()
            .request("mmr_generateProof", params)
            .await
            .unwrap();
        Ok(leaves_proof_result)
    }
}

pub fn convert_mmrproof(proofs: mmr_rpc::LeavesProof<H256>) -> Result<ibc_relayer_types::clients::ics10_grandpa::header::beefy_mmr::mmr_leaves_and_batch_proof::MmrLeavesAndBatchProof, Error>{
    let mmr_leavfs = proofs.leaves.0;
    let mmr_proofs = proofs.proof.0;

    let leaves = beefy_light_client::mmr::decode_mmr_leaves(mmr_leavfs).unwrap().into_iter().map(|v| {
        let version = v.version.0 as u32;
        let parent_number_and_hash = ibc_relayer_types::clients::ics10_grandpa::header::beefy_mmr::mmr_leaves_and_batch_proof::ParentNumberAndHash {
            // parent block for this leaf
            parent_number: v.parent_number_and_hash.0,
            // parent hash for this leaf
            parent_hash: v.parent_number_and_hash.1.to_vec(),
        };

        let beefy_next_authority_set =  ibc_relayer_types::clients::ics10_grandpa::beefy_authority_set::BeefyAuthoritySet {
            // Id of the authority set, it should be strictly increasing
            id: v.beefy_next_authority_set.id,
            // Number of validators in the set.
            len: v.beefy_next_authority_set.len,
            // Merkle Root Hash build from BEEFY uncompressed AuthorityIds.
            root: v.beefy_next_authority_set.root.to_vec(),
        };
        let parachain_heads = v.leaf_extra;


        ibc_relayer_types::clients::ics10_grandpa::header::beefy_mmr::mmr_leaves_and_batch_proof::MmrLeaf {
            // leaf version
            version,
            // parent number and hash
            parent_number_and_hash,
            // beefy next authority set.
            beefy_next_authority_set ,
            // merkle root hash of parachain heads included in the leaf.
            parachain_heads,
    }}).collect::<Vec<_>>();

    let mmr_leaves_proof = beefy_light_client::mmr::MmrLeavesProof::try_from(mmr_proofs).unwrap();

    let mmr_batch_proof = ibc_relayer_types::clients::ics10_grandpa::header::beefy_mmr::mmr_leaves_and_batch_proof::MmrBatchProof {
        // The index of the leaf the proof is for.
        leaf_indexes: mmr_leaves_proof.leaf_indices,
        // Number of leaves in MMR, when the proof was generated.
        leaf_count: mmr_leaves_proof.leaf_count,
        // Proof elements (hashes of siblings of inner nodes on the path to the leaf).
        items: mmr_leaves_proof.items.into_iter().map(|v| v.to_vec()).collect(),
    };

    Ok(ibc_relayer_types::clients::ics10_grandpa::header::beefy_mmr::mmr_leaves_and_batch_proof::MmrLeavesAndBatchProof {
        leaves,
        mmr_batch_proof: Some(mmr_batch_proof)
    })
}

pub fn to_pb_beefy_mmr(
    bsc: beefy_light_client::commitment::SignedCommitment,
    mmr_batch_proof: mmr_rpc::LeavesProof<H256>,
    authority_proof: Vec<beefy_light_client::ValidatorMerkleProof>,
) -> ibc_relayer_types::clients::ics10_grandpa::header::beefy_mmr::BeefyMmr {
    let data = bsc
        .commitment
        .payload
        .get_raw(&beefy_light_client::commitment::known_payload_ids::MMR_ROOT_ID)
        .unwrap();

    let payload_item = ibc_relayer_types::clients::ics10_grandpa::header::beefy_mmr::signed_commitment::PayloadItem {
        // 2-byte payload id
        id: beefy_light_client::commitment::known_payload_ids::MMR_ROOT_ID.to_vec(),
        // arbitrary length payload data., eg mmr_root_hash
        data: data.clone(),
    };
    let commitment = ibc_relayer_types::clients::ics10_grandpa::header::beefy_mmr::signed_commitment::Commitment {
        // array of payload items signed by Beefy validators
        payloads: vec![payload_item],
        // block number for this commitment
        block_number: bsc.commitment.block_number,
        // validator set that signed this commitment
        validator_set_id: bsc.commitment.validator_set_id,
    };

    let mut signatures = vec![];
    bsc.signatures.iter().enumerate().for_each(|(idx, value)| {
        if let Some(v) = value {
            let ret = ibc_relayer_types::clients::ics10_grandpa::header::beefy_mmr::signed_commitment::Signature {
                index: idx as u32,
               signature: v.0.to_vec()
            };
            signatures.push(ret);
        }
    });

    let pb_commitment = ibc_relayer_types::clients::ics10_grandpa::header::beefy_mmr::signed_commitment::SignedCommitment {
        /// commitment data being signed
        commitment: Some(commitment),
        /// all the signatures
        signatures,
    };

    let leaf_index = convert_block_number_to_mmr_leaf_index(0, bsc.commitment.block_number); // todo first paramment

    let size = beefy_light_client::mmr::NodesUtils::new(leaf_index).size(); // todo need correct function calclute
    ibc_relayer_types::clients::ics10_grandpa::header::beefy_mmr::BeefyMmr {
        // signed commitment data
        signed_commitment: Some(pb_commitment),
        // build merkle tree based on all the signature in signed commitment
        // and generate the signature proof
        signature_proofs: authority_proof.into_iter().map(|v| v.encode()).collect(),
        // mmr proof
        mmr_leaves_and_batch_proof: Some(convert_mmrproof(mmr_batch_proof).unwrap()),
        // size of the mmr for the given proof
        mmr_size: leaf_index, // todo
    }
}

pub fn convert_block_number_to_mmr_leaf_index(
    beefy_activation_block: u32,
    block_number: u32,
) -> u64 {
    let mut leaf_index: u32 = 0;
    if beefy_activation_block == 0 {
        leaf_index = block_number - 1;
    } else {
        leaf_index = (block_number + 1) - beefy_activation_block
    }
    leaf_index as u64
}
