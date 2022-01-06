use crate::alloc::string::ToString;
use core::convert::{TryFrom, TryInto};
use core::str::FromStr;
use alloc::vec::Vec;
use core::time::Duration;

// mock grandpa as tendermint
use ibc_proto::ibc::lightclients::grandpa::v1::ClientState as RawClientState;

use crate::ics02_client::client_state::AnyClientState;
use crate::ics02_client::client_type::ClientType;
use crate::ics10_grandpa::error::Error;
use crate::ics10_grandpa::header::Header;
use crate::ics24_host::identifier::ChainId;
use crate::Height;
use serde::{Deserialize, Serialize};
use tendermint_proto::Protobuf;
use super::help::Commitment;
use super::help::ValidatorSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientState {
    pub chain_id: ChainId,
    pub block_number: u32,
    pub frozen_height: Height,
    pub latest_commitment: Option<Commitment>,
    pub validator_set: Option<ValidatorSet>,
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            chain_id: Default::default(),
            block_number: 0,
            frozen_height: Default::default(),
            latest_commitment: Some(Commitment::default()),
            validator_set: Some(ValidatorSet::default()),
        }
    }
}

impl ClientState {
    pub fn new(
        chain_id: ChainId,
        block_number: u32,
        frozen_height: Height,
        latest_commitment: Option<Commitment>,
        validator_set: Option<ValidatorSet>,
    ) -> Result<Self, Error> {
       let client_state = ClientState {
            chain_id,
            block_number,
            frozen_height,
            latest_commitment,
            validator_set,
       };

       Ok(client_state)
    }

    pub fn with_header(self, h: Header) -> Self {
        // TODO: Clarify which fields should update.
        ClientState {
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
            latest_commitment: Some(raw.latest_commitment.unwrap().into()),
            validator_set: Some(raw.validator_set.unwrap().into()),
        })
    }
}

impl From<ClientState> for RawClientState {
    fn from(value: ClientState) -> Self {
        Self {
            chain_id: value.chain_id.to_string(),
            block_number: value.block_number,
            frozen_height: value.frozen_height.revision_height as u32,
            latest_commitment: Some(value.latest_commitment.unwrap().into()),
            validator_set: Some(value.validator_set.unwrap().into()),
        }
    }
}
