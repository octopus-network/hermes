//! This is the definition of a transfer messages that an application submits to a chain.

use crate::core::ics24_host::error::ValidationError;
use crate::prelude::*;

use tendermint_proto::Protobuf;

use ibc_proto::ibc::apps::transfer::v1::MsgTransfer as RawMsgTransfer;

use crate::applications::ics20_fungible_token_transfer::error::Error;
use crate::applications::ics20_fungible_token_transfer::msgs::denom_trace;
use crate::core::ics02_client::height::Height;
use crate::core::ics24_host::identifier::{ChannelId, PortId};
use crate::signer::Signer;
use crate::timestamp::Timestamp;
use crate::tx_msg::Msg;

pub const TYPE_URL: &str = "/ibc.applications.transfer.v1.MsgTransfer";

/// Message definition for the "packet receiving" datagram.
#[derive(Clone, Debug, PartialEq)]
pub struct MsgTransfer {
    /// the port on which the packet will be sent
    pub source_port: PortId,
    /// the channel by which the packet will be sent
    pub source_channel: ChannelId,
    /// the tokens to be transferred
    pub token: Option<ibc_proto::cosmos::base::v1beta1::Coin>,
    /// the sender address
    pub sender: Signer,
    /// the recipient address on the destination chain
    pub receiver: Signer,
    /// Timeout height relative to the current block height.
    /// The timeout is disabled when set to 0.
    pub timeout_height: Height,
    /// Timeout timestamp relative to the current block timestamp.
    /// The timeout is disabled when set to 0.
    pub timeout_timestamp: Timestamp,
}

impl Msg for MsgTransfer {
    type ValidationError = Error;
    type Raw = RawMsgTransfer;

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
        // 	if err := host.PortIdentifierValidator(msg.SourcePort); err != nil {
        // 		return sdkerrors.Wrap(err, "invalid source port ID")
        // 	}
        // 	if err := host.ChannelIdentifierValidator(msg.SourceChannel); err != nil {
        // 		return sdkerrors.Wrap(err, "invalid source channel ID")
        // 	}
        // 	if !msg.Token.IsValid() {
        // 		return sdkerrors.Wrap(sdkerrors.ErrInvalidCoins, msg.Token.String())
        // 	}
        // 	if !msg.Token.IsPositive() {
        // 		return sdkerrors.Wrap(sdkerrors.ErrInsufficientFunds, msg.Token.String())
        // 	}
        // 	// NOTE: sender format must be validated as it is required by the GetSigners function.
        // 	_, err := sdk.AccAddressFromBech32(msg.Sender)
        // 	if err != nil {
        // 		return sdkerrors.Wrapf(sdkerrors.ErrInvalidAddress, "string could not be parsed as address: %v", err)
        // 	}
        // 	if strings.TrimSpace(msg.Receiver) == "" {
        // 		return sdkerrors.Wrap(sdkerrors.ErrInvalidAddress, "missing recipient address")
        // 	}
        let denom = self
            .token
            .as_ref()
            .map(|coin| coin.denom.as_str())
            .unwrap_or("");

        if let Err(err) = denom_trace::validate_ibc_denom(denom) {
            return Err(ValidationError::invalid_denom(err.to_string()));
        }
        Ok(())
    }
}

impl Protobuf<RawMsgTransfer> for MsgTransfer {}

impl TryFrom<RawMsgTransfer> for MsgTransfer {
    type Error = Error;

    fn try_from(raw_msg: RawMsgTransfer) -> Result<Self, Self::Error> {
        let timeout_timestamp = Timestamp::from_nanoseconds(raw_msg.timeout_timestamp)
            .map_err(|_| Error::invalid_packet_timeout_timestamp(raw_msg.timeout_timestamp))?;

        let timeout_height = match raw_msg.timeout_height.clone() {
            None => Height::zero(),
            Some(raw_height) => raw_height.try_into().map_err(|e| {
                Error::invalid_packet_timeout_height(format!("invalid timeout height {}", e))
            })?,
        };

        Ok(MsgTransfer {
            source_port: raw_msg
                .source_port
                .parse()
                .map_err(|e| Error::invalid_port_id(raw_msg.source_port.clone(), e))?,
            source_channel: raw_msg
                .source_channel
                .parse()
                .map_err(|e| Error::invalid_channel_id(raw_msg.source_channel.clone(), e))?,
            token: raw_msg.token,
            sender: raw_msg.sender.into(),
            receiver: raw_msg.receiver.into(),
            timeout_height,
            timeout_timestamp,
        })
    }
}

impl From<MsgTransfer> for RawMsgTransfer {
    fn from(domain_msg: MsgTransfer) -> Self {
        RawMsgTransfer {
            source_port: domain_msg.source_port.to_string(),
            source_channel: domain_msg.source_channel.to_string(),
            token: domain_msg.token,
            sender: domain_msg.sender.to_string(),
            receiver: domain_msg.receiver.to_string(),
            timeout_height: Some(domain_msg.timeout_height.into()),
            timeout_timestamp: domain_msg.timeout_timestamp.nanoseconds(),
        }
    }
}

#[cfg(test)]
pub mod test_util {
    use core::ops::Add;
    use core::time::Duration;

    use crate::{
        core::ics24_host::identifier::{ChannelId, PortId},
        test_utils::get_dummy_account_id,
        timestamp::Timestamp,
        Height,
    };

    use super::MsgTransfer;

    // Returns a dummy `RawMsgTransfer`, for testing only!
    pub fn get_dummy_msg_transfer(height: u64) -> MsgTransfer {
        let id = get_dummy_account_id();

        MsgTransfer {
            source_port: PortId::default(),
            source_channel: ChannelId::default(),
            token: None,
            sender: id.clone(),
            receiver: id,
            timeout_timestamp: Timestamp::now().add(Duration::from_secs(10)).unwrap(),
            timeout_height: Height {
                revision_number: 0,
                revision_height: height,
            },
        }
    }
}
