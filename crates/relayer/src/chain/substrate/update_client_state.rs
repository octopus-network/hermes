use super::config::{ibc_node, MyConfig};
use super::rpc::get_latest_height;
use super::rpc::get_mmr_leaf_and_mmr_proof;
use crate::client_state::AnyClientState;
use anyhow::Result;
use beefy_light_client::commitment::SignedCommitment;
use beefy_light_client::{beefy_ecdsa_to_ethereum, commitment, Error};
use beefy_merkle_tree::{merkle_proof, verify_proof, Hash, Keccak256};
use codec::{Decode, Encode};
use ibc_proto::protobuf::Protobuf;
use ibc_relayer_types::core::ics24_host::path::ClientStatePath;
use ibc_relayer_types::core::ics24_host::Path;
use ibc_relayer_types::{
    clients::ics10_grandpa::help,
    core::{ics02_client::client_type::ClientType, ics24_host::identifier::ClientId},
};
use sp_core::{hexdisplay::HexDisplay, H256};
use sp_keyring::AccountKeyring;
use std::str::FromStr;
use subxt::rpc::BlockNumber;
use subxt::tx::PairSigner;
use subxt::OnlineClient;

/// mmr proof struct
#[derive(Clone, Debug, Default)]
pub struct MmrProof {
    pub mmr_leaf: Vec<u8>,
    pub mmr_leaf_proof: Vec<u8>,
}

/// build merkle proof for validator
pub async fn build_validator_proof(
    src_client: OnlineClient<MyConfig>,
    block_number: u32,
) -> Result<Vec<help::ValidatorMerkleProof>, subxt::error::Error> {
    // get block hash
    let block_hash = src_client
        .rpc()
        .block_hash(Some(BlockNumber::from(block_number)))
        .await?;

    //get validator set(authorities)
    let authorities = src_client.storage().beefy().authorities(block_hash).await?;

    // covert authorities to strings
    let authority_strs: Vec<String> = authorities
        .into_iter()
        .map(|authority| format!("{}", HexDisplay::from(&authority.to_raw_vec())))
        .collect();

    // Convert BEEFY secp256k1 public keys into Ethereum addresses
    let validators: Vec<Vec<u8>> = authority_strs
        .into_iter()
        .map(|authority| {
            hex::decode(&authority)
                .map(|compressed_key| beefy_ecdsa_to_ethereum(&compressed_key))
                .unwrap_or_default()
        })
        .collect();

    let mut validator_merkle_proofs: Vec<help::ValidatorMerkleProof> = Vec::new();
    for l in 0..validators.len() {
        // when
        let proof = merkle_proof::<Keccak256, _, _>(validators.clone(), l);

        let validator_merkle_proof =
            help::ValidatorMerkleProof::from(beefy_light_client::ValidatorMerkleProof {
                proof: proof.proof,
                number_of_leaves: proof.number_of_leaves,
                leaf_index: proof.leaf_index,
                leaf: proof.leaf,
            });

        validator_merkle_proofs.push(validator_merkle_proof);
    }

    Ok(validator_merkle_proofs)
}

/// build mmr proof
pub async fn build_mmr_proof(
    src_client: OnlineClient<MyConfig>,
    block_number: u32,
) -> Result<MmrProof, subxt::error::Error> {
    // asset block block number < get laset height
    {
        let latest_height = get_latest_height(src_client.clone()).await?;

        assert!(
            u64::from(block_number) <= latest_height,
            "block_number must less than or equal latest height"
        );
    }

    //get block hash by block_number
    let block_hash = src_client
        .rpc()
        .block_hash(Some(BlockNumber::from(block_number)))
        .await?;

    //get mmr leaf and proof
    // Note: target_height = signed_commitment.commitment.block_number-1
    let target_height = BlockNumber::from(block_number - 1);
    let (block_hash, mmr_leaf, mmr_leaf_proof) = get_mmr_leaf_and_mmr_proof(
        Some(target_height),
        Some(block_hash.unwrap()),
        src_client.clone(),
    )
    .await?;

    let mmr_proof = MmrProof {
        mmr_leaf,
        mmr_leaf_proof,
    };

    Ok(mmr_proof)
}

/// build mmr root
pub async fn build_mmr_root(
    src_client: OnlineClient<MyConfig>,
    signed_commitment: SignedCommitment,
) -> Result<help::MmrRoot, Box<dyn std::error::Error>> {
    // get commitment
    let commitment::Commitment { block_number, .. } = signed_commitment.clone().commitment;

    // build validator proof
    let validator_merkle_proofs = build_validator_proof(src_client.clone(), block_number)
        .await
        .unwrap();

    // build mmr proof
    let mmr_proof = build_mmr_proof(src_client.clone(), block_number)
        .await
        .unwrap();

    // build mmr root
    let mmr_root = help::MmrRoot {
        signed_commitment: help::SignedCommitment::from(signed_commitment),
        validator_merkle_proofs,
        mmr_leaf: mmr_proof.mmr_leaf,
        mmr_leaf_proof: mmr_proof.mmr_leaf_proof,
    };

    Ok(mmr_root)
}
/// send Update client state request
pub async fn send_update_state_request(
    client: OnlineClient<MyConfig>,
    client_id: ClientId,
    mmr_root: help::MmrRoot,
) -> Result<H256> {
    tracing::info!("in call_ibc: [update_client_state]");
    let signer = PairSigner::new(AccountKeyring::Bob.pair());

    let encode_client_id = client_id.as_bytes().to_vec();
    let encode_mmr_root = help::MmrRoot::encode(&mmr_root);

    let result = client
        .tx()
        .ibc()
        .update_client_state(encode_client_id, encode_mmr_root)?
        .sign_and_submit_default(&signer)
        .await?;

    tracing::info!("update client state result: {:?}", result);

    Ok(result)
}

/// update client state by cli for single.
pub async fn update_client_state(
    src_client: OnlineClient<MyConfig>,
    target_client: OnlineClient<MyConfig>,
) -> Result<()> {
    // subscribe beefy justification for src chain
    let mut sub = src_client.rpc().subscribe_beefy_justifications().await?;

    let raw_signed_commitment = sub.next().await.unwrap().unwrap().0;
    // decode signed commitment
    let signed_commitment =
        commitment::SignedCommitment::decode(&mut &raw_signed_commitment.clone()[..]).unwrap();

    // get commitment
    let commitment::Commitment { block_number, .. } = signed_commitment.clone().commitment;

    // build validator proof
    let validator_merkle_proofs = build_validator_proof(src_client.clone(), block_number)
        .await
        .unwrap();

    // build mmr proof
    let mmr_proof = build_mmr_proof(src_client.clone(), block_number)
        .await
        .unwrap();

    // build mmr root
    let mmr_root = help::MmrRoot {
        signed_commitment: help::SignedCommitment::from(signed_commitment),
        validator_merkle_proofs,
        mmr_leaf: mmr_proof.mmr_leaf,
        mmr_leaf_proof: mmr_proof.mmr_leaf_proof,
    };

    // get client id from target chain
    let client_ids = get_client_ids(target_client.clone(), ClientType::Grandpa)
        .await
        .unwrap();
    for client_id in client_ids {
        let result =
            send_update_state_request(target_client.clone(), client_id, mmr_root.clone()).await;

        match result {
            Ok(r) => {
                println!("update client state success and result is : {:?}", r);
            }
            Err(e) => {
                println!("update client state client failure and error is : {:?}", e);
            }
        }
    }
    // send mmr root to substrate-ibc

    Ok(())
}

/// update client state service.
pub async fn update_client_state_service(
    src_client: OnlineClient<MyConfig>,
    target_client: OnlineClient<MyConfig>,
) -> Result<()> {
    // subscribe beefy justification for src chain
    let mut sub = src_client.rpc().subscribe_beefy_justifications().await?;

    // msg loop for handle the beefy SignedCommitment
    loop {
        let raw = sub.next().await.unwrap().unwrap().0;
        // let target_raw = raw.clone();
        let signed_commitment = commitment::SignedCommitment::decode(&mut &raw[..]).unwrap();

        let commitment::Commitment { block_number, .. } = signed_commitment.clone().commitment;

        let signatures: Vec<String> = signed_commitment
            .signatures
            .clone()
            .into_iter()
            .map(|signature| format!("{}", HexDisplay::from(&signature.unwrap().0)))
            .collect();

        // build validator proof
        let validator_merkle_proofs = build_validator_proof(src_client.clone(), block_number)
            .await
            .unwrap();

        // build mmr proof
        let mmr_proof = build_mmr_proof(src_client.clone(), block_number)
            .await
            .unwrap();

        // build mmr root
        let mmr_root = help::MmrRoot {
            signed_commitment: help::SignedCommitment::from(signed_commitment),
            validator_merkle_proofs,
            mmr_leaf: mmr_proof.mmr_leaf,
            mmr_leaf_proof: mmr_proof.mmr_leaf_proof,
        };
        // get client id from target chain
        let client_ids = get_client_ids(target_client.clone(), ClientType::Grandpa)
            .await
            .unwrap();
        for client_id in client_ids {
            let result =
                send_update_state_request(target_client.clone(), client_id, mmr_root.clone()).await;

            match result {
                Ok(r) => {
                    println!("update client state success and result is : {:?}", r);
                }
                Err(e) => {
                    println!("update client state client failure and error is : {:?}", e);
                }
            }
        }
    }
}

// verify commitment signatures,copy from beefy light client
// #[warn(unused_must_use)]
pub fn verify_commitment_signatures(
    commitment_hash: &Hash,
    signatures: &[Option<commitment::Signature>],
    validator_set_root: &Hash,
    validator_proofs: &[beefy_light_client::ValidatorMerkleProof],
    start_position: usize,
    interations: usize,
) -> core::result::Result<(), Error> {
    let msg =
        libsecp256k1::Message::parse_slice(&commitment_hash[..]).or(Err(Error::InvalidMessage))?;

    for signature in signatures
        .iter()
        .skip(start_position)
        .take(interations)
        .flatten()
    {
        let sig = libsecp256k1::Signature::parse_standard_slice(&signature.0[..64])
            .or(Err(Error::InvalidSignature))?;

        let recovery_id =
            libsecp256k1::RecoveryId::parse(signature.0[64]).or(Err(Error::InvalidRecoveryId))?;

        let validator = libsecp256k1::recover(&msg, &sig, &recovery_id)
            .or(Err(Error::WrongSignature))?
            .serialize()
            .to_vec();
        let validator_address = Keccak256::hash(&validator[1..])[12..].to_vec();

        let mut found = false;
        for proof in validator_proofs.iter() {
            if validator_address == *proof.leaf {
                found = true;
                if !verify_proof::<Keccak256, _, _>(
                    validator_set_root,
                    proof.proof.clone(),
                    proof.number_of_leaves,
                    proof.leaf_index,
                    &proof.leaf,
                ) {
                    return Err(Error::InvalidValidatorProof);
                }
                break;
            }
        }
        if !found {
            return Err(Error::ValidatorNotFound);
        }
    }

    Ok(())
}

/// get client ids store in chain
pub async fn get_client_ids(
    client: OnlineClient<MyConfig>,
    expect_client_type: ClientType,
) -> Result<Vec<ClientId>, subxt::error::Error> {

    let mut client_ids = vec![];

    let mut block = client.rpc().subscribe_finalized_blocks().await?;

    let block_header = block.next().await.unwrap().unwrap();

    let block_hash: H256 = block_header.hash();

    let address = ibc_node::storage().ibc().client_states_root();

    // Iterate over keys and values at that address.
    let mut iter = client
    .storage()
    .iter(address, 10, None)
    .await
    .unwrap();

    // prefix(32) + hash(data)(16) + data
    while let Some((key, value)) = iter.next().await? {
        let raw_key = key.0[48..].to_vec();
        let raw_key = Vec::<u8>::decode(&mut &*raw_key).map_err(|_| subxt::error::Error::Other("decode vec<u8> error".to_string()))?;
        let client_state_path = String::from_utf8(raw_key).map_err(|_| subxt::error::Error::Other("decode string error".to_string()))?;
        // decode key
        let path = Path::from_str(&client_state_path)
        .map_err(|_| subxt::error::Error::Other("decode path error".to_string()))?;

        let client_id = match path {
            Path::ClientState(ClientStatePath(client_id)) => client_id,
            _ => unimplemented!(),
        };

        let any_client_state = AnyClientState::decode_vec(&*value).unwrap();
        let client_type = any_client_state.client_type();
        if client_type == expect_client_type {
            client_ids.push(client_id)
        }
    }
    println!("client ids :  {:?}", client_ids);

    Ok(client_ids)
}
