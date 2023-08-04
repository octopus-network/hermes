use candid::CandidType;
use ic_cdk::export::serde::Deserialize;

pub const LOCAL_NET: &str = "http://localhost:4943";
pub const MAIN_NET: &str = "https://ic0.app";

#[derive(CandidType, Deserialize, Clone, Default, Debug)]
pub struct SmState {
    pub client_state: Vec<u8>,
    pub consensus_state: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone, Default, Debug)]
pub struct Proofs {
    pub height: String,
    pub object_proof: Vec<u8>,
    pub sm_client_state: Vec<u8>,
    pub client_state_proof: Vec<u8>,
    pub consensue_height: String,
    pub consensus_state_proof: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug)]
pub(crate) enum TextResult {
    Ok(String),
    Err(String),
}

#[derive(CandidType, Deserialize, Debug)]
pub(crate) enum SmStateResult {
    Ok(SmState),
    Err(String),
}

#[derive(CandidType, Deserialize, Debug)]
pub(crate) enum VecResult {
    Ok(Vec<u8>),
    Err(String),
}

#[derive(CandidType, Deserialize, Debug)]
pub(crate) enum NullResult {
    Ok(()),
    Err(String),
}

#[derive(CandidType, Deserialize, Debug)]
pub(crate) enum ProofsResult {
    Ok(Proofs),
    Err(String),
}

#[derive(CandidType, Deserialize, Debug)]
pub(crate) enum U64Result {
    Ok(u64),
    Err(String),
}
