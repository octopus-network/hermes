use super::header::Header;
use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// Misbehaviour is a wrapper over two conflicting Headers
/// that implements Misbehaviour interface expected by ICS-02
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Misbehaviour {
    pub client_id: String,
    pub header_1: Option<Header>,
    pub header_2: Option<Header>,
}
