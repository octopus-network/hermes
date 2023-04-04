use crate::clients::ics10_grandpa::error::Error;
use crate::clients::ics10_grandpa::header::Header;
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
    pub header_1: Header,
    pub header_2: Header,
}

impl crate::core::ics02_client::misbehaviour::Misbehaviour for Misbehaviour {
    fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    fn height(&self) -> Height {
        self.header_1.height()
    }
}

impl Protobuf<RawMisbehaviour> for Misbehaviour {}

impl TryFrom<RawMisbehaviour> for Misbehaviour {
    type Error = Error;

    fn try_from(raw: RawMisbehaviour) -> Result<Self, Self::Error> {
        Ok(Self {
            client_id: Default::default(),
            header_1: raw
                .header_1
                .ok_or_else(|| Error::invalid_raw_misbehaviour("missing header1".into()))?
                .try_into()?,
            header_2: raw
                .header_2
                .ok_or_else(|| Error::invalid_raw_misbehaviour("missing header2".into()))?
                .try_into()?,
        })
    }
}

impl From<Misbehaviour> for RawMisbehaviour {
    fn from(value: Misbehaviour) -> Self {
        RawMisbehaviour {
            client_id: value.client_id.to_string(),
            header_1: Some(value.header_1.into()),
            header_2: Some(value.header_2.into()),
        }
    }
}

impl core::fmt::Display for Misbehaviour {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{} h1: {} h2: {}",
            self.client_id,
            self.header_1.height(),
            self.header_2.height(),
        )
    }
}
