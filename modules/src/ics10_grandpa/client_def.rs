use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use codec::{Decode, Encode};
use core::convert::From;
use core::convert::TryInto;
use tendermint_proto::Protobuf;

use ibc_proto::ibc::core::commitment::v1::MerkleProof;

use crate::ics02_client::client_consensus::AnyConsensusState;
use crate::ics02_client::client_def::ClientDef;
use crate::ics02_client::client_state::AnyClientState;
use crate::ics02_client::error::Error;
use crate::ics03_connection::connection::ConnectionEnd;
use crate::ics03_connection::context::ConnectionReader;
use crate::ics04_channel::channel::ChannelEnd;
use crate::ics04_channel::packet::Sequence;
use crate::ics10_grandpa::client_state::ClientState;
use crate::ics10_grandpa::consensus_state::ConsensusState;
use crate::ics10_grandpa::header::Header;
use crate::ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot};
use crate::ics24_host::identifier::ConnectionId;
use crate::ics24_host::identifier::{ChannelId, ClientId, PortId};
use crate::Height;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrandpaClient;

impl ClientDef for GrandpaClient {
    type Header = Header;
    type ClientState = ClientState;
    type ConsensusState = ConsensusState;

    fn check_header_and_update_state(
        &self,
        client_state: Self::ClientState,
        header: Self::Header,
    ) -> Result<(Self::ClientState, Self::ConsensusState), Error> {
        tracing::info!("in ics10 client_def[check_header_and_update_state]");

        // if client_state.latest_height() >= header.height() {
        //     return Err(Error::low_header_height(
        //         header.height(),
        //         client_state.latest_height(),
        //     ));
        // }

        if client_state.latest_commitment.payload.is_empty() {
            return Err(Error::empty_mmr_root());
        }

        let mut mmr_root = [0u8; 32];
        mmr_root.copy_from_slice(&client_state.latest_commitment.payload);

        let mmr_proof = header.clone().mmr_leaf_proof;

        let mut items = vec![];
        for item in mmr_proof.items {
            let mut temp = [0u8; 32];
            if item.is_empty() {
                return Err(Error::empry_mmr_leaf_proof_items());
            }
            temp.copy_from_slice(item.as_slice());
            items.push(temp);
        }

        // convert beefy_light_client MmrLeafProof
        let mmr_proof = beefy_light_client::mmr::MmrLeafProof {
            leaf_index: mmr_proof.leaf_index,
            leaf_count: mmr_proof.leaf_count,
            items,
        };

        let mmt_lead_encode = header.clone().mmr_leaf.encode();
        let mmr_leaf_hash = beefy_merkle_tree::Keccak256::hash(&mmt_lead_encode[..]);
        let mmr_leaf = header.clone().mmr_leaf;

        let header_hash = header.hash();
        let mut parent_mmr_root = [0u8; 32];
        if mmr_leaf.parent_number_and_hash.mmr_root.is_empty() {
            return Err(Error::empty_mmr_leaf_parent_hash_mmr_root());
        }

        parent_mmr_root.copy_from_slice(mmr_leaf.parent_number_and_hash.mmr_root.as_slice());

        // TODO fix header hash not match
        // if header_hash != parent_mmr_root {
        //     tracing::info!("ics1 client_def :[check_header_and_update_state] >> header_hash = {:?}", header_hash);
        //     tracing::info!("ics1 client_def :[check_header_and_update_state] >> parent_mmr_root = {:?}", parent_mmr_root);
        //     return Err(Error::header_hash_not_match());
        // }

        // let result = beefy_light_client::mmr::verify_leaf_proof(mmr_root, mmr_leaf_hash, mmr_proof)
        //     .map_err(|_| Error::invalid_mmr_leaf_proof())?;
        //
        // if !result {
        //     return Err(Error::invalid_mmr_leaf_proof());
        // }

        let client_state = ClientState {
            block_header: header.clone().block_header,
            block_number: header.clone().block_header.block_number,
            ..client_state
        };

        tracing::info!(
            "in client_def: [check_header_and_update_state] >> client_state = {:?}",
            client_state
        );
        tracing::info!(
            "in client_def: [check_header_and_update_state] >> consensus_state = {:?}",
            ConsensusState::from(header.clone())
        );

        Ok((client_state, ConsensusState::from(header.clone())))
    }

    fn verify_client_consensus_state(
        &self,
        _client_state: &Self::ClientState,
        _height: Height,
        _prefix: &CommitmentPrefix,
        _proof: &CommitmentProofBytes,
        _client_id: &ClientId,
        _consensus_height: Height,
        _expected_consensus_state: &AnyConsensusState,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn verify_connection_state(
        &self,
        _client_state: &Self::ClientState,
        _height: Height,
        _prefix: &CommitmentPrefix,
        _proof: &CommitmentProofBytes,
        _connection_id: Option<&ConnectionId>,
        _expected_connection_end: &ConnectionEnd,
    ) -> Result<(), Error> {

        let key_encoded: &[u8] = &_connection_id.unwrap().as_bytes().encode();
        let storage_result = Self::get_storage_via_proof(_client_state, _height, _proof, key_encoded).unwrap();
        let connection_end = ConnectionEnd::decode(&mut &*storage_result).unwrap();
        tracing::info!(
            "In ics10-client_def.rs: [verify_connection_state] >> connection_end: {:?}",
            connection_end
        );

        if !(connection_end.encode_vec().unwrap() == _expected_connection_end.encode_vec().unwrap())
        {
            return Err(Error::invalid_connection_state());
        }
        Ok(())
    }

    fn verify_channel_state(
        &self,
        _client_state: &Self::ClientState,
        _height: Height,
        _prefix: &CommitmentPrefix,
        _proof: &CommitmentProofBytes,
        _port_id: &PortId,
        _channel_id: &ChannelId,
        _expected_channel_end: &ChannelEnd,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn verify_client_full_state(
        &self,
        _client_state: &Self::ClientState,
        _height: Height,
        _root: &CommitmentRoot,
        _prefix: &CommitmentPrefix,
        _client_id: &ClientId,
        _proof: &CommitmentProofBytes,
        _expected_client_state: &AnyClientState,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn verify_packet_data(
        &self,
        _client_state: &Self::ClientState,
        _height: Height,
        _proof: &CommitmentProofBytes,
        _port_id: &PortId,
        _channel_id: &ChannelId,
        _seq: &Sequence,
        _data: String,
    ) -> Result<(), Error> {
        Ok(()) // Todo:
    }

    fn verify_packet_acknowledgement(
        &self,
        _client_state: &Self::ClientState,
        _height: Height,
        _proof: &CommitmentProofBytes,
        _port_id: &PortId,
        _channel_id: &ChannelId,
        _seq: &Sequence,
        _data: Vec<u8>,
    ) -> Result<(), Error> {
        Ok(()) // todo!()
    }

    fn verify_next_sequence_recv(
        &self,
        _client_state: &Self::ClientState,
        _height: Height,
        _proof: &CommitmentProofBytes,
        _port_id: &PortId,
        _channel_id: &ChannelId,
        _seq: &Sequence,
    ) -> Result<(), Error> {
        Ok(()) // todo!()
    }

    fn verify_packet_receipt_absence(
        &self,
        _client_state: &Self::ClientState,
        _height: Height,
        _proof: &CommitmentProofBytes,
        _port_id: &PortId,
        _channel_id: &ChannelId,
        _seq: &Sequence,
    ) -> Result<(), Error> {
        Ok(()) // todo:
    }

    fn verify_upgrade_and_update_state(
        &self,
        client_state: &Self::ClientState,
        consensus_state: &Self::ConsensusState,
        _proof_upgrade_client: MerkleProof,
        _proof_upgrade_consensus_state: MerkleProof,
    ) -> Result<(Self::ClientState, Self::ConsensusState), Error> {
        // TODO
        Ok((client_state.clone(), consensus_state.clone()))
    }
}

impl GrandpaClient {
    /// Extract on-chain storage value by proof, path, and state root
    fn get_storage_via_proof(_client_state: &ClientState, _height: Height, _proof: &CommitmentProofBytes, _key_encoded: &[u8])
                             -> Result<Vec<u8>, Error>
    {
        tracing::info!("In ics10-client_def.rs: [extract_verify_beefy_proof] >> _client_state: {:?}, _height: {:?}, _key_encoded: {:?}", _client_state, _height, _key_encoded);
        use sp_runtime::traits::BlakeTwo256;
        use sp_trie::StorageProof;

        use crate::ics10_grandpa::state_machine::read_proof_check;
        use core::convert::TryFrom;
        use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
        use ibc_proto::ics23::commitment_proof::Proof::Exist;
        use sp_runtime::traits::BlakeTwo256;
        use sp_trie::StorageProof;

        let _storage_keys = Self::storagge_map_final_key(_key_encoded);
        /*        while _client_state.block_number < (_height.revision_height as u32) {
            let sleep_duration = Duration::from_micros(500);
            // wasm_timer::sleep(sleep_duration);
        }*/
        use serde::{Deserialize, Serialize};
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct ReadProofU8 {
            pub at: String,
            pub proof: Vec<Vec<u8>>,
        }

        let merkel_proof = RawMerkleProof::try_from(_proof.clone()).unwrap();
        let _merkel_proof = merkel_proof.proofs[0].proof.clone().unwrap();
        let storage_proof = match _merkel_proof {
            Exist(_exist_proof) => {
                let _proof_str = String::from_utf8(_exist_proof.value).unwrap();
                tracing::info!(
                    "In ics10-client_def.rs: [extract_verify_beefy_proof] >> _proof_str: {:?}",
                    _proof_str
                );
                let _storage_proof: ReadProofU8 = serde_json::from_str(&_proof_str).unwrap();
                tracing::info!("In ics10-client_def.rs: [extract_verify_beefy_proof] >> _storage_proof: {:?}", _storage_proof);
                _storage_proof
            }
            _ => unimplemented!(),
        };

        tracing::info!("In ics10-client_def.rs: [extract_verify_beefy_proof] >> storage_keys: {:?}", _storage_keys);
        let state_root = _client_state.clone().block_header.state_root;
        tracing::info!(
            "In ics10-client_def.rs: [extract_verify_beefy_proof] >> storage_root: {:?}",
            state_root
        );
        let state_root_ = vector_to_array::<u8, 32>(state_root);

        tracing::info!("In ics10-client_def.rs: [extract_verify_beefy_proof] >> storage_root_: {:?}", state_root_);

        let storage_result = read_proof_check::<BlakeTwo256>(
            sp_core::H256::from(state_root_),
            StorageProof::new(storage_proof.proof),
            &_storage_keys,
        ).unwrap().unwrap();

        tracing::info!("In ics10-client_def.rs: [verify_storage_proof] >> storage_result: {:?}", storage_result);

        let connection_end = ConnectionEnd::decode(&mut &*storage_result).unwrap();
        tracing::info!(
            "In ics10-client_def.rs: [verify_storage_proof] >> connection_end: {:?}",
            connection_end
        );

        Ok(connection_end.encode_vec().unwrap())
    }

    /// Migrate from substrate: https://github.com/paritytech/substrate/blob/32b71896df8a832e7c139a842e46710e4d3f70cd/frame/support/src/storage/generator/map.rs?_pjax=%23js-repo-pjax-container%2C%20div%5Bitemtype%3D%22http%3A%2F%2Fschema.org%2FSoftwareSourceCode%22%5D%20main%2C%20%5Bdata-pjax-container%5D#L66
    fn storagge_map_final_key(_key_encoded: &[u8]) -> Vec<u8> {
        use frame_support::{Blake2_128Concat, StorageHasher};
        use frame_support::storage::storage_prefix;

        let key_hashed: &[u8] = &Blake2_128Concat::hash(_key_encoded);
        let storage_prefix = storage_prefix("Babe".as_bytes(), "NextEpochConfig".as_bytes());
        let mut final_key = Vec::with_capacity(storage_prefix.len() + key_hashed.as_ref().len());
        final_key.extend_from_slice(&storage_prefix);
        final_key.extend_from_slice(key_hashed.as_ref());
        final_key
    }
}

fn vector_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
