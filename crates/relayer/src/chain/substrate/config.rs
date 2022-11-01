// mod codegen;
//use codec::{Decode, Encode};
//use core::str::FromStr;
//use prost_types::Any;
use subxt::{Config, DefaultConfig};

pub const REVISION_NUMBER: u64 = 0;

/// A struct representing the signed extra and additional parameters required
/// to construct a transaction for a substrate node template.
pub type SubstrateNodeTemplateExtrinsicParams<T> =
subxt::extrinsic::BaseExtrinsicParams<T, subxt::extrinsic::PlainTip>;

/// A builder which leads to [`SubstrateNodeTemplateExtrinsicParams`] being constructed.
/// This is what you provide to methods like `sign_and_submit()`.
pub type SubstrateNodeTemplateExtrinsicParamsBuilder<T> =
subxt::extrinsic::BaseExtrinsicParams<T, subxt::extrinsic::PlainTip>;
//
//impl From<ibc_relayer_types::core::ics02_client::client_type::ClientType>
//for ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ClientType
//{
//    fn from(client_type: ibc_relayer_types::core::ics02_client::client_type::ClientType) -> Self {
//        match client_type {
//            ibc_relayer_types::core::ics02_client::client_type::ClientType::Tendermint => Self::Tendermint,
//            ibc_relayer_types::core::ics02_client::client_type::ClientType::Grandpa => Self::Grandpa,
//            _ => todo!()
//        }
//    }
//}
//
//impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ClientType>
//for ibc_relayer_types::core::ics02_client::client_type::ClientType
//{
//    fn from(
//            client_type: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ClientType,
//            ) -> Self {
//        match client_type {
//            ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ClientType::Tendermint => Self::Tendermint,
//            ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ClientType::Grandpa => Self::Grandpa,
//        }
//    }
//}
//
//#[derive(Encode, Decode)]
//pub struct MessageQueueChain(pub sp_core::H256);

#[subxt::subxt(runtime_metadata_path = "src/chain/substrate/metadata/metadata.scale")]
pub mod ibc_node {
//    #[subxt(substitute_type = "pallet_ibc::event::primitive::ClientType")]
//    use crate::ClientType;
//
//    #[subxt(substitute_type = "cumulus_pallet_parachain_system::MessageQueueChain")]
//    use crate::MessageQueueChain;
//
//    #[subxt(substitute_type = "beefy_primitives::crypto::Public")]
//    use beefy_primitives::crypto::Public;
//    use ibc::core::ics02_client::client_type::ClientType;
}

#[tokio::test]
async fn test_subxt() -> Result<(), Box<dyn std::error::Error>> {
    use sp_keyring::AccountKeyring;
    use subxt::{
        tx::PairSigner,
        OnlineClient,
    };
    tracing_subscriber::fmt::init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    // Create a client to use:
    let api = OnlineClient::<MyConfig>::new().await?;

    // Create a transaction to submit:
    let tx = ibc_node::tx()
    .balances()
    .transfer(dest, 123_456_789_012_345);

    // Submit the transaction with default params:
    let hash = api.tx().sign_and_submit_default(&tx, &signer).await?;

    println!("Balance transfer extrinsic submitted: {}", hash);

    Ok(())
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MyConfig;
impl Config for MyConfig {
    // This is different from the default `u32`.
    //
    // *Note* that in this example it does differ from the actual `Index` type in the
    // polkadot runtime used, so some operations will fail. Normally when using a custom `Config`
    // impl types MUST match exactly those used in the actual runtime.
    type Index = u64;
    type BlockNumber = <DefaultConfig as Config>::BlockNumber;
    type Hash = <DefaultConfig as Config>::Hash;
    type Hashing = <DefaultConfig as Config>::Hashing;
    type AccountId = <DefaultConfig as Config>::AccountId;
    type Address = <DefaultConfig as Config>::Address;
    type Header = <DefaultConfig as Config>::Header;
    type Signature = <DefaultConfig as Config>::Signature;
    type Extrinsic = <DefaultConfig as Config>::Extrinsic;
}

//impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Height> for ibc::Height {
//    fn from(height: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Height) -> Self {
//        ibc_relayer_types::Height::new(REVISION_NUMBER, height.revision_height).expect("REVISION_NUMBER is 8888")
//    }
//}
//
//impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Packet>
//for ibc_relayer_types::core::ics04_channel::packet::Packet
//{
//    fn from(packet: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Packet) -> Self {
//        Self {
//            sequence: packet.sequence.into(),
//            source_port: packet.source_port.into(),
//            source_channel: packet.source_channel.into(),
//            destination_port: packet.destination_port.into(),
//            destination_channel: packet.destination_channel.into(),
//            data: packet.data,
//            timeout_height: match packet.timeout_height {
//                ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::TimeoutHeight::Never =>
//                ibc_relayer_types::core::ics04_channel::timeout::TimeoutHeight::Never,
//                ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::TimeoutHeight::At(value) =>
//                ibc_relayer_types::core::ics04_channel::timeout::TimeoutHeight::
//                At(ibc_relayer_types::core::ics02_client::height::Height::new(
//                        value.revision_number,
//                value.revision_height
//                ).unwrap()),
//            },
//            timeout_timestamp: packet.timeout_timestamp.into(),
//        }
//    }
//}
//
//impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId>
//for ibc_relayer_types::core::ics24_host::identifier::ConnectionId
//{
//    fn from(
//            connection_id: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ConnectionId,
//            ) -> Self {
//        let value = String::from_utf8(connection_id.0).unwrap();
//        Self(value)
//    }
//}
//
//impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ChannelId>
//for ibc_relayer_types::core::ics24_host::identifier::ChannelId
//{
//    fn from(
//            channel_id: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ChannelId,
//            ) -> Self {
//        let value = String::from_utf8(channel_id.0).unwrap();
//        Self::from_str(&value).unwrap()
//    }
//}
//
//impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::PortId>
//for ibc_relayer_types::core::ics24_host::identifier::PortId
//{
//    fn from(
//            port_id: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::PortId,
//            ) -> Self {
//        let value = String::from_utf8(port_id.0).unwrap();
//        Self(value)
//    }
//}
//
//impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ClientId>
//for ibc_relayer_types::core::ics24_host::identifier::ClientId
//{
//    fn from(
//            client_id: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::ClientId,
//            ) -> Self {
//        let value = String::from_utf8(client_id.0).unwrap();
//        Self(value)
//    }
//}
//
//impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Sequence>
//for ibc_relayer_types::core::ics04_channel::packet::Sequence
//{
//    fn from(
//            sequence: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Sequence,
//            ) -> Self {
//        Self::from(sequence.0)
//    }
//}
//
//impl From<ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Timestamp>
//for ibc_relayer_types::timestamp::Timestamp
//{
//    fn from(
//            time_stamp: ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Timestamp,
//            ) -> Self {
//        let value = String::from_utf8(time_stamp.time).unwrap();
//        Self::from_str(&value).unwrap()
//    }
//}
//
//impl From<Any> for ibc_node::runtime_types::pallet_ibc::Any {
//    fn from(value: Any) -> Self {
//        ibc_node::runtime_types::pallet_ibc::Any {
//            type_url: value.type_url.as_bytes().to_vec(),
//            value: value.value,
//        }
//    }
//}
//
////impl Copy for ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Height {}
//
//impl Clone for ibc_node::runtime_types::pallet_ibc::module::core::ics24_host::Height {
//    fn clone(&self) -> Self {
//        Self {
//            revision_number: self.revision_number,
//            revision_height: self.revision_height,
//        }
//    }
//}
//
//impl From<ibc_node::runtime_types::pallet_ibc::events::ModuleId>
//for ibc_relayer_types::core::ics26_routing::context::ModuleId
//{
//    fn from(module_id: ibc_node::runtime_types::pallet_ibc::events::ModuleId) -> Self {
//        let inner_module_id = String::from_utf8(module_id.0).expect("convert module id error");
//        Self::from_str(&inner_module_id).expect("convert to ibc MoudleId error")
//    }
//}
//
//impl From<ibc_node::runtime_types::pallet_ibc::events::ModuleEventAttribute>
//for ibc_relayer_types::events::ModuleEventAttribute
//{
//    fn from(
//            module_event_attribute: ibc_node::runtime_types::pallet_ibc::events::ModuleEventAttribute,
//            ) -> Self {
//        let key = String::from_utf8(module_event_attribute.key)
//        .expect("convert ModuleEventAttribute key error");
//        let value = String::from_utf8(module_event_attribute.value)
//        .expect("convert ModuleEventAttribute value error");
//        Self { key, value }
//    }
//}