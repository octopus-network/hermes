use crate::clients::ics12_near::consensus_state::NEAR_CONSENSUS_STATE_TYPE_URL;
use crate::core::ics02_client::client_state::{ClientState as Ics2ClientState, UpgradeOptions};
use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::error::Error;
use crate::core::ics02_client::height::Height;
use crate::core::ics24_host::identifier::ChainId;
use bytes::Buf;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::near::v1::ClientState as RawClientState;
use ibc_proto::protobuf::Protobuf;
use serde::{Deserialize, Serialize};
use std::prelude::rust_2015::ToString;
use std::time::Duration;

pub const NEAR_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.near.v1.ClientState";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ClientState {
    pub chain_id: ChainId,
    pub trusting_period: u64,
    pub latest_height: u64,
    pub latest_timestamp: u64,
    pub frozen_height: Option<u64>,
    pub upgrade_commitment_prefix: Vec<u8>,
    pub upgrade_key: Vec<u8>,
}

impl ClientState {
    /// Get the refresh time to ensure the state does not expire
    pub fn refresh_time(&self) -> Option<Duration> {
        //TODO
        Some(Duration::new(3, 0))
    }
}

impl Ics2ClientState for ClientState {
    fn chain_id(&self) -> ChainId {
        self.chain_id.clone()
    }

    fn client_type(&self) -> ClientType {
        ClientType::Near
    }

    fn latest_height(&self) -> Height {
        Height::new(0, self.latest_height + 1).expect("faild to create ibc height")
    }

    fn frozen_height(&self) -> Option<crate::Height> {
        self.frozen_height.map(|frozen_height| {
            Height::new(0, frozen_height + 1).expect("faild to create ibc height")
        })
    }

    fn expired(&self, _elapsed: Duration) -> bool {
        false
    }

    fn upgrade(
        &mut self,
        _upgrade_height: crate::Height,
        _upgrade_options: &dyn UpgradeOptions,
        _chain_id: ChainId,
    ) {
    }
}

impl Protobuf<RawClientState> for ClientState {}
impl TryFrom<RawClientState> for ClientState {
    type Error = Error;

    fn try_from(raw: RawClientState) -> Result<Self, Self::Error> {
        Ok(Self {
            chain_id: ChainId::new("ibc".to_string(), 1),
            trusting_period: raw
                .trusting_period
                .ok_or(Error::custom_error("trusting period is empty".into()))?
                .nanos as u64,
            latest_height: raw
                .latest_height
                .ok_or(Error::custom_error("latest height is empty".into()))?
                .revision_height,
            latest_timestamp: raw.latest_timestamp,
            frozen_height: raw.frozen_height.map(|h| h.revision_height),
            upgrade_commitment_prefix: raw.upgrade_commitment_prefix,
            upgrade_key: raw.upgrade_key,
        })
    }
}

impl From<ClientState> for RawClientState {
    fn from(value: ClientState) -> Self {
        RawClientState {
            trusting_period: Some(Duration::from_nanos(value.trusting_period).into()),
            frozen_height: value.frozen_height.map(|h| {
                Height::new(1, h)
                    .expect("failed to create ibc height")
                    .into()
            }),
            latest_height: Some(
                Height::new(1, value.latest_height)
                    .expect("failed to create ibc height")
                    .into(),
            ),
            latest_timestamp: value.latest_timestamp,
            upgrade_commitment_prefix: value.upgrade_commitment_prefix,
            upgrade_key: value.upgrade_key,
        }
    }
}

impl Protobuf<Any> for ClientState {}

impl From<ClientState> for Any {
    fn from(_client_state: ClientState) -> Self {
        Any {
            type_url: NEAR_CONSENSUS_STATE_TYPE_URL.to_string(),
            value: vec![],
        }
    }
}

impl TryFrom<Any> for ClientState {
    type Error = Error;

    fn try_from(raw: Any) -> Result<Self, Error> {
        use core::ops::Deref;

        fn decode_header<B: Buf>(_buf: B) -> Result<ClientState, Error> {
            Ok(ClientState::default())
        }

        match raw.type_url.as_str() {
            NEAR_CONSENSUS_STATE_TYPE_URL => decode_header(raw.value.deref()).map_err(Into::into),
            _ => Err(Error::unknown_header_type(raw.type_url)),
        }
    }
}
