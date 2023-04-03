use super::header::Header;
use crate::clients::ics10_grandpa::error::Error;
use crate::core::ics24_host::identifier::ClientId;
use crate::prelude::*;
use crate::Height;
use ibc_proto::ibc::lightclients::grandpa::v1::Misbehaviour as RawMisbehaviour;
use ibc_proto::protobuf::Protobuf;
use serde::{Deserialize, Serialize};

pub const GRANDPA_MISBEHAVIOR_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.Misbehaviour";

/// Misbehaviour is a wrapper over two conflicting Headers
/// that implements Misbehaviour interface expected by ICS-02
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Misbehaviour {
    pub client_id: ClientId,
    pub header_1: Option<Header>,
    pub header_2: Option<Header>,
}

impl crate::core::ics02_client::misbehaviour::Misbehaviour for Misbehaviour {
    fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    fn height(&self) -> Height {
        todo!()
    }
}

impl Protobuf<RawMisbehaviour> for Misbehaviour {}

impl TryFrom<RawMisbehaviour> for Misbehaviour {
    type Error = Error;

    fn try_from(raw: RawMisbehaviour) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<Misbehaviour> for RawMisbehaviour {
    fn from(value: Misbehaviour) -> Self {
        todo!()
    }
}

impl core::fmt::Display for Misbehaviour {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        todo!()
    }
}
