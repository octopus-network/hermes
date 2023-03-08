use alloc::sync::Arc;
// use bytes::{Buf, Bytes};
// use core::{
//     convert::{TryFrom, TryInto},
//     future::Future,
//     str::FromStr,
//     time::Duration,
// };
// use futures::future::join_all;
// use num_bigint::BigInt;
// use std::{cmp::Ordering, thread};

use tokio::runtime::Runtime as TokioRuntime;
use tonic::codegen::http::Uri;
// use tracing::{error, instrument, trace, warn};

// use ibc_proto::cosmos::base::node::v1beta1::ConfigResponse;
// use ibc_proto::cosmos::staking::v1beta1::Params as StakingParams;
use ibc_proto::protobuf::Protobuf;
use ibc_relayer_types::applications::ics31_icq::response::CrossChainQueryResponse;
use ibc_relayer_types::clients::ics07_tendermint::client_state::{
    AllowUpdate, ClientState as TmClientState,
};
use ibc_relayer_types::clients::ics07_tendermint::consensus_state::ConsensusState as TMConsensusState;
use ibc_relayer_types::clients::ics07_tendermint::header::Header as TmHeader;
// use ibc_relayer_types::core::ics02_client::client_type::ClientType;
use ibc_relayer_types::core::ics02_client::error::Error as ClientError;
use ibc_relayer_types::core::ics02_client::events::UpdateClient;
use ibc_relayer_types::core::ics03_connection::connection::{
    ConnectionEnd, IdentifiedConnectionEnd,
};
use ibc_relayer_types::core::ics04_channel::channel::{ChannelEnd, IdentifiedChannelEnd};
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentPrefix;
use ibc_relayer_types::core::ics23_commitment::merkle::MerkleProof;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ChannelId, ConnectionId, PortId};
// use ibc_relayer_types::core::ics24_host::path::{
//     AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath, CommitmentsPath,
//     ConnectionsPath, ReceiptsPath, SeqRecvsPath,
// };
// use ibc_relayer_types::core::ics24_host::{
//     ClientUpgradePath, Path, IBC_QUERY_PATH, SDK_UPGRADE_QUERY_PATH,
// };
use ibc_relayer_types::signer::Signer;
use ibc_relayer_types::Height as ICSHeight;

// use tendermint::block::Height as TmHeight;
// use tendermint::node::info::TxIndexStatus;
use tendermint_light_client_verifier::types::LightBlock as TmLightBlock;
use tendermint_rpc::endpoint::broadcast::tx_sync::Response;
// use tendermint_rpc::endpoint::status;
use tendermint_rpc::HttpClient;

use crate::account::Balance;
use crate::chain::client::ClientSettings;
// use crate::chain::cosmos::batch::{
//     send_batched_messages_and_wait_check_tx, send_batched_messages_and_wait_commit,
//     sequential_send_batched_messages_and_wait_commit,
// };
// use crate::chain::cosmos::encode::key_pair_to_signer;
// use crate::chain::cosmos::fee::maybe_register_counterparty_payee;
// use crate::chain::cosmos::gas::{calculate_fee, mul_ceil};
// use crate::chain::cosmos::query::account::get_or_fetch_account;
// use crate::chain::cosmos::query::balance::{query_all_balances, query_balance};
// use crate::chain::cosmos::query::custom::cross_chain_query_via_rpc;
// use crate::chain::cosmos::query::denom_trace::query_denom_trace;
// use crate::chain::cosmos::query::status::query_status;
// use crate::chain::cosmos::query::tx::{
//     filter_matching_event, query_packets_from_block, query_packets_from_txs, query_txs,
// };
// use crate::chain::cosmos::query::{abci_query, fetch_version_specs, packet_query, QueryResponse};
use crate::chain::cosmos::types::account::Account;
use crate::chain::cosmos::types::config::TxConfig;
// use crate::chain::cosmos::types::gas::{
//     default_gas_from_config, gas_multiplier_from_config, max_gas_from_config,
// };
use crate::chain::endpoint::{ChainEndpoint, ChainStatus, HealthCheck};
use crate::chain::handle::Subscription;
use crate::chain::requests::*;
use crate::chain::tracking::TrackedMsgs;
use crate::client_state::{AnyClientState, IdentifiedAnyClientState};
use crate::config::ChainConfig;
use crate::consensus_state::{AnyConsensusState, AnyConsensusStateWithHeight};
use crate::denom::DenomTrace;
use crate::error::Error;
use crate::event::monitor::{EventMonitor, TxMonitorCmd};
use crate::event::IbcEventWithHeight;
use ibc_relayer_types::events::IbcEvent;

use crate::keyring::{KeyRing, Secp256k1KeyPair, SigningKeyPair};
use crate::light_client::tendermint::LightClient as TmLightClient;
// use crate::light_client::{LightClient, Verified};
use crate::misbehaviour::MisbehaviourEvidence;
// use crate::util::pretty::{
//     PrettyConsensusStateWithHeight, PrettyIdentifiedChannel, PrettyIdentifiedClientState,
//     PrettyIdentifiedConnection,
// };

use core::str::FromStr;
// use ibc_proto::google::protobuf::Any;
use crate::config::AddressType;
use hdpath::StandardHDPath;
use ibc_relayer_types::clients::ics06_solomachine::ClientState as SmClientState;
use ibc_relayer_types::clients::ics06_solomachine::ConsensusState as SmConsensusState;
use ibc_relayer_types::clients::ics06_solomachine::HeaderData as SmHeaderData;
use ibc_relayer_types::clients::ics06_solomachine::PublicKey;
use ibc_relayer_types::clients::ics06_solomachine::{Header as SmHeader, HeaderData, SignBytes};
use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentRoot;
use ibc_relayer_types::timestamp::Timestamp;
use sp_keyring::AccountKeyring;
use std::time::{Duration, SystemTime};
use subxt::{tx::PairSigner, OnlineClient, SubstrateConfig};
use tracing::info;

// pub mod batch;
// pub mod client;
// pub mod compatibility;
// pub mod encode;
// pub mod estimate;
// pub mod fee;
// pub mod gas;
// pub mod query;
// pub mod retry;
// pub mod simulate;
// pub mod tx;
// pub mod types;
// pub mod version;
// pub mod wait;

/// Defines an upper limit on how large any transaction can be.
/// This upper limit is defined as a fraction relative to the block's
/// maximum bytes. For example, if the fraction is `0.9`, then
/// `max_tx_size` will not be allowed to exceed 0.9 of the
/// maximum block size of any Cosmos SDK network.
///
/// The default fraction we use is `0.9`; anything larger than that
/// would be risky, as transactions might be rejected; a smaller value
/// might be un-necessarily restrictive on the relayer side.
/// The [default max. block size in Tendermint 0.37 is 21MB](tm-37-max).
/// With a fraction of `0.9`, then Hermes will never permit the configuration
/// of `max_tx_size` to exceed ~18.9MB.
///
/// [tm-37-max]: https://github.com/tendermint/tendermint/blob/v0.37.0-rc1/types/params.go#L79
// pub const BLOCK_MAX_BYTES_MAX_FRACTION: f64 = 0.9;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubLightBlock {}

#[subxt::subxt(runtime_metadata_path = "./metadata.scale")]
pub mod substrate {}

pub mod subxt_ibc_event {
    use core::time::Duration;

    use super::substrate::runtime_types::ibc::core::ics02_client::client_type::ClientType as SubxtClientType;
    use super::substrate::runtime_types::ibc::core::ics02_client::height::Height as SubxtHeight;
    use super::substrate::runtime_types::ibc::core::ics03_connection::connection::{
        sealed::ConnectionEnd as SubxtConnectionEnd, Counterparty as SubxtConnectionCounterparty,
        State as SubxtConnectionState,
    };
    use super::substrate::runtime_types::ibc::core::ics03_connection::version::Version as SubxtConnectionVersion;
    use super::substrate::runtime_types::ibc::core::ics04_channel::channel::{
        ChannelEnd as SubxtChannelEnd, Counterparty as SubxtChannelCounterparty,
        Order as SubxtChannelOrder, State as SubxtChannelState,
    };
    use super::substrate::runtime_types::ibc::core::ics04_channel::commitment::{
        AcknowledgementCommitment as SubxtAcknowledgementCommitment,
        PacketCommitment as SubxtPacketCommitment,
    };
    use super::substrate::runtime_types::ibc::core::ics04_channel::msgs::acknowledgement::Acknowledgement as SubxtAcknowledgement;
    use super::substrate::runtime_types::ibc::core::ics04_channel::packet::{
        Receipt as SubxtReceipt, Sequence as SubxtSequence,
    };
    use super::substrate::runtime_types::ibc::core::ics04_channel::timeout::TimeoutHeight as SubxtTimeoutHeight;
    use super::substrate::runtime_types::ibc::core::ics04_channel::version::Version as SubxtChannelVersion;
    use super::substrate::runtime_types::ibc::core::ics23_commitment::commitment::CommitmentPrefix as SubxtCommitmentPrefix;
    use super::substrate::runtime_types::ibc::core::ics24_host::identifier::{
        ChannelId as SubxtChannelId, ClientId as SubxtClientId, ConnectionId as SubxtConnectionId,
        PortId as SubxtPortId,
    };
    use super::substrate::runtime_types::ibc::core::ics26_routing::context::ModuleId as SubxtModuleId;
    use super::substrate::runtime_types::ibc::core::{
        ics02_client::events::{
            ClientMisbehaviour as SubxtClientMisbehaviour, CreateClient as SubxtCreateClient,
            UpdateClient as SubxtUpdateClient, UpgradeClient as SubxtUpgradeClient,
        },
        ics03_connection::events::{
            OpenAck as SubxtConnectionOpenAck, OpenConfirm as SubxtConnectionOpenConfirm,
            OpenInit as SubxtConnectionOpenInit, OpenTry as SubxtConnectionOpenTry,
        },
        ics04_channel::events::{
            AcknowledgePacket as SubxtAcknowledgePacket,
            ChannelClosed as SubxtChannelClosed, // todo channelClose
            CloseConfirm as SubxtChannelCloseConfirm,
            CloseInit as SubxtChannelCloseInit,
            OpenAck as SubxtChannelOpenAck,
            OpenConfirm as SubxtChannelOpenConfirm,
            OpenInit as SubxtChannelOpenInit,
            OpenTry as SubxtChannelOpenTry,
            ReceivePacket as SubxtReceivePacket,
            SendPacket as SubxtSendPacket,
            TimeoutPacket as SubxtTimeoutPacket,
            WriteAcknowledgement as SubxtWriteAcknowledgement,
        },
    };
    use super::substrate::runtime_types::ibc::events::{
        ModuleEvent as SubxtModuleEvent, ModuleEventAttribute as SubxtModuleEventAttribute,
    };
    use super::substrate::runtime_types::ibc::timestamp::Timestamp as SubxtTimestamp;
    use super::substrate::runtime_types::{self, ibc::events::IbcEvent as SubxtIbcEvent};
    use ibc_relayer_types::core::ics02_client::client_type::ClientType;
    use ibc_relayer_types::core::ics02_client::events::Attributes as ClientAttributes;
    use ibc_relayer_types::core::ics03_connection::connection::{
        ConnectionEnd, Counterparty as ConnectionCounterparty, State as ConnectionState,
    };
    use ibc_relayer_types::core::ics03_connection::events::Attributes as ConnectionAttributes;
    use ibc_relayer_types::core::ics03_connection::version::Version as ConnectionVersion;
    use ibc_relayer_types::core::ics04_channel::channel::{
        ChannelEnd, Counterparty as ChannelCounterparty, Order as ChannelOrder,
        State as ChannelState,
    };
    use ibc_relayer_types::core::ics04_channel::commitment::{
        AcknowledgementCommitment, PacketCommitment,
    };
    use ibc_relayer_types::core::ics04_channel::msgs::acknowledgement::Acknowledgement;
    use ibc_relayer_types::core::ics04_channel::packet::{Packet, Receipt, Sequence};
    use ibc_relayer_types::core::ics04_channel::timeout::TimeoutHeight;
    use ibc_relayer_types::core::ics04_channel::version::Version as ChannelVersion;
    use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentPrefix;
    use ibc_relayer_types::core::ics24_host::identifier::{
        ChannelId, ClientId, ConnectionId, PortId,
    };
    use ibc_relayer_types::core::{
        ics02_client::events::{ClientMisbehaviour, CreateClient, UpdateClient, UpgradeClient},
        ics03_connection::events::{
            OpenAck as ConnectionOpenAck, OpenConfirm as ConnectionOpenConfirm,
            OpenInit as ConnectionOpenInit, OpenTry as ConnectionOpenTry,
        },
        ics04_channel::events::{
            AcknowledgePacket, CloseConfirm as ChannelCloseConfirm, CloseInit as ChannelCloseInit,
            OpenAck as ChannelOpenAck, OpenConfirm as ChannelOpenConfirm,
            OpenInit as ChannelOpenInit, OpenTry as ChannelOpenTry, ReceivePacket, SendPacket,
            TimeoutPacket, WriteAcknowledgement,
        },
    };
    use ibc_relayer_types::events::{self, IbcEvent, ModuleEventAttribute, ModuleId};
    use ibc_relayer_types::timestamp::Timestamp;
    use ibc_relayer_types::Height;
    use std::str::FromStr;

    // -------ics 02 client
    // client type
    impl From<SubxtClientType> for ClientType {
        fn from(value: SubxtClientType) -> Self {
            match value.0.as_ref() {
                "07-tendermint" => ClientType::Tendermint,
                "06-solomachine" => ClientType::Solomachine,
                _ => panic!("Unknown client type: {:?}", value),
            }
        }
    }
    // CreateClient
    impl From<SubxtCreateClient> for CreateClient {
        fn from(value: SubxtCreateClient) -> Self {
            let client_id = value.client_id.client_id;
            let client_type = value.client_type.client_type;
            let consensus_height = value.consensus_height.consensus_height;
            Self(ClientAttributes {
                client_id: client_id.into(),
                client_type: client_type.into(),
                consensus_height: consensus_height.into(),
            })
        }
    }

    // UpdateClient
    impl From<SubxtUpdateClient> for UpdateClient {
        fn from(value: SubxtUpdateClient) -> Self {
            let client_id = value.client_id.client_id;
            let client_type = value.client_type.client_type;
            let consensus_height = value.consensus_height.consensus_height;
            let _header = value.header.header;
            Self {
                common: ClientAttributes {
                    client_id: client_id.into(),
                    client_type: client_type.into(),
                    consensus_height: consensus_height.into(),
                },
                // todo, this type is `Option<Box<dyn Header>>` but get Any. So At Present We set None.
                header: None,
            }
        }
    }

    // UpgradeClient
    impl From<SubxtUpgradeClient> for UpgradeClient {
        fn from(value: SubxtUpgradeClient) -> Self {
            let client_id = value.client_id.client_id;
            let client_type = value.client_type.client_type;
            let consensus_height = value.consensus_height.consensus_height;
            Self(ClientAttributes {
                client_id: client_id.into(),
                client_type: client_type.into(),
                consensus_height: consensus_height.into(),
            })
        }
    }

    impl From<SubxtClientMisbehaviour> for ClientMisbehaviour {
        fn from(value: SubxtClientMisbehaviour) -> Self {
            let client_id = value.client_id.client_id;
            let client_type = value.client_type.client_type;
            // NOTICE, in ibc-rs  ClientMisbehaviour don't have consensus_height.
            let consensus_height = Height::new(0, 1).unwrap();
            Self(ClientAttributes {
                client_id: client_id.into(),
                client_type: client_type.into(),
                consensus_height,
            })
        }
    }

    // Height
    impl From<SubxtHeight> for Height {
        fn from(height: SubxtHeight) -> Self {
            Self::new(height.revision_number, height.revision_height)
                .expect("convert height: Never failed")
        }
    }

    // ------------ ics03 connection
    // ConnectionEnd
    impl From<SubxtConnectionEnd> for ConnectionEnd {
        fn from(value: SubxtConnectionEnd) -> Self {
            Self::new(
                value.state.into(),
                value.client_id.into(),
                value.counterparty.into(),
                value.versions.into_iter().map(|v| v.into()).collect(),
                Duration::new(value.delay_period_secs, value.delay_period_nanos),
            )
        }
    }

    // Counterparty
    impl From<SubxtConnectionCounterparty> for ConnectionCounterparty {
        fn from(value: SubxtConnectionCounterparty) -> Self {
            Self::new(
                value.client_id.into(),
                value.connection_id.map(|v| v.into()),
                value
                    .prefix
                    .bytes
                    .try_into()
                    .expect("never failed convert prefix from vec<u8>"),
            )
        }
    }

    // State
    impl From<SubxtConnectionState> for ConnectionState {
        fn from(value: SubxtConnectionState) -> Self {
            match value {
                SubxtConnectionState::Uninitialized => Self::Uninitialized,
                SubxtConnectionState::Init => Self::Init,
                SubxtConnectionState::TryOpen => Self::TryOpen,
                SubxtConnectionState::Open => Self::Open,
            }
        }
    }

    // OpenAck
    impl From<SubxtConnectionOpenAck> for ConnectionOpenAck {
        fn from(value: SubxtConnectionOpenAck) -> Self {
            let connection_id = value.0.connection_id;
            let client_id = value.0.client_id;
            let counterparty_connection_id = value.0.counterparty_connection_id;
            let counterparty_client_id = value.0.counterparty_client_id;
            Self(ConnectionAttributes {
                connection_id: Some(connection_id.into()),
                client_id: client_id.into(),
                counterparty_connection_id: counterparty_connection_id.map(|v| v.into()),
                counterparty_client_id: counterparty_client_id.into(),
            })
        }
    }

    // OpenConfirm
    impl From<SubxtConnectionOpenConfirm> for ConnectionOpenConfirm {
        fn from(value: SubxtConnectionOpenConfirm) -> Self {
            let connection_id = value.0.connection_id;
            let client_id = value.0.client_id;
            let counterparty_connection_id = value.0.counterparty_connection_id;
            let counterparty_client_id = value.0.counterparty_client_id;
            Self(ConnectionAttributes {
                connection_id: Some(connection_id.into()),
                client_id: client_id.into(),
                counterparty_connection_id: counterparty_connection_id.map(|v| v.into()),
                counterparty_client_id: counterparty_client_id.into(),
            })
        }
    }

    // OpenInit
    impl From<SubxtConnectionOpenInit> for ConnectionOpenInit {
        fn from(value: SubxtConnectionOpenInit) -> Self {
            let connection_id = value.0.connection_id;
            let client_id = value.0.client_id;
            let counterparty_connection_id = value.0.counterparty_connection_id;
            let counterparty_client_id = value.0.counterparty_client_id;
            Self(ConnectionAttributes {
                connection_id: Some(connection_id.into()),
                client_id: client_id.into(),
                counterparty_connection_id: counterparty_connection_id.map(|v| v.into()),
                counterparty_client_id: counterparty_client_id.into(),
            })
        }
    }

    // OpenTry
    impl From<SubxtConnectionOpenTry> for ConnectionOpenTry {
        fn from(value: SubxtConnectionOpenTry) -> Self {
            let connection_id = value.0.connection_id;
            let client_id = value.0.client_id;
            let counterparty_connection_id = value.0.counterparty_connection_id;
            let counterparty_client_id = value.0.counterparty_client_id;
            Self(ConnectionAttributes {
                connection_id: Some(connection_id.into()),
                client_id: client_id.into(),
                counterparty_connection_id: counterparty_connection_id.map(|v| v.into()),
                counterparty_client_id: counterparty_client_id.into(),
            })
        }
    }

    // Version
    impl From<SubxtConnectionVersion> for ConnectionVersion {
        fn from(value: SubxtConnectionVersion) -> Self {
            Self {
                identifier: value.identifier,
                features: value.features,
            }
        }
    }

    // ------------ ibc04 channel
    // channelEnd
    impl From<SubxtChannelEnd> for ChannelEnd {
        fn from(value: SubxtChannelEnd) -> Self {
            Self {
                state: value.state.into(),
                ordering: value.ordering.into(),
                remote: value.remote.into(),
                connection_hops: value
                    .connection_hops
                    .into_iter()
                    .map(|v| v.into())
                    .collect(),
                version: value.version.into(),
            }
        }
    }
    // Counterparty
    impl From<SubxtChannelCounterparty> for ChannelCounterparty {
        fn from(value: SubxtChannelCounterparty) -> Self {
            Self {
                port_id: value.port_id.into(),
                channel_id: value.channel_id.map(|v| v.into()),
            }
        }
    }
    // Order
    impl From<SubxtChannelOrder> for ChannelOrder {
        fn from(value: SubxtChannelOrder) -> Self {
            match value {
                SubxtChannelOrder::None => Self::None,
                SubxtChannelOrder::Unordered => Self::Unordered,
                SubxtChannelOrder::Ordered => Self::Ordered,
            }
        }
    }
    // state
    impl From<SubxtChannelState> for ChannelState {
        fn from(value: SubxtChannelState) -> Self {
            match value {
                SubxtChannelState::Uninitialized => Self::Uninitialized,
                SubxtChannelState::Init => Self::Init,
                SubxtChannelState::TryOpen => Self::TryOpen,
                SubxtChannelState::Open => Self::Open,
                SubxtChannelState::Closed => Self::Closed,
            }
        }
    }
    // AcknowledgementCommitment
    impl From<SubxtAcknowledgementCommitment> for AcknowledgementCommitment {
        fn from(value: SubxtAcknowledgementCommitment) -> Self {
            Self::from(value.0)
        }
    }
    // packetCommitment todo
    impl From<SubxtPacketCommitment> for PacketCommitment {
        fn from(value: SubxtPacketCommitment) -> Self {
            Self::from(value.0)
        }
    }
    // AcknowledgePacket
    impl From<SubxtAcknowledgePacket> for AcknowledgePacket {
        fn from(value: SubxtAcknowledgePacket) -> Self {
            let timeout_height = value.timeout_height.timeout_height;
            let timeout_timestamp = value.timeout_timestamp.timeout_timestamp;
            let sequence = value.sequence.sequence;
            let src_port_id = value.src_port_id.src_port_id;
            let src_channel_id = value.src_channel_id.src_channel_id;
            let dst_port_id = value.dst_port_id.dst_port_id;
            let dst_channel_id = value.dst_channel_id.dst_channel_id;
            let _channel_ordering = value.channel_ordering.order;
            let _src_connection_id = value.src_connection_id.connection_id;
            Self {
                packet: Packet {
                    sequence: sequence.into(),
                    source_port: src_port_id.into(),
                    source_channel: src_channel_id.into(),
                    destination_port: dst_port_id.into(),
                    destination_channel: dst_channel_id.into(),
                    data: b"ack".to_vec(),
                    timeout_height: timeout_height.into(),
                    timeout_timestamp: timeout_timestamp.into(),
                },
            }
        }
    }
    // ChannelClosed (todo in ibc-rs have this data struct but in ibc-relayer-type have not this)
    // CloseConfirm
    impl From<SubxtChannelCloseConfirm> for ChannelCloseConfirm {
        fn from(value: SubxtChannelCloseConfirm) -> Self {
            let channel_id = value.channel_id.channel_id;
            let port_id = value.port_id.port_id;
            let connection_id = value.connection_id.connection_id;
            let counterparty_port_id = value.counterparty_port_id.counterparty_port_id;
            let counterparty_channel_id = value.counterparty_channel_id.counterparty_channel_id;
            Self {
                channel_id: Some(channel_id.into()),
                port_id: port_id.into(),
                connection_id: connection_id.into(),
                counterparty_port_id: counterparty_port_id.into(),
                counterparty_channel_id: Some(counterparty_channel_id.into()),
            }
        }
    }
    // CloseInit
    impl From<SubxtChannelCloseInit> for ChannelCloseInit {
        fn from(value: SubxtChannelCloseInit) -> Self {
            let channel_id = value.channel_id.channel_id;
            let port_id = value.port_id.port_id;
            let connection_id = value.connection_id.connection_id;
            let counterparty_port_id = value.counterparty_port_id.counterparty_port_id;
            let counterparty_channel_id = value.counterparty_channel_id.counterparty_channel_id;
            Self {
                channel_id: channel_id.into(),
                port_id: port_id.into(),
                connection_id: connection_id.into(),
                counterparty_port_id: counterparty_port_id.into(),
                counterparty_channel_id: Some(counterparty_channel_id.into()),
            }
        }
    }
    // OpenAck
    impl From<SubxtChannelOpenAck> for ChannelOpenAck {
        fn from(value: SubxtChannelOpenAck) -> Self {
            let channel_id = value.channel_id.channel_id;
            let port_id = value.port_id.port_id;
            let connection_id = value.connection_id.connection_id;
            let counterparty_port_id = value.counterparty_port_id.counterparty_port_id;
            let counterparty_channel_id = value.counterparty_channel_id.counterparty_channel_id;
            Self {
                channel_id: Some(channel_id.into()),
                port_id: port_id.into(),
                connection_id: connection_id.into(),
                counterparty_port_id: counterparty_port_id.into(),
                counterparty_channel_id: Some(counterparty_channel_id.into()),
            }
        }
    }
    // OpenConfirm
    impl From<SubxtChannelOpenConfirm> for ChannelOpenConfirm {
        fn from(value: SubxtChannelOpenConfirm) -> Self {
            let channel_id = value.channel_id.channel_id;
            let port_id = value.port_id.port_id;
            let connection_id = value.connection_id.connection_id;
            let counterparty_port_id = value.counterparty_port_id.counterparty_port_id;
            let counterparty_channel_id = value.counterparty_channel_id.counterparty_channel_id;
            Self {
                channel_id: Some(channel_id.into()),
                port_id: port_id.into(),
                connection_id: connection_id.into(),
                counterparty_port_id: counterparty_port_id.into(),
                counterparty_channel_id: Some(counterparty_channel_id.into()),
            }
        }
    }
    // OpenInit
    impl From<SubxtChannelOpenInit> for ChannelOpenInit {
        fn from(value: SubxtChannelOpenInit) -> Self {
            let channel_id = value.channel_id.channel_id;
            let port_id = value.port_id.port_id;
            let connection_id = value.connection_id.connection_id;
            let counterparty_port_id = value.counterparty_port_id.counterparty_port_id;
            let _version = value.version.version;
            Self {
                channel_id: Some(channel_id.into()),
                port_id: port_id.into(),
                connection_id: connection_id.into(),
                counterparty_port_id: counterparty_port_id.into(),
                counterparty_channel_id: None,
            }
        }
    }
    // OpenTry
    impl From<SubxtChannelOpenTry> for ChannelOpenTry {
        fn from(value: SubxtChannelOpenTry) -> Self {
            let channel_id = value.channel_id.channel_id;
            let port_id = value.port_id.port_id;
            let connection_id = value.connection_id.connection_id;
            let counterparty_port_id = value.counterparty_port_id.counterparty_port_id;
            let counterparty_channel_id = value.counterparty_channel_id.counterparty_channel_id;
            Self {
                channel_id: Some(channel_id.into()),
                port_id: port_id.into(),
                connection_id: connection_id.into(),
                counterparty_port_id: counterparty_port_id.into(),
                counterparty_channel_id: Some(counterparty_channel_id.into()),
            }
        }
    }
    // ReceivePacket
    impl From<SubxtReceivePacket> for ReceivePacket {
        fn from(value: SubxtReceivePacket) -> Self {
            let packet_data = value.packet_data.packet_data;
            let timeout_height = value.timeout_height.timeout_height;
            let timeout_timestamp = value.timeout_timestamp.timeout_timestamp;
            let sequence = value.sequence.sequence;
            let src_port_id = value.src_port_id.src_port_id;
            let src_channel_id = value.src_channel_id.src_channel_id;
            let dst_port_id = value.dst_port_id.dst_port_id;
            let dst_channel_id = value.dst_channel_id.dst_channel_id;
            let _channel_ordering = value.channel_ordering.order;
            let _src_connection_id = value.dst_connection_id.connection_id;
            Self {
                packet: Packet {
                    sequence: sequence.into(),
                    source_port: src_port_id.into(),
                    source_channel: src_channel_id.into(),
                    destination_port: dst_port_id.into(),
                    destination_channel: dst_channel_id.into(),
                    data: packet_data,
                    timeout_height: timeout_height.into(),
                    timeout_timestamp: timeout_timestamp.into(),
                },
            }
        }
    }
    // SendPacket
    impl From<SubxtSendPacket> for SendPacket {
        fn from(value: SubxtSendPacket) -> Self {
            let packet_data = value.packet_data.packet_data;
            let timeout_height = value.timeout_height.timeout_height;
            let timeout_timestamp = value.timeout_timestamp.timeout_timestamp;
            let sequence = value.sequence.sequence;
            let src_port_id = value.src_port_id.src_port_id;
            let src_channel_id = value.src_channel_id.src_channel_id;
            let dst_port_id = value.dst_port_id.dst_port_id;
            let dst_channel_id = value.dst_channel_id.dst_channel_id;
            let _channel_ordering = value.channel_ordering.order;
            let _src_connection_id = value.src_connection_id.connection_id;
            Self {
                packet: Packet {
                    sequence: sequence.into(),
                    source_port: src_port_id.into(),
                    source_channel: src_channel_id.into(),
                    destination_port: dst_port_id.into(),
                    destination_channel: dst_channel_id.into(),
                    data: packet_data,
                    timeout_height: timeout_height.into(),
                    timeout_timestamp: timeout_timestamp.into(),
                },
            }
        }
    }
    // TimeoutPacket
    impl From<SubxtTimeoutPacket> for TimeoutPacket {
        fn from(value: SubxtTimeoutPacket) -> Self {
            let timeout_height = value.timeout_height.timeout_height;
            let timeout_timestamp = value.timeout_timestamp.timeout_timestamp;
            let sequence = value.sequence.sequence;
            let src_port_id = value.src_port_id.src_port_id;
            let src_channel_id = value.src_channel_id.src_channel_id;
            let dst_port_id = value.dst_port_id.dst_port_id;
            let dst_channel_id = value.dst_channel_id.dst_channel_id;
            let _channel_ordering = value.channel_ordering.order;

            Self {
                packet: Packet {
                    sequence: sequence.into(),
                    source_port: src_port_id.into(),
                    source_channel: src_channel_id.into(),
                    destination_port: dst_port_id.into(),
                    destination_channel: dst_channel_id.into(),
                    data: b"timeout".to_vec(),
                    timeout_height: timeout_height.into(),
                    timeout_timestamp: timeout_timestamp.into(),
                },
            }
        }
    }
    // WriteAcknowledgement
    impl From<SubxtWriteAcknowledgement> for WriteAcknowledgement {
        fn from(value: SubxtWriteAcknowledgement) -> Self {
            let packet_data = value.packet_data.packet_data;
            let timeout_height = value.timeout_height.timeout_height;
            let timeout_timestamp = value.timeout_timestamp.timeout_timestamp;
            let sequence = value.sequence.sequence;
            let src_port_id = value.src_port_id.src_port_id;
            let src_channel_id = value.src_channel_id.src_channel_id;
            let dst_port_id = value.dst_port_id.dst_port_id;
            let dst_channel_id = value.dst_channel_id.dst_channel_id;
            let _src_connection_id = value.dst_connection_id.connection_id;
            let acknowledgement = value.acknowledgement.acknowledgement.0;
            Self {
                packet: Packet {
                    sequence: sequence.into(),
                    source_port: src_port_id.into(),
                    source_channel: src_channel_id.into(),
                    destination_port: dst_port_id.into(),
                    destination_channel: dst_channel_id.into(),
                    data: packet_data,
                    timeout_height: timeout_height.into(),
                    timeout_timestamp: timeout_timestamp.into(),
                },
                ack: acknowledgement,
            }
        }
    }
    // Acknowledgement todo
    impl From<SubxtAcknowledgement> for Acknowledgement {
        fn from(ack: SubxtAcknowledgement) -> Self {
            Self::from(ack.0)
        }
    }
    // Receipt todo
    impl From<SubxtReceipt> for Receipt {
        fn from(receipt: SubxtReceipt) -> Self {
            match receipt {
                SubxtReceipt::Ok => Self::Ok,
            }
        }
    }
    // Sequence
    impl From<SubxtSequence> for Sequence {
        fn from(value: SubxtSequence) -> Self {
            Sequence::from(value.0)
        }
    }
    // TimeoutHeight
    impl From<SubxtTimeoutHeight> for TimeoutHeight {
        fn from(value: SubxtTimeoutHeight) -> Self {
            match value {
                SubxtTimeoutHeight::Never => Self::Never,
                SubxtTimeoutHeight::At(v) => Self::At(v.into()),
            }
        }
    }
    // Version  todo
    impl From<SubxtChannelVersion> for ChannelVersion {
        fn from(value: SubxtChannelVersion) -> Self {
            Self(value.0)
        }
    }

    // ------- ics23 commitment
    // CommitmentPrefix todo
    impl From<SubxtCommitmentPrefix> for CommitmentPrefix {
        fn from(value: SubxtCommitmentPrefix) -> Self {
            CommitmentPrefix::try_from(value.bytes)
                .expect("converty failed because subxt commitment Prefix is empty")
        }
    }

    // -------ics24 host
    // ChannelId
    impl From<SubxtChannelId> for ChannelId {
        fn from(value: SubxtChannelId) -> Self {
            ChannelId::from_str(value.0.as_ref()).expect("convert channelId: Never failed")
        }
    }
    // clientId
    impl From<SubxtClientId> for ClientId {
        fn from(value: SubxtClientId) -> Self {
            ClientId::from_str(value.0.as_ref()).expect("convert clientId: Never failed")
        }
    }
    // connectionId
    impl From<SubxtConnectionId> for ConnectionId {
        fn from(value: SubxtConnectionId) -> Self {
            ConnectionId::from_str(value.0.as_ref()).expect("convert connectionid: Never failed")
        }
    }
    // PortId
    impl From<SubxtPortId> for PortId {
        fn from(value: SubxtPortId) -> Self {
            PortId::from_str(value.0.as_ref()).expect("convert PortId: Never failed")
        }
    }

    // -------- ics26 routing
    // ModuleId
    impl From<SubxtModuleId> for ModuleId {
        fn from(value: SubxtModuleId) -> Self {
            ModuleId::from_str(value.0.as_ref()).expect("conver moudleid: never failed ")
        }
    }

    // --- events
    impl From<SubxtIbcEvent> for IbcEvent {
        fn from(value: SubxtIbcEvent) -> Self {
            match value {
                SubxtIbcEvent::CreateClient(value) => IbcEvent::CreateClient(value.into()),
                SubxtIbcEvent::UpdateClient(value) => IbcEvent::UpdateClient(value.into()),
                SubxtIbcEvent::UpgradeClient(value) => IbcEvent::UpgradeClient(value.into()),
                SubxtIbcEvent::ClientMisbehaviour(value) => {
                    IbcEvent::ClientMisbehaviour(value.into())
                }
                SubxtIbcEvent::OpenInitConnection(value) => {
                    IbcEvent::OpenInitConnection(value.into())
                }
                SubxtIbcEvent::OpenTryConnection(value) => {
                    IbcEvent::OpenTryConnection(value.into())
                }
                SubxtIbcEvent::OpenAckConnection(value) => {
                    IbcEvent::OpenAckConnection(value.into())
                }
                SubxtIbcEvent::OpenConfirmConnection(value) => {
                    IbcEvent::OpenConfirmConnection(value.into())
                }
                SubxtIbcEvent::OpenInitChannel(value) => IbcEvent::OpenInitChannel(value.into()),
                SubxtIbcEvent::OpenTryChannel(value) => IbcEvent::OpenTryChannel(value.into()),
                SubxtIbcEvent::OpenAckChannel(value) => IbcEvent::OpenAckChannel(value.into()),
                SubxtIbcEvent::OpenConfirmChannel(value) => {
                    IbcEvent::OpenConfirmChannel(value.into())
                }
                SubxtIbcEvent::CloseInitChannel(value) => IbcEvent::CloseInitChannel(value.into()),
                SubxtIbcEvent::CloseConfirmChannel(value) => {
                    IbcEvent::CloseConfirmChannel(value.into())
                }
                SubxtIbcEvent::SendPacket(value) => IbcEvent::SendPacket(value.into()),
                SubxtIbcEvent::ReceivePacket(value) => IbcEvent::ReceivePacket(value.into()),
                SubxtIbcEvent::WriteAcknowledgement(value) => {
                    IbcEvent::WriteAcknowledgement(value.into())
                }
                SubxtIbcEvent::AcknowledgePacket(value) => {
                    IbcEvent::AcknowledgePacket(value.into())
                }
                SubxtIbcEvent::TimeoutPacket(value) => IbcEvent::TimeoutPacket(value.into()),
                SubxtIbcEvent::ChannelClosed(value) => IbcEvent::ChainError(format!("{:?}", value)), // todo ibc_relayer_type::events don't have ChannelClosed variant.
                SubxtIbcEvent::AppModule(value) => IbcEvent::AppModule(value.into()),
            }
        }
    }

    impl From<SubxtModuleEvent> for events::ModuleEvent {
        fn from(value: SubxtModuleEvent) -> Self {
            let kind = value.kind;
            let module_name = value.module_name;
            let attributes = value.attributes.into_iter().map(|v| v.into()).collect();
            Self {
                kind,
                module_name: module_name.into(),
                attributes,
            }
        }
    }

    impl From<SubxtModuleEventAttribute> for ModuleEventAttribute {
        fn from(value: SubxtModuleEventAttribute) -> Self {
            Self {
                key: value.key,
                value: value.value,
            }
        }
    }

    // ------------------- timestamp
    impl From<SubxtTimestamp> for Timestamp {
        fn from(value: SubxtTimestamp) -> Self {
            if let Some(v) = value.time {
                // todo unwrap need hanele
                Timestamp::from_nanoseconds(v as u64).ok().unwrap()
            } else {
                Timestamp::none()
            }
        }
    }
}

pub struct SubstrateChain {
    config: ChainConfig,
    // tx_config: TxConfig,
    rpc_client: OnlineClient<SubstrateConfig>,
    // grpc_addr: Uri,
    // light_client: TmLightClient,
    rt: Arc<TokioRuntime>,
    // keybase: KeyRing<Secp256k1KeyPair>,

    // account: Option<Account>,

    // tx_monitor_cmd: Option<TxMonitorCmd>,
}

impl ChainEndpoint for SubstrateChain {
    type LightBlock = SubLightBlock;
    type Header = SmHeader;
    type ConsensusState = SmConsensusState;
    type ClientState = SmClientState;
    type SigningKeyPair = Secp256k1KeyPair;

    fn bootstrap(config: ChainConfig, rt: Arc<TokioRuntime>) -> Result<Self, Error> {
        let rpc_client = rt.block_on(OnlineClient::<SubstrateConfig>::new()).unwrap();

        Ok(Self {
            config,
            rpc_client,
            rt,
        })
    }

    fn shutdown(self) -> Result<(), Error> {
        unimplemented!();
    }

    fn keybase(&self) -> &KeyRing<Self::SigningKeyPair> {
        unimplemented!();
    }

    fn keybase_mut(&mut self) -> &mut KeyRing<Self::SigningKeyPair> {
        unimplemented!();
    }

    fn subscribe(&mut self) -> Result<Subscription, Error> {
        unimplemented!();
    }

    /// Does multiple RPC calls to the full node, to check for
    /// reachability and some basic APIs are available.
    ///
    /// Currently this checks that:
    ///     - the node responds OK to `/health` RPC call;
    ///     - the node has transaction indexing enabled;
    ///     - the SDK & IBC versions are supported;
    ///
    /// Emits a log warning in case anything is amiss.
    /// Exits early if any health check fails, without doing any
    /// further checks.
    fn health_check(&self) -> Result<HealthCheck, Error> {
        unimplemented!();
    }

    /// Fetch a header from the chain at the given height and verify it.
    fn verify_header(
        &mut self,
        trusted: ICSHeight,
        target: ICSHeight,
        client_state: &AnyClientState,
    ) -> Result<Self::LightBlock, Error> {
        println!(
            "trusted: {:?}, target: {:?}, client_state: {:?}",
            trusted, target, client_state
        );
        Ok(SubLightBlock {})
    }

    /// Given a client update event that includes the header used in a client update,
    /// look for misbehaviour by fetching a header at same or latest height.
    fn check_misbehaviour(
        &mut self,
        update: &UpdateClient,
        client_state: &AnyClientState,
    ) -> Result<Option<MisbehaviourEvidence>, Error> {
        unimplemented!();
    }

    // Queries

    /// Send one or more transactions that include all the specified messages.
    /// The `proto_msgs` are split in transactions such they don't exceed the configured maximum
    /// number of messages per transaction and the maximum transaction size.
    /// Then `send_tx()` is called with each Tx. `send_tx()` determines the fee based on the
    /// on-chain simulation and if this exceeds the maximum gas specified in the configuration file
    /// then it returns error.
    /// TODO - more work is required here for a smarter split maybe iteratively accumulating/ evaluating
    /// msgs in a Tx until any of the max size, max num msgs, max fee are exceeded.
    fn send_messages_and_wait_commit(
        &mut self,
        tracked_msgs: TrackedMsgs,
    ) -> Result<Vec<IbcEventWithHeight>, Error> {
        let proto_msgs = tracked_msgs.msgs;

        // let msg: Vec<pallet_ibc::Any> = proto_msgs.iter().map(|m| pallet_ibc::Any{type_url: m.type_url.as_bytes().to_vec(), value: m.value}).collect();
        let msg: Vec<substrate::runtime_types::ibc_proto::google::protobuf::Any> = proto_msgs
            .iter()
            .map(
                |m| substrate::runtime_types::ibc_proto::google::protobuf::Any {
                    type_url: m.type_url.clone(),
                    value: m.value.clone(),
                },
            )
            .collect();
        let signer: PairSigner<SubstrateConfig, subxt::ext::sp_core::sr25519::Pair> =
            PairSigner::new(AccountKeyring::Alice.pair());

        let binding = self.rpc_client.tx();
        let tx = substrate::tx().ibc().deliver(msg);

        let runtime = self.rt.clone();
        let deliver = binding.sign_and_submit_then_watch_default(&tx, &signer);
        let result = runtime.block_on(deliver);
        // println!("send_messages_and_wait_commit result: {:?}", result);
        let events = runtime.block_on(result.unwrap().wait_for_finalized_success());

        let ibc_events = events
            .unwrap()
            .find_first::<substrate::ibc::events::IbcEvents>()
            .unwrap()
            .unwrap();
        let es: Vec<IbcEventWithHeight> = ibc_events
            .events
            .into_iter()
            .map(|e| match e {
                _ => IbcEventWithHeight {
                    event: IbcEvent::from(e),
                    height: ICSHeight::new(0, 10).unwrap(),
                },
            })
            .collect();

        Ok(es)
    }

    fn send_messages_and_wait_check_tx(
        &mut self,
        tracked_msgs: TrackedMsgs,
    ) -> Result<Vec<Response>, Error> {
        unimplemented!();
    }

    /// Get the account for the signer
    fn get_signer(&self) -> Result<Signer, Error> {
        // $ subkey inspect //Alice
        // Secret Key URI `//Alice` is account:
        // Network ID:        substrate
        // Secret seed:       0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a
        // Public key (hex):  0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
        // Account ID:        0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
        // Public key (SS58): 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
        // SS58 Address:      5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
        Ok(
            Signer::from_str("0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a")
                .unwrap(),
        )
    }

    /// Get the chain configuration
    fn config(&self) -> &ChainConfig {
        &self.config
    }

    fn ibc_version(&self) -> Result<Option<semver::Version>, Error> {
        unimplemented!();
    }

    fn query_balance(&self, key_name: Option<&str>, denom: Option<&str>) -> Result<Balance, Error> {
        unimplemented!();
    }

    fn query_all_balances(&self, key_name: Option<&str>) -> Result<Vec<Balance>, Error> {
        unimplemented!();
    }

    fn query_denom_trace(&self, hash: String) -> Result<DenomTrace, Error> {
        unimplemented!();
    }

    fn query_commitment_prefix(&self) -> Result<CommitmentPrefix, Error> {
        crate::time!("query_commitment_prefix");
        crate::telemetry!(query, self.id(), "query_commitment_prefix");

        // TODO - do a real chain query
        CommitmentPrefix::try_from(self.config.store_prefix.as_bytes().to_vec())
            .map_err(|_| Error::ics02(ClientError::empty_prefix()))
    }

    /// Query the application status
    fn query_application_status(&self) -> Result<ChainStatus, Error> {
        crate::time!("query_application_status");
        crate::telemetry!(query, self.id(), "query_application_status");

        let finalized_head = self
            .rt
            .block_on(self.rpc_client.rpc().finalized_head())
            .unwrap();
        info!("finalized_header '{}'", finalized_head);
        let block = self
            .rt
            .block_on(self.rpc_client.rpc().block(Some(finalized_head)))
            .unwrap();
        // info!("extrinsics[0] '{:?}'", block.unwrap().block.extrinsics[0]);
        // let extrinsic = block.unwrap().block.extrinsics[0];
        // let b = parity_scale_codec::Decode::decode::<substrate::timestamp::calls::TransactionApi>(&mut extrinsic.0[..]);
        // let b = parity_scale_codec::Decode::decode::<substrate::runtime_types::kitchensink_runtime::RuntimeCall>(&mut extrinsic.0[..]);

        // let a = substrate::runtime_types::kitchensink_runtime::RuntimeCall::Timestamp.decode(&mut extrinsic);
        // let a = parity_scale_codec::Decode::decode::<substrate::timestamp::calls::Set>(&mut block.unwrap().block.extrinsics[0]);

        Ok(ChainStatus {
            height: ICSHeight::new(0, u64::from(block.unwrap().block.header.number)).unwrap(),
            timestamp: Timestamp::default(),
        })
    }

    fn query_clients(
        &self,
        request: QueryClientStatesRequest,
    ) -> Result<Vec<IdentifiedAnyClientState>, Error> {
        unimplemented!();
    }

    fn query_client_state(
        &self,
        request: QueryClientStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyClientState, Option<MerkleProof>), Error> {
        println!("query_client_state: request: {:?}", request);
        println!("query_client_state: include_proof: {:?}", include_proof);

        let client_id = substrate::runtime_types::ibc::core::ics24_host::identifier::ClientId(
            request.client_id.to_string(),
        );
        let storage = substrate::storage().ibc().client_states(client_id);

        let states = self
            .rt
            .block_on(self.rpc_client.storage().fetch(&storage, None))
            .unwrap();

        let client_state = AnyClientState::decode_vec(&states.unwrap()).map_err(Error::decode)?;

        println!("states: {:?}", client_state);
        Ok((client_state, None))
    }

    fn query_upgraded_client_state(
        &self,
        request: QueryUpgradedClientStateRequest,
    ) -> Result<(AnyClientState, MerkleProof), Error> {
        unimplemented!();
    }

    fn query_upgraded_consensus_state(
        &self,
        request: QueryUpgradedConsensusStateRequest,
    ) -> Result<(AnyConsensusState, MerkleProof), Error> {
        unimplemented!();
    }

    fn query_consensus_state(
        &self,
        request: QueryConsensusStateRequest,
        include_proof: IncludeProof,
    ) -> Result<(AnyConsensusState, Option<MerkleProof>), Error> {
        let client_id = substrate::runtime_types::ibc::core::ics24_host::identifier::ClientId(
            request.client_id.to_string(),
        );

        let height = substrate::runtime_types::ibc::core::ics02_client::height::Height {
            revision_number: request.consensus_height.revision_number(),
            revision_height: request.consensus_height.revision_height(),
        };
        let storage = substrate::storage()
            .ibc()
            .consensus_states(client_id, height);

        let consensus_states = self
            .rt
            .block_on(self.rpc_client.storage().fetch(&storage, None))
            .unwrap();

        let consensus_state =
            AnyConsensusState::decode_vec(&consensus_states.unwrap()).map_err(Error::decode)?;

        println!("consensus_state: {:?}", consensus_state);
        Ok((consensus_state, None))
    }

    /// Query the heights of every consensus state for a given client.
    fn query_consensus_state_heights(
        &self,
        request: QueryConsensusStateHeightsRequest,
    ) -> Result<Vec<ICSHeight>, Error> {
        unimplemented!();
    }

    fn query_client_connections(
        &self,
        request: QueryClientConnectionsRequest,
    ) -> Result<Vec<ConnectionId>, Error> {
        unimplemented!();
    }

    fn query_connections(
        &self,
        request: QueryConnectionsRequest,
    ) -> Result<Vec<IdentifiedConnectionEnd>, Error> {
        unimplemented!();
    }

    fn query_connection(
        &self,
        request: QueryConnectionRequest,
        include_proof: IncludeProof,
    ) -> Result<(ConnectionEnd, Option<MerkleProof>), Error> {
        let connection_id =
            substrate::runtime_types::ibc::core::ics24_host::identifier::ConnectionId(
                request.connection_id.to_string(),
            );
        let storage = substrate::storage().ibc().connections(connection_id);
        let connection = self
            .rt
            .block_on(self.rpc_client.storage().fetch(&storage, None))
            .unwrap();

        let conn = connection.unwrap();

        println!("connection: {:?}", conn);
        Ok((conn.into(), None))
    }

    fn query_connection_channels(
        &self,
        request: QueryConnectionChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        unimplemented!();
    }

    fn query_channels(
        &self,
        request: QueryChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        unimplemented!();
    }

    fn query_channel(
        &self,
        request: QueryChannelRequest,
        include_proof: IncludeProof,
    ) -> Result<(ChannelEnd, Option<MerkleProof>), Error> {
        unimplemented!();
    }

    fn query_channel_client_state(
        &self,
        request: QueryChannelClientStateRequest,
    ) -> Result<Option<IdentifiedAnyClientState>, Error> {
        unimplemented!();
    }

    fn query_packet_commitment(
        &self,
        request: QueryPacketCommitmentRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        unimplemented!();
    }

    /// Queries the packet commitment hashes associated with a channel.
    fn query_packet_commitments(
        &self,
        request: QueryPacketCommitmentsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), Error> {
        unimplemented!();
    }

    fn query_packet_receipt(
        &self,
        request: QueryPacketReceiptRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        unimplemented!();
    }

    /// Queries the unreceived packet sequences associated with a channel.
    fn query_unreceived_packets(
        &self,
        request: QueryUnreceivedPacketsRequest,
    ) -> Result<Vec<Sequence>, Error> {
        unimplemented!();
    }

    fn query_packet_acknowledgement(
        &self,
        request: QueryPacketAcknowledgementRequest,
        include_proof: IncludeProof,
    ) -> Result<(Vec<u8>, Option<MerkleProof>), Error> {
        unimplemented!();
    }

    /// Queries the packet acknowledgment hashes associated with a channel.
    fn query_packet_acknowledgements(
        &self,
        request: QueryPacketAcknowledgementsRequest,
    ) -> Result<(Vec<Sequence>, ICSHeight), Error> {
        unimplemented!();
    }

    /// Queries the unreceived acknowledgements sequences associated with a channel.
    fn query_unreceived_acknowledgements(
        &self,
        request: QueryUnreceivedAcksRequest,
    ) -> Result<Vec<Sequence>, Error> {
        unimplemented!();
    }

    fn query_next_sequence_receive(
        &self,
        request: QueryNextSequenceReceiveRequest,
        include_proof: IncludeProof,
    ) -> Result<(Sequence, Option<MerkleProof>), Error> {
        unimplemented!();
    }

    /// This function queries transactions for events matching certain criteria.
    /// 1. Client Update request - returns a vector with at most one update client event
    /// 2. Transaction event request - returns all IBC events resulted from a Tx execution
    fn query_txs(&self, request: QueryTxRequest) -> Result<Vec<IbcEventWithHeight>, Error> {
        unimplemented!();
    }

    /// This function queries transactions for packet events matching certain criteria.
    /// It returns at most one packet event for each sequence specified in the request.
    ///    Note - there is no way to format the packet query such that it asks for Tx-es with either
    ///    sequence (the query conditions can only be AND-ed).
    ///    There is a possibility to include "<=" and ">=" conditions but it doesn't work with
    ///    string attributes (sequence is emmitted as a string).
    ///    Therefore, for packets we perform one tx_search for each sequence.
    ///    Alternatively, a single query for all packets could be performed but it would return all
    ///    packets ever sent.
    fn query_packet_events(
        &self,
        mut request: QueryPacketEventDataRequest,
    ) -> Result<Vec<IbcEventWithHeight>, Error> {
        unimplemented!();
    }

    fn query_host_consensus_state(
        &self,
        request: QueryHostConsensusStateRequest,
    ) -> Result<Self::ConsensusState, Error> {
        unimplemented!();
    }

    fn build_client_state(
        &self,
        height: ICSHeight,
        settings: ClientSettings,
    ) -> Result<Self::ClientState, Error> {
        // Build the client state.
        let pk = PublicKey(
            tendermint::PublicKey::from_raw_secp256k1(&hex_literal::hex!(
                "02c88aca653727db28e0ade87497c1f03b551143dedfd4db8de71689ad5e38421c"
            ))
            .unwrap(),
        );
        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let timestamp_nanos = duration_since_epoch.as_nanos(); // u128
        Ok(SmClientState {
            sequence: height.revision_height(),
            frozen_sequence: u64::max_value(),
            consensus_state: SmConsensusState {
                public_key: pk,
                diversifier: "oct".to_string(),
                // timestamp: timestamp_nanos as u64,
                timestamp: 9999,
                root: CommitmentRoot::from_bytes(&pk.to_bytes()),
            },
            allow_update_after_proposal: false,
        })
    }

    fn build_consensus_state(
        &self,
        light_block: Self::LightBlock,
    ) -> Result<Self::ConsensusState, Error> {
        let pk = PublicKey(
            tendermint::PublicKey::from_raw_secp256k1(&hex_literal::hex!(
                "02c88aca653727db28e0ade87497c1f03b551143dedfd4db8de71689ad5e38421c"
            ))
            .unwrap(),
        );
        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let timestamp_nanos = duration_since_epoch.as_nanos(); // u128

        Ok(SmConsensusState {
            public_key: pk,
            diversifier: "oct".to_string(),
            // timestamp: timestamp_nanos as u64,
            timestamp: 9999,
            root: CommitmentRoot::from_bytes(&pk.to_bytes()),
        })
    }

    fn build_header(
        &mut self,
        trusted_height: ICSHeight,
        target_height: ICSHeight,
        client_state: &AnyClientState,
    ) -> Result<(Self::Header, Vec<Self::Header>), Error> {
        println!(
            "trusted_height: {:?}, target_height: {:?}, client_state: {:?}",
            trusted_height, target_height, client_state
        );
        let pk = PublicKey(
            tendermint::PublicKey::from_raw_secp256k1(&hex_literal::hex!(
                "02c88aca653727db28e0ade87497c1f03b551143dedfd4db8de71689ad5e38421c"
            ))
            .unwrap(),
        );
        println!("pk: {:?}", pk);
        let duration_since_epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let timestamp_nanos = duration_since_epoch
            .checked_sub(Duration::from_secs(5))
            .unwrap()
            .as_nanos() as u64; // u128
        let data = HeaderData {
            new_pub_key: Some(pk),
            new_diversifier: "oct".to_string(),
        };

        let bytes = SignBytes {
            sequence: client_state.latest_height().revision_height(),
            timestamp: timestamp_nanos,
            diversifier: "oct".to_string(),
            data_type: 9,
            // data: Protobuf::<SmHeaderData>::encode_vec(&data).expect("encoding to `Any` from `HeaderData`"),
            data: data.encode_vec().unwrap(),
        };

        let encoded_bytes = bytes.encode_vec().unwrap();
        println!("encoded_bytes: {:?}", encoded_bytes);
        let standard = StandardHDPath::from_str("m/44'/60'/0'/0/0").unwrap();
        println!("standard: {:?}", standard);

        // m/44'/60'/0'/0/0
        // 0xd73E35f53b8180b241E70C0e9040173dd8D0e2A0
        // 0x02c88aca653727db28e0ade87497c1f03b551143dedfd4db8de71689ad5e38421c
        // 0x281afd44d50ffd0bab6502cbb9bc58a7f9b53813c862db01836d46a27b51168c

        let key_pair = Secp256k1KeyPair::from_mnemonic("captain walk infant web eye return ahead once face sunny usage devote cotton car old check symbol antique derive wire kid solve forest fish", &standard, &AddressType::Cosmos, "oct").unwrap();
        println!("key_pair: {:?}", key_pair.public_key.to_string());
        let signature = key_pair.sign(&encoded_bytes).unwrap();
        println!("signature: {:?}", signature);

        let header = SmHeader {
            sequence: client_state.latest_height().revision_height(),
            timestamp: timestamp_nanos,
            signature,
            new_public_key: Some(pk),
            new_diversifier: "oct".to_string(),
        };

        Ok((header, vec![]))
    }

    fn maybe_register_counterparty_payee(
        &mut self,
        channel_id: &ChannelId,
        port_id: &PortId,
        counterparty_payee: &Signer,
    ) -> Result<(), Error> {
        unimplemented!();
    }

    fn cross_chain_query(
        &self,
        requests: Vec<CrossChainQueryRequest>,
    ) -> Result<Vec<CrossChainQueryResponse>, Error> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {}
