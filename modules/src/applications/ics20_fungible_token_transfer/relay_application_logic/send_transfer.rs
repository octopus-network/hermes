use crate::applications::ics20_fungible_token_transfer::context::Ics20Context;
use crate::applications::ics20_fungible_token_transfer::error::Error;
use crate::applications::ics20_fungible_token_transfer::msgs::fungible_token_packet_data::FungibleTokenPacketData;
use crate::applications::ics20_fungible_token_transfer::msgs::transfer::MsgTransfer;
use crate::core::ics04_channel::handler::send_packet::send_packet;
use crate::core::ics04_channel::packet::Packet;
use crate::core::ics04_channel::packet::PacketResult;
use crate::handler::HandlerOutput;
use crate::prelude::*;
use crate::signer::Signer;

pub(crate) fn send_transfer<Ctx>(
    ctx: &Ctx,
    msg: MsgTransfer,
) -> Result<HandlerOutput<PacketResult>, Error>
where
    Ctx: Ics20Context,
{
    let source_channel_end = ctx
        .channel_end(&(msg.source_port.clone(), msg.source_channel.clone()))
        .map_err(Error::ics04_channel)?;

    let destination_port = source_channel_end.counterparty().port_id().clone();
    let destination_channel = source_channel_end
        .counterparty()
        .channel_id()
        .ok_or_else(|| {
            Error::destination_channel_not_found(
                msg.source_port.clone(),
                msg.source_channel.clone(),
            )
        })?;

    // get the next sequence
    let sequence = ctx
        .get_next_sequence_send(&(msg.source_port.clone(), msg.source_channel.clone()))
        .map_err(Error::ics04_channel)?;

    tracing::trace!(target:"ibc-rs::ics20","🤮in ics20 [send_transfer]: sequence = {:?}",sequence);
    // log::trace!(target:"ibc-rs::ics20","🤮in ics20 [send_transfer]: sequence = {:?}",sequence);
    //TODO: Application LOGIC.

    //TODO: build packet data
    // refer to ibc-go https://github.com/octopus-network/ibc-go/blob/e40cdec6a3413fb3c8ea2a7ccad5e363ecd5a695/modules/apps/transfer/keeper/relay.go#L146
    // packetData := types.NewFungibleTokenPacketData(
    // 	fullDenomPath, token.Amount.String(), sender.String(), receiver,
    // )

    tracing::trace!(target:"ibc-rs::ics20","🤮in ics20 [send_transfer]: token = {:?}", msg.token.clone());
    // log::trace!(target:"ibc-rs::ics20","🤮in ics20 [send_transfer]: token = {:?}", msg.token.clone());

    let denom = msg.token.clone().ok_or(Error::empty_token())?.denom;
    let amount = msg.token.ok_or(Error::empty_token())?.amount;

    // contruct fungible token packet data
    let packet_data = FungibleTokenPacketData {
        denom: denom,
        amount: amount,
        sender: msg.sender,
        receiver: msg.receiver,
    };

    // endocde packet data
    let encode_packet_data =
        serde_json::to_vec(&packet_data).map_err(|_| Error::invalid_serde_data())?;

    let packet = Packet {
        sequence,
        source_port: msg.source_port,
        source_channel: msg.source_channel,
        destination_port,
        destination_channel: destination_channel.clone(),
        data: encode_packet_data,
        timeout_height: msg.timeout_height,
        timeout_timestamp: msg.timeout_timestamp,
    };

    // tracing::trace!(target:"ibc-rs::ics20","🤮in ics20 [send_transfer]: packet = {:?}", packet);
    log::trace!(target:"ibc-rs::ics20","🤮in ics20 [send_transfer]: packet = {:?}", packet);
    let handler_output = send_packet(ctx, packet).map_err(Error::ics04_channel)?;

    //TODO:  add event/atributes and writes to the store issued by the application logic for packet sending.
    Ok(handler_output)
}
