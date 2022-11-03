// mod codegen;
use crate::chain::substrate::config::ibc_node::runtime_types::ibc_support::Any as RuntimeAny;
use core::str::FromStr;
use prost_types::Any;
use subxt::{
    config::{Config, SubstrateConfig},
    tx::SubstrateExtrinsicParams,
};

pub const REVISION_NUMBER: u64 = 0;

pub const TENDERMINT_TYPE: &'static str = "07-tendermint";
pub const GRANDPA_TYPE: &'static str = "10-grandpa";
pub const MOCK_STR: &'static str = "9999-mock";

impl From<ibc_relayer_types::core::ics02_client::client_type::ClientType>
    for ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ClientType
{
    fn from(client_type: ibc_relayer_types::core::ics02_client::client_type::ClientType) -> Self {
        match client_type {
            ibc_relayer_types::core::ics02_client::client_type::ClientType::Tendermint => {
                Self(TENDERMINT_TYPE.as_bytes().to_vec())
            }
            ibc_relayer_types::core::ics02_client::client_type::ClientType::Grandpa => {
                Self(GRANDPA_TYPE.as_bytes().to_vec())
            }
            ibc_relayer_types::core::ics02_client::client_type::ClientType::Mock => {
                Self(MOCK_STR.as_bytes().to_vec())
            }
        }
    }
}

impl<T> AsRef<[T]> for ibc_node::runtime_types::sp_core::bounded::bounded_vec::BoundedVec<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

impl<T> ibc_node::runtime_types::sp_core::bounded::bounded_vec::BoundedVec<T> {
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ClientType>
    for ibc_relayer_types::core::ics02_client::client_type::ClientType
{
    fn from(
        client_type: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ClientType,
    ) -> Self {
        let client_type = String::from_utf8(client_type.0).expect("hex encode never fialed");

        match client_type.as_str() {
            TENDERMINT_TYPE => Self::Tendermint,
            GRANDPA_TYPE => Self::Grandpa,
            MOCK_STR => Self::Mock,
        }
    }
}

#[subxt::subxt(runtime_metadata_path = "src/chain/substrate/metadata/metadata.scale")]
pub mod ibc_node {}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MyConfig;
impl Config for MyConfig {
    // This is different from the default `u32`.
    //
    // *Note* that in this example it does differ from the actual `Index` type in the
    // polkadot runtime used, so some operations will fail. Normally when using a custom `Config`
    // impl types MUST match exactly those used in the actual runtime.
    type Index = u64;
    type BlockNumber = <SubstrateConfig as Config>::BlockNumber;
    type Hash = <SubstrateConfig as Config>::Hash;
    type Hashing = <SubstrateConfig as Config>::Hashing;
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = <SubstrateConfig as Config>::Address;
    type Header = <SubstrateConfig as Config>::Header;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Extrinsic = <SubstrateConfig as Config>::Extrinsic;
    // ExtrinsicParams makes use of the index type, so we need to adjust it
    // too to align with our modified index type, above:
    type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
}

impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Height>
    for ibc_relayer_types::Height
{
    fn from(height: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Height) -> Self {
        ibc_relayer_types::Height::new(REVISION_NUMBER, height.revision_height)
            .expect("REVISION_NUMBER is 8888")
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Packet>
    for ibc_relayer_types::core::ics04_channel::packet::Packet
{
    fn from(packet: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Packet) -> Self {
        Self {
            sequence: packet.sequence.into(),
            source_port: packet.source_port.into(),
            source_channel: packet.source_channel.into(),
            destination_port: packet.destination_port.into(),
            destination_channel: packet.destination_channel.into(),
            data: packet.data,
            timeout_height: match packet.timeout_height {
                ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::TimeoutHeight::Never =>
                ibc_relayer_types::core::ics04_channel::timeout::TimeoutHeight::Never,
                ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::TimeoutHeight::At(value) =>
                ibc_relayer_types::core::ics04_channel::timeout::TimeoutHeight::
                At(ibc_relayer_types::core::ics02_client::height::Height::new(
                        value.revision_number,
                value.revision_height
                ).unwrap()),
            },
            timeout_timestamp: packet.timeout_timestamp.into(),
        }
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId>
    for ibc_relayer_types::core::ics24_host::identifier::ConnectionId
{
    fn from(
        connection_id: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
    ) -> Self {
        let value = String::from_utf8(connection_id.0).unwrap();
        Self(value)
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ChannelId>
    for ibc_relayer_types::core::ics24_host::identifier::ChannelId
{
    fn from(
        channel_id: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
    ) -> Self {
        let value = String::from_utf8(channel_id.0).unwrap();
        Self::from_str(&value).unwrap()
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::PortId>
    for ibc_relayer_types::core::ics24_host::identifier::PortId
{
    fn from(
        port_id: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::PortId,
    ) -> Self {
        let value = String::from_utf8(port_id.0).unwrap();
        Self(value)
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ClientId>
    for ibc_relayer_types::core::ics24_host::identifier::ClientId
{
    fn from(
        client_id: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
    ) -> Self {
        let value = String::from_utf8(client_id.0).unwrap();
        Self(value)
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Sequence>
    for ibc_relayer_types::core::ics04_channel::packet::Sequence
{
    fn from(
        sequence: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Sequence,
    ) -> Self {
        Self::from(sequence.0)
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Timestamp>
    for ibc_relayer_types::timestamp::Timestamp
{
    fn from(
        time_stamp: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Timestamp,
    ) -> Self {
        let value = String::from_utf8(time_stamp.time).unwrap();
        Self::from_str(&value).unwrap()
    }
}

impl From<Any> for RuntimeAny {
    fn from(value: Any) -> Self {
        RuntimeAny {
            type_url: value.type_url.as_bytes().to_vec(),
            value: value.value,
        }
    }
}

//impl Copy for ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Height {}

impl Clone for ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Height {
    fn clone(&self) -> Self {
        Self {
            revision_number: self.revision_number,
            revision_height: self.revision_height,
        }
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::events::ModuleId>
    for ibc_relayer_types::events::ModuleId
{
    fn from(module_id: ibc_node::runtime_types::pallet_ibc::events::ModuleId) -> Self {
        let inner_module_id = String::from_utf8(module_id.0).expect("convert module id error");
        Self::from_str(&inner_module_id).expect("convert to ibc MoudleId error")
    }
}

impl From<ibc_node::runtime_types::pallet_ibc::events::ModuleEventAttribute>
    for ibc_relayer_types::events::ModuleEventAttribute
{
    fn from(
        module_event_attribute: ibc_node::runtime_types::pallet_ibc::events::ModuleEventAttribute,
    ) -> Self {
        let key = String::from_utf8(module_event_attribute.key)
            .expect("convert ModuleEventAttribute key error");
        let value = String::from_utf8(module_event_attribute.value)
            .expect("convert ModuleEventAttribute value error");
        Self { key, value }
    }
}
