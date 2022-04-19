use crate::core::ics04_channel::error as channel_error;
use crate::core::ics04_channel::Version;
use crate::core::ics04_channel::channel::Order;
use crate::core::ics24_host::error::ValidationError;
use crate::core::ics24_host::identifier::{ChannelId, PortId};
use crate::prelude::*;

use alloc::string::FromUtf8Error;
use flex_error::{define_error, DisplayOnly};

define_error! {
    #[derive(Debug, PartialEq, Eq)]
    Error {
        UnknowMessageTypeUrl
            { url: String }
            | e | { format_args!("unrecognized ICS-20 transfer message type URL {0}", e.url) },

        Ics04Channel
            [ channel_error::Error ]
            |_ | { "Ics04 channel error" },

        DestinationChannelNotFound
            { port_id: PortId, channel_id: ChannelId }
            | e | { format_args!("destination channel not found in the counterparty of port_id {0} and channel_id {1} ", e.port_id, e.channel_id) },

        InvalidPortId
            { context: String }
            [ ValidationError ]
            | _ | { "invalid port identifier" },

        InvalidChannelId
            { context: String }
            [ ValidationError ]
            | _ | { "invalid channel identifier" },

        InvalidPacketTimeoutHeight
            { context: String }
            | _ | { "invalid packet timeout height value" },

        InvalidPacketTimeoutTimestamp
            { timestamp: u64 }
            | _ | { "invalid packet timeout timestamp value" },

        Utf8
            [ DisplayOnly<FromUtf8Error> ]
            | _ | { "utf8 decoding error" },

        DenomTraceNotFound
            { context: String }
            | _ | { "denom trace not found" },

        InvalidDenomForTransfer
             { denom_err: String }
            | e | { format_args!("{0}", e.denom_err) },

        AcknowledgementResponseEmpty
            | _ | { "acknowledgement response is empty" },

        InvalidValidation
            [ ValidationError ]
            | _ | { "invalid validation error" },

        InvalidSerdeIbcFungibleTokenPacketData
            [ DisplayOnly<serde_json::Error> ]
            | _ | { "invalid serde ibc fungible token packet data" },
        
        InvalidDecode
            [ DisplayOnly<tendermint_proto::Error>]
            |_| { "invalid decode denom trace" },
        
        InvalidEncode
            [ DisplayOnly<tendermint_proto::Error>]
            |_| { "invalid decode denom trace" },

        InvalidSplit 
            |_| {"invalid split get a none"},
        
        InvalidParse
            |_| { "invalid parse" },
        
        InvalidVersion 
            { 
                result_version: Version,
                expect_version: Version,
            }
            | e | { format_args!("Error invalid version, got {}, expected {}", e.result_version, e.expect_version)},
        
        InvalidRequest
            | _ | { "Error invalid request: user cannot close channel" },
        
        InvalidEqualPortId
            { 
                result_port_id: PortId,
                expect_port_id: PortId,
            }
            | e | { format_args!("Error invalid prot, invalid prot: {}, expected: {}", e.result_port_id, e.expect_port_id) },
        
        InvalidEqualOrder 
            { 
                result_order: Order,
                expect_order: Order,
            }
            | e| { format_args!("Error invalid channle ordering, expected {} channel, got {}", e.result_order, e.expect_order)},

        OverflowChannelSequence 
            { result_seq: u64, expect_seq: u64}
            | e | { format_args!("Error Max Transfer Channel: Channel sequence {} is greater than max allowd transfer channels {}", e.result_seq,e.expect_seq) },
        
        InvalidConvertNumber 
            | _ | { "invalid conversion number" },
        
        InvalidTransfer 
            | _| { "invalid transfer"},
        
        InvalidMint 
            | _ | { "invalid mint" },
        
        InvalidBurn
            | _ | { "invalid burn" },

    }
}
