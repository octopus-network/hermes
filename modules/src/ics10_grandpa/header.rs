use core::convert::{TryFrom, TryInto};
use core::fmt;
use alloc::vec::Vec;

use ibc_proto::ibc::lightclients::grandpa::v1::Header as RawHeader;

use crate::ics02_client::client_type::ClientType;
use crate::ics02_client::header::AnyHeader;
use crate::ics10_grandpa::error::Error;
use crate::ics24_host::identifier::ChainId;
use crate::Height;
use bytes::Buf;
use prost::Message;
use serde::{Deserialize, Serialize};
use tendermint_proto::Protobuf;
use super::help::SignedCommitment;
use super::help::ValidatorMerkleProof;
use super::help::MmrLeaf;
use super::help::MmrLeafProof;


#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Header {
    pub signed_commitment: SignedCommitment,
    pub validator_merkle_proof: ValidatorMerkleProof,
    pub mmr_leaf: MmrLeaf,
    pub mmr_leaf_proof: MmrLeafProof,
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, " Header {{...}}")
    }
}

impl  Default for Header {
    fn default() -> Self {
        Self {
            signed_commitment: SignedCommitment::default(),
            validator_merkle_proof: ValidatorMerkleProof::default(),
            mmr_leaf: MmrLeaf::default(),
            mmr_leaf_proof: MmrLeafProof::default(),
        }
    }
}

impl Header {
    pub fn new(
        signed_commitment: SignedCommitment,
        validator_merkle_proof: ValidatorMerkleProof,
        mmr_leaf: MmrLeaf,
        mmr_leaf_proof: MmrLeafProof,
    ) -> Self {
        Self {
            signed_commitment,
            validator_merkle_proof,
            mmr_leaf,
            mmr_leaf_proof,
        }
    }

    pub fn height(&self) -> Height {
        Height::default()
    }
}

impl crate::ics02_client::header::Header for Header {
    fn client_type(&self) -> ClientType {
        ClientType::Grandpa
    }

    fn height(&self) -> Height {
        self.height()
    }

    fn wrap_any(self) -> AnyHeader {
        AnyHeader::Grandpa(self)
    }
}

impl Protobuf<RawHeader> for Header {}

impl TryFrom<RawHeader> for Header {
    type Error = Error;

    fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
        Ok(
            Self {
                signed_commitment: raw.signed_commitment.unwrap().into(),
                validator_merkle_proof: raw.validator_merkle_proof.unwrap().into(),
                mmr_leaf: raw.mmr_leaf.unwrap().into(),
                mmr_leaf_proof: raw.mmr_leaf_proof.unwrap().into(),
            }
        )
    }
}

pub fn decode_header<B: Buf>(buf: B) -> Result<Header, Error> {
    RawHeader::decode(buf).map_err(Error::decode)?.try_into()
}

impl From<Header> for RawHeader {
    fn from(value: Header) -> Self {
        RawHeader {
            signed_commitment: Some(value.signed_commitment.into()),
            validator_merkle_proof: Some(value.validator_merkle_proof.into()),
            mmr_leaf: Some(value.mmr_leaf.into()),
            mmr_leaf_proof: Some(value.mmr_leaf_proof.into()),
        }
    }
}
