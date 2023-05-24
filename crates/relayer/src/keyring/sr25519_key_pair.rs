use core::any::Any;

use bip39::{Language, Mnemonic, Seed};
use bitcoin::{
    network::constants::Network,
    util::bip32::{ChildNumber, DerivationPath, ExtendedPrivKey, ExtendedPubKey},
};
use digest::Digest;
use generic_array::{typenum::U32, GenericArray};
use hdpath::StandardHDPath;
use ripemd::Ripemd160;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use strum::{EnumIter, IntoEnumIterator};

use super::{
    errors::Error,
    key_utils::{decode_bech32, encode_bech32, keccak256_hash},
    pub_key::EncodedPubKey,
    KeyFile, KeyType, SigningKeyPair,
};
use crate::config::AddressType;
pub use sp_core::sr25519;
use sp_core::{
    sr25519::{Pair, Public, Signature},
    ByteArray, Pair as PairT, H256,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sr25519KeyPair {
    network_id: String,
    account_id: String,
    address: String,
    pub public_key: String,
    pub seed: String,
}

impl Sr25519KeyPair {
    pub fn pair(self) -> Pair {
        // 	Pair::from_string(&format!("//{}", <&'static str>::from(self)), None)
        // 		.expect("static values are known good; qed")
        //
        let pair = Pair::from_string(&self.seed, None).unwrap();
        // tracing::debug!("🐙🐙 ics10::substrate -> Sr25519KeyPair pair: {:?}", pair);
        pair
    }
}

impl SigningKeyPair for Sr25519KeyPair {
    const KEY_TYPE: KeyType = KeyType::Sr25519;

    fn from_key_file(key_file: KeyFile, hd_path: &StandardHDPath) -> Result<Self, Error> {
        tracing::debug!(
            "🐙🐙 sr25519_key_pair -> from_key_file key_file: {:?}",
            key_file
        );

        Ok(Self {
            network_id: key_file.r#type,
            account_id: key_file.pubkey.clone(),
            address: key_file.address,
            public_key: key_file.pubkey,
            seed: key_file.mnemonic,
        })
    }

    fn from_mnemonic(
        mnemonic: &str,
        hd_path: &StandardHDPath,
        address_type: &AddressType,
        account_prefix: &str,
    ) -> Result<Self, Error> {
        todo!()
    }

    fn account(&self) -> String {
        self.account_id.to_owned()
    }

    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
