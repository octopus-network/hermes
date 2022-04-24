use alloc::vec::Vec;

use super::*;
use crate::{
    applications::ics20_fungible_token_transfer::{
        context::Ics20Context, error::Error as Ics20Error,
    },
    core::{
        ics02_client::{
            client_consensus::AnyConsensusState, client_state::AnyClientState,
            context::ClientReader,
        },
        ics03_connection::{
            connection::ConnectionEnd, context::ConnectionReader, error::Error as ICS03Error,
        },
        ics04_channel::{
            channel::ChannelEnd,
            channel::Counterparty,
            channel::Order,
            context::{ChannelKeeper, ChannelReader},
            error::Error as ICS04Error,
            packet::{Packet, Receipt, Sequence},
            Version,
        },
        ics05_port::{capabilities::Capability, context::PortReader, error::Error as Ics05Error},
        ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
        ics26_routing::{context::Ics26Context, error::Error as Ics26Error},
    },
    signer::Signer,
};

pub trait IBCModule: Clone {
    // OnChanOpenInit will verify that the relayer-chosen parameters are
    // valid and perform any custom INIT logic.It may return an error if
    // the chosen parameters are invalid in which case the handshake is aborted.
    // OnChanOpenInit should return an error if the provided version is invalid.
    #![allow(clippy::too_many_arguments)]
    fn on_chan_open_init<Ctx>(
        &self,
        ctx: &Ctx,
        order: Order,
        connection_hops: Vec<ConnectionId>,
        port_id: PortId,
        channel_id: ChannelId,
        channel_cap: &Capability,
        counterparty: Counterparty,
        version: Version,
    ) -> Result<(), Ics20Error>
    where
        Ctx: Ics20Context;

    // OnChanOpenTry will verify the relayer-chosen parameters along with the
    // counterparty-chosen version string and perform custom TRY logic.
    // If the relayer-chosen parameters are invalid, the callback must return
    // an error to abort the handshake. If the counterparty-chosen version is not
    // compatible with this modules supported versions, the callback must return
    // an error to abort the handshake. If the versions are compatible, the try callback
    // must select the final version string and return it to core IBC.
    // OnChanOpenTry may also perform custom initialization logic
    fn on_chan_open_try<Ctx>(
        &self,
        ctx: &Ctx,
        order: Order,
        connection_hops: Vec<ConnectionId>,
        port_id: PortId,
        channel_id: ChannelId,
        channel_cap: &Capability,
        counterparty: Counterparty,
        counterparty_version: Version,
    ) -> Result<Version, Ics20Error>
    where
        Ctx: Ics20Context;

    // OnChanOpenAck will error if the counterparty selected version string
    // is invalid to abort the handshake. It may also perform custom ACK logic.
    fn on_chan_open_ack<Ctx>(
        &self,
        ctx: &Ctx,
        port_id: PortId,
        channel_id: ChannelId,
        counterparty_version: Version,
    ) -> Result<(), Ics20Error>
    where
        Ctx: Ics20Context;

    // OnChanOpenConfirm will perform custom CONFIRM logic and may error to abort the handshake.
    fn on_chan_open_confirm<Ctx>(
        &self,
        ctx: &Ctx,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<(), Ics20Error>
    where
        Ctx: Ics20Context;

    fn on_chan_close_init<Ctx>(
        &self,
        ctx: &Ctx,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<(), Ics20Error>
    where
        Ctx: Ics20Context;

    fn on_chan_close_confirm<Ctx>(
        &self,
        ctx: &Ctx,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<(), Ics20Error>
    where
        Ctx: Ics20Context;

    // OnRecvPacket must return an acknowledgement that implements the Acknowledgement interface.
    // In the case of an asynchronous acknowledgement, nil should be returned.
    // If the acknowledgement returned is successful, the state changes on callback are written,
    // otherwise the application state changes are discarded. In either case the packet is received
    // and the acknowledgement is written (in synchronous cases).
    fn on_recv_packet<Ctx>(
        &self,
        ctx: &Ctx,
        packet: Packet,
        relayer: Signer,
    ) -> Result<Vec<u8>, Ics20Error>
    where
        Ctx: Ics20Context;

    fn on_acknowledgement_packet<Ctx>(
        &self,
        ctx: &Ctx,
        packet: Packet,
        acknowledgement: Vec<u8>,
        relayer: Signer,
    ) -> Result<(), Ics20Error>
    where
        Ctx: Ics20Context;

    fn on_timeout_packet<Ctx>(
        &self,
        ctx: &Ctx,
        packet: Packet,
        relayer: Signer,
    ) -> Result<(), Ics20Error>
    where
        Ctx: Ics20Context;
}
