use crate::alloc::string::ToString;
use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};
use core::str::FromStr;
use core::time::Duration;

// mock grandpa as tendermint
use ibc_proto::ibc::lightclients::grandpa::v1::ClientState as RawClientState;

use super::help::Commitment;
use super::help::ValidatorSet;
use super::help::BlockHeader;

use crate::ics02_client::client_state::AnyClientState;
use crate::ics02_client::client_type::ClientType;
use crate::ics10_grandpa::error::Error;
use crate::ics10_grandpa::header::Header;
use crate::ics24_host::identifier::ChainId;
use crate::Height;
use serde::{Deserialize, Serialize};
use tendermint_proto::Protobuf;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientState {
    pub chain_id: ChainId,
    /// block_number is height?
    pub block_number: u32,
    /// Block height when the client was frozen due to a misbehaviour
    pub frozen_height: Height,
    pub block_header: BlockHeader,
    pub latest_commitment: Commitment,
    pub validator_set: ValidatorSet,
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            chain_id: ChainId::default(),
            block_number: u32::default(),
            frozen_height: Height::default(),
            block_header: BlockHeader::default(),
            latest_commitment: Commitment::default(),
            validator_set: ValidatorSet::default(),
        }
    }
}

impl ClientState {
    pub fn new(
        chain_id: ChainId,
        block_number: u32,
        frozen_height: Height,
        block_header: BlockHeader,
        latest_commitment: Commitment,
        validator_set: ValidatorSet,
    ) -> Result<Self, Error> {
        let client_state = ClientState {
            chain_id,
            block_number,
            frozen_height,
            block_header,
            latest_commitment,
            validator_set,
        };

        Ok(client_state)
    }

    pub fn with_header(self, h: Header) -> Self {
        // TODO: Clarify which fields should update.
        ClientState {
            block_number: h.height().revision_number as u32,
            ..self
        }
    }

    /// Get the refresh time to ensure the state does not expire
    pub fn refresh_time(&self) -> Option<Duration> {
        //TODO
        Some(Duration::new(3, 0))
    }

    /// Check if the state is expired when `elapsed` time has passed since the latest consensus
    /// state timestamp
    pub fn expired(&self, elapsed: Duration) -> bool {
        //TODO
        false
        // elapsed > self.trusting_period
    }

    pub fn latest_height(&self) -> Height {
        Height::new(0, self.block_number as u64)
    }
}

impl Protobuf<RawClientState> for ClientState {}

impl crate::ics02_client::client_state::ClientState for ClientState {
    fn chain_id(&self) -> ChainId {
        self.chain_id.clone()
    }

    fn client_type(&self) -> ClientType {
        ClientType::Grandpa
    }

    fn latest_height(&self) -> Height {
        Height::new(0, self.block_number as u64)
    }

    fn is_frozen(&self) -> bool {
        // If 'frozen_height' is set to a non-zero value, then the client state is frozen.
        !self.frozen_height.is_zero()
    }

    fn wrap_any(self) -> AnyClientState {
        AnyClientState::Grandpa(self)
    }
}

impl TryFrom<RawClientState> for ClientState {
    type Error = Error;

    fn try_from(raw: RawClientState) -> Result<Self, Self::Error> {
        Ok(Self {
            chain_id: ChainId::from_str(raw.chain_id.as_str()).unwrap(),
            block_number: raw.block_number,
            frozen_height: Height::new(0, raw.frozen_height as u64),
            block_header: raw.block_header.unwrap().into(),
            latest_commitment: raw.latest_commitment.unwrap().into(),
            validator_set: raw.validator_set.unwrap().into(),
        })
    }
}

impl From<ClientState> for RawClientState {
    fn from(value: ClientState) -> Self {
        Self {
            chain_id: value.chain_id.to_string(),
            block_number: value.block_number,
            frozen_height: value.frozen_height.revision_height as u32,
            block_header: Some(value.block_header.into()),
            latest_commitment: Some(value.latest_commitment.into()),
            validator_set: Some(value.validator_set.into()),
        }
    }
}
