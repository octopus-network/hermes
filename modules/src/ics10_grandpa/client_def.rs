use crate::ics02_client::client_def::{AnyClientState, AnyConsensusState, ClientDef};
use crate::ics02_client::header::Header as ICS2Header;
use crate::ics03_connection::connection::ConnectionEnd;
use crate::ics10_grandpa::client_state::ClientState;
use crate::ics10_grandpa::consensus_state::ConsensusState;
use crate::ics10_grandpa::header::Header;
use crate::ics10_grandpa::justification::GrandpaJustification;
use crate::ics10_grandpa::state_machine::read_proof_check;
use crate::ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes, CommitmentRoot};
use crate::ics24_host::identifier::ClientId;
use crate::ics24_host::identifier::ConnectionId;
use crate::Height;
use codec::Decode;
use finality_grandpa::voter_set::VoterSet;
use sp_finality_grandpa::{AuthorityList, VersionedAuthorityList, GRANDPA_AUTHORITIES_KEY};
use sp_runtime::{generic, traits::BlakeTwo256, OpaqueExtrinsic as UncheckedExtrinsic};
use tendermint::time::Time;

type BlockNumber = u32;
type Block = generic::Block<generic::Header<BlockNumber, BlakeTwo256>, UncheckedExtrinsic>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GRANDPAClient;

impl ClientDef for GRANDPAClient {
    type Header = Header;
    type ClientState = ClientState;
    type ConsensusState = ConsensusState;

    fn check_header_and_update_state(
        &self,
        client_state: Self::ClientState,
        header: Self::Header,
    ) -> Result<(Self::ClientState, Self::ConsensusState), Box<dyn std::error::Error>> {
        if client_state.latest_height() >= header.height() {
            return Err(
                format!("received header height ({:?}) is lower than (or equal to) client latest height ({:?})",
                    header.height(), client_state.latest_height).into(),
            );
        }
        println!(
            "header height: {:?}, hash: {:?}",
            header.height, header.block_hash
        );

        // TODO: Additional verifications should be implemented here.
        if let Some(justification) = header.justification {
            println!("with justification");
            let justification = GrandpaJustification::<Block>::decode(&mut &*justification);
            let authorities = VoterSet::new(client_state.authorities.iter().cloned());
            // ensure!(authorities.is_some(), "Invalid authorities set");
            let authorities = authorities.unwrap();
            println!("old authorities: {:?}", authorities);
            if let Ok(justification) = justification {
                let result = justification.verify(client_state.set_id, &authorities);
                if result.is_ok() {
                    println!("justification is_ok");
                    println!("target_hash: {:?}", justification.commit.target_hash);
                // assert_eq!(header.block_hash, justification.commit.target_hash);
                } else {
                    println!("justification is_err");
                }
            }
        }

        let result = read_proof_check::<BlakeTwo256>(
            header.commitment_root,
            header.authorities_proof,
            &GRANDPA_AUTHORITIES_KEY.to_vec(),
        );
        // TODO
        let result = result.unwrap().unwrap();
        let new_authorities: AuthorityList = VersionedAuthorityList::decode(&mut &*result)
            .unwrap()
            .into();
        println!("new authorities: {:?}", new_authorities);
        let new_consensus_state = ConsensusState {
            timestamp: Time::now(),
            root: header.commitment_root,
        };

        let new_client_state = ClientState {
            chain_id: client_state.chain_id,
            frozen_height: client_state.frozen_height,
            latest_height: header.height,
            set_id: 0,
            authorities: new_authorities,
        };

        Ok((new_client_state, new_consensus_state))
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
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn verify_connection_state(
        &self,
        _client_state: &Self::ClientState,
        _height: Height,
        _prefix: &CommitmentPrefix,
        _proof: &CommitmentProofBytes,
        _connection_id: &ConnectionId,
        _expected_connection_end: &ConnectionEnd,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
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
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
