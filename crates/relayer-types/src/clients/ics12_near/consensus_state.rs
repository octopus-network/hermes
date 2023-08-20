use crate::clients::ics12_near::header::Header;
use crate::clients::ics12_near::near_types::signature::PublicKey;
use crate::clients::ics12_near::near_types::{hash::CryptoHash, LightClientBlock};
use crate::clients::ics12_near::near_types::{AccountId, Balance};
use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::error::Error;
use crate::core::ics23_commitment::commitment::CommitmentRoot;
use crate::timestamp::Timestamp;
use borsh::{BorshDeserialize, BorshSerialize};
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::near::v1::ConsensusState as RawConsensusState;
use ibc_proto::ibc::lightclients::near::v1::{
    CryptoHash as RawCryptoHash, Header as RawHeader, ValidatorStakeView as RawValidatorStakeView,
};
use ibc_proto::protobuf::Protobuf;
use prost::Message;
use serde::{Deserialize, Serialize};

pub const NEAR_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.lightclients.near.v1.ConsensusState";

/// The consensus state of NEAR light client.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsensusState {
    /// Block producers of current epoch
    pub current_bps: Vec<ValidatorStakeView>,
    /// Header data
    pub header: Header,

    /// todo
    pub commitment_root: CommitmentRoot,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct ValidatorStakeViewV1 {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub stake: Balance,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub enum ValidatorStakeView {
    V1(ValidatorStakeViewV1),
}

impl crate::core::ics02_client::consensus_state::ConsensusState for ConsensusState {
    fn client_type(&self) -> ClientType {
        ClientType::Near
    }

    fn root(&self) -> &CommitmentRoot {
        &self.commitment_root
    }

    fn timestamp(&self) -> Timestamp {
        Timestamp::from_nanoseconds(self.header.light_client_block.inner_lite.timestamp)
            .expect("failed to create Timestamp")
    }
}

impl Protobuf<RawConsensusState> for ConsensusState {}

impl TryFrom<RawConsensusState> for ConsensusState {
    type Error = Error;

    fn try_from(raw: RawConsensusState) -> Result<Self, Self::Error> {
        let h = raw.header.ok_or(Error::custom_error(
            "Consensus State header is empty".into(),
        ))?;
        Ok(Self {
            current_bps: raw
                .current_bps
                .iter()
                .map(|bps| ValidatorStakeView::try_from_slice(&bps.raw_data))
                .collect::<Result<Vec<ValidatorStakeView>, _>>()
                .map_err(|e| Error::custom_error(e.to_string()))?,
            header: Header {
                light_client_block: LightClientBlock::try_from_slice(&h.light_client_block)
                    .map_err(|e| Error::custom_error(e.to_string()))?,
                prev_state_root_of_chunks: h
                    .prev_state_root_of_chunks
                    .iter()
                    .map(|c| CryptoHash::try_from(&c.raw_data[..]))
                    .collect::<Result<Vec<CryptoHash>, _>>()
                    .map_err(|e| Error::custom_error(e.to_string()))?,
            },
            commitment_root: CommitmentRoot::from(vec![]),
        })
    }
}

impl From<ConsensusState> for RawConsensusState {
    fn from(value: ConsensusState) -> Self {
        RawConsensusState {
            current_bps: value
                .current_bps
                .iter()
                .map(|bps| RawValidatorStakeView {
                    raw_data: bps
                        .try_to_vec()
                        .expect("failed serialize to RawValidatorStakeView"),
                })
                .collect(),
            header: Some(RawHeader {
                light_client_block: value
                    .header
                    .light_client_block
                    .try_to_vec()
                    .expect("failed serialize to light client block"),
                prev_state_root_of_chunks: value
                    .header
                    .prev_state_root_of_chunks
                    .iter()
                    .map(|c| RawCryptoHash {
                        raw_data: c.0.to_vec(),
                    })
                    .collect(),
            }),
        }
    }
}

impl Protobuf<Any> for ConsensusState {}

impl TryFrom<Any> for ConsensusState {
    type Error = Error;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use bytes::Buf;
        use core::ops::Deref;

        fn decode_consensus_state<B: Buf>(buf: B) -> Result<ConsensusState, Error> {
            RawConsensusState::decode(buf)
                .map_err(Error::decode)?
                .try_into()
        }

        match raw.type_url.as_str() {
            NEAR_CONSENSUS_STATE_TYPE_URL => {
                decode_consensus_state(raw.value.deref()).map_err(Into::into)
            }
            _ => Err(Error::unknown_consensus_state_type(raw.type_url)),
        }
    }
}

impl From<ConsensusState> for Any {
    fn from(consensus_state: ConsensusState) -> Self {
        Any {
            type_url: NEAR_CONSENSUS_STATE_TYPE_URL.to_string(),
            value: Protobuf::<RawConsensusState>::encode_vec(&consensus_state),
        }
    }
}
