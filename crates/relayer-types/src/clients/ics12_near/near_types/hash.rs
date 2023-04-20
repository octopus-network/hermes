use alloc::string::{String, ToString};
use borsh::{BorshDeserialize, BorshSerialize};
use core::fmt::{self, Debug, Display};
use sha2::Digest;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BorshDeserialize, BorshSerialize, Hash)]
pub struct CryptoHash(pub [u8; 32]);

impl CryptoHash {
    //
    pub const fn new() -> Self {
        Self([0; 32])
    }
    //
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
    /// Calculates hash of given bytes.
    pub fn hash_bytes(bytes: &[u8]) -> CryptoHash {
        CryptoHash(sha2::Sha256::digest(bytes).into())
    }
    /// Calculates hash of borsh-serialised representation of an object.
    ///
    /// Note that if you have a slice of objects to serialise, you might
    /// prefer using [`Self::hash_borsh_slice`] instead.
    pub fn hash_borsh<T: BorshSerialize>(value: &T) -> CryptoHash {
        let mut hasher = sha2::Sha256::default();
        BorshSerialize::serialize(value, &mut hasher).unwrap();
        CryptoHash(hasher.finalize().into())
    }
}

impl Default for CryptoHash {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<[u8]> for CryptoHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl TryFrom<&[u8]> for CryptoHash {
    type Error = String;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 32 {
            return Err("Wrong size.".to_string());
        }
        let inner: [u8; 32] = bytes.try_into().unwrap();
        Ok(CryptoHash(inner))
    }
}

impl Debug for CryptoHash {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt::Debug::fmt(&bs58::encode(self.0).into_string(), f)
    }
}

impl Display for CryptoHash {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt::Display::fmt(&bs58::encode(self.0).into_string(), f)
    }
}

pub fn sha256(data: &[u8]) -> [u8; 32] {
    sha2::Sha256::digest(data).into()
}

pub fn combine_hash(hash1: &CryptoHash, hash2: &CryptoHash) -> CryptoHash {
    CryptoHash(sha256(&[hash1.0.as_ref(), hash2.0.as_ref()].concat()))
}