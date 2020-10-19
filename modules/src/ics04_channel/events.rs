//! Types for the IBC events emitted from Tendermint Websocket by the channels module.
use crate::attribute;
use crate::events::{IBCEvent, RawObject};
use crate::ics02_client::height::Height;
use crate::ics04_channel::channel::Order;
use crate::ics24_host::identifier::{ChannelId, ConnectionId, PortId};
use anomaly::BoxError;
use serde_derive::{Deserialize, Serialize};
use std::convert::TryFrom;
use tendermint::block;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenInit {
    pub height: block::Height,
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub counterparty_port_id: PortId,
    pub counterparty_channel_id: ChannelId,
    pub connection_id: ConnectionId,
}

impl TryFrom<RawObject> for OpenInit {
    type Error = BoxError;
    fn try_from(obj: RawObject) -> Result<Self, Self::Error> {
        Ok(OpenInit {
            height: obj.height,
            port_id: attribute!(obj, "channel_open_init.port_id"),
            channel_id: attribute!(obj, "channel_open_init.channel_id"),
            counterparty_port_id: attribute!(obj, "channel_open_init.counterparty_port_id"),
            counterparty_channel_id: attribute!(obj, "channel_open_init.counterparty_channel_id"),
            connection_id: attribute!(obj, "channel_open_init.connection_id"),
        })
    }
}

impl From<OpenInit> for IBCEvent {
    fn from(v: OpenInit) -> Self {
        IBCEvent::OpenInitChannel(v)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenTry {
    pub height: block::Height,
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub counterparty_port_id: PortId,
    pub counterparty_channel_id: ChannelId,
    pub connection_id: ConnectionId,
}

impl TryFrom<RawObject> for OpenTry {
    type Error = BoxError;
    fn try_from(obj: RawObject) -> Result<Self, Self::Error> {
        Ok(OpenTry {
            height: obj.height,
            port_id: attribute!(obj, "channel_open_try.port_id"),
            channel_id: attribute!(obj, "channel_open_try.channel_id"),
            counterparty_port_id: attribute!(obj, "channel_open_try.counterparty_port_id"),
            counterparty_channel_id: attribute!(obj, "channel_open_try.counterparty_channel_id"),
            connection_id: attribute!(obj, "channel_open_try.connection_id"),
        })
    }
}

impl From<OpenTry> for IBCEvent {
    fn from(v: OpenTry) -> Self {
        IBCEvent::OpenTryChannel(v)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAck {
    pub height: block::Height,
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub counterparty_port_id: PortId,
    pub counterparty_channel_id: ChannelId,
    pub connection_id: ConnectionId,
}

impl TryFrom<RawObject> for OpenAck {
    type Error = BoxError;
    fn try_from(obj: RawObject) -> Result<Self, Self::Error> {
        Ok(OpenAck {
            height: obj.height,
            port_id: attribute!(obj, "channel_open_ack.port_id"),
            channel_id: attribute!(obj, "channel_open_ack.channel_id"),
            counterparty_port_id: attribute!(obj, "channel_open_ack.counterparty_port_id"),
            counterparty_channel_id: attribute!(obj, "channel_open_ack.counterparty_channel_id"),
            connection_id: attribute!(obj, "channel_open_ack.connection_id"),
        })
    }
}

impl From<OpenAck> for IBCEvent {
    fn from(v: OpenAck) -> Self {
        IBCEvent::OpenAckChannel(v)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenConfirm {
    pub height: block::Height,
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub counterparty_port_id: PortId,
    pub counterparty_channel_id: ChannelId,
    pub connection_id: ConnectionId,
}

impl TryFrom<RawObject> for OpenConfirm {
    type Error = BoxError;
    fn try_from(obj: RawObject) -> Result<Self, Self::Error> {
        Ok(OpenConfirm {
            height: obj.height,
            port_id: attribute!(obj, "channel_open_confirm.port_id"),
            channel_id: attribute!(obj, "channel_open_confirm.channel_id"),
            counterparty_port_id: attribute!(obj, "channel_open_confirm.counterparty_port_id"),
            counterparty_channel_id: attribute!(
                obj,
                "channel_open_confirm.counterparty_channel_id"
            ),
            connection_id: attribute!(obj, "channel_open_confirm.connection_id"),
        })
    }
}

impl From<OpenConfirm> for IBCEvent {
    fn from(v: OpenConfirm) -> Self {
        IBCEvent::OpenConfirmChannel(v)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CloseInit {
    pub height: block::Height,
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub counterparty_port_id: PortId,
    pub counterparty_channel_id: ChannelId,
    pub connection_id: ConnectionId,
}

impl TryFrom<RawObject> for CloseInit {
    type Error = BoxError;
    fn try_from(obj: RawObject) -> Result<Self, Self::Error> {
        Ok(CloseInit {
            height: obj.height,
            port_id: attribute!(obj, "channel_close_init.port_id"),
            channel_id: attribute!(obj, "channel_close_init.channel_id"),
            counterparty_port_id: attribute!(obj, "channel_close_init.counterparty_port_id"),
            counterparty_channel_id: attribute!(obj, "channel_close_init.counterparty_channel_id"),
            connection_id: attribute!(obj, "channel_close_init.connection_id"),
        })
    }
}

impl From<CloseInit> for IBCEvent {
    fn from(v: CloseInit) -> Self {
        IBCEvent::CloseInitChannel(v)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CloseConfirm {
    pub height: block::Height,
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub counterparty_port_id: PortId,
    pub counterparty_channel_id: ChannelId,
    pub connection_id: ConnectionId,
}

impl TryFrom<RawObject> for CloseConfirm {
    type Error = BoxError;
    fn try_from(obj: RawObject) -> Result<Self, Self::Error> {
        Ok(CloseConfirm {
            height: obj.height,
            port_id: attribute!(obj, "channel_close_confirm.port_id"),
            channel_id: attribute!(obj, "channel_close_confirm.channel_id"),
            counterparty_port_id: attribute!(obj, "channel_close_confirm.counterparty_port_id"),
            counterparty_channel_id: attribute!(
                obj,
                "channel_close_confirm.counterparty_channel_id"
            ),
            connection_id: attribute!(obj, "channel_close_confirm.connection_id"),
        })
    }
}

impl From<CloseConfirm> for IBCEvent {
    fn from(v: CloseConfirm) -> Self {
        IBCEvent::CloseConfirmChannel(v)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SendPacket {
    pub height: block::Height,
    pub packet_data: Vec<u8>,
    pub packet_timeout_height: Height,
    pub packet_timeout_timestamp: u64,
    pub packet_sequence: u64,
    pub packet_src_port: PortId,
    pub packet_src_channel: ChannelId,
    pub packet_dst_port: PortId,
    pub packet_dst_channel: ChannelId,
    pub packet_channel_ordering: Order,
}

impl TryFrom<RawObject> for SendPacket {
    type Error = BoxError;
    fn try_from(obj: RawObject) -> Result<Self, Self::Error> {
        Ok(SendPacket {
            height: obj.height,
            packet_src_port: attribute!(obj, "send_packet.packet_src_port"),
            packet_src_channel: attribute!(obj, "send_packet.packet_src_channel"),
            packet_dst_port: attribute!(obj, "send_packet.packet_dst_port"),
            packet_dst_channel: attribute!(obj, "send_packet.packet_dst_channel"),
            packet_sequence: attribute!(obj, "send_packet.packet_sequence"),
            packet_timeout_height: attribute!(obj, "send_packet.packet_timeout_height"),
            packet_timeout_timestamp: attribute!(obj, "send_packet.packet_timeout_timestamp"),
            packet_channel_ordering: attribute!(obj, "send_packet.packet_channel_ordering"),
            packet_data: obj
                .events
                .get("send_packet.packet_data")
                .ok_or("send_packet.packet_data")?[obj.idx]
                .as_str()
                .as_bytes()
                .to_vec(),
        })
    }
}

impl From<SendPacket> for IBCEvent {
    fn from(v: SendPacket) -> Self {
        IBCEvent::SendPacketChannel(v)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ReceivePacket {
    pub height: block::Height,
    pub packet_data: Vec<u8>,
    pub packet_timeout_height: Height,
    pub packet_timeout_timestamp: u64,
    pub packet_sequence: u64,
    pub packet_src_port: PortId,
    pub packet_src_channel: ChannelId,
    pub packet_dst_port: PortId,
    pub packet_dst_channel: ChannelId,
    pub packet_channel_ordering: Order,
}

impl TryFrom<RawObject> for ReceivePacket {
    type Error = BoxError;
    fn try_from(obj: RawObject) -> Result<Self, Self::Error> {
        Ok(ReceivePacket {
            height: obj.height,
            packet_src_port: attribute!(obj, "recv_packet.packet_src_port"),
            packet_src_channel: attribute!(obj, "recv_packet.packet_src_channel"),
            packet_dst_port: attribute!(obj, "recv_packet.packet_dst_port"),
            packet_dst_channel: attribute!(obj, "recv_packet.packet_dst_channel"),
            packet_sequence: attribute!(obj, "recv_packet.packet_sequence"),
            packet_timeout_height: attribute!(obj, "recv_packet.packet_timeout_height"),
            packet_timeout_timestamp: attribute!(obj, "recv_packet.packet_timeout_timestamp"),
            packet_channel_ordering: attribute!(obj, "recv_packet.packet_channel_ordering"),
            packet_data: obj
                .events
                .get("recv_packet.packet_data")
                .ok_or("recv_packet.packet_data")?[obj.idx]
                .as_str()
                .as_bytes()
                .to_vec(),
        })
    }
}

impl From<ReceivePacket> for IBCEvent {
    fn from(v: ReceivePacket) -> Self {
        IBCEvent::ReceivePacketChannel(v)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AcknowledgePacket {
    pub height: block::Height,
    pub packet_timeout_height: Height,
    pub packet_timeout_timestamp: u64,
    pub packet_sequence: u64,
    pub packet_src_port: PortId,
    pub packet_src_channel: ChannelId,
    pub packet_dst_port: PortId,
    pub packet_dst_channel: ChannelId,
    pub packet_channel_ordering: Order,
}

impl TryFrom<RawObject> for AcknowledgePacket {
    type Error = BoxError;
    fn try_from(obj: RawObject) -> Result<Self, Self::Error> {
        Ok(AcknowledgePacket {
            height: obj.height,
            packet_src_port: attribute!(obj, "acknowledge_packet.packet_src_port"),
            packet_src_channel: attribute!(obj, "acknowledge_packet.packet_src_channel"),
            packet_dst_port: attribute!(obj, "acknowledge_packet.packet_dst_port"),
            packet_dst_channel: attribute!(obj, "acknowledge_packet.packet_dst_channel"),
            packet_sequence: attribute!(obj, "acknowledge_packet.packet_sequence"),
            packet_timeout_height: attribute!(obj, "acknowledge_packet.packet_timeout_height"),
            packet_timeout_timestamp: attribute!(
                obj,
                "acknowledge_packet.packet_timeout_timestamp"
            ),
            packet_channel_ordering: attribute!(obj, "acknowledge_packet.packet_channel_ordering"),
        })
    }
}

impl From<AcknowledgePacket> for IBCEvent {
    fn from(v: AcknowledgePacket) -> Self {
        IBCEvent::AcknowledgePacketChannel(v)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TimeoutPacket {
    pub height: block::Height,
    pub packet_timeout_height: Height,
    pub packet_timeout_timestamp: u64,
    pub packet_sequence: u64,
    pub packet_src_port: PortId,
    pub packet_src_channel: ChannelId,
    pub packet_dst_port: PortId,
    pub packet_dst_channel: ChannelId,
    pub packet_channel_ordering: Order,
}

impl TryFrom<RawObject> for TimeoutPacket {
    type Error = BoxError;
    fn try_from(obj: RawObject) -> Result<Self, Self::Error> {
        Ok(TimeoutPacket {
            height: obj.height,
            packet_src_port: attribute!(obj, "timeout_packet.packet_src_port"),
            packet_src_channel: attribute!(obj, "timeout_packet.packet_src_channel"),
            packet_dst_port: attribute!(obj, "timeout_packet.packet_dst_port"),
            packet_dst_channel: attribute!(obj, "timeout_packet.packet_dst_channel"),
            packet_sequence: attribute!(obj, "timeout_packet.packet_sequence"),
            packet_timeout_height: attribute!(obj, "timeout_packet.packet_timeout_height"),
            packet_timeout_timestamp: attribute!(obj, "timeout_packet.packet_timeout_timestamp"),
            packet_channel_ordering: attribute!(obj, "timeout_packet.packet_channel_ordering"),
        })
    }
}

impl From<TimeoutPacket> for IBCEvent {
    fn from(v: TimeoutPacket) -> Self {
        IBCEvent::TimeoutPacketChannel(v)
    }
}
