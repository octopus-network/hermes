use candid::CandidType;
use ic_cdk::export::serde::Deserialize;

pub const LOCAL_NET: &str = "http://localhost:4943";

#[derive(CandidType, Deserialize, Debug)]
pub(crate) enum VecResult {
    Ok(Vec<u8>),
    Err(String),
}
