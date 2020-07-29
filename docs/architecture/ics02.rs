//! Client Query interface

/// Chain queries
trait ChainQuery<C>
where
    C: Chain,
{
    /// MUST be defined by the chain which is validated by a particular client,
    /// and should allow for retrieval of headers by height.
    ///
    /// This endpoint is assumed to be untrusted.
    fn query_header(height: Height) -> Result<C::Header, Error>;

    /// MAY be defined by the chain which is validated by a particular client,
    /// to allow for the retrieval of the current consensus state which can be
    /// used to construct a new client. When used in this fashion, the returned
    /// ConsensusState MUST be manually confirmed by the querying entity,
    /// since it is subjective.
    ///
    /// This endpoint is assumed to be untrusted.
    /// The precise nature of the ConsensusState may vary per client type.
    fn query_chain_consensus_state(height: Height) -> Result<C::ConsensusState, Error>;
}

/// On-chain state queries
trait OnChainStateQuery<C>
where
    C: Chain,
{
    fn query_client_state(identifier: Identifier) -> Result<C::ClientState, Error>; // looks into private store
}

trait ClientType<C>
where
    C: Chain,
{
    fn check_validity_and_update_state(&self, header: C::Header) -> Result<(), Error>;
    fn check_misbehaviour_and_update_state(&self, evidence: Vec<u8>) -> Result<(), Error>;
}

trait ClientState<C>
where
    C: Chain,
{
    /// latest verified height (from which the consensus state can then be retrieved using `query_consensus_state` if desired).
    fn latest_height(&self) -> Height;

    /// allows stored consensus states to be retrieved by height.
    fn query_consensus_state(
        identifier: Identifier, // which identifier?
        height: Height,
    ) -> Result<C::ConsensusState, Error>;

    /// initialize a client state with a provided consensus state, writing to internal state as appropriate.
    fn initialize(consensusState: C::ConsensusState) -> Result<Self, Error>;

    type Prover: ClientProver;

    fn prover(&self) -> Self::Prover;
}

trait ClientProver {
    fn query_and_prove_client_consensus_state(
        client_identifier: ClientIdentifier,
        height: Height,
        prefix: CommitmentPrefix,
        consensus_state_height: Height,
    ) -> (ConsensusState, Proof);

    fn query_and_prove_connection_state(
        connection_identifier: ConnectionIdentifier,
        height: Height,
        prefix: CommitmentPrefix,
    ) -> (ConnectionEnd, Proof);

    fn query_and_prove_channel_state(
        port_identifier: PortIdentifier,
        channel_identifier: ChannelIdentifier,
        height: Height,
        prefix: CommitmentPrefix,
    ) -> (ChannelEnd, Proof);

    fn query_and_prove_packet_data(
        port_identifier: PortIdentifier,
        channel_identifier: ChannelIdentifier,
        height: Height,
        prefix: CommitmentPrefix,
        sequence: u64,
    ) -> (Vec<u8>, Proof);

    fn query_and_prove_packet_acknowledgement(
        port_identifier: PortIdentifier,
        channel_identifier: ChannelIdentifier,
        height: Height,
        prefix: CommitmentPrefix,
        sequence: u64,
    ) -> (Vec<u8>, Proof);

    fn query_and_prove_packet_acknowledgement_absence(
        port_identifier: PortIdentifier,
        channel_identifier: ChannelIdentifier,
        height: Height,
        prefix: CommitmentPrefix,
        sequence: u64,
    ) -> Proof;

    fn query_and_prove_next_sequence_recv(
        port_identifier: PortIdentifier,
        channel_identifier: ChannelIdentifier,
        height: Height,
        prefix: CommitmentPrefix,
    ) -> (u64, Proof);
}

fn create_client<C>(
    client_id: ClientIdentifier,
    client_type: C::ClientType,
    consensus_state: C::ConsensusState,
) -> Result<(), Error>
where
    C: Chain,
{
    validate_client_identifier(client_id)?;

    if private_store.exists(client_state_path(client_id)) {
        bail!(ErrorKind::ClientStateAlreadyExists);
    }

    if provable_store.exists(client_type_path(client_id)) {
        bail!(ErrorKind::ClientTypeAlreadyExists);
    }

    client_type.initialise(consensus_state)?;

    provable_store.set(client_type_path(client_id), clientType);
}

fn update_client<C>(client_id: ClientIdentifier, header: C::Header) -> Result<(), Error>
where
    C: Chain,
{
    let client_type = provable_store.get(client_type_path(client_id))?;
    let client_state = private_store.get(client_state_path(client_id))?;

    client_type.check_validity_and_update_state(client_state, header)
}

fn submit_misbehaviour_to_client(client_id: Identifier, evidence: Vec<u8>) -> Result<(), Error> {
    let client_type = provable_store.get(client_type_path(client_id))?;
    let client_state = private_store.get(client_state_path(client_id))?;

    client_type.check_misbehaviour_and_update_state(client_state, evidence)
}

