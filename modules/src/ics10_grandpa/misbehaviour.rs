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
pub struct Misbehaviour {}

impl crate::ics02_client::misbehaviour::Misbehaviour for Misbehaviour {
    fn client_id(&self) -> &ClientId {
        unimplemented!()
    }

    fn height(&self) -> Height {
        unimplemented!()
    }

    fn wrap_any(self) -> AnyMisbehaviour {
        AnyMisbehaviour::Grandpa(self)
    }
}

impl Protobuf<RawMisbehaviour> for Misbehaviour {}

impl TryFrom<RawMisbehaviour> for Misbehaviour {
    type Error = Error;

    fn try_from(raw: RawMisbehaviour) -> Result<Self, Self::Error> {
        Ok(Misbehaviour {})
    }
}

impl From<Misbehaviour> for RawMisbehaviour {
    fn from(value: Misbehaviour) -> Self {
        Self {}
    }
}

impl fmt::Display for Misbehaviour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Misbehaviour")
    }
}
