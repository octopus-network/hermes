use crate::prelude::*;

use alloc::borrow::Cow;
use core::convert::{TryFrom, TryInto};
use core::fmt::{Display, Error as FmtError, Formatter};
use core::str::FromStr;
use serde_derive::{Deserialize, Serialize};
use tendermint::abci::Event as AbciEvent;

use crate::core::ics02_client::events::NewBlock;
use ibc::core::ics02_client::events as ClientEvents;
use ibc::core::ics03_connection::events as ConnectionEvents;
use ibc::core::ics03_connection::events::Attributes as ConnectionAttributes;
use ibc::core::ics04_channel::events as ChannelEvents;
use ibc::core::ics04_channel::events::Attributes as ChannelAttributes;
use ibc::core::ics04_channel::packet::Packet;
use ibc::events::Error;
use ibc::events::ModuleEvent;

/// Events whose data is not included in the app state and must be extracted using tendermint RPCs
/// (i.e. /tx_search or /block_search)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum WithBlockDataType {
    CreateClient,
    UpdateClient,
    SendPacket,
    WriteAck,
}

impl WithBlockDataType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            WithBlockDataType::CreateClient => "create_client",
            WithBlockDataType::UpdateClient => "update_client",
            WithBlockDataType::SendPacket => "send_packet",
            WithBlockDataType::WriteAck => "write_acknowledgement",
        }
    }
}

const NEW_BLOCK_EVENT: &str = "new_block";
const EMPTY_EVENT: &str = "empty";
const CHAIN_ERROR_EVENT: &str = "chain_error";
const APP_MODULE_EVENT: &str = "app_module";
/// Client event types
const CREATE_CLIENT_EVENT: &str = "create_client";
const UPDATE_CLIENT_EVENT: &str = "update_client";
const CLIENT_MISBEHAVIOUR_EVENT: &str = "client_misbehaviour";
const UPGRADE_CLIENT_EVENT: &str = "upgrade_client";
/// Connection event types
const CONNECTION_INIT_EVENT: &str = "connection_open_init";
const CONNECTION_TRY_EVENT: &str = "connection_open_try";
const CONNECTION_ACK_EVENT: &str = "connection_open_ack";
const CONNECTION_CONFIRM_EVENT: &str = "connection_open_confirm";
/// Channel event types
const CHANNEL_OPEN_INIT_EVENT: &str = "channel_open_init";
const CHANNEL_OPEN_TRY_EVENT: &str = "channel_open_try";
const CHANNEL_OPEN_ACK_EVENT: &str = "channel_open_ack";
const CHANNEL_OPEN_CONFIRM_EVENT: &str = "channel_open_confirm";
const CHANNEL_CLOSE_INIT_EVENT: &str = "channel_close_init";
const CHANNEL_CLOSE_CONFIRM_EVENT: &str = "channel_close_confirm";
/// Packet event types
const SEND_PACKET_EVENT: &str = "send_packet";
const RECEIVE_PACKET_EVENT: &str = "receive_packet";
const WRITE_ACK_EVENT: &str = "write_acknowledgement";
const ACK_PACKET_EVENT: &str = "acknowledge_packet";
const TIMEOUT_EVENT: &str = "timeout_packet";
const TIMEOUT_ON_CLOSE_EVENT: &str = "timeout_packet_on_close";

/// Events types
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum IbcEventType {
    NewBlock,
    CreateClient,
    UpdateClient,
    UpgradeClient,
    ClientMisbehaviour,
    OpenInitConnection,
    OpenTryConnection,
    OpenAckConnection,
    OpenConfirmConnection,
    OpenInitChannel,
    OpenTryChannel,
    OpenAckChannel,
    OpenConfirmChannel,
    CloseInitChannel,
    CloseConfirmChannel,
    SendPacket,
    ReceivePacket,
    WriteAck,
    AckPacket,
    Timeout,
    TimeoutOnClose,
    AppModule,
    Empty,
    ChainError,
}

impl IbcEventType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            IbcEventType::NewBlock => NEW_BLOCK_EVENT,
            IbcEventType::CreateClient => CREATE_CLIENT_EVENT,
            IbcEventType::UpdateClient => UPDATE_CLIENT_EVENT,
            IbcEventType::UpgradeClient => UPGRADE_CLIENT_EVENT,
            IbcEventType::ClientMisbehaviour => CLIENT_MISBEHAVIOUR_EVENT,
            IbcEventType::OpenInitConnection => CONNECTION_INIT_EVENT,
            IbcEventType::OpenTryConnection => CONNECTION_TRY_EVENT,
            IbcEventType::OpenAckConnection => CONNECTION_ACK_EVENT,
            IbcEventType::OpenConfirmConnection => CONNECTION_CONFIRM_EVENT,
            IbcEventType::OpenInitChannel => CHANNEL_OPEN_INIT_EVENT,
            IbcEventType::OpenTryChannel => CHANNEL_OPEN_TRY_EVENT,
            IbcEventType::OpenAckChannel => CHANNEL_OPEN_ACK_EVENT,
            IbcEventType::OpenConfirmChannel => CHANNEL_OPEN_CONFIRM_EVENT,
            IbcEventType::CloseInitChannel => CHANNEL_CLOSE_INIT_EVENT,
            IbcEventType::CloseConfirmChannel => CHANNEL_CLOSE_CONFIRM_EVENT,
            IbcEventType::SendPacket => SEND_PACKET_EVENT,
            IbcEventType::ReceivePacket => RECEIVE_PACKET_EVENT,
            IbcEventType::WriteAck => WRITE_ACK_EVENT,
            IbcEventType::AckPacket => ACK_PACKET_EVENT,
            IbcEventType::Timeout => TIMEOUT_EVENT,
            IbcEventType::TimeoutOnClose => TIMEOUT_ON_CLOSE_EVENT,
            IbcEventType::AppModule => APP_MODULE_EVENT,
            IbcEventType::Empty => EMPTY_EVENT,
            IbcEventType::ChainError => CHAIN_ERROR_EVENT,
        }
    }
}

impl FromStr for IbcEventType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            NEW_BLOCK_EVENT => Ok(IbcEventType::NewBlock),
            CREATE_CLIENT_EVENT => Ok(IbcEventType::CreateClient),
            UPDATE_CLIENT_EVENT => Ok(IbcEventType::UpdateClient),
            UPGRADE_CLIENT_EVENT => Ok(IbcEventType::UpgradeClient),
            CLIENT_MISBEHAVIOUR_EVENT => Ok(IbcEventType::ClientMisbehaviour),
            CONNECTION_INIT_EVENT => Ok(IbcEventType::OpenInitConnection),
            CONNECTION_TRY_EVENT => Ok(IbcEventType::OpenTryConnection),
            CONNECTION_ACK_EVENT => Ok(IbcEventType::OpenAckConnection),
            CONNECTION_CONFIRM_EVENT => Ok(IbcEventType::OpenConfirmConnection),
            CHANNEL_OPEN_INIT_EVENT => Ok(IbcEventType::OpenInitChannel),
            CHANNEL_OPEN_TRY_EVENT => Ok(IbcEventType::OpenTryChannel),
            CHANNEL_OPEN_ACK_EVENT => Ok(IbcEventType::OpenAckChannel),
            CHANNEL_OPEN_CONFIRM_EVENT => Ok(IbcEventType::OpenConfirmChannel),
            CHANNEL_CLOSE_INIT_EVENT => Ok(IbcEventType::CloseInitChannel),
            CHANNEL_CLOSE_CONFIRM_EVENT => Ok(IbcEventType::CloseConfirmChannel),
            SEND_PACKET_EVENT => Ok(IbcEventType::SendPacket),
            RECEIVE_PACKET_EVENT => Ok(IbcEventType::ReceivePacket),
            WRITE_ACK_EVENT => Ok(IbcEventType::WriteAck),
            ACK_PACKET_EVENT => Ok(IbcEventType::AckPacket),
            TIMEOUT_EVENT => Ok(IbcEventType::Timeout),
            TIMEOUT_ON_CLOSE_EVENT => Ok(IbcEventType::TimeoutOnClose),
            EMPTY_EVENT => Ok(IbcEventType::Empty),
            CHAIN_ERROR_EVENT => Ok(IbcEventType::ChainError),
            // from_str() for `APP_MODULE_EVENT` MUST fail because a `ModuleEvent`'s type isn't constant
            _ => Err(Error::incorrect_event_type(s.to_string())),
        }
    }
}

/// Events created by the IBC component of a chain, destined for a relayer.
#[derive(Debug, Clone, Serialize)]
pub enum IbcEvent {
    NewBlock(NewBlock),

    CreateClient(ClientEvents::CreateClient),
    UpdateClient(ClientEvents::UpdateClient),
    UpgradeClient(ClientEvents::UpgradeClient),
    ClientMisbehaviour(ClientEvents::ClientMisbehaviour),

    OpenInitConnection(ConnectionEvents::OpenInit),
    OpenTryConnection(ConnectionEvents::OpenTry),
    OpenAckConnection(ConnectionEvents::OpenAck),
    OpenConfirmConnection(ConnectionEvents::OpenConfirm),

    OpenInitChannel(ChannelEvents::OpenInit),
    OpenTryChannel(ChannelEvents::OpenTry),
    OpenAckChannel(ChannelEvents::OpenAck),
    OpenConfirmChannel(ChannelEvents::OpenConfirm),
    CloseInitChannel(ChannelEvents::CloseInit),
    CloseConfirmChannel(ChannelEvents::CloseConfirm),

    SendPacket(ChannelEvents::SendPacket),
    ReceivePacket(ChannelEvents::ReceivePacket),
    WriteAcknowledgement(ChannelEvents::WriteAcknowledgement),
    AcknowledgePacket(ChannelEvents::AcknowledgePacket),
    TimeoutPacket(ChannelEvents::TimeoutPacket),
    TimeoutOnClosePacket(ChannelEvents::TimeoutOnClosePacket),

    AppModule(ModuleEvent),

    ChainError(String), // Special event, signifying an error on CheckTx or DeliverTx
}

impl Display for IbcEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        match self {
            IbcEvent::NewBlock(ev) => write!(f, "NewBlock({})", ev.height),

            IbcEvent::CreateClient(ev) => write!(f, "CreateClient({})", ev),
            IbcEvent::UpdateClient(ev) => write!(f, "UpdateClient({})", ev),
            IbcEvent::UpgradeClient(ev) => write!(f, "UpgradeClient({})", ev),
            IbcEvent::ClientMisbehaviour(ev) => write!(f, "ClientMisbehaviour({})", ev),

            IbcEvent::OpenInitConnection(ev) => write!(f, "OpenInitConnection({})", ev),
            IbcEvent::OpenTryConnection(ev) => write!(f, "OpenTryConnection({})", ev),
            IbcEvent::OpenAckConnection(ev) => write!(f, "OpenAckConnection({})", ev),
            IbcEvent::OpenConfirmConnection(ev) => write!(f, "OpenConfirmConnection({})", ev),

            IbcEvent::OpenInitChannel(ev) => write!(f, "OpenInitChannel({})", ev),
            IbcEvent::OpenTryChannel(ev) => write!(f, "OpenTryChannel({})", ev),
            IbcEvent::OpenAckChannel(ev) => write!(f, "OpenAckChannel({})", ev),
            IbcEvent::OpenConfirmChannel(ev) => write!(f, "OpenConfirmChannel({})", ev),
            IbcEvent::CloseInitChannel(ev) => write!(f, "CloseInitChannel({})", ev),
            IbcEvent::CloseConfirmChannel(ev) => write!(f, "CloseConfirmChannel({})", ev),

            IbcEvent::SendPacket(ev) => write!(f, "SendPacket({})", ev),
            IbcEvent::ReceivePacket(ev) => write!(f, "ReceivePacket({})", ev),
            IbcEvent::WriteAcknowledgement(ev) => write!(f, "WriteAcknowledgement({})", ev),
            IbcEvent::AcknowledgePacket(ev) => write!(f, "AcknowledgePacket({})", ev),
            IbcEvent::TimeoutPacket(ev) => write!(f, "TimeoutPacket({})", ev),
            IbcEvent::TimeoutOnClosePacket(ev) => write!(f, "TimeoutOnClosePacket({})", ev),

            IbcEvent::AppModule(ev) => write!(f, "AppModule({})", ev),

            IbcEvent::ChainError(ev) => write!(f, "ChainError({})", ev),
        }
    }
}

impl TryFrom<IbcEvent> for AbciEvent {
    type Error = Error;

    fn try_from(event: IbcEvent) -> Result<Self, Self::Error> {
        Ok(match event {
            IbcEvent::CreateClient(event) => event.into(),
            IbcEvent::UpdateClient(event) => event.into(),
            IbcEvent::UpgradeClient(event) => event.into(),
            IbcEvent::ClientMisbehaviour(event) => event.into(),
            IbcEvent::OpenInitConnection(event) => event.into(),
            IbcEvent::OpenTryConnection(event) => event.into(),
            IbcEvent::OpenAckConnection(event) => event.into(),
            IbcEvent::OpenConfirmConnection(event) => event.into(),
            IbcEvent::OpenInitChannel(event) => event.into(),
            IbcEvent::OpenTryChannel(event) => event.into(),
            IbcEvent::OpenAckChannel(event) => event.into(),
            IbcEvent::OpenConfirmChannel(event) => event.into(),
            IbcEvent::CloseInitChannel(event) => event.into(),
            IbcEvent::CloseConfirmChannel(event) => event.into(),
            IbcEvent::SendPacket(event) => event.try_into().map_err(Error::channel)?,
            IbcEvent::ReceivePacket(event) => event.try_into().map_err(Error::channel)?,
            IbcEvent::WriteAcknowledgement(event) => event.try_into().map_err(Error::channel)?,
            IbcEvent::AcknowledgePacket(event) => event.try_into().map_err(Error::channel)?,
            IbcEvent::TimeoutPacket(event) => event.try_into().map_err(Error::channel)?,
            IbcEvent::TimeoutOnClosePacket(event) => event.try_into().map_err(Error::channel)?,
            IbcEvent::AppModule(event) => event.try_into()?,
            IbcEvent::NewBlock(_) | IbcEvent::ChainError(_) => {
                return Err(Error::incorrect_event_type(event.to_string()))
            }
        })
    }
}

impl IbcEvent {
    pub fn to_json(&self) -> String {
        match serde_json::to_string(self) {
            Ok(value) => value,
            Err(_) => format!("{:?}", self), // Fallback to debug printing
        }
    }

    pub fn event_type(&self) -> IbcEventType {
        match self {
            IbcEvent::NewBlock(_) => IbcEventType::NewBlock,
            IbcEvent::CreateClient(_) => IbcEventType::CreateClient,
            IbcEvent::UpdateClient(_) => IbcEventType::UpdateClient,
            IbcEvent::ClientMisbehaviour(_) => IbcEventType::ClientMisbehaviour,
            IbcEvent::UpgradeClient(_) => IbcEventType::UpgradeClient,
            IbcEvent::OpenInitConnection(_) => IbcEventType::OpenInitConnection,
            IbcEvent::OpenTryConnection(_) => IbcEventType::OpenTryConnection,
            IbcEvent::OpenAckConnection(_) => IbcEventType::OpenAckConnection,
            IbcEvent::OpenConfirmConnection(_) => IbcEventType::OpenConfirmConnection,
            IbcEvent::OpenInitChannel(_) => IbcEventType::OpenInitChannel,
            IbcEvent::OpenTryChannel(_) => IbcEventType::OpenTryChannel,
            IbcEvent::OpenAckChannel(_) => IbcEventType::OpenAckChannel,
            IbcEvent::OpenConfirmChannel(_) => IbcEventType::OpenConfirmChannel,
            IbcEvent::CloseInitChannel(_) => IbcEventType::CloseInitChannel,
            IbcEvent::CloseConfirmChannel(_) => IbcEventType::CloseConfirmChannel,
            IbcEvent::SendPacket(_) => IbcEventType::SendPacket,
            IbcEvent::ReceivePacket(_) => IbcEventType::ReceivePacket,
            IbcEvent::WriteAcknowledgement(_) => IbcEventType::WriteAck,
            IbcEvent::AcknowledgePacket(_) => IbcEventType::AckPacket,
            IbcEvent::TimeoutPacket(_) => IbcEventType::Timeout,
            IbcEvent::TimeoutOnClosePacket(_) => IbcEventType::TimeoutOnClose,
            IbcEvent::AppModule(_) => IbcEventType::AppModule,
            IbcEvent::ChainError(_) => IbcEventType::ChainError,
        }
    }

    pub fn channel_attributes(self) -> Option<ChannelAttributes> {
        match self {
            IbcEvent::OpenInitChannel(ev) => Some(ev.into()),
            IbcEvent::OpenTryChannel(ev) => Some(ev.into()),
            IbcEvent::OpenAckChannel(ev) => Some(ev.into()),
            IbcEvent::OpenConfirmChannel(ev) => Some(ev.into()),
            _ => None,
        }
    }

    pub fn connection_attributes(&self) -> Option<&ConnectionAttributes> {
        match self {
            IbcEvent::OpenInitConnection(ev) => Some(ev.attributes()),
            IbcEvent::OpenTryConnection(ev) => Some(ev.attributes()),
            IbcEvent::OpenAckConnection(ev) => Some(ev.attributes()),
            IbcEvent::OpenConfirmConnection(ev) => Some(ev.attributes()),
            _ => None,
        }
    }

    pub fn packet(&self) -> Option<&Packet> {
        match self {
            IbcEvent::SendPacket(ev) => Some(&ev.packet),
            IbcEvent::ReceivePacket(ev) => Some(&ev.packet),
            IbcEvent::WriteAcknowledgement(ev) => Some(&ev.packet),
            IbcEvent::AcknowledgePacket(ev) => Some(&ev.packet),
            IbcEvent::TimeoutPacket(ev) => Some(&ev.packet),
            IbcEvent::TimeoutOnClosePacket(ev) => Some(&ev.packet),
            _ => None,
        }
    }

    pub fn ack(&self) -> Option<&[u8]> {
        match self {
            IbcEvent::WriteAcknowledgement(ev) => Some(&ev.ack),
            _ => None,
        }
    }
}

impl From<ClientEvents::CreateClient> for IbcEvent {
    fn from(v: ClientEvents::CreateClient) -> Self {
        IbcEvent::CreateClient(v)
    }
}

impl From<ClientEvents::UpdateClient> for IbcEvent {
    fn from(v: ClientEvents::UpdateClient) -> Self {
        IbcEvent::UpdateClient(v)
    }
}

impl From<ClientEvents::ClientMisbehaviour> for IbcEvent {
    fn from(v: ClientEvents::ClientMisbehaviour) -> Self {
        IbcEvent::ClientMisbehaviour(v)
    }
}

impl From<ClientEvents::UpgradeClient> for IbcEvent {
    fn from(v: ClientEvents::UpgradeClient) -> Self {
        IbcEvent::UpgradeClient(v)
    }
}

impl From<ConnectionEvents::OpenInit> for IbcEvent {
    fn from(v: ConnectionEvents::OpenInit) -> Self {
        IbcEvent::OpenInitConnection(v)
    }
}

impl From<ConnectionEvents::OpenTry> for IbcEvent {
    fn from(v: ConnectionEvents::OpenTry) -> Self {
        IbcEvent::OpenTryConnection(v)
    }
}

impl From<ConnectionEvents::OpenAck> for IbcEvent {
    fn from(v: ConnectionEvents::OpenAck) -> Self {
        IbcEvent::OpenAckConnection(v)
    }
}

impl From<ConnectionEvents::OpenConfirm> for IbcEvent {
    fn from(v: ConnectionEvents::OpenConfirm) -> Self {
        IbcEvent::OpenConfirmConnection(v)
    }
}

impl From<ChannelEvents::OpenInit> for IbcEvent {
    fn from(v: ChannelEvents::OpenInit) -> Self {
        IbcEvent::OpenInitChannel(v)
    }
}

impl From<ChannelEvents::OpenTry> for IbcEvent {
    fn from(v: ChannelEvents::OpenTry) -> Self {
        IbcEvent::OpenTryChannel(v)
    }
}

impl From<ChannelEvents::OpenAck> for IbcEvent {
    fn from(v: ChannelEvents::OpenAck) -> Self {
        IbcEvent::OpenAckChannel(v)
    }
}

impl From<ChannelEvents::OpenConfirm> for IbcEvent {
    fn from(v: ChannelEvents::OpenConfirm) -> Self {
        IbcEvent::OpenConfirmChannel(v)
    }
}

impl From<ChannelEvents::CloseInit> for IbcEvent {
    fn from(v: ChannelEvents::CloseInit) -> Self {
        IbcEvent::CloseInitChannel(v)
    }
}

impl From<ChannelEvents::CloseConfirm> for IbcEvent {
    fn from(v: ChannelEvents::CloseConfirm) -> Self {
        IbcEvent::CloseConfirmChannel(v)
    }
}

impl From<ChannelEvents::SendPacket> for IbcEvent {
    fn from(v: ChannelEvents::SendPacket) -> Self {
        IbcEvent::SendPacket(v)
    }
}

impl From<ChannelEvents::ReceivePacket> for IbcEvent {
    fn from(v: ChannelEvents::ReceivePacket) -> Self {
        IbcEvent::ReceivePacket(v)
    }
}

impl From<ChannelEvents::WriteAcknowledgement> for IbcEvent {
    fn from(v: ChannelEvents::WriteAcknowledgement) -> Self {
        IbcEvent::WriteAcknowledgement(v)
    }
}

impl From<ChannelEvents::AcknowledgePacket> for IbcEvent {
    fn from(v: ChannelEvents::AcknowledgePacket) -> Self {
        IbcEvent::AcknowledgePacket(v)
    }
}

impl From<ChannelEvents::TimeoutPacket> for IbcEvent {
    fn from(v: ChannelEvents::TimeoutPacket) -> Self {
        IbcEvent::TimeoutPacket(v)
    }
}

impl From<ChannelEvents::TimeoutOnClosePacket> for IbcEvent {
    fn from(v: ChannelEvents::TimeoutOnClosePacket) -> Self {
        IbcEvent::TimeoutOnClosePacket(v)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidModuleId;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct ModuleId(String);

impl ModuleId {
    pub fn new(s: Cow<'_, str>) -> Result<Self, InvalidModuleId> {
        if !s.trim().is_empty() && s.chars().all(char::is_alphanumeric) {
            Ok(Self(s.into_owned()))
        } else {
            Err(InvalidModuleId)
        }
    }
}

impl Display for ModuleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ModuleId {
    type Err = InvalidModuleId;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(Cow::Borrowed(s))
    }
}
