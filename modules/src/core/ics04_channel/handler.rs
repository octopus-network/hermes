//! This module implements the processing logic for ICS4 (channel) messages.

use crate::core::ics04_channel::channel::ChannelEnd;
use crate::core::ics04_channel::context::ChannelReader;
use crate::core::ics04_channel::error::Error;
use crate::core::ics04_channel::msgs::ChannelMsg;
use crate::core::ics04_channel::{msgs::PacketMsg, packet::PacketResult};
use crate::core::ics05_port::capabilities::Capability;
use crate::core::ics24_host::identifier::{ChannelId, PortId};
use crate::core::ics26_routing::context::Ics26Context;
use crate::handler::HandlerOutput;

pub mod acknowledgement;
pub mod chan_close_confirm;
pub mod chan_close_init;
pub mod chan_open_ack;
pub mod chan_open_confirm;
pub mod chan_open_init;
pub mod chan_open_try;
pub mod recv_packet;
pub mod send_packet;
pub mod timeout;
pub mod timeout_on_close;
pub mod verify;
pub mod write_acknowledgement;

/// Defines the possible states of a channel identifier in a `ChannelResult`.
#[derive(Clone, Debug)]
pub enum ChannelIdState {
    /// Specifies that the channel handshake handler allocated a new channel identifier. This
    /// happens during the processing of either the `MsgChannelOpenInit` or `MsgChannelOpenTry`.
    Generated,

    /// Specifies that the handler reused a previously-allocated channel identifier.
    Reused,
}

#[derive(Clone, Debug)]
pub struct ChannelResult {
    pub port_id: PortId,
    pub channel_id: ChannelId,
    pub channel_id_state: ChannelIdState,
    pub channel_cap: Capability,
    pub channel_end: ChannelEnd,
}

/// General entry point for processing any type of message related to the ICS4 channel open and
/// channel close handshake protocols.
pub fn channel_dispatch<Ctx>(
    ctx: &Ctx,
    msg: ChannelMsg,
) -> Result<HandlerOutput<ChannelResult>, Error>
where
    Ctx: ChannelReader,
{
    match msg {
        ChannelMsg::ChannelOpenInit(msg) => chan_open_init::process(ctx, msg),
        ChannelMsg::ChannelOpenTry(msg) => chan_open_try::process(ctx, msg),
        ChannelMsg::ChannelOpenAck(msg) => chan_open_ack::process(ctx, msg),
        ChannelMsg::ChannelOpenConfirm(msg) => chan_open_confirm::process(ctx, msg),
        ChannelMsg::ChannelCloseInit(msg) => chan_close_init::process(ctx, msg),
        ChannelMsg::ChannelCloseConfirm(msg) => chan_close_confirm::process(ctx, msg),
    }
}

/// Dispatcher for processing any type of message related to the ICS4 packet protocols.
pub fn packet_dispatch<Ctx>(
    ctx: &mut Ctx,
    msg: PacketMsg,
) -> Result<HandlerOutput<PacketResult>, Error>
where
    Ctx: Ics26Context,
{
    match msg {
        PacketMsg::RecvPacket(msg) => {
            use alloc::vec;
            let recv_handler_out = recv_packet::process(ctx, msg.clone())?;
            // store_packet_result not really unimploment!
            ctx.store_packet_result(recv_handler_out.result)?;

            // TODO: callback and return ack
            // let act = ctx.OnRecvPacket(msg.packet);
            // handle write ack
            let ack = vec![1]; // a mock ack value
            let packet = msg.packet;
            let write_ack_handler_out = write_acknowledgement::process(ctx, packet, ack)?;

            // 把recv packet和write ack两个事件及log合并到一起返回，但result只能返回一个，因此会丢掉另外一个result，M1阶段暂时返回write ack的result
            // 有两个方案可以考虑把多个result同时返回：
            // 方案1：修改HandlerOut中的Result类型，改为Vec<HandlerResult>，容纳多个result，但这样以来所有用到handlerout的地方都想需要重构！
            // 方案2：想办法再调用一次deliever(),然后匹配write_acknowledgement消息，做一次单独处理，改动也不小！
            // 可能还存在其他解决方案，但是需要结合需求以及目前的架构综合考虑。
            // 幸好substrate pallet 那边只需要保存height数据，其他暂时用不到，而这个height可以从event中获取！
            Ok(HandlerOutput::builder()
                .with_log(recv_handler_out.log)
                .with_events(recv_handler_out.events)
                .with_log(write_ack_handler_out.log)
                .with_events(write_ack_handler_out.events)
                .with_result(write_ack_handler_out.result))
        }
        PacketMsg::AckPacket(msg) => acknowledgement::process(ctx, msg),
        PacketMsg::ToPacket(msg) => timeout::process(ctx, msg),
        PacketMsg::ToClosePacket(msg) => timeout_on_close::process(ctx, msg),
    }
}
