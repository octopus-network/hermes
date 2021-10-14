//! Implements support for the pallet_ibc module.
use codec::Decode;
use codec::Encode;
use core::marker::PhantomData;
use pallet_ibc::event::primitive::{ClientId, ClientType, ConnectionId, Height, PortId, ChannelId, Packet};
use sp_core::H256;
use substrate_subxt::{balances::Balances, module, system::System, Call, Store};
use substrate_subxt_proc_macro::Event;


/// The subset of the `pallet_ibc::Trait` that a client must implement.
#[module]
pub trait Ibc: System + Balances {}

#[derive(Encode, Store)]
pub struct ClientStatesStore<T: Ibc> {
    #[store(returns = Vec<u8>)]
    pub key: Vec<u8>,
    pub _runtime: PhantomData<T>,
}

#[derive(Encode, Store)]
pub struct ConsensusStatesStore<T: Ibc> {
    #[store(returns = Vec<(Vec<u8>, Vec<u8>)>)]
    pub key: Vec<u8>,
    pub _runtime: PhantomData<T>,
}

#[derive(Encode, Store)]
pub struct ConnectionClientStore<T: Ibc> {
    #[store(returns = Vec<u8>)]
    pub key: Vec<u8>,
    pub _runtime: PhantomData<T>,
}

#[derive(Encode, Store)]
pub struct ChannelsConnectionStore<T: Ibc> {
    #[store(returns = Vec<(Vec<u8>, Vec<u8>)>)]
    pub key: Vec<u8>,
    pub _runtime: PhantomData<T>,
}

#[derive(Encode, Store)]
pub struct ChannelsStore<T: Ibc> {
    #[store(returns = Vec<u8>)]
    pub key: (Vec<u8>, Vec<u8>),
    pub _runtime: PhantomData<T>,
}

// #[derive(Encode, Store)]
// pub struct ClientStatesStore<T: Ibc> {
//     #[store(returns = Vec<u8>)]
//     pub key: H256,
//     pub _runtime: PhantomData<T>,
// }

// #[derive(Encode, Store)]
// pub struct ConsensusStatesStore<T: Ibc> {
//     #[store(returns = Vec<u8>)]
//     pub key: (H256, u32),
//     pub _runtime: PhantomData<T>,
// }

#[derive(Encode, Store)]
pub struct ConnectionsStore<T: Ibc> {
    #[store(returns = Vec<u8>)]
    pub key: Vec<u8>,
    pub _runtime: PhantomData<T>,
}



#[derive(Encode, Store)]
pub struct PacketsStore<T: Ibc> {
    #[store(returns = H256)]
    pub key: (Vec<u8>, H256, u64),
    pub _runtime: PhantomData<T>,
}

#[derive(Encode, Store)]
pub struct AcknowledgementsStore<T: Ibc> {
    #[store(returns = H256)]
    pub key: (Vec<u8>, H256, u64),
    pub _runtime: PhantomData<T>,
}

#[derive(Encode, Call)]
pub struct SubmitDatagramCall<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub datagram: Vec<u8>,
}

/// NewBlock
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct NewBlockEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
}

/// CreateClient Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct CreateClientEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub client_id: ClientId,
    pub client_type: ClientType,
    pub consensus_height: Height,
}

/// UpdateClient Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct UpdateClientEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub client_id: ClientId,
    pub client_type: ClientType,
    pub consensus_height: Height,
}


/// ClientMisbehaviour Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ClientMisbehaviourEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub client_id: ClientId,
    pub client_type: ClientType,
    pub consensus_height: Height,
}

/// OpenInitConnection Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct OpenInitConnectionEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub connection_id: Option<ConnectionId>,
    pub client_id: ClientId,
    pub counterparty_connection_id: Option<ConnectionId>,
    pub counterparty_client_id: ClientId,
}

/// OpenTryConnection Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct OpenTryConnectionEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub connection_id: Option<ConnectionId>,
    pub client_id: ClientId,
    pub counterparty_connection_id: Option<ConnectionId>,
    pub counterparty_client_id: ClientId,
}

/// OpenAckConnection Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct OpenAckConnectionEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub connection_id: Option<ConnectionId>,
    pub client_id: ClientId,
    pub counterparty_connection_id: Option<ConnectionId>,
    pub counterparty_client_id: ClientId,
}

/// OpenConfirmConnection Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct OpenConfirmConnectionEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub connection_id: Option<ConnectionId>,
    pub client_id: ClientId,
    pub counterparty_connection_id: Option<ConnectionId>,
    pub counterparty_client_id: ClientId,
}

/// OpenInitChannel Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct OpenInitChannelEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub port_id: PortId,
    pub channel_id: Option<ChannelId>,
    pub connection_id: ConnectionId,
    pub counterparty_port_id: PortId,
    pub counterparty_channel_id: Option<ChannelId>
}

/// OpenTryChannel Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct OpenTryChannelEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub port_id: PortId,
    pub channel_id: Option<ChannelId>,
    pub connection_id: ConnectionId,
    pub counterparty_port_id: PortId,
    pub counterparty_channel_id: Option<ChannelId>
}

/// OpenAckChannel Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct OpenAckChannelEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub port_id: PortId,
    pub channel_id: Option<ChannelId>,
    pub connection_id: ConnectionId,
    pub counterparty_port_id: PortId,
    pub counterparty_channel_id: Option<ChannelId>
}


/// OpenConfirmChannel Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct OpenConfirmChannelEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub port_id: PortId,
    pub channel_id: Option<ChannelId>,
    pub connection_id: ConnectionId,
    pub counterparty_port_id: PortId,
    pub counterparty_channel_id: Option<ChannelId>
}

/// CloseInitChannel Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct CloseInitChannelEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub port_id: PortId,
    pub channel_id: Option<ChannelId>,
    pub connection_id: ConnectionId,
    pub counterparty_port_id: PortId,
    pub counterparty_channel_id: Option<ChannelId>
}

/// CloseConfirmChannel Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct CloseConfirmChannelEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub port_id: PortId,
    pub channel_id: Option<ChannelId>,
    pub connection_id: ConnectionId,
    pub counterparty_port_id: PortId,
    pub counterparty_channel_id: Option<ChannelId>
}

/// SendPacket Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct SendPacketEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub packet: Packet,
}


/// ReceivePacket Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ReceivePacketEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub packet: Packet,
}


/// WriteAcknowledgement Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct WriteAcknowledgementEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub packet: Packet,
    pub ack: Vec<u8>,
}

/// AcknowledgePacket Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct AcknowledgePacketEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub packet: Packet,
}

/// TimeoutPacket Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TimeoutPacketEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub packet: Packet,
}

/// TimeoutOnClosePacket Event
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TimeoutOnClosePacketEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub height: Height,
    pub packet: Packet,
}

/// Empty(String) Special event, signifying empty response
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct EmptyEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub data: Vec<u8>,
}

/// ChainError(String) Special event, signifying an error an CheckTx or DeliverTx
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ChainErrorEvent<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub data: Vec<u8>,
}


#[derive(Encode, Call)]
pub struct DeliverCall<T: Ibc> {
    pub _runtime: PhantomData<T>,
    pub messages: Vec<pallet_ibc::Any>,
    pub tmp: u8,
}
