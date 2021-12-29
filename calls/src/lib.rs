mod codegen;

use codec::{Decode, Encode};
use core::str::FromStr;
use prost_types::Any;

// #[derive(Eq, PartialEq, Encode, Decode)]
// pub struct ParachainId(u32);

#[derive(Encode, Decode)]
pub enum ClientType {
    Tendermint,
    Grandpa,
}

impl ClientType {
    pub fn to_ibc_client_type(self) -> ibc::ics02_client::client_type::ClientType {
        match self {
            ClientType::Tendermint => ibc::ics02_client::client_type::ClientType::Tendermint,
            ClientType::Grandpa => ibc::ics02_client::client_type::ClientType::Grandpa,
            _ => unreachable!(),
        }
    }
}

#[derive(Encode, Decode)]
pub struct MessageQueueChain(pub subxt::sp_core::H256);

#[subxt::subxt(runtime_metadata_path = "metadata_file/metadata.scale")]
pub mod ibc_node {
    // #[subxt(substitute_type = "polkadot_parachain::primitives::Id")]
    // use crate::ParachainId;

    #[subxt(substitute_type = "pallet_ibc::event::primitive::ClientType")]
    use crate::ClientType;

    // #[subxt(substitute_type = "polkadot_core_primitives::InboundHrmpMessage")]
    // use crate::ibc_node::runtime_types::polkadot_core_primitives::InboundHrmpMessage;

    #[subxt(substitute_type = "cumulus_pallet_parachain_system::MessageQueueChain")]
    use crate::MessageQueueChain;
}

// for parachain
// const _: () = {
//     use ibc_node::runtime_types::polkadot_parachain::primitives::Id;

//     impl PartialEq for Id {
//         fn eq(&self, other: &Self) -> bool {
//             self.0 == other.0
//         }
//     }

//     impl Eq for Id {

//     }

//     impl PartialOrd for Id {
//         fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//             self.0.partial_cmp(&other.0)
//         }
//     }

//     impl Ord for Id {
//         fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//             self.0.cmp(&other.0)
//         }
//     }
// };

impl ibc_node::runtime_types::pallet_ibc::event::primitive::Height {
    pub fn to_ibc_height(self) -> ibc::Height {
        ibc::Height {
            revision_number: self.revision_number,
            revision_height: self.revision_height,
        }
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::Packet {
    pub fn to_ibc_packet(self) -> ibc::ics04_channel::packet::Packet {
        ibc::ics04_channel::packet::Packet {
            sequence: self.sequence.to_ibc_sequence(),
            source_port: self.source_port.to_ibc_port_id(),
            source_channel: self.source_channel.to_ibc_channel_id(),
            destination_port: self.destination_port.to_ibc_port_id(),
            destination_channel: self.destination_channel.to_ibc_channel_id(),
            data: self.data,
            timeout_height: self.timeout_height.to_ibc_height(),
            timeout_timestamp: self.timeout_timestamp.to_ibc_timestamp(),
        }
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::ConnectionId {
    pub fn to_ibc_connection_id(self) -> ibc::ics24_host::identifier::ConnectionId {
        let value = String::from_utf8(self.0).unwrap();
        ibc::ics24_host::identifier::ConnectionId(value)
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::ChannelId {
    pub fn to_ibc_channel_id(self) -> ibc::ics24_host::identifier::ChannelId {
        let value = String::from_utf8(self.0).unwrap();
        ibc::ics24_host::identifier::ChannelId(value)
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::PortId {
    pub fn to_ibc_port_id(self) -> ibc::ics24_host::identifier::PortId {
        let value = String::from_utf8(self.0).unwrap();
        ibc::ics24_host::identifier::PortId(value)
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::ClientId {
    pub fn to_ibc_client_id(self) -> ibc::ics24_host::identifier::ClientId {
        let value = String::from_utf8(self.0).unwrap();
        ibc::ics24_host::identifier::ClientId(value)
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::Sequence {
    pub fn to_ibc_sequence(self) -> ibc::ics04_channel::packet::Sequence {
        ibc::ics04_channel::packet::Sequence(self.0)
    }
}

impl ibc_node::runtime_types::pallet_ibc::event::primitive::Timestamp {
    pub fn to_ibc_timestamp(self) -> ibc::timestamp::Timestamp {
        let value = String::from_utf8(self.time).unwrap();
        let timestamp = ibc::timestamp::Timestamp::from_str(&value).unwrap();
        timestamp
    }
}

impl From<Any> for ibc_node::runtime_types::pallet_ibc::Any {
    fn from(value: Any) -> Self {
        ibc_node::runtime_types::pallet_ibc::Any {
            type_url: value.type_url.as_bytes().to_vec(),
            value: value.value,
        }
    }
}

impl Copy for ibc_node::runtime_types::pallet_ibc::event::primitive::Height {}

impl Clone for ibc_node::runtime_types::pallet_ibc::event::primitive::Height {
    fn clone(&self) -> Self {
        Self {
            revision_number: self.revision_number,
            revision_height: self.revision_height,
        }
    }
}
