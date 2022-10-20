use core::convert::{TryFrom, TryInto};
use core::str::FromStr;
use core::time::Duration;

use prost::Message;
use serde::{Deserialize, Serialize};

use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::Height as RawHeight;
use ibc_proto::ibc::lightclients::grandpa::v1::ClientState as RawGpClientState;

//use super::help::BlockHeader;
use super::help::Commitment;
use super::help::ValidatorSet;

use crate::clients::ics10_grandpa::error::Error;
use crate::clients::ics10_grandpa::header::Header as TmHeader;
use crate::core::ics02_client::client_state::{
    ClientState as Ics2ClientState, UpgradeOptions as CoreUpgradeOptions,
};
use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::error::Error as Ics02Error;
use ibc_proto::protobuf::Protobuf;
//use crate::core::ics02_client::trust_threshold::TrustThreshold;
//use crate::core::ics23_commitment::specs::ProofSpecs;
use crate::core::ics24_host::identifier::ChainId;
use crate::prelude::*;
use crate::timestamp::Timestamp;
use crate::Height;

pub const GRANDPA_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.ClientState";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientState {
    pub chain_id: ChainId,
    /// block_number is height?
    pub latest_height: u32,
    /// Block height when the client was frozen due to a misbehaviour
    pub frozen_height: Option<Height>,
    pub latest_commitment: Commitment,
    pub validator_set: ValidatorSet,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllowUpdate {
    pub after_expiry: bool,
    pub after_misbehaviour: bool,
}

impl ClientState {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        chain_id: ChainId,
        latest_height: u32,
        latest_commitment: Commitment,
        validator_set: ValidatorSet,
    ) -> Result<ClientState, Error> {
        let client_state = ClientState {
            chain_id,
            latest_height,
            latest_commitment,
            validator_set,
            frozen_height: None,
        };

        Ok(client_state)
    }

    pub fn latest_height(&self) -> Height {
        Height::new(0, self.latest_height as u64).unwrap()
    }

    pub fn with_header(self, h: TmHeader) -> Result<Self, Error> {
        // TODO: Clarify which fields should update.
        Ok(ClientState {
            latest_height: h.height().revision_number() as u32,
            ..self
        })
    }

    pub fn with_frozen_height(self, h: Height) -> Result<Self, Error> {
        Ok(Self {
            frozen_height: Some(h),
            ..self
        })
    }

    /// Get the refresh time to ensure the state does not expire
    pub fn refresh_time(&self) -> Option<Duration> {
        //TODO
        Some(Duration::new(3, 0))
    }

    /// Verify the time and height delays
    pub fn verify_delay_passed(
        _current_time: Timestamp,
        _current_height: Height,
        _processed_time: Timestamp,
        _processed_height: Height,
        _delay_period_time: Duration,
        _delay_period_blocks: u64,
    ) -> Result<(), Error> {
        Ok(())
    }

    /// Verify that the client is at a sufficient height and unfrozen at the given height
    pub fn verify_height(&self, _height: Height) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpgradeOptions {
    pub unbonding_period: Duration,
}

impl CoreUpgradeOptions for UpgradeOptions {}

impl Ics2ClientState for ClientState {
    fn chain_id(&self) -> ChainId {
        self.chain_id.clone()
    }

    fn client_type(&self) -> ClientType {
        ClientType::Grandpa
    }

    fn latest_height(&self) -> Height {
        Height::new(0, self.latest_height as u64).unwrap()
    }

    fn frozen_height(&self) -> Option<Height> {
        self.frozen_height.clone()
    }

    fn upgrade(
        &mut self,
        _upgrade_height: Height,
        _upgrade_options: &dyn CoreUpgradeOptions,
        _chain_id: ChainId,
    ) {
    }

    fn expired(&self, _elapsed: Duration) -> bool {
        // TODO
        false
    }
}

impl Protobuf<RawGpClientState> for ClientState {}

impl TryFrom<RawGpClientState> for ClientState {
    type Error = Error;

    fn try_from(raw: RawGpClientState) -> Result<Self, Self::Error> {
        let frozen_height = raw
            .frozen_height
            .and_then(|raw_height| raw_height.try_into().ok());

        Ok(Self {
            chain_id: ChainId::from_str(raw.chain_id.as_str())
                .map_err(|_| Error::invalid_chain_id())?,
            latest_height: raw.latest_height,
            frozen_height,
            latest_commitment: raw
                .latest_commitment
                .ok_or_else(Error::empty_latest_commitment)?
                .into(),
            validator_set: raw
                .validator_set
                .ok_or_else(Error::empty_validator_set)?
                .into(),
        })
    }
}

impl From<ClientState> for RawGpClientState {
    fn from(value: ClientState) -> Self {
        Self {
            chain_id: value.chain_id.to_string(),
            latest_height: value.latest_height,
            frozen_height: Some(value.frozen_height.map(|height| height.into()).unwrap_or(
                RawHeight {
                    revision_number: 0,
                    revision_height: 0,
                },
            )),
            latest_commitment: Some(value.latest_commitment.into()),
            validator_set: Some(value.validator_set.into()),
        }
    }
}

impl Protobuf<Any> for ClientState {}

impl TryFrom<Any> for ClientState {
    type Error = Ics02Error;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use bytes::Buf;
        use core::ops::Deref;

        fn decode_client_state<B: Buf>(buf: B) -> Result<ClientState, Error> {
            RawGpClientState::decode(buf)
                .map_err(Error::decode)?
                .try_into()
        }

        match raw.type_url.as_str() {
            GRANDPA_CLIENT_STATE_TYPE_URL => {
                decode_client_state(raw.value.deref()).map_err(Into::into)
            }
            _ => Err(Ics02Error::unknown_client_state_type(raw.type_url)),
        }
    }
}

impl From<ClientState> for Any {
    fn from(client_state: ClientState) -> Self {
        Any {
            type_url: GRANDPA_CLIENT_STATE_TYPE_URL.to_string(),
            value: Protobuf::<RawGpClientState>::encode_vec(&client_state)
                .expect("encoding to `Any` from `RawGpClientState`"),
        }
    }
}
