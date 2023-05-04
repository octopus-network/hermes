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
    Subchain = 0,
    Parachian = 1,
}

impl From<u8> for ChaninType {
    fn from(value: u8) -> Self {
        match value {
            0 => ChaninType::Subchain,
            1 => ChaninType::Parachian,
            _ => panic!("not support chain type"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientState {
    /// 0: subchain
    /// 1: parachain
    pub chain_type: ChaninType,
    /// chain_id string type, eg: ibc-1
    pub chain_id: ChainId,
    /// parachain id is uint type
    pub parachain_id: u32,
    /// the latest mmr_root_hash height
    pub latest_beefy_height: Height,
    /// Latest mmr root hash
    pub latest_mmr_root: Vec<u8>,
    /// latest subchain or parachain height
    pub latest_chain_height: Height,
    /// Block height when the client was frozen due to a misbehaviour
    pub frozen_height: Option<Height>,
    /// authorities for the current round
    pub latest_authority_set: Option<BeefyAuthoritySet>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllowUpdate {
    pub after_expiry: bool,
    pub after_misbehaviour: bool,
}

impl ClientState {
    pub fn latest_height(&self) -> Height {
        self.latest_chain_height
    }

    pub fn refresh_time(&self) -> Option<Duration> {
        None
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
        self.latest_height()
    }

    // https://github.com/octopus-network/ibc-go/blob/cc25e9b73c3daa2269081f65b23971e7030864d5/modules/light-clients/10-grandpa/types/client_state.go#L80
    fn frozen_height(&self) -> Option<Height> {
        None
    }

    fn upgrade(
        &mut self,
        upgrade_height: Height,
        upgrade_options: &dyn CoreUpgradeOptions,
        chain_id: ChainId,
    ) {
    }

    // https://github.com/octopus-network/ibc-go/blob/cc25e9b73c3daa2269081f65b23971e7030864d5/modules/light-clients/10-grandpa/types/client_state.go#L97
    fn expired(&self, elapsed: Duration) -> bool {
        false
    }
}

impl Protobuf<RawGpClientState> for ClientState {}

impl TryFrom<RawGpClientState> for ClientState {
    type Error = Error;

    fn try_from(raw: RawGpClientState) -> Result<Self, Self::Error> {
        // In `RawClientState`, a `frozen_height` of `0` means "not frozen".
        // See:
        // https://github.com/cosmos/ibc-go/blob/8422d0c4c35ef970539466c5bdec1cd27369bab3/modules/light-clients/07-tendermint/types/client_state.go#L74
        let frozen_height = raw
            .frozen_height
            .and_then(|raw_height| raw_height.try_into().ok());

        Ok(Self {
            chain_type: match raw.chain_type {
                0 => ChaninType::Subchain,
                1 => ChaninType::Parachian,
                _ => panic!("unknow chain type"),
            },
            chain_id: ChainId::from_string(raw.chain_id.as_str()),
            parachain_id: raw.parachain_id,
            latest_beefy_height: raw
                .latest_beefy_height
                .ok_or_else(Error::missing_latest_beefy_height)?
                .try_into()
                .map_err(|_| Error::missing_latest_beefy_height())?,
            latest_mmr_root: raw.latest_mmr_root,
            latest_chain_height: raw
                .latest_chain_height
                .ok_or_else(Error::missing_latest_chain_height)?
                .try_into()
                .map_err(|_| Error::missing_latest_chain_height())?,
            frozen_height,
            latest_authority_set: raw.latest_authority_set.map(Into::into),
        })
    }
}

impl From<ClientState> for RawGpClientState {
    fn from(value: ClientState) -> Self {
        Self {
            chain_type: value.chain_type as u32,
            chain_id: value.chain_id.to_string(),
            parachain_id: value.parachain_id,
            latest_beefy_height: Some(value.latest_beefy_height.into()),
            latest_mmr_root: value.latest_mmr_root,
            latest_chain_height: Some(value.latest_chain_height.into()),
            frozen_height: Some(value.frozen_height.map(|height| height.into()).unwrap_or(
                RawHeight {
                    revision_number: 0,
                    revision_height: 0,
                },
            )),
            latest_authority_set: value.latest_authority_set.map(Into::into),
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
