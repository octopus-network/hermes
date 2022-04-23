use crate::applications::ics20_fungible_token_transfer::error::Error;
use crate::core::ics24_host::error::ValidationError;
use crate::prelude::*;
use crate::signer::Signer;
use crate::tx_msg::Msg;
use ibc_proto::ibc::apps::transfer::v2::FungibleTokenPacketData as RawFungibleTokenPacketData;
use serde::{Deserialize, Serialize};
use tendermint_proto::Protobuf;

pub const TYPE_URL: &str = "/ibc.applications.transfer.v2.FungibleTokenPacketData";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FungibleTokenPacketData {
    /// the token denomination to be transferred
    pub denom: String,
    /// the token amount to be transferred
    pub amount: String,
    /// the sender address
    pub sender: Signer,
    /// the recipient address on the destination chain
    pub receiver: Signer,
}

impl Msg for FungibleTokenPacketData {
    type ValidationError = ValidationError;
    type Raw = RawFungibleTokenPacketData;

    fn route(&self) -> String {
        crate::keys::ROUTER_KEY.to_string()
    }

    fn type_url(&self) -> String {
        TYPE_URL.to_string()
    }
    // ValidateBasic performs a basic check of the MsgTransfer fields.
    // NOTE: timeout height or timestamp values can be 0 to disable the timeout.
    // NOTE: The recipient addresses format is not validated as the format defined by
    // the chain is not known to IBC.
    fn validate_basic(&self) -> Result<(), ValidationError> {
        // TODO
        Ok(())
    }
}

impl Protobuf<RawFungibleTokenPacketData> for FungibleTokenPacketData {}

impl TryFrom<RawFungibleTokenPacketData> for FungibleTokenPacketData {
    type Error = Error;

    fn try_from(value: RawFungibleTokenPacketData) -> Result<Self, Self::Error> {
        Ok(FungibleTokenPacketData {
            denom: value.denom,
            amount: value.amount,
            sender: value.sender.into(),
            receiver: value.receiver.into(),
        })
    }
}

impl From<FungibleTokenPacketData> for RawFungibleTokenPacketData {
    fn from(msg: FungibleTokenPacketData) -> Self {
        RawFungibleTokenPacketData {
            denom: msg.denom.to_string(),
            amount: msg.amount.to_string(),
            sender: msg.sender.to_string(),
            receiver: msg.receiver.to_string(),
        }
    }
}
