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
    rt: Arc<TokioRuntime>,
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
        let timestamp = build_time_stamp_proof(rt.clone(), relay_rpc_client, block_hash).unwrap();
        subchain_header_map.subchain_header_map.insert(
            block_number as u32,
            SubchainHeader {
                chain_id: ChainId::from(chain_id.clone()),
                block_header: encode_header,
                timestamp: Some(timestamp),
            },
        );
    }
    Ok(subchain_header_map)
}

/// build merkle proof for validator
pub fn build_validator_proof(
    rt: Arc<TokioRuntime>,
    relay_rpc_client: &OnlineClient<PolkadotConfig>,
    block_number: u32,
) -> Result<Vec<beefy_light_client::ValidatorMerkleProof>, Error> {
    let closure = async {
        // get block hash
        let block_hash = relay_rpc_client
            .rpc()
            .block_hash(Some(BlockNumber::from(block_number)))
            .await
            .unwrap();

        let storage = relaychain_node::storage().beefy().authorities();

        relay_rpc_client
            .storage()
            .at(block_hash)
            .await
            .unwrap()
            .fetch(&storage)
            .await
            .unwrap()
    };
    let authorities = rt.block_on(closure).unwrap();

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

pub fn build_time_stamp_proof(
    rt: Arc<TokioRuntime>,
    relay_rpc_client: &OnlineClient<PolkadotConfig>,
    block_hash: H256,
) -> Result<StateProof, Error> {
    let time_stamp_value = get_time_stamp_value(rt.clone(), relay_rpc_client, block_hash)?;
    let time_stamp_proof = get_time_stamp_proof(rt, relay_rpc_client, block_hash)?;

    let storage_key = relaychain_node::storage().timestamp().now();
    let time_stamp_proof = StateProof {
        key: storage_key.to_bytes(),
        value: time_stamp_value.encode(),
        proofs: time_stamp_proof.proof.into_iter().map(|v| v.0).collect(),
    };

    Ok(time_stamp_proof)
}

// ref: https://github.com/octopus-network/beefy-go/blob/2768424da33fecd82f3145f0e683820600b7f944/beefy/timestamp.go#L42
pub fn get_time_stamp_value(
    rt: Arc<TokioRuntime>,
    relay_rpc_client: &OnlineClient<PolkadotConfig>,
    block_hash: H256,
) -> Result<u64, Error> {
    let storage = relaychain_node::storage().timestamp().now();

    let closure = async {
        relay_rpc_client
            .storage()
            .at(Some(block_hash))
            .await
            .unwrap()
            .fetch(&storage)
            .await
            .unwrap()
    };
    let time_stamp = rt.block_on(closure).unwrap();

    Ok(time_stamp)
}

pub fn get_time_stamp_proof(
    rt: Arc<TokioRuntime>,
    relay_rpc_client: &OnlineClient<PolkadotConfig>,
    block_hash: H256,
) -> Result<subxt::rpc::types::ReadProof<H256>, Error> {
    let call_closure = async {
        let storage_key = relaychain_node::storage().timestamp().now();
        relay_rpc_client
            .rpc()
            .read_proof(vec![storage_key.to_bytes().as_ref()], Some(block_hash))
            .await
            .unwrap()
    };
    let result = rt.block_on(call_closure);
    Ok(result)
}

// get mmr proofs for the given indexes without blockhash
pub fn build_mmr_proofs(
    rt: Arc<TokioRuntime>,
    relay_rpc_client: &OnlineClient<PolkadotConfig>,
    block_numbers: Vec<u32>,
    best_known_block_number: Option<u32>,
    at: Option<H256>,
) -> Result<mmr_rpc::LeavesProof<H256>, Error> {
    if let Some(best_know_block_number) = best_known_block_number {
        let call_closure = async {
            let mut block_number = block_numbers.clone();
            block_number.sort();
            let block_number: Vec<BlockNumber> =
                block_number.into_iter().map(|v| v.into()).collect();
            // best_known_block_number must ET all the blockNumbers
            if best_know_block_number
                < u32::try_from(block_numbers[block_numbers.len() - 1]).unwrap()
            {
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
            leaves_proof_result
        };
        let result = rt.block_on(call_closure);
        Ok(result)
    } else {
        let call_closure = async {
            let block_numner: Vec<BlockNumber> =
                block_numbers.into_iter().map(|v| v.into()).collect();
            let best_known_block_number: Option<BlockNumber> = None;

            let params = subxt::rpc_params![block_numner, best_known_block_number, at];
            let leaves_proof_result: mmr_rpc::LeavesProof<H256> = relay_rpc_client
                .rpc()
                .request("mmr_generateProof", params)
                .await
                .unwrap();
            leaves_proof_result
        };
        let result = rt.block_on(call_closure);
        Ok(result)
    }
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

    todo!()
}

// func ToPBBeefyMMR(bsc beefy.SignedCommitment, mmrBatchProof beefy.MmrProofsResp, authorityProof [][]byte) BeefyMMR {

// 	// bsc := beefy.ConvertCommitment(sc)
// 	pbPalyloads := make([]PayloadItem, len(bsc.Commitment.Payload))
// 	for i, v := range bsc.Commitment.Payload {
// 		pbPalyloads[i] = PayloadItem{
// 			Id:   v.ID[:],
// 			Data: v.Data,
// 		}

// 	}

// 	pbCommitment := Commitment{
// 		Payloads:       pbPalyloads,
// 		BlockNumber:    bsc.Commitment.BlockNumber,
// 		ValidatorSetId: bsc.Commitment.ValidatorSetID,
// 	}

// 	pb := make([]Signature, len(bsc.Signatures))
// 	for i, v := range bsc.Signatures {
// 		pb[i] = Signature(v)
// 	}

// 	pbsc := SignedCommitment{
// 		Commitment: pbCommitment,
// 		Signatures: pb,
// 	}
// 	// convert mmrleaf
// 	var pbMMRLeaves []MMRLeaf

// 	leafNum := len(mmrBatchProof.Leaves)
// 	for i := 0; i < leafNum; i++ {
// 		leaf := mmrBatchProof.Leaves[i]
// 		parentNumAndHash := ParentNumberAndHash{
// 			ParentNumber: uint32(leaf.ParentNumberAndHash.ParentNumber),
// 			ParentHash:   []byte(leaf.ParentNumberAndHash.Hash[:]),
// 		}
// 		nextAuthoritySet := BeefyAuthoritySet{
// 			Id:   uint64(leaf.BeefyNextAuthoritySet.ID),
// 			Len:  uint32(leaf.BeefyNextAuthoritySet.Len),
// 			Root: []byte(leaf.BeefyNextAuthoritySet.Root[:]),
// 		}
// 		parachainHeads := []byte(leaf.ParachainHeads[:])
// 		gLeaf := MMRLeaf{
// 			Version:               uint32(leaf.Version),
// 			ParentNumberAndHash:   parentNumAndHash,
// 			BeefyNextAuthoritySet: nextAuthoritySet,
// 			ParachainHeads:        parachainHeads,
// 		}
// 		// Logger.Info("gLeaf: ", gLeaf)
// 		pbMMRLeaves = append(pbMMRLeaves, gLeaf)
// 	}

// 	// convert mmr batch proof
// 	pbLeafIndexes := make([]uint64, len(mmrBatchProof.Proof.LeafIndexes))
// 	for i, v := range mmrBatchProof.Proof.LeafIndexes {
// 		pbLeafIndexes[i] = uint64(v)
// 	}

// 	pbProofItems := [][]byte{}
// 	itemNum := len(mmrBatchProof.Proof.Items)
// 	for i := 0; i < itemNum; i++ {
// 		item := mmrBatchProof.Proof.Items[i][:]
// 		pbProofItems = append(pbProofItems, item)

// 	}

// 	pbBatchProof := MMRBatchProof{
// 		LeafIndexes: pbLeafIndexes,
// 		LeafCount:   uint64(mmrBatchProof.Proof.LeafCount),
// 		Items:       pbProofItems,
// 	}

// 	pbMmrLevavesAndProof := MMRLeavesAndBatchProof{
// 		Leaves:        pbMMRLeaves,
// 		MmrBatchProof: pbBatchProof,
// 	}
// 	leafIndex := beefy.ConvertBlockNumberToMmrLeafIndex(uint32(beefy.BEEFY_ACTIVATION_BLOCK), bsc.Commitment.BlockNumber)
// 	mmrSize := mmr.LeafIndexToMMRSize(uint64(leafIndex))
// 	// build pbBeefyMMR
// 	pbBeefyMMR := BeefyMMR{
// 		SignedCommitment:       pbsc,
// 		SignatureProofs:        authorityProof,
// 		MmrLeavesAndBatchProof: pbMmrLevavesAndProof,
// 		MmrSize:                mmrSize,
// 	}
// 	return pbBeefyMMR
// }
