use crate::ics02_client::{client_def::AnyHeader, client_type::ClientType};
use crate::ics10_grandpa::error::{Error, Kind};
use crate::Height;
use ibc_proto::ibc::lightclients::grandpa::v1::Header as RawHeader;
use sp_core::H256;
use sp_runtime::Justification;
use sp_trie::StorageProof;
use std::convert::TryFrom;
use tendermint_proto::Protobuf;

/// GRANDPA consensus header
#[derive(Clone, Debug, PartialEq)]
pub struct Header {
    pub height: u64,
    pub commitment_root: H256,
    pub block_hash: H256,
    pub justification: Option<Justification>,
    pub authorities_proof: StorageProof,
}

impl crate::ics02_client::header::Header for Header {
    fn client_type(&self) -> ClientType {
        ClientType::GRANDPA
    }

    fn height(&self) -> Height {
        Height::new(
            0, // TODO
            self.height,
        )
    }

    fn wrap_any(self) -> AnyHeader {
        AnyHeader::GRANDPA(self)
    }
}

impl Protobuf<RawHeader> for Header {}

impl TryFrom<RawHeader> for Header {
    type Error = Error;

    fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
        Ok(Self {
            height: raw
                .height
                .ok_or_else(|| Kind::InvalidRawHeader.context("missing signed header"))?
                .revision_height,
            commitment_root: H256::from_slice(&raw.commitment_root),
            block_hash: H256::from_slice(&raw.block_hash),
            justification: if raw.justification.len() == 0 {
                None
            } else {
                Some(raw.justification)
            },
            authorities_proof: StorageProof::new(raw.authorities_proof),
        })
    }
}

impl From<Header> for RawHeader {
    fn from(value: Header) -> Self {
        RawHeader {
            height: Some(Height::new(0, value.height).into()),
            commitment_root: value.commitment_root.as_bytes().to_vec(),
            block_hash: value.block_hash.as_bytes().to_vec(),
            justification: match value.justification {
                None => vec![],
                Some(data) => data,
            },
            authorities_proof: value.authorities_proof.iter_nodes().collect(),
        }
    }
}
