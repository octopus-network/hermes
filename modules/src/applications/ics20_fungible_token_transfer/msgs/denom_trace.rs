///
/// DenomTrace imploement
/// refer to https://github.com/octopus-network/ibc-go/blob/main/modules/apps/transfer/types/trace.go
///
use crate::prelude::*;
use serde::{Deserialize, Serialize};
use tendermint_proto::Protobuf;

use ibc_proto::ibc::apps::transfer::v1::DenomTrace as RawDenomTrace;

use crate::applications::ics20_fungible_token_transfer::error::Error;
use sha2::{Digest, Sha256};
use subtle_encoding::{hex, Error as HexError};
/// DenomPrefix.
pub const DENOM_PREFIX: &str = "ibc";
// DenomTrace contains the base denomination for ICS20 fungible tokens and the
/// source tracing information path.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DenomTrace {
    /// path defines the chain of port/channel identifiers used for tracing the
    /// source of the fungible token.
    pub path: String,
    /// base denomination of the relayed fungible token.
    pub base_denom: String,
}
impl DenomTrace {
    pub fn new(path: String, base_denom: String) -> Self {
        Self { path, base_denom }
    }

    // Hash returns the hex bytes of the SHA256 hash of the DenomTrace fields using the following formula:
    // hash = sha256(tracePath + "/" + baseDenom)
    pub fn hash(&self) -> Result<Vec<u8>, Error> {
        //hash := sha256.Sum256([]byte(dt.GetFullDenomPath()))
        // return hash[:]
        let mut hasher = Sha256::new();

        hasher.update(self.get_full_denom_path()?.as_bytes());
        let denom_bytes = hasher.finalize();
        // let hash = vec![0u8; 32];
        Ok(denom_bytes.to_vec())
    }

    // GetPrefix returns the receiving denomination prefix composed by the trace info and a separator.
    pub fn get_prefix(&self) -> String {
        if self.path.is_empty() {
            return self.path.clone();
        }
        self.path.clone() + "/"
    }

    // IBCDenom a coin denomination for an ICS20 fungible token in the format
    // 'ibc/{hash(tracePath + baseDenom)}'. If the trace is empty, it will return the base denomination.
    pub fn ibc_denom(&self) -> Result<String, Error> {
        if !self.path.is_empty() {
            let denom_hex =
                String::from_utf8(hex::encode_upper(self.hash()?)).map_err(Error::utf8)?;
            return Ok(format!("{}/{}", DENOM_PREFIX, denom_hex));
        }
        Ok(self.base_denom.clone())
    }

    // GetFullDenomPath returns the full denomination according to the ICS20 specification:
    // tracePath + "/" + baseDenom
    // If there exists no trace then the base denomination is returned.
    pub fn get_full_denom_path(&self) -> Result<String, Error> {
        if self.path.is_empty() {
            return Ok(self.base_denom.clone());
        }
        // self.get_prefix() + &self.base_denom.clone()
        let transfer_path = format!("{}{}", self.get_prefix(), self.base_denom);
        Ok(transfer_path)
    }
}

// ParseDenomTrace parses a string with the ibc prefix (denom trace) and the base denomination
// into a DenomTrace type.
//
// Examples:
//
// 	- "portidone/channelidone/uatom" => DenomTrace{Path: "portidone/channelidone", BaseDenom: "uatom"}
// 	- "uatom" => DenomTrace{Path: "", BaseDenom: "uatom"}
pub fn parse_denom_trace(raw_denom: &str) -> Result<DenomTrace, Error> {
    let denom_split = raw_denom.split('/').collect::<Vec<&str>>();
    if denom_split[0] == raw_denom {
        return Ok(DenomTrace {
            path: "".to_string(),
            base_denom: raw_denom.to_string(),
        });
    }
    Ok(DenomTrace {
        path: denom_split[0..denom_split.len() - 1].join("/"),
        base_denom: denom_split[denom_split.len() - 1].to_string(),
    })
}

// ValidateIBCDenom validates that the given denomination is either:
//  - A valid base denomination (eg: 'uatom')
//  - A valid fungible token representation (i.e 'ibc/{hash}') per ADR 001 https://github.com/cosmos/ibc-go/blob/main/docs/architecture/adr-001-coin-source-tracing.md
// refer to https://github.com/octopus-network/ibc-go/blob/f5962c3324ee7e69eeaa9918b65eb1b089da6095/modules/apps/transfer/types/trace.go#L167
pub fn validate_ibc_denom(denom: &str) -> Result<(), Error> {
    //TODO: This check maybe put in the module related to the specific chain sdk
    // if err := sdk.ValidateDenom(denom); err != nil {
    // 	return err
    // }

    if denom.trim() == "" {
        return Err(Error::invalid_denom_for_transfer(
            "denomination should not be empty".to_string(),
        ));
    }

    let denom_split = denom.split('/').collect::<Vec<&str>>();

    if denom_split.len() == 1 && denom_split[0] == denom && denom != DENOM_PREFIX {
        return Ok(());
    }

    if denom_split.len() == 2 && denom_split[0] == DENOM_PREFIX && denom_split[1].trim() != "" {
        if let Err(err) = parse_hex_hash(denom_split[1]) {
            Err(Error::invalid_denom_for_transfer(err.to_string()))
        } else {
            Ok(())
        }
    } else {
        Err(Error::invalid_denom_for_transfer(
            "denomination should be like: 'ibc/1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef'".to_string()
        ))
    }
}

// ParseHexHash parses a hex hash in string format to bytes and validates its correctness.
pub fn parse_hex_hash(hex_hash: &str) -> Result<Vec<u8>, Error> {
    let hash =
        hex::decode(hex_hash).map_err(|err| Error::invalid_denom_for_transfer(err.to_string()))?;
    // validate hash returns an error if the hash is not empty, but its
    // size != tmhash.Size.
    if !hash.is_empty() && hash.len() != Sha256::output_size() {
        return Err(Error::invalid_denom_for_transfer(format!(
            "expected size to be {} bytes, got {} bytes",
            Sha256::output_size(),
            hash.len()
        )));
    }

    Ok(hash)
}

impl Protobuf<RawDenomTrace> for DenomTrace {}

impl TryFrom<RawDenomTrace> for DenomTrace {
    type Error = Error;

    fn try_from(value: RawDenomTrace) -> Result<Self, Self::Error> {
        let path = value.path;
        let base_denom = value.base_denom;
        Ok(DenomTrace { path, base_denom })
    }
}

impl From<DenomTrace> for RawDenomTrace {
    fn from(value: DenomTrace) -> Self {
        RawDenomTrace {
            path: value.path,
            base_denom: value.base_denom,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    // use hex_literal::hex;
    // pub(crate) use hex;
    use std::println;

    #[test]
    pub fn test_denom_trace() {
        let mut denom_trace = DenomTrace {
            path: "".to_string(),
            base_denom: "".to_string(),
        };
        let prefix = denom_trace.get_prefix();
        println!("prefix: {}", prefix);
        assert_eq!(prefix, "");

        let full_path = denom_trace.get_full_denom_path();
        println!("full_path: {:?}", full_path);
        assert_eq!(full_path.unwrap(), denom_trace.base_denom);

        let hash = denom_trace.hash();
        println!("hash: {:?}", hash);
        assert!(hash.is_ok());

        let ibc_denom = denom_trace.ibc_denom();
        println!("ibc_denom: {:?}", ibc_denom);
        assert_eq!(ibc_denom.unwrap(), denom_trace.base_denom);

        // set base denom
        denom_trace.base_denom = "uatom".to_string();

        let prefix = denom_trace.get_prefix();
        println!("prefix: {}", prefix);
        assert_eq!(prefix, "");

        let full_path = denom_trace.get_full_denom_path();
        println!("full_path: {:?}", full_path);
        assert_eq!(full_path.unwrap(), denom_trace.base_denom);

        let hash = denom_trace.hash();
        println!("hash: {:?}", hash);
        assert!(hash.is_ok());

        let ibc_denom = denom_trace.ibc_denom();
        println!("ibc_denom: {:?}", ibc_denom);
        assert_eq!(ibc_denom.unwrap(), denom_trace.base_denom);

        // set path
        denom_trace.path = "tansfer/channel-1".to_string();
        let prefix = denom_trace.get_prefix();
        println!("prefix: {}", prefix);
        assert_eq!(prefix, denom_trace.path.clone() + "/");

        let full_path = denom_trace.get_full_denom_path();
        println!("full_path: {:?}", full_path);
        assert_eq!(full_path.unwrap(), prefix + &denom_trace.base_denom);

        let hash = denom_trace.hash();
        println!("hash: {:?}", hash);
        assert!(hash.is_ok());

        let ibc_denom = denom_trace.ibc_denom();
        println!("ibc_denom: {:?}", ibc_denom);
        // assert_eq!(
        //     ibc_denom.unwrap(),
        //     "ibc/" + hex::decode(denom_trace.hash().unwrap())
        // );
        let denom_hex = String::from_utf8(hex::encode_upper(hash.unwrap())).map_err(Error::utf8);
        assert_eq!(ibc_denom.unwrap(), "ibc/".to_string() + &denom_hex.unwrap());
    }

    #[test]
    pub fn test_parse_hex_hash() {
        // wrong hash with 0x
        let hash =
            parse_hex_hash("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef");
        assert!(hash.is_err());

        // wrong hash with wrong character, eg: 1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdeg
        let hash =
            parse_hex_hash("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdeg");
        assert!(hash.is_err());

        // wrong hash, > 32, eg: 1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdefg
        let hash =
            parse_hex_hash("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdefg");
        assert!(hash.is_err());

        // wrong hash, < 32, eg: 1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdefg
        let hash_size = Sha256::output_size();
        println!("hash_size: {}", hash_size);
        let hash =
            parse_hex_hash("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcde");
        assert!(hash.is_err());

        //correct hash
        let hash =
            parse_hex_hash("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef");
        assert!(hash.is_ok());
    }

    #[test]
    pub fn test_validate_ibc_denom() {
        // empty denom
        let denom = "";
        let res = validate_ibc_denom(denom);
        println!("{:?}", res);
        assert!(res.is_err());

        // only prefix
        let denom = "ibc";
        let res = validate_ibc_denom(denom);
        println!("{:?}", res);
        assert!(res.is_err());
        // assert!(res.is_ok());
        let denom = "ibc/";
        let res = validate_ibc_denom(denom);
        println!("{:?}", res);
        assert!(res.is_err());
        // assert!(res.is_ok());

        // correct denom
        let denom = "atom";
        let res = validate_ibc_denom(denom);
        println!("{:?}", res);
        assert!(res.is_ok());

        // wrong prefix
        let denom = "abc/1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdeff";
        let res = validate_ibc_denom(denom);
        // assert!(res.is_ok());
        assert!(res.is_err());

        // wrong format,
        let denom = "ibc/1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef/ibc/1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let res = validate_ibc_denom(denom);
        // assert!(res.is_ok());
        assert!(res.is_err());

        // correct denom
        let denom = "ibc/1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
        let res = validate_ibc_denom(denom);
        assert!(res.is_ok());
    }
    #[test]
    pub fn test_parse_denom_trace() {
        // denom_trace is empty
        let denom_trace = "";
        let res = parse_denom_trace(denom_trace);
        assert!(res.is_ok());
        let res = res.unwrap();
        println!(" trace path = {:?} ", res.path);
        println!(" base denom = {:?} ", res.base_denom);

        // denom_trace is base denom
        let denom_trace = "uatom";
        let res = parse_denom_trace(denom_trace);
        assert!(res.is_ok());
        let res = res.unwrap();
        println!(" trace path = {:?} ", res.path);
        println!(" base denom = {:?} ", res.base_denom);

        // first denom
        let denom_trace = "transfer/channel-01/uatom";
        let res = parse_denom_trace(denom_trace);
        assert!(res.is_ok());
        let res = res.unwrap();
        println!(" trace path = {:?} ", res.path);
        println!(" base denom = {:?} ", res.base_denom);

        // second denom
        let denom_trace = "transfer/channel-02/transfer/channel-01/uatom";
        let res = parse_denom_trace(denom_trace);
        assert!(res.is_ok());
        let res = res.unwrap();
        println!(" trace path = {:?} ", res.path);
        println!(" base denom = {:?} ", res.base_denom);
    }
}
