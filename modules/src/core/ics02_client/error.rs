use crate::prelude::*;

use flex_error::{define_error, DisplayOnly, TraceError};

use crate::clients::ics07_tendermint::error::Error as Ics07Error;
use crate::clients::ics10_grandpa::error::Error as Ics10Error;
use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::height::HeightError;
use crate::core::ics04_channel::packet::Sequence;
use crate::core::ics23_commitment::error::Error as Ics23Error;
use crate::core::ics24_host::error::ValidationError;
use crate::core::ics24_host::identifier::ClientId;
use crate::timestamp::Timestamp;
use crate::Height;

use tendermint::Error as TendermintError;
use tendermint_proto::Error as TendermintProtoError;

define_error! {
    #[derive(Debug, PartialEq, Eq)]
    Error {
        UnknownClientType
            { client_type: String }
            | e | { format_args!("unknown client type: {0}", e.client_type) },

        ClientIdentifierConstructor
            { client_type: ClientType, counter: u64 }
            [ ValidationError ]
            | e | {
                format_args!("Client identifier constructor failed for type {0} with counter {1}",
                    e.client_type, e.counter)
            },

        ClientAlreadyExists
            { client_id: ClientId }
            | e | { format_args!("client already exists: {0}", e.client_id) },

        ClientNotFound
            { client_id: ClientId }
            | e | { format_args!("client not found: {0}", e.client_id) },

        ClientFrozen
            { client_id: ClientId }
            | e | { format_args!("client is frozen: {0}", e.client_id) },

        ConsensusStateNotFound
            { client_id: ClientId, height: Height }
            | e | {
                format_args!("consensus state not found at: {0} at height {1}",
                    e.client_id, e.height)
            },

        ImplementationSpecific
            | _ | { "implementation specific error" },

        HeaderVerificationFailure
            { reason: String }
            | e | { format_args!("header verification failed with reason: {}", e.reason) },

        InvalidTrustThreshold
            { numerator: u64, denominator: u64 }
            | e | { format_args!("failed to build trust threshold from fraction: {}/{}", e.numerator, e.denominator) },

        FailedTrustThresholdConversion
            { numerator: u64, denominator: u64 }
            [ TendermintError ]
            | e | { format_args!("failed to build Tendermint domain type trust threshold from fraction: {}/{}", e.numerator, e.denominator) },

        UnknownClientStateType
            { client_state_type: String }
            | e | { format_args!("unknown client state type: {0}", e.client_state_type) },

        EmptyClientStateResponse
            | _ | { "the client state was not found" },

        EmptyPrefix
            | _ | { "empty prefix" },

        UnknownConsensusStateType
            { consensus_state_type: String }
            | e | {
                format_args!("unknown client consensus state type: {0}",
                    e.consensus_state_type)
            },

        EmptyConsensusStateResponse
            | _ | { "the client consensus state was not found" },

        UnknownHeaderType
            { header_type: String }
            | e | {
                format_args!("unknown header type: {0}",
                    e.header_type)
            },

        UnknownMisbehaviourType
            { misbehavior_type: String }
            | e | {
                format_args!("unknown misbehaviour type: {0}",
                    e.misbehavior_type)
            },

        InvalidRawClientId
            { client_id: String }
            [ ValidationError ]
            | e | {
                format_args!("invalid raw client identifier {0}",
                    e.client_id)
            },

        DecodeRawClientState
            [ TraceError<TendermintProtoError> ]
            | _ | { "error decoding raw client state" },

        MissingRawClientState
            | _ | { "missing raw client state" },

        InvalidRawConsensusState
            [ TraceError<TendermintProtoError> ]
            | _ | { "invalid raw client consensus state" },

        MissingRawConsensusState
            | _ | { "missing raw client consensus state" },

        InvalidMsgUpdateClientId
            [ ValidationError ]
            | _ | { "invalid client id in the update client message" },

        Decode
            [ TraceError<prost::DecodeError> ]
            | _ | { "decode error" },

        MissingHeight
            | _ | { "invalid raw client consensus state: the height field is missing" },

        InvalidClientIdentifier
            [ ValidationError ]
            | _ | { "invalid client identifier" },

        InvalidRawHeader
            [ TraceError<TendermintProtoError> ]
            | _ | { "invalid raw header" },

        MissingRawHeader
            | _ | { "missing raw header" },

        DecodeRawMisbehaviour
            [ TraceError<TendermintProtoError> ]
            | _ | { "invalid raw misbehaviour" },

        InvalidRawMisbehaviour
            [ ValidationError ]
            | _ | { "invalid raw misbehaviour" },

        MissingRawMisbehaviour
            | _ | { "missing raw misbehaviour" },

        InvalidStringAsHeight
            { value: String }
            [ HeightError ]
            | e | { format_args!("String {0} cannnot be converted to height", e.value) },

        InvalidHeightResult
            | _ | { "height cannot end up zero or negative" },

        InvalidAddress
            | _ | { "invalid address" },

        InvalidUpgradeClientProof
            [ Ics23Error ]
            | _ | { "invalid proof for the upgraded client state" },

        InvalidUpgradeConsensusStateProof
            [ Ics23Error ]
            | _ | { "invalid proof for the upgraded consensus state" },

        InvalidCommitmentProof
            [ Ics23Error ]
            | _ | { "invalid commitment proof bytes" },

        Tendermint
            [ Ics07Error ]
            | _ | { "tendermint error" },

        Grandpa
            [ Ics10Error ]
            | _ | { "grandpa error" },

        InvalidPacketTimestamp
            [ crate::timestamp::ParseTimestampError ]
            | _ | { "invalid packet timeout timestamp value" },

        ClientArgsTypeMismatch
            { client_type: ClientType }
            | e | {
                format_args!("mismatch between client and arguments types, expected: {0:?}",
                    e.client_type)
            },

        InsufficientVotingPower
            { reason: String }
            | e | {
                format_args!("Insufficient overlap {}", e.reason)
            },

        RawClientAndConsensusStateTypesMismatch
            {
                state_type: ClientType,
                consensus_type: ClientType,
            }
            | e | {
                format_args!("mismatch in raw client consensus state {} with expected state {}",
                    e.state_type, e.consensus_type)
            },

        LowHeaderHeight
            {
                header_height: Height,
                latest_height: Height
            }
            | e | {
                format!("received header height ({:?}) is lower than (or equal to) client latest height ({:?})",
                    e.header_height, e.latest_height)
            },

        LowUpgradeHeight
            {
                upgraded_height: Height,
                client_height: Height,
            }
            | e | {
                format_args!("upgraded client height {} must be at greater than current client height {}",
                    e.upgraded_height, e.client_height)
            },


        CantDecodeMmrProof
            | _ | { "cant decode Mmr Proof" },

        CantDecodeMmrRoot
            | _ | { "cant decode MMR root" },

        CantDecodeMmrLeaf
            | _ | { "cant decode MMR leaf" },

        FailedToVerifyMmrProof
            | _ | { "failed to verify MMR proof" },

        InvalidConnectionState
            | _ | { "invalid connection state"},

        InvalidClientState
            | _ | { "invalid client state"},

        WrongKeyNumber
            {
                key_number: u8,
            }
            | e | {
                format_args!("Key number of hashmap or doublehashmap must be 1 or 2, but {0} found",
                    e.key_number)
            },

        StorageSizeExceed
            {
                storage_size: u32,
            }
            | e | {
                format_args!("Storage size is {0}, which is over limit",
                    e.storage_size)
            },

        HeaderHashNotMatch
            | _ | { "Header hash not match" },

        InvalidMmrLeafProof
            | _ | { "Invalid Mmr Leaf Proof" },

        MissingLatestCommitment
            | _ | { "Missing Latest Commitment" },

        InvalidSignedCommitment
            | _ | { "Invalid Signed Commitment" },

        EmptyMmrRoot
            | _ | { "Empty Mmr Root" },

        EmpryMmrLeafProofItems
            | _ | { "Empty Mmr Proof Items" },

        EmptyMmrLeafParentHashMmrRoot
            | _ | { "empty Mmr Leaf parent hash Mmr Root" },

        InvalidMmrRootHeight
            {
                mmr_root_height: u32,
                header_height: u32,
            }
            | e | { format!("invalid Mmr Root height, it's not can verify header, header height ({0})> mmr root height ({1})",
                e.header_height, e.mmr_root_height)
            },
        InvalidHeaderHeight
            {
                client_state_height: u32,
                header_height: u32,
            }
            | e | { format!("invalid header height, it's not can verify header, header height ({0}) < client state height ({1})",
                e.header_height, e.client_state_height)
            },
        InvalidConsensusStateTimestamp
            {
                time1: Timestamp,
                time2: Timestamp,
            }
            | e | {
                format_args!("timestamp is invalid or missing, timestamp={0},  now={1}", e.time1, e.time2)
            },

        HeaderNotWithinTrustPeriod
            {
                latest_time:Timestamp,
                update_time: Timestamp,
            }
            | e | {
                format_args!("header not withing trusting period: expires_at={0} now={1}", e.latest_time, e.update_time)
            },

        TendermintHandlerError
            [ Ics07Error ]
            | _ | { format_args!("Tendermint-specific handler error") },

        MissingLocalConsensusState
            { height: Height }
            | e | { format_args!("the local consensus state could not be retrieved for height {}", e.height) },

        InvalidPacketCommitment
            { sequence: Sequence }
            | e | {
                format_args!(
                    "The stored commitment of the packet {0} is invaid",
                    e.sequence)
            },

        InvalidPacketAck
            { sequence: Sequence }
            | e | {
                format_args!(
                    "The stored acknowledgement of the packet {0} is invaid",
                    e.sequence)
            },

        InvalidNextSequenceRecv
            { sequence_restored: u64, sequence: u64}
            | e | {
                format_args!(
                    "The next_sequence_recv is {0}, which is should not be greater than the current sequence {1} in packet timeout verification of ordered channel",
                    e.sequence_restored, e.sequence)
            },

        InvalidFromUtf8
            [ TraceError<alloc::string::FromUtf8Error>]
            | _ | { "invalid from utf8"},

        InvalidDecode
            [ DisplayOnly<tendermint_proto::Error> ]
            | _ | { "invalid decode" },

        InvalidCodecDecode
            [ DisplayOnly<codec::Error> ]
            |_| { "invalid codec decode" },

        InvalidIncreaseClientCounter
            | _ | { "invalid client counter" },

        InvalidEncode
            [ DisplayOnly<tendermint_proto::Error> ]
            | _ | { "invalid encode" },

        InvalidSerdeJsonEncode
            [ DisplayOnly<serde_json::Error> ]
            |_| { "invalid serde json encode"},

        EmptyProof
            | _ | { "empty proof" },

        InvalidMerkleProof
            | _ | { "invalid merkle proof" },

        ReadProofCheck
            | _ | {"read proof check error" },

        InvalidHexDecode
            [ DisplayOnly<subtle_encoding::Error> ]
            | _ | { "invalid hex decode"},

        InvalidConnectionEnd
            [ TraceError<TendermintProtoError>]
            | _ | { "invalid connection end" },

        InvalidChannelEnd
            [ TraceError<TendermintProtoError>]
            | _ | { "invalid channel end" },

        InvalidAnyClientState
            [ TraceError<TendermintProtoError>]
            | _ | { "invalid any client state" },

        InvalidAnyConsensusState
            [ TraceError<TendermintProtoError> ]
            | _ | { "invalid any client consensus state" },
    }
}

impl From<Ics07Error> for Error {
    fn from(e: Ics07Error) -> Error {
        Error::tendermint_handler_error(e)
    }
}
