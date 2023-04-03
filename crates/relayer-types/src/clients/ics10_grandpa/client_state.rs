use core::convert::{TryFrom, TryInto};
use core::time::Duration;

use prost::Message;
use serde::{Deserialize, Serialize};

use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::Height as RawHeight;
// use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as RawTmClientState;
use ibc_proto::ibc::lightclients::grandpa::v1::ClientState as RawGpClientState;
use ibc_proto::protobuf::Protobuf;

use tendermint_light_client_verifier::options::Options;
use tendermint_light_client_verifier::ProdVerifier;

use crate::clients::ics07_tendermint::error::Error;
use crate::clients::ics07_tendermint::header::Header as TmHeader;
use crate::core::ics02_client::client_state::{
    ClientState as Ics2ClientState, UpgradeOptions as CoreUpgradeOptions,
};
use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::error::Error as Ics02Error;
use crate::core::ics02_client::trust_threshold::TrustThreshold;
use crate::core::ics23_commitment::specs::ProofSpecs;
use crate::core::ics24_host::identifier::ChainId;
use crate::prelude::*;
use crate::timestamp::{Timestamp, ZERO_DURATION};
use crate::Height;

pub const GRANDPA_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.ClientState";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientState {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllowUpdate {
    pub after_expiry: bool,
    pub after_misbehaviour: bool,
}

impl ClientState {
    #[allow(clippy::too_many_arguments)]
    pub fn new() -> Result<ClientState, Error> {
        todo!()
    }

    pub fn latest_height(&self) -> Height {
        todo!()
    }

    pub fn with_header(self, h: TmHeader) -> Result<Self, Error> {
        todo!()
    }

    pub fn with_frozen_height(self, h: Height) -> Result<Self, Error> {
        todo!()
    }

    /// Get the refresh time to ensure the state does not expire
    pub fn refresh_time(&self) -> Option<Duration> {
        todo!()
    }

    /// Helper method to produce a [`Options`] struct for use in
    /// Tendermint-specific light client verification.
    pub fn as_light_client_options(&self) -> Result<Options, Error> {
        todo!()
    }

    /// Verify that the client is at a sufficient height and unfrozen at the given height
    pub fn verify_height(&self, height: Height) -> Result<(), Error> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpgradeOptions {
    pub unbonding_period: Duration,
}

impl CoreUpgradeOptions for UpgradeOptions {}

impl Ics2ClientState for ClientState {
    fn chain_id(&self) -> ChainId {
        todo!()
    }

    fn client_type(&self) -> ClientType {
        todo!()
    }

    fn latest_height(&self) -> Height {
        todo!()
    }

    fn frozen_height(&self) -> Option<Height> {
        todo!()
    }

    fn upgrade(
        &mut self,
        upgrade_height: Height,
        upgrade_options: &dyn CoreUpgradeOptions,
        chain_id: ChainId,
    ) {
        todo!()
    }

    fn expired(&self, elapsed: Duration) -> bool {
        todo!()
    }
}

impl Protobuf<RawGpClientState> for ClientState {}

impl TryFrom<RawGpClientState> for ClientState {
    type Error = Error;

    fn try_from(raw: RawGpClientState) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<ClientState> for RawGpClientState {
    fn from(value: ClientState) -> Self {
        todo!()
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
                .expect("encoding to `Any` from `TmClientState`"),
        }
    }
}
