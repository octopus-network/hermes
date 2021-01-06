/// ClientState from Tendermint tracks the current validator set, latest height,
/// and a possible frozen height.
///
/// option (gogoproto.goproto_getters) = false;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientState {
    #[prost(string, tag="1")]
    pub chain_id: std::string::String,
    /// Fraction trust_level = 2 [(gogoproto.nullable) = false, (gogoproto.moretags) = "yaml:\"trust_level\""];
    /// duration of the period since the LastestTimestamp during which the
    /// submitted headers are valid for upgrade
    /// google.protobuf.Duration trusting_period = 3
    ///     [(gogoproto.nullable) = false, (gogoproto.stdduration) = true, (gogoproto.moretags) = "yaml:\"trusting_period\""];
    /// // duration of the staking unbonding period
    /// google.protobuf.Duration unbonding_period = 4 [
    ///   (gogoproto.nullable)    = false,
    ///   (gogoproto.stdduration) = true,
    ///   (gogoproto.moretags)    = "yaml:\"unbonding_period\""
    /// ];
    /// // defines how much new (untrusted) header's Time can drift into the future.
    /// google.protobuf.Duration max_clock_drift = 5
    ///     [(gogoproto.nullable) = false, (gogoproto.stdduration) = true, (gogoproto.moretags) = "yaml:\"max_clock_drift\""];
    /// Block height when the client was frozen due to a misbehaviour
    #[prost(uint64, tag="2")]
    pub frozen_height: u64,
    /// Latest height the client was updated to
    #[prost(uint64, tag="3")]
    pub latest_height: u64,
    #[prost(uint64, tag="4")]
    pub set_id: u64,
    #[prost(message, repeated, tag="5")]
    pub authority_list: ::std::vec::Vec<Authority>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authority {
    #[prost(bytes, tag="1")]
    pub authority_id: std::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub authority_weight: u64,
}
/// ConsensusState defines the consensus state from Tendermint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusState {
    /// timestamp that corresponds to the block height in which the ConsensusState
    /// was stored.
    #[prost(message, optional, tag="1")]
    pub timestamp: ::std::option::Option<::prost_types::Timestamp>,
    /// commitment root (i.e app hash)
    #[prost(bytes, tag="2")]
    pub root: std::vec::Vec<u8>,
}
/// Misbehaviour is a wrapper over two conflicting Headers
/// that implements Misbehaviour interface expected by ICS-02
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Misbehaviour {
    #[prost(string, tag="1")]
    pub client_id: std::string::String,
    #[prost(string, tag="2")]
    pub chain_id: std::string::String,
    #[prost(message, optional, tag="3")]
    pub header_1: ::std::option::Option<Header>,
    #[prost(message, optional, tag="4")]
    pub header_2: ::std::option::Option<Header>,
}
/// Header defines the Tendermint client consensus Header.
/// It encapsulates all the information necessary to update from a trusted
/// Tendermint ConsensusState. The inclusion of TrustedHeight and
/// TrustedValidators allows this update to process correctly, so long as the
/// ConsensusState for the TrustedHeight exists, this removes race conditions
/// among relayers The SignedHeader and ValidatorSet are the new untrusted update
/// fields for the client. The TrustedHeight is the height of a stored
/// ConsensusState on the client that will be used to verify the new untrusted
/// header. The Trusted ConsensusState must be within the unbonding period of
/// current time in order to correctly verify, and the TrustedValidators must
/// hash to TrustedConsensusState.NextValidatorsHash since that is the last
/// trusted validator set at the TrustedHeight.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(message, optional, tag="1")]
    pub height: ::std::option::Option<super::super::super::core::client::v1::Height>,
    #[prost(bytes, tag="2")]
    pub commitment_root: std::vec::Vec<u8>,
    #[prost(bytes, tag="3")]
    pub block_hash: std::vec::Vec<u8>,
    #[prost(bytes, tag="4")]
    pub justification: std::vec::Vec<u8>,
    /// .tendermint.types.SignedHeader signed_header = 1
    /// [(gogoproto.embed) = true, (gogoproto.moretags) = "yaml:\"signed_header\""];
    #[prost(bytes, repeated, tag="5")]
    pub authorities_proof: ::std::vec::Vec<std::vec::Vec<u8>>,
}
/// Fraction defines the protobuf message type for tmmath.Fraction
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fraction {
    #[prost(int64, tag="1")]
    pub numerator: i64,
    #[prost(int64, tag="2")]
    pub denominator: i64,
}
