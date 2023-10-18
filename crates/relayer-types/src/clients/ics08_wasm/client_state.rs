use super::proto::wasm::ClientState as RawClientState;
use crate::clients::ics12_near::client_state::ClientState as NearClientState;
use crate::core::ics02_client::client_state::{ClientState as Ics2ClientState, UpgradeOptions};
use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::error::Error;
use crate::core::ics24_host::identifier::ChainId;
use crate::Height;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::Height as RawHeight;
use ibc_proto::protobuf::Protobuf;
use prost::Message;
use serde_derive::{Deserialize, Serialize};
use std::time::Duration;

pub const WASM_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.wasm.v1.ClientState";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientState {
    pub code_hash: Vec<u8>,
    pub data: Vec<u8>,
    pub latest_height: Height,
}

impl ClientState {
    /// Get the refresh time to ensure the state does not expire
    pub fn refresh_time(&self) -> Option<Duration> {
        //TODO
        Some(Duration::new(3, 0))
    }

    pub fn near_client_state(&self) -> NearClientState {
        let any = Any::decode(self.data.as_slice()).unwrap();
        NearClientState::try_from(any).unwrap()
    }
}

impl Ics2ClientState for ClientState {
    fn chain_id(&self) -> ChainId {
        self.near_client_state().chain_id.clone()
    }

    fn client_type(&self) -> ClientType {
        ClientType::Wasm
    }

    fn latest_height(&self) -> Height {
        self.latest_height
    }

    fn frozen_height(&self) -> Option<Height> {
        let frozen_height = self.near_client_state().frozen_height;
        frozen_height
            .map(|frozen_height| Height::new(0, frozen_height).expect("faild to create ibc height"))
    }

    fn expired(&self, _elapsed: Duration) -> bool {
        false
    }

    fn upgrade(
        &mut self,
        _upgrade_height: Height,
        _upgrade_options: &dyn UpgradeOptions,
        _chain_id: ChainId,
    ) {
    }
}

impl Protobuf<RawClientState> for ClientState {}

impl TryFrom<RawClientState> for ClientState {
    type Error = Error;

    fn try_from(raw: RawClientState) -> Result<Self, Self::Error> {
        let height = raw.latest_height.unwrap();
        Ok(Self {
            code_hash: raw.code_hash,
            data: raw.data,
            latest_height: Height::new(height.revision_number, height.revision_height).unwrap(),
        })
    }
}

impl From<ClientState> for RawClientState {
    fn from(value: ClientState) -> Self {
        RawClientState {
            code_hash: value.code_hash,
            data: value.data,
            latest_height: Some(RawHeight {
                revision_number: 0,
                revision_height: value.latest_height.revision_height(),
            }),
        }
    }
}

impl Protobuf<Any> for ClientState {}

impl From<ClientState> for Any {
    fn from(client_state: ClientState) -> Self {
        Any {
            type_url: WASM_CLIENT_STATE_TYPE_URL.to_string(),
            value: Protobuf::<RawClientState>::encode_vec(&client_state),
        }
    }
}

impl TryFrom<Any> for ClientState {
    type Error = Error;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use bytes::Buf;
        use core::ops::Deref;

        fn decode_client_state<B: Buf>(buf: B) -> Result<ClientState, Error> {
            RawClientState::decode(buf)
                .map_err(Error::decode)?
                .try_into()
        }

        match raw.type_url.as_str() {
            WASM_CLIENT_STATE_TYPE_URL => {
                decode_client_state(raw.value.deref()).map_err(Into::into)
            }
            _ => Err(Error::unknown_client_state_type(raw.type_url)),
        }
    }
}
