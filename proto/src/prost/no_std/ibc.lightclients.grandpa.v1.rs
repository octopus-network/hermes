/// Client state
/// The GRANDPA client state tracks latest height and a possible frozen height.
/// interface ClientState {
///   latestHeight: uint64
///   frozenHeight: Maybe<uint64>
/// }
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientState {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// Latest height the client was updated to
    #[prost(message, optional, tag = "2")]
    pub latest_height: ::core::option::Option<super::super::super::core::client::v1::Height>,
    /// Block height when the client was frozen due to a misbehaviour
    #[prost(message, optional, tag = "3")]
    pub frozen_height: ::core::option::Option<super::super::super::core::client::v1::Height>,
}
/// Consensus state
/// The GRANDPA client tracks authority set and commitment root for all previously verified consensus states.
/// interface ConsensusState {
///   authoritySet: AuthoritySet
///   commitmentRoot: []byte
/// }
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusState {
    /// commitment root (i.e app hash)
    ///
    /// authoritySet
    /// AuthoritySet authority_set = 2 [(gogoproto.nullable) = false];
    #[prost(message, optional, tag = "1")]
    pub root: ::core::option::Option<super::super::super::core::commitment::v1::MerkleRoot>,
}
///   ibc.core.client.v1.Height from_height = 1 [(gogoproto.nullable) = false, (gogoproto.moretags) = "yaml:\"frome_height\""];
///   Header header_1 = 2 [(gogoproto.customname) = "Header1", (gogoproto.moretags) = "yaml:\"header_1\""];
///   Header header_2 = 3 [(gogoproto.customname) = "Header2", (gogoproto.moretags) = "yaml:\"header_2\""];
/// }
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Misbehaviour {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub header_1: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "3")]
    pub header_2: ::core::option::Option<Header>,
}
/// Headers
/// The GRANDPA client headers include the height, the commitment root,a justification of block and authority set.
/// (In fact, here is a proof of authority set rather than the authority set itself, but we can using a fixed key
/// to verify the proof and extract the real set, the details are ignored here)
/// interface Header {
///   height: uint64
///   commitmentRoot: []byte
///   justification: Justification
///   authoritySet: AuthoritySet
/// }
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(message, optional, tag = "1")]
    pub height: ::core::option::Option<super::super::super::core::client::v1::Height>,
}
// Justification
// A GRANDPA justification for block finality, it includes a commit message and an ancestry proof including
// all headers routing all precommit target blocks to the commit target block. For example, the latest blocks
// are A - B - C - D - E - F, where A is the last finalised block, F is the point where a majority for vote
//(they may on B, C, D, E, F) can be collected. Then the proof need to include all headers from F back to A.

/// interface Justification {
///   round: uint64
///   commit: Commit
///   votesAncestries: []Header
/// }
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Justification {
    #[prost(uint64, tag = "1")]
    pub round: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub commit: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "3")]
    pub votes_ancestry: ::prost::alloc::vec::Vec<Header>,
}
/// Authority set
/// A set of authorities for GRANDPA.
/// interface AuthoritySet {
///   // this is incremented every time the set changes
///   setId: uint64
///   authorities: List<Pair<AuthorityId, AuthorityWeight>>
/// }
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthoritySet {
    #[prost(uint64, tag = "1")]
    pub set_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub authority: ::prost::alloc::vec::Vec<Authority>,
}
/// Authority
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authority {
    #[prost(bytes = "vec", tag = "1")]
    pub authority_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub authority_weight: ::prost::alloc::vec::Vec<u8>,
}
/// Commit
/// A commit message which is an aggregate of signed precommits.
/// interface Commit {
///   precommits: []SignedPrecommit
/// }
/// interface SignedPrecommit {
///   targetHash: Hash
///   signature: Signature
///   id: AuthorityId
/// }
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commit {
    #[prost(message, repeated, tag = "1")]
    pub precommit: ::prost::alloc::vec::Vec<SignedPrecommit>,
}
/// SignedPrecommit
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedPrecommit {
    #[prost(bytes = "vec", tag = "1")]
    pub authority_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub target_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
