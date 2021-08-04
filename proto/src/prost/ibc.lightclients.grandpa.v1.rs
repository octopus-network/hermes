/// ClientState from Tendermint tracks the current validator set, latest height,
/// and a possible frozen height.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientState;

/// ConsensusState defines the consensus state from Tendermint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusState;

/// Misbehaviour is a wrapper over two conflicting Headers
/// that implements Misbehaviour interface expected by ICS-02
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Misbehaviour;

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
pub struct Header;

/// Fraction defines the protobuf message type for tmmath.Fraction that only supports positive values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fraction;
