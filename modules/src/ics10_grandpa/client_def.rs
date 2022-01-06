use alloc::string::String;
use alloc::vec::Vec;
use core::convert::TryInto;

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
        // if client_state.latest_height() >= header.height() {
        //     return Err(Error::low_header_height(
        //         header.height(),
        //         client_state.latest_height(),
        //     ));
        // }

        Ok((
            client_state.with_header(header.clone()),
            ConsensusState::from(header),
        ))
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
        use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
        use core::convert::TryFrom;
        use ibc_proto::ics23::commitment_proof::Proof::Exist;
        use beefy_merkle_tree::Keccak256;
        use codec::Decode;
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
        pub struct LeafProof {
            /// Block hash the proof was generated for.
            pub block_hash: String,
            /// SCALE-encoded leaf data.
            pub leaf: sp_core::Bytes,
            /// SCALE-encoded proof data. See [pallet_mmr_primitives::Proof].
            pub proof: sp_core::Bytes,
        }

        let merkel_proof = RawMerkleProof::try_from(_proof.clone()).unwrap();
        let _merkel_proof = merkel_proof.proofs[0].proof.clone().unwrap();
        let leaf_proof = match _merkel_proof {
            Exist(_exist_proof) => {
                let _proof_str = String::from_utf8_lossy(&*_exist_proof.value);
                tracing::debug!("In ics10-client_def.rs: [verify_connection_state] >> _proof_str: {:?}", _proof_str);
                let leaf_proof: LeafProof = serde_json::from_str(&*_proof_str).unwrap();
                leaf_proof
            }
            _ => unimplemented!()
        };

        let mmr_root: [u8; 32] = _client_state.
            latest_commitment.as_ref().unwrap().payload.as_slice().try_into().map_err(|_| Error::cant_decode_mmr_proof())?;

        let mmr_leaf: Vec<u8> =
            Decode::decode(&mut &leaf_proof.leaf[..]).map_err(|_| Error::cant_decode_mmr_proof())?;
        let mmr_leaf_hash = Keccak256::hash(&mmr_leaf[..]);

        let mmr_leaf_proof = leaf_proof.proof;
        let mmr_proof = beefy_light_client::mmr::MmrLeafProof::decode(&mut &mmr_leaf_proof[..])
            .map_err(|_| Error::cant_decode_mmr_proof())?;

        beefy_light_client::mmr::verify_leaf_proof(mmr_root, mmr_leaf_hash, mmr_proof);

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
