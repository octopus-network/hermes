use crate::prelude::*;
use core::convert::{TryFrom, TryInto};
use core::fmt;

use ibc_proto::ibc::lightclients::grandpa::v1::Misbehaviour as RawMisbehaviour;

use crate::ics02_client::misbehaviour::AnyMisbehaviour;
use crate::ics10_grandpa::error::Error;
use crate::ics10_grandpa::header::Header;
use crate::ics24_host::identifier::ClientId;
use crate::Height;
use tendermint_proto::Protobuf;

#[derive(Clone, Debug, PartialEq)]
pub struct Misbehaviour {
    pub client_id: ClientId,
    pub header1: Header,
    pub header2: Header,
}

impl crate::ics02_client::misbehaviour::Misbehaviour for Misbehaviour {
    fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    fn height(&self) -> Height {
        self.header1.height()
    }

    fn wrap_any(self) -> AnyMisbehaviour {
        AnyMisbehaviour::Grandpa(self)
    }
}

impl Protobuf<RawMisbehaviour> for Misbehaviour {}

impl TryFrom<RawMisbehaviour> for Misbehaviour {
    type Error = Error;

    fn try_from(raw: RawMisbehaviour) -> Result<Self, Self::Error> {
        Ok(Self {
            client_id: Default::default(),
            header1: raw
                .header_1
                .ok_or_else(|| Error::invalid_raw_misbehaviour("missing header1".into()))?
                .try_into()?,
            header2: raw
                .header_2
                .ok_or_else(|| Error::invalid_raw_misbehaviour("missing header2".into()))?
                .try_into()?,
        })
    }
}

impl From<Misbehaviour> for RawMisbehaviour {
    fn from(value: Misbehaviour) -> Self {
        todo!()
    }
}

impl fmt::Display for Misbehaviour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{:?} h1: {:?} h2: {:?}",
            self.client_id,
            self.header1.height(),
            // self.header1.trusted_height,
            self.header2.height(),
            // self.header2.trusted_height,
        )
    }
}
