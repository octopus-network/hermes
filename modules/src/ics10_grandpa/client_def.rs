use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use core::convert::TryInto;
use codec::{Encode, Decode};
use tendermint_proto::Protobuf;

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
use crate::ics03_connection::context::ConnectionReader;
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
        // if client_state.latest_height() >= header.height() {
        //     return Err(Error::low_header_height(
        //         header.height(),
        //         client_state.latest_height(),
        //     ));
        // }

        // tracing::info!("in ics10 client_def [check_header_and_update_state] >> header = {:?}", header);
        // // destruct header
        // let Header {
        //     block_header,
        //     mmr_leaf,
        //     mmr_leaf_proof,
        // } = header;
        //
        //
        // if client_state.latest_commitment.is_none() {
        //     let new_client_state = ClientState {
        //         chain_id: client_state.chain_id,
        //         block_number: signed_commitment.clone().commitment.unwrap().block_number,
        //         frozen_height: client_state.frozen_height,
        //         latest_commitment: signed_commitment.clone().commitment,
        //         validator_set: mmr_leaf.beefy_next_authority_set
        //     };
        //
        //     let new_consensus_state = ConsensusState::from_commit(signed_commitment.commitment.unwrap());
        //
        //     return Ok((new_client_state, new_consensus_state));
        // }
        //
        // let mut beefy_light_client = beefy_light_client::LightClient {
        //     latest_commitment: Some(client_state.latest_commitment.unwrap().into()),
        //     validator_set: client_state.validator_set.unwrap().into(),
        //     in_process_state: None
        // };
        //
        // let encode_signed_commitment = signed_commitment.encode();
        // let validator_proofs = vec![validator_merkle_proof.into()];
        // let encode_mmr_leaf = mmr_leaf.encode();
        // let encode_mmr_leaf_proof = mmr_leaf_proof.encode();
        //
        // beefy_light_client.update_state(&encode_signed_commitment, &validator_proofs, &encode_mmr_leaf,&encode_mmr_leaf_proof);
        //
        // tracing::info!("in ics10 client_def [check_header_and_update_state] >> beefy_light_client = {:?}", beefy_light_client);
        //
        //
        // let new_client_state = ClientState {
        //     chain_id: client_state.chain_id,
        //     // TODO Need later to fix
        //     // block_number: beefy_light_client.latest_commitment.as_ref().unwrap().block_number,
        //     block_number: signed_commitment.commitment.as_ref().unwrap().block_number,
        //     frozen_height: client_state.frozen_height,
        //     latest_commitment: Some(beefy_light_client.latest_commitment.clone().unwrap().into()),
        //     validator_set: Some(beefy_light_client.validator_set.into()),
        // };
        //
        // let new_consensus_state = ConsensusState::from_commit(beefy_light_client.latest_commitment.unwrap().into());
        //
        // tracing::info!("in ics10 client_def [check_header_and_update_state] >> client_state = {:?}", new_client_state);
        // tracing::info!("in ics10 client_def [check_header_and_update_state] >> new_consensus_state = {:?}", new_consensus_state);

        // Ok((new_client_state ,new_consensus_state))
        Ok((client_state.with_header(header.clone()), ConsensusState::from(header)))
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
        _ctx: Option<&dyn ConnectionReader>,
    ) -> Result<(), Error> {
        let storage_keys = _ctx.unwrap().connection_storage_key(_connection_id.unwrap()).unwrap();
        let storage_result = Self::get_storage_via_proof(_client_state, _height, _proof, storage_keys).unwrap();
        let connection_end = ConnectionEnd::decode(&mut &*storage_result).unwrap();
        tracing::info!("In ics10-client_def.rs: [verify_connection_state] >> connection_end: {:?}", connection_end);

        if !(connection_end.encode_vec().unwrap() == _expected_connection_end.encode_vec().unwrap()) {
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
    fn get_storage_via_proof(_client_state: &ClientState, _height: Height, _proof: &CommitmentProofBytes, storage_keys: Vec<u8>)
                             -> Result<Vec<u8>, Error>
    {
        use sp_runtime::traits::BlakeTwo256;
        use sp_trie::StorageProof;
        use crate::ics10_grandpa::state_machine::read_proof_check;
        use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
        use ibc_proto::ics23::commitment_proof::Proof::Exist;
        use core::convert::TryFrom;

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
                tracing::info!("In ics10-client_def.rs: [extract_verify_beefy_proof] >> _proof_str: {:?}", _proof_str);
                let _storage_proof: ReadProofU8 = serde_json::from_str(&_proof_str).unwrap();
                tracing::info!("In ics10-client_def.rs: [extract_verify_beefy_proof] >> leaf_proof: {:?}", _storage_proof);
                _storage_proof
            }
            _ => unimplemented!()
        };

        tracing::info!("In ics10-client_def.rs: [extract_verify_beefy_proof] >> storage_keys: {:?}", storage_keys);
        let state_root = _client_state.clone().block_header.state_root;
        tracing::info!("In ics10-client_def.rs: [extract_verify_beefy_proof] >> storage_root: {:?}", state_root);
        let state_root_ = vector_to_array::<u8, 32>(state_root);
        tracing::info!("In ics10-client_def.rs: [extract_verify_beefy_proof] >> storage_root: {:?}", state_root_);

        let storage_result = read_proof_check::<BlakeTwo256>(
            sp_core::H256::from(state_root_),
            StorageProof::new(storage_proof.proof),
            &storage_keys,
        ).unwrap().unwrap();

        tracing::info!("In ics10-client_def.rs: [verify_storage_proof] >> storage_result: {:?}", storage_result);
        let connection_end = ConnectionEnd::decode(&mut &*storage_result).unwrap();
        tracing::info!("In ics10-client_def.rs: [verify_storage_proof] >> connection_end: {:?}", connection_end);

        Ok(connection_end.encode_vec().unwrap())
    }
}

fn vector_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}