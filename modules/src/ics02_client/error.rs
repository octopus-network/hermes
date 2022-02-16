use crate::prelude::*;

use core::num::TryFromIntError;

use flex_error::{define_error, TraceError};

use crate::ics02_client::client_type::ClientType;
use crate::ics04_channel::packet::Sequence;
use crate::ics07_tendermint::error::Error as Ics07Error;
use crate::ics10_grandpa::error::Error as Ics10Error;
use crate::ics23_commitment::error::Error as Ics23Error;
use crate::ics24_host::error::ValidationError;
use crate::ics24_host::identifier::ClientId;
use crate::Height;

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
            [ tendermint::Error ]
            | e | { format_args!("failed to build Tendermint domain type trust threshold from fraction: {}/{}", e.numerator, e.denominator) },

        UnknownClientStateType
            { client_state_type: String }
            | e | { format_args!("unknown client state type: {0}", e.client_state_type) },

        EmptyClientStateResponse
            | _ | { "the client state was not found" },

        EmptyPrefix
            { source: crate::ics23_commitment::merkle::EmptyPrefixError }
            | _ | { "empty prefix" },

        UnknownConsensusStateType
            { consensus_state_type: String }
            | e | {
                format_args!("unknown client consensus state type: {0}",
                    e.consensus_state_type)
            },

        EmptyConsensusStateResponse
            | _ | { "the client state was not found" },

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
            [ TraceError<tendermint_proto::Error> ]
            | _ | { "error decoding raw client state" },

        MissingRawClientState
            | _ | { "missing raw client state" },

        InvalidRawConsensusState
            [ TraceError<tendermint_proto::Error> ]
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
            [ TraceError<tendermint_proto::Error> ]
            | _ | { "invalid raw header" },

        MissingRawHeader
            | _ | { "missing raw header" },

        DecodeRawMisbehaviour
            [ TraceError<tendermint_proto::Error> ]
            | _ | { "invalid raw misbehaviour" },

        InvalidRawMisbehaviour
            [ ValidationError ]
            | _ | { "invalid raw misbehaviour" },

        MissingRawMisbehaviour
            | _ | { "missing raw misbehaviour" },

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

        Tendermint
            [ Ics07Error ]
            | _ | { "tendermint error" },

        Grandpa
            [ Ics10Error ]
            | _ | { "grandpa error" },

        InvalidPacketTimestamp
            [ TraceError<TryFromIntError> ]
            | _ | { "invalid packet timeout timestamp value" },

        ClientArgsTypeMismatch
            { client_type: ClientType }
            | e | {
                format_args!("mismatch between client and arguments types, expected: {0:?}",
                    e.client_type)
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
                format_args!("upgraded client height {0} must be at greater than current client height {1}",
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

        InvalidPacketCommitment
            { sequence: Sequence }
            | e | {
                format_args!(
                    "The stored commitment of the packet {0} is invaid",
                    e.sequence)
            },

    }
}
