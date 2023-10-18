use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::Height;
use prost::{EncodeError, Message};

/// GenesisState defines 08-wasm's keeper genesis state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// uploaded light client wasm contracts
    #[prost(message, repeated, tag = "1")]
    pub contracts: ::prost::alloc::vec::Vec<Contract>,
}
/// Contract stores contract code
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    /// contract byte code
    #[prost(bytes = "vec", tag = "1")]
    pub code_bytes: ::prost::alloc::vec::Vec<u8>,
}

/// Wasm light client's Client state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientState {
    /// bytes encoding the client state of the underlying light client
    /// implemented as a Wasm contract.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub latest_height: ::core::option::Option<Height>,
}
/// Wasm light client's ConsensusState
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusState {
    /// bytes encoding the consensus state of the underlying light client
    /// implemented as a Wasm contract.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Wasm light client message (either header(s) or misbehaviour)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientMessage {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}

impl TryFrom<ClientState> for Any {
    type Error = EncodeError;

    fn try_from(cs: ClientState) -> Result<Self, Self::Error> {
        Ok(Any {
            type_url: "/ibc.lightclients.wasm.v1.ClientState".to_string(),
            value: cs.encode_to_vec(),
        })
    }
}

impl TryFrom<ConsensusState> for Any {
    type Error = EncodeError;

    fn try_from(cs: ConsensusState) -> Result<Self, Self::Error> {
        Ok(Any {
            type_url: "/ibc.lightclients.wasm.v1.ConsensusState".to_string(),
            value: cs.encode_to_vec(),
        })
    }
}
