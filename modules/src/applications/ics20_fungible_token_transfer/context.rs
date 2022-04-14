use crate::applications::ics20_fungible_token_transfer::error::Error;
use crate::applications::ics20_fungible_token_transfer::msgs::denom_trace::DenomTrace;
use crate::core::ics04_channel::context::{ChannelKeeper, ChannelReader};
use crate::core::ics24_host::identifier::PortId;
use alloc::vec::Vec;

/// Captures all the dependencies which the ICS20 module requires to be able to dispatch and
/// process IBC messages.
pub trait Ics20Context: ChannelReader + ChannelKeeper {
    fn get_denom_trace(&self, denom_trace_hash: &[u8]) -> Result<DenomTrace, Error>;
    fn has_denom_trace(&self, denom_trace_hash: &[u8]) -> bool;
    fn set_denom_trace(&self, denom_trace: &DenomTrace) -> Result<(), Error>;
    fn get_port(&self) -> Result<PortId, Error>;
}
