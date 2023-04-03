use core::convert::{TryFrom, TryInto};
use core::time::Duration;

use prost::Message;
use serde::{Deserialize, Serialize};

use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::Height as RawHeight;
use ibc_proto::ibc::lightclients::grandpa::v1::ClientState as RawGpClientState;
use ibc_proto::protobuf::Protobuf;

use tendermint_light_client_verifier::options::Options;
use tendermint_light_client_verifier::ProdVerifier;

use super::beefy_authority_set::BeefyAuthoritySet;
use crate::clients::ics10_grandpa::error::Error;
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
pub enum ChaninType {
    Solochain = 0,
    Parachian = 1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientState {
    /// 0: solochain
    /// 1: parachain
    pub chain_type: ChaninType,
    /// chain_id string type, eg: ibc-1
    pub chain_id: ChainId,
    /// parachain id is uint type
    pub parachain_id: u32,
    /// block number that the beefy protocol was activated on the relay chain.
    /// This should be the first block in the merkle-mountain-range tree.
    pub beefy_activation_block: u32,
    /// the latest mmr_root_hash height
    pub latest_beefy_height: u32,
    /// Latest mmr root hash
    pub mmr_root_hash: Vec<u8>,
    /// latest solochain or parachain height
    pub latest_chain_height: u32,
    /// Block height when the client was frozen due to a misbehaviour
    pub frozen_height: u32,
    /// authorities for the current round
    pub authority_set: Option<BeefyAuthoritySet>,
    /// authorities for the next round
    pub next_authority_set: Option<BeefyAuthoritySet>,
}

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
        self.chain_id.clone()
    }

    fn client_type(&self) -> ClientType {
        ClientType::Grandpa
    }

    fn latest_height(&self) -> Height {
        todo!()
    }

    // todo
    fn frozen_height(&self) -> Option<Height> {
        None
    }

    fn upgrade(
        &mut self,
        upgrade_height: Height,
        upgrade_options: &dyn CoreUpgradeOptions,
        chain_id: ChainId,
    ) {
        todo!()
    }

    // todo
    fn expired(&self, elapsed: Duration) -> bool {
        false
    }
}

impl Protobuf<RawGpClientState> for ClientState {}

impl TryFrom<RawGpClientState> for ClientState {
    type Error = Error;

    fn try_from(raw: RawGpClientState) -> Result<Self, Self::Error> {
        Ok(Self {
            chain_type: match raw.chain_type {
                0 => ChaninType::Solochain,
                1 => ChaninType::Parachian,
                _ => panic!("unknow chain type"),
            },
            chain_id: ChainId::from_string(raw.chain_id.as_str()),
            parachain_id: raw.parachain_id,
            beefy_activation_block: raw.beefy_activation_block,
            latest_beefy_height: raw.latest_beefy_height,
            mmr_root_hash: raw.mmr_root_hash,
            latest_chain_height: raw.latest_chain_height,
            frozen_height: raw.frozen_height,
            authority_set: raw.authority_set.map(TryInto::try_into).transpose()?,
            next_authority_set: raw.next_authority_set.map(TryInto::try_into).transpose()?,
        })
    }
}

impl From<ClientState> for RawGpClientState {
    fn from(value: ClientState) -> Self {
        Self {
            chain_type: value.chain_type as u32,
            chain_id: value.chain_id.to_string(),
            parachain_id: value.parachain_id,
            beefy_activation_block: value.beefy_activation_block,
            latest_beefy_height: value.latest_beefy_height,
            mmr_root_hash: value.mmr_root_hash,
            latest_chain_height: value.latest_chain_height,
            frozen_height: value.frozen_height,
            authority_set: value.authority_set.map(Into::into),
            next_authority_set: value.next_authority_set.map(Into::into),
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
                .expect("encoding to `Any` from `TmClientState`"),
        }
    }
}
