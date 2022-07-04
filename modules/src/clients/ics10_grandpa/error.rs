use alloc::string::String;

use crate::timestamp::Timestamp;
use crate::timestamp::TimestampOverflowError;
use crate::Height;
use crate::core::ics02_client;
use crate::core::ics24_host::identifier::ClientId;
use crate::core::ics24_host::error::ValidationError;
use crate::core::ics23_commitment::error::Error as Ics23Error;
use flex_error::{define_error, DisplayOnly, TraceError};

define_error! {
     #[derive(Debug, PartialEq, Eq)]
    Error{
        Dummy
            |_| { format_args!("dummy error") },

        Decode
            [ TraceError<prost::DecodeError> ]
            | _ | { "decode error" },

        MissingLatestHeight
            | _ | { "missing latest height" },

        MissingHeight
            | _ | { "missing height" },

        InvalidChainIdentifier
            [ ValidationError ]
            | _ | { "invalid chain identifier" },

        MissingFrozenHeight
            | _ | { "missing frozen height" },

        InvalidRawConsensusState
            { reason: String }
            | _ | { "invalid raw client consensus state" },

        InvalidRawMisbehaviour
            { reason: String }
            | _ | { "invalid raw misbehaviour" },

        Encode
            [ TraceError<prost::EncodeError> ]
            | _ | { "encode error" },

        EmptyCommitment
            | _ | { "empty commitment"},

        InvalidSignedCommitment
            | _ | { "invalid Signed Commitment" },

        InvalidValidatorMerkleProof
            | _ | { "invalid Validator Merkle Proof" },

        InvalidMmrLeaf
            | _ | { "invalid Mmr Leaf" },

        InvalidMmrLeafProof
            | _ | { "invalid Mmr Lead Proof" },

        InvalidCommitment
            | _ | { "Invalid commitment"},

        InvalidStorageProof
            | _ | { "invalid storage Proof" },

        GetStorageByProofErr
            {
                e: String,
            }
            | e | {
                format_args!("failed to get storage by proof: {0}", e)
            },

        InvalidChainId
            | _ | { "invalid chain id" },

        EmptyBlockHeader
            | _ | { "empty block header" },

        EmptyLatestCommitment
            | _ | { "empty latest commitment" },

        EmptyValidatorSet
            | _ | { "empty validator set" },

        EmptyMmrLeaf
            | _ | { "empty mmr leaf" },

        EmptyMmrLeafProof
            | _ | { "empty mmr leaf proof" },

        InvalidConvertHash
            | _ | { "invalid convert hash" },

        InvalidConvertSignature
            | _ | { "invalid convert signature" },

        EmptyParentNumberAndHash
            | _ | { "empty parent and hash" },

        EmptyBeefyNextAuthoritySet
            | _ | { "empty next authority set" },

        InvalidCodecDecode
            [ DisplayOnly<codec::Error> ]
            |_| { "invalid codec decode" },

        Ics23Error
            [ DisplayOnly<Ics23Error>]
            | _ | { "ics23 error" },

        NotEnoughTimeElapsed
            {
                current_time: Timestamp,
                earliest_time: Timestamp,
            }
            | e | {
                format_args!("not enough time elapsed, current timestamp {0} is still less than earliest acceptable timestamp {1}", e.current_time, e.earliest_time)
            },

        NotEnoughBlocksElapsed
            {
                current_height: Height,
                earliest_height: Height,
            }
            | e | {
                format_args!("not enough blocks elapsed, current height {0} is still less than earliest acceptable height {1}", e.current_height, e.earliest_height)
            },

        ProcessedTimeNotFound
            {
                client_id: ClientId,
                height: Height,
            }
            | e | {
                format_args!(
                    "Processed time for the client {0} at height {1} not found",
                    e.client_id, e.height)
            },

        ProcessedHeightNotFound
            {
                client_id: ClientId,
                height: Height,
            }
            | e | {
                format_args!(
                    "Processed height for the client {0} at height {1} not found",
                    e.client_id, e.height)
            },

        TimestampOverflow
            [ TimestampOverflowError ]
            |_| { "timestamp overflowed" }
    }
}
