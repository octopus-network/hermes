use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use core::convert::TryInto;
use codec::{Encode, Decode};
use core::convert::From;

use ibc_proto::ibc::core::commitment::v1::MerkleProof;

use crate::ics02_client::client_consensus::AnyConsensusState;
use crate::ics02_client::client_def::ClientDef;
use crate::ics02_client::client_state::AnyClientState;
use crate::ics02_client::error::Error;
use crate::ics03_connection::connection::ConnectionEnd;
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
            return Err(Error::empty_mmr_root())
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

        tracing::info!("in client_def: [check_header_and_update_state] >> client_state = {:?}", client_state);
        tracing::info!("in client_def: [check_header_and_update_state] >> consensus_state = {:?}", ConsensusState::from(header.clone()));

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

        // Self::extract_verify_beefy_proof(_client_state, _height, _proof)
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
        // Self::extract_verify_beefy_proof(_client_state, _height, _proof)
        use core::time::Duration;
        // use sp_core::{storage::StorageKey, Bytes};
        use  alloc::vec;
        // use subxt::sp_core::H256;

/*        while _client_state.block_number < (_height.revision_height as u32) {
            let sleep_duration = Duration::from_micros(500);
            // wasm_timer::sleep(sleep_duration);
        }*/

        use serde::{Deserialize, Serialize};
        // #[derive(Debug, PartialEq, Serialize, Deserialize)]
        // #[serde(rename_all = "camelCase")]
        // pub struct ReadProof_ {
        //     pub at: String,
        //     pub proof: Vec<Vec<u8>>,
        // }
        //
        // use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
        // use core::convert::TryFrom;
        // use ibc_proto::ics23::commitment_proof::Proof::Exist;
        // use beefy_merkle_tree::Keccak256;
        // use codec::Decode;
        //
        // // The latest height was increased here: https://github.com/octopus-network/ibc-rs/blob/b98094a57620d0b3d9f8d2caced09abfc14ab00f/relayer/src/chain.rs?_pjax=%23js-repo-pjax-container%2C%20div%5Bitemtype%3D%22http%3A%2F%2Fschema.org%2FSoftwareSourceCode%22%5D%20main%2C%20%5Bdata-pjax-container%5D#L438
        // // Call decrement() to restore the latest height
        // let _height = _height.decrement();
        //
        // let merkel_proof = RawMerkleProof::try_from(_proof.clone()).unwrap();
        // let _merkel_proof = merkel_proof.proofs[0].proof.clone().unwrap();
        // let leaf_proof = match _merkel_proof {
        //     Exist(_exist_proof) => {
        //         let _proof_str = String::from_utf8(_exist_proof.value).unwrap();
        //         // tracing::info!("In ics10-client_def.rs: [verify_connection_state] >> _proof_str: {:?}", _proof_str);
        //         let leaf_proof: String = serde_json::from_str(&_proof_str).unwrap();
        //         tracing::info!("In ics10-client_def.rs: [verify_connection_state] >> leaf_proof: {:?}", leaf_proof);
        //         leaf_proof
        //     }
        //     _ => unimplemented!()
        // };
        //
        // let storage_key = (vec![_connection_id.unwrap().as_bytes().to_vec()]).iter();

/*        tracing::info!("In ics10-client_def.rs: [verify_connection_state] >> _client_state: {:?}", _client_state);
        let mmr_root: [u8; 32] = _client_state.
            latest_commitment.as_ref().unwrap().payload.as_slice().try_into().map_err(|_| Error::cant_decode_mmr_root())?;
        tracing::info!("In ics10-client_def.rs: [verify_connection_state] >> mmr_root: {:?}", mmr_root);

        let mmr_leaf: Vec<u8> =
            Decode::decode(&mut &leaf_proof.leaf[..]).map_err(|_| Error::cant_decode_mmr_leaf())?;
        tracing::info!("In ics10-client_def.rs: [verify_connection_state] >> mmr_leaf: {:?}", mmr_leaf);
        let mmr_leaf_hash = Keccak256::hash(&mmr_leaf[..]);
        tracing::info!("In ics10-client_def.rs: [verify_connection_state] >> mmr_leaf_hash: {:?}", mmr_leaf_hash);

        let mmr_leaf_proof = leaf_proof.proof;
        let mmr_proof = beefy_light_client::mmr::MmrLeafProof::decode(&mut &mmr_leaf_proof[..])
            .map_err(|_| Error::cant_decode_mmr_proof())?;
        tracing::info!("In ics10-client_def.rs: [verify_connection_state] >> mmr_proof: {:?}", mmr_proof);

        let result = beefy_light_client::mmr::verify_leaf_proof(mmr_root, mmr_leaf_hash, mmr_proof).unwrap();
        if !result {
            return Err(Error::failed_to_verify_mmr_proof());
        }*/

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
        // Self::extract_verify_beefy_proof(_client_state, _height, _proof)
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
    /// Extract `LeafProof_` and verify its validity
    fn extract_verify_beefy_proof(_client_state: &ClientState, _height: Height, _proof: &CommitmentProofBytes) -> Result<(), Error> {
        use core::time::Duration;
        // use sp_core::{storage::StorageKey, Bytes};
        // use subxt::sp_core::H256;

        while _client_state.block_number < (_height.revision_height as u32) {
            let sleep_duration = Duration::from_micros(500);
            // wasm_timer::sleep(sleep_duration);
        }

/*        use serde::{Deserialize, Serialize};
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct ReadProof_ {
            pub at: String,
            pub proof: Vec<Bytes>,
        }*/

        use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
        use core::convert::TryFrom;
        use ibc_proto::ics23::commitment_proof::Proof::Exist;
        use beefy_merkle_tree::Keccak256;
        use codec::Decode;

        use serde::{Deserialize, Serialize};
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
        #[serde(rename_all = "camelCase")]
        pub struct LeafProof_ {
            pub block_hash: String,
            pub leaf: Vec<u8>,
            pub proof: Vec<u8>,
        }

        // The latest height was increased here: https://github.com/octopus-network/ibc-rs/blob/b98094a57620d0b3d9f8d2caced09abfc14ab00f/relayer/src/chain.rs?_pjax=%23js-repo-pjax-container%2C%20div%5Bitemtype%3D%22http%3A%2F%2Fschema.org%2FSoftwareSourceCode%22%5D%20main%2C%20%5Bdata-pjax-container%5D#L438
        // Call decrement() to restore the latest height
        let _height = _height.decrement();
        let merkel_proof = RawMerkleProof::try_from(_proof.clone()).unwrap();
        let _merkel_proof = merkel_proof.proofs[0].proof.clone().unwrap();
        let leaf_proof = match _merkel_proof {
            Exist(_exist_proof) => {
                let _proof_str = String::from_utf8(_exist_proof.value).unwrap();
                // tracing::info!("In ics10-client_def.rs: [verify_connection_state] >> _proof_str: {:?}", _proof_str);
                let leaf_proof: LeafProof_ = serde_json::from_str(&_proof_str).unwrap();
                tracing::info!("In ics10-client_def.rs: [verify_connection_state] >> leaf_proof: {:?}", leaf_proof);
                leaf_proof
            }
            _ => unimplemented!()
        };

        tracing::info!("In ics10-client_def.rs: [verify_connection_state] >> _client_state: {:?}", _client_state);
        let mmr_root: [u8; 32] = _client_state.
            latest_commitment.payload.as_slice().try_into().map_err(|_| Error::cant_decode_mmr_root())?;
        tracing::info!("In ics10-client_def.rs: [verify_connection_state] >> mmr_root: {:?}", mmr_root);

        let mmr_leaf: Vec<u8> =
            Decode::decode(&mut &leaf_proof.leaf[..]).map_err(|_| Error::cant_decode_mmr_leaf())?;
        tracing::info!("In ics10-client_def.rs: [verify_connection_state] >> mmr_leaf: {:?}", mmr_leaf);
        let mmr_leaf_hash = Keccak256::hash(&mmr_leaf[..]);
        tracing::info!("In ics10-client_def.rs: [verify_connection_state] >> mmr_leaf_hash: {:?}", mmr_leaf_hash);

        let mmr_leaf_proof = leaf_proof.proof;
        let mmr_proof = beefy_light_client::mmr::MmrLeafProof::decode(&mut &mmr_leaf_proof[..])
            .map_err(|_| Error::cant_decode_mmr_proof())?;
        tracing::info!("In ics10-client_def.rs: [verify_connection_state] >> mmr_proof: {:?}", mmr_proof);

        let result = beefy_light_client::mmr::verify_leaf_proof(mmr_root, mmr_leaf_hash, mmr_proof).unwrap();
        if !result {
            return Err(Error::failed_to_verify_mmr_proof());
        }

        Ok(())
    }
}
