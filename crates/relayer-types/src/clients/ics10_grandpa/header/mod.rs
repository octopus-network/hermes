use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub mod beefy_mmr;
pub mod message;

/// header wrapper
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    /// the latest mmr data
    pub beefy_mmr: Option<beefy_mmr::BeefyMmr>,
    /// only one header
    pub message: Option<message::Message>,
}
