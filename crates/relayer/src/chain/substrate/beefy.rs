use codec::{Decode, Encode};
use core::fmt::Debug;
use core::marker::PhantomData;
use sp_core::keccak_256;
use sp_core::H256;
use sp_io::crypto;
// use sp_runtime::traits::BlakeTwo256;
use sp_std::prelude::*;

// pub trait CommonHostFunctions: Clone + Send + Sync + Eq + Debug + Default {
//     /// Blake2-256 hashing implementation
//     type BlakeTwo256: hash_db::Hasher<Out = H256> + Debug + 'static;
// }
// pub mod beefy;
/// Host functions that allow the light client perform cryptographic operations in native.
// pub trait HostFunctions: CommonHostFunctions {
    pub trait HostFunctions {
    /// Keccak 256 hash function
    fn keccak_256(input: &[u8]) -> [u8; 32];

    /// Compressed Ecdsa public key recovery from a signature
    fn secp256k1_ecdsa_recover_compressed(
        signature: &[u8; 65],
        value: &[u8; 32],
    ) -> Option<Vec<u8>>;
}

/// Host function implementation for beefy light client.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Crypto;

// impl CommonHostFunctions for Crypto {
//     type BlakeTwo256 = sp_runtime::traits::BlakeTwo256;
// }

impl HostFunctions for Crypto {
    fn keccak_256(input: &[u8]) -> [u8; 32] {
        keccak_256(input)
    }

    fn secp256k1_ecdsa_recover_compressed(
        signature: &[u8; 65],
        value: &[u8; 32],
    ) -> Option<Vec<u8>> {
        crypto::secp256k1_ecdsa_recover_compressed(signature, value)
            .ok()
            .map(|val| val.to_vec())
    }
}

/// Merkle Hasher for mmr library
#[derive(Clone)]
pub struct MerkleHasher<T: HostFunctions>(PhantomData<T>);

impl<T: HostFunctions + Clone> mmr_lib::Merge for MerkleHasher<T> {
    type Item = H256;

    fn merge(left: &Self::Item, right: &Self::Item) -> Self::Item {
        let mut concat = left.as_bytes().to_vec();
        concat.extend_from_slice(right.as_bytes());
        T::keccak_256(&*concat).into()
    }
}

impl<T: HostFunctions + Clone> rs_merkle::Hasher for MerkleHasher<T> {
    type Hash = [u8; 32];
    fn hash(data: &[u8]) -> Self::Hash {
        T::keccak_256(data)
    }
}
