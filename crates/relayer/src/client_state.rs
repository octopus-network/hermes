use core::time::Duration;

use ibc_proto::ibc::core::client::v1::IdentifiedClientState;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as RawClientState;
#[cfg(test)]
use ibc_proto::ibc::mock::ClientState as RawMockClientState;
use ibc_proto::protobuf::Protobuf;
use serde::{Deserialize, Serialize};

use ibc::clients::ics07_tendermint::client_state::{
    ClientState as TmClientState, UpgradeOptions as TmUpgradeOptions,
    TENDERMINT_CLIENT_STATE_TYPE_URL,
};
use ibc::core::ics02_client::client_state::UpdatedState;
use ibc::core::ics02_client::client_state::{downcast_client_state, ClientState, UpgradeOptions};
use ibc::core::ics02_client::client_type::ClientType;
use ibc::core::ics02_client::consensus_state::ConsensusState;
use ibc::core::ics02_client::context::ClientReader;
use ibc::core::ics02_client::error::Error;
use ibc::core::ics02_client::trust_threshold::TrustThreshold;
use ibc::core::ics03_connection::connection::ConnectionEnd;
use ibc::core::ics04_channel::channel::ChannelEnd;
use ibc::core::ics04_channel::commitment::AcknowledgementCommitment;
use ibc::core::ics04_channel::commitment::PacketCommitment;
use ibc::core::ics04_channel::context::ChannelReader;
use ibc::core::ics04_channel::packet::Sequence;
use ibc::core::ics23_commitment::commitment::CommitmentPrefix;
use ibc::core::ics23_commitment::commitment::CommitmentProofBytes;
use ibc::core::ics23_commitment::commitment::CommitmentRoot;
use ibc::core::ics24_host::identifier::ChannelId;
use ibc::core::ics24_host::identifier::ConnectionId;
use ibc::core::ics24_host::identifier::PortId;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::commitment::v1::MerkleProof;

use ibc::core::ics24_host::error::ValidationError;
use ibc::core::ics24_host::identifier::{ChainId, ClientId};
#[cfg(test)]
use ibc::mock::client_state::MockClientState;
#[cfg(test)]
use ibc::mock::client_state::MOCK_CLIENT_STATE_TYPE_URL;
use ibc::Height;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnyUpgradeOptions {
    Tendermint(TmUpgradeOptions),

    #[cfg(test)]
    Mock(()),
}

impl AnyUpgradeOptions {
    fn as_tm_upgrade_options(&self) -> Option<&TmUpgradeOptions> {
        match self {
            AnyUpgradeOptions::Tendermint(tm) => Some(tm),
            #[cfg(test)]
            AnyUpgradeOptions::Mock(_) => None,
        }
    }
}

impl UpgradeOptions for AnyUpgradeOptions {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnyClientState {
    Tendermint(TmClientState),

    #[cfg(test)]
    Mock(MockClientState),
}

impl AnyClientState {
    pub fn latest_height(&self) -> Height {
        match self {
            Self::Tendermint(tm_state) => tm_state.latest_height(),

            #[cfg(test)]
            Self::Mock(mock_state) => mock_state.latest_height(),
        }
    }

    pub fn frozen_height(&self) -> Option<Height> {
        match self {
            Self::Tendermint(tm_state) => tm_state.frozen_height(),

            #[cfg(test)]
            Self::Mock(mock_state) => mock_state.frozen_height(),
        }
    }

    pub fn trust_threshold(&self) -> Option<TrustThreshold> {
        match self {
            AnyClientState::Tendermint(state) => Some(state.trust_level().clone()),

            #[cfg(test)]
            AnyClientState::Mock(_) => None,
        }
    }

    pub fn max_clock_drift(&self) -> Duration {
        match self {
            AnyClientState::Tendermint(state) => state.max_clock_drift().clone(),

            #[cfg(test)]
            AnyClientState::Mock(_) => Duration::new(0, 0),
        }
    }

    pub fn client_type(&self) -> ClientType {
        match self {
            Self::Tendermint(state) => state.client_type(),

            #[cfg(test)]
            Self::Mock(state) => state.client_type(),
        }
    }

    pub fn refresh_period(&self) -> Option<Duration> {
        match self {
            AnyClientState::Tendermint(tm_state) => tm_state.refresh_time(),

            #[cfg(test)]
            AnyClientState::Mock(mock_state) => mock_state.refresh_time(),
        }
    }
}

impl Protobuf<Any> for AnyClientState {}

impl TryFrom<Any> for AnyClientState {
    type Error = Error;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        match raw.type_url.as_str() {
            "" => Err(Error::empty_client_state_response()),

            TENDERMINT_CLIENT_STATE_TYPE_URL => Ok(AnyClientState::Tendermint(
                Protobuf::<RawClientState>::decode_vec(&raw.value)
                    .map_err(Error::decode_raw_client_state)?,
            )),

            #[cfg(test)]
            MOCK_CLIENT_STATE_TYPE_URL => Ok(AnyClientState::Mock(
                Protobuf::<RawMockClientState>::decode_vec(&raw.value)
                    .map_err(Error::decode_raw_client_state)?,
            )),

            _ => Err(Error::unknown_client_state_type(raw.type_url)),
        }
    }
}

impl From<AnyClientState> for Any {
    fn from(value: AnyClientState) -> Self {
        match value {
            AnyClientState::Tendermint(value) => Any {
                type_url: TENDERMINT_CLIENT_STATE_TYPE_URL.to_string(),
                value: Protobuf::<RawClientState>::encode_vec(&value)
                    .expect("encoding to `Any` from `AnyClientState::Tendermint`"),
            },
            #[cfg(test)]
            AnyClientState::Mock(value) => Any {
                type_url: MOCK_CLIENT_STATE_TYPE_URL.to_string(),
                value: Protobuf::<RawMockClientState>::encode_vec(&value)
                    .expect("encoding to `Any` from `AnyClientState::Mock`"),
            },
        }
    }
}

impl ClientState for AnyClientState {
    fn chain_id(&self) -> ChainId {
        match self {
            AnyClientState::Tendermint(tm_state) => tm_state.chain_id(),

            #[cfg(test)]
            AnyClientState::Mock(mock_state) => mock_state.chain_id(),
        }
    }

    fn client_type(&self) -> ClientType {
        self.client_type()
    }

    fn latest_height(&self) -> Height {
        self.latest_height()
    }

    fn frozen_height(&self) -> Option<Height> {
        self.frozen_height()
    }

    fn upgrade(
        &mut self,
        upgrade_height: Height,
        upgrade_options: &dyn UpgradeOptions,
        chain_id: ChainId,
    ) {
        let upgrade_options = upgrade_options
            .as_any()
            .downcast_ref::<AnyUpgradeOptions>()
            .expect("UpgradeOptions not of type AnyUpgradeOptions");
        match self {
            AnyClientState::Tendermint(tm_state) => tm_state.upgrade(
                upgrade_height,
                upgrade_options.as_tm_upgrade_options().unwrap(),
                chain_id,
            ),

            #[cfg(test)]
            AnyClientState::Mock(mock_state) => {
                mock_state.upgrade(upgrade_height, upgrade_options, chain_id)
            }
        }
    }

    fn expired(&self, elapsed_since_latest: Duration) -> bool {
        match self {
            AnyClientState::Tendermint(tm_state) => tm_state.expired(elapsed_since_latest),

            #[cfg(test)]
            AnyClientState::Mock(mock_state) => mock_state.expired(elapsed_since_latest),
        }
    }

    fn initialise(&self, consensus_state: Any) -> Result<Box<dyn ConsensusState>, Error> {
        match self {
            AnyClientState::Tendermint(tm_state) => tm_state.initialise(consensus_state),

            #[cfg(test)]
            AnyClientState::Mock(mock_state) => mock_state.initialise(consensus_state),
        }
    }

    fn check_header_and_update_state(
        &self,
        ctx: &dyn ClientReader,
        client_id: ClientId,
        header: Any,
    ) -> Result<UpdatedState, Error> {
        match self {
            AnyClientState::Tendermint(tm_state) => {
                tm_state.check_header_and_update_state(ctx, client_id, header)
            }

            #[cfg(test)]
            AnyClientState::Mock(mock_state) => {
                mock_state.check_header_and_update_state(ctx, client_id, header)
            }
        }
    }

    fn verify_upgrade_and_update_state(
        &self,
        consensus_state: Any,
        proof_upgrade_client: MerkleProof,
        proof_upgrade_consensus_state: MerkleProof,
    ) -> Result<UpdatedState, Error> {
        match self {
            AnyClientState::Tendermint(tm_state) => tm_state.verify_upgrade_and_update_state(
                consensus_state,
                proof_upgrade_client,
                proof_upgrade_consensus_state,
            ),

            #[cfg(test)]
            AnyClientState::Mock(mock_state) => mock_state.verify_upgrade_and_update_state(
                consensus_state,
                proof_upgrade_client,
                proof_upgrade_consensus_state,
            ),
        }
    }

    /// Verification functions as specified in:
    /// <https://github.com/cosmos/ibc/tree/master/spec/core/ics-002-client-semantics>
    ///
    /// Verify a `proof` that the consensus state of a given client (at height `consensus_height`)
    /// matches the input `consensus_state`. The parameter `counterparty_height` represent the
    /// height of the counterparty chain that this proof assumes (i.e., the height at which this
    /// proof was computed).
    #[allow(clippy::too_many_arguments)]
    fn verify_client_consensus_state(
        &self,
        height: Height,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        client_id: &ClientId,
        consensus_height: Height,
        expected_consensus_state: &dyn ConsensusState,
    ) -> Result<(), Error> {
        match self {
            AnyClientState::Tendermint(tm_state) => tm_state.verify_client_consensus_state(
                height,
                prefix,
                proof,
                root,
                client_id,
                consensus_height,
                expected_consensus_state,
            ),

            #[cfg(test)]
            AnyClientState::Mock(mock_state) => mock_state.verify_client_consensus_state(
                height,
                prefix,
                proof,
                root,
                client_id,
                consensus_height,
                expected_consensus_state,
            ),
        }
    }

    /// Verify a `proof` that a connection state matches that of the input `connection_end`.
    #[allow(clippy::too_many_arguments)]
    fn verify_connection_state(
        &self,
        height: Height,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        connection_id: &ConnectionId,
        expected_connection_end: &ConnectionEnd,
    ) -> Result<(), Error> {
        match self {
            AnyClientState::Tendermint(tm_state) => tm_state.verify_connection_state(
                height,
                prefix,
                proof,
                root,
                connection_id,
                expected_connection_end,
            ),

            #[cfg(test)]
            AnyClientState::Mock(mock_state) => mock_state.verify_connection_state(
                height,
                prefix,
                proof,
                root,
                connection_id,
                expected_connection_end,
            ),
        }
    }

    /// Verify a `proof` that a channel state matches that of the input `channel_end`.
    #[allow(clippy::too_many_arguments)]
    fn verify_channel_state(
        &self,
        height: Height,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        port_id: &PortId,
        channel_id: &ChannelId,
        expected_channel_end: &ChannelEnd,
    ) -> Result<(), Error> {
        match self {
            AnyClientState::Tendermint(tm_state) => tm_state.verify_channel_state(
                height,
                prefix,
                proof,
                root,
                port_id,
                channel_id,
                expected_channel_end,
            ),

            #[cfg(test)]
            AnyClientState::Mock(mock_state) => mock_state.verify_channel_state(
                height,
                prefix,
                proof,
                root,
                port_id,
                channel_id,
                expected_channel_end,
            ),
        }
    }

    /// Verify the client state for this chain that it is stored on the counterparty chain.
    #[allow(clippy::too_many_arguments)]
    fn verify_client_full_state(
        &self,
        height: Height,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        client_id: &ClientId,
        expected_client_state: Any,
    ) -> Result<(), Error> {
        match self {
            AnyClientState::Tendermint(tm_state) => tm_state.verify_client_full_state(
                height,
                prefix,
                proof,
                root,
                client_id,
                expected_client_state,
            ),

            #[cfg(test)]
            AnyClientState::Mock(mock_state) => mock_state.verify_client_full_state(
                height,
                prefix,
                proof,
                root,
                client_id,
                expected_client_state,
            ),
        }
    }

    /// Verify a `proof` that a packet has been commited.
    #[allow(clippy::too_many_arguments)]
    fn verify_packet_data(
        &self,
        ctx: &dyn ChannelReader,
        height: Height,
        connection_end: &ConnectionEnd,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
        commitment: PacketCommitment,
    ) -> Result<(), Error> {
        match self {
            AnyClientState::Tendermint(tm_state) => tm_state.verify_packet_data(
                ctx,
                height,
                connection_end,
                proof,
                root,
                port_id,
                channel_id,
                sequence,
                commitment,
            ),

            #[cfg(test)]
            AnyClientState::Mock(mock_state) => mock_state.verify_packet_data(
                ctx,
                height,
                connection_end,
                proof,
                root,
                port_id,
                channel_id,
                sequence,
                commitment,
            ),
        }
    }

    /// Verify a `proof` that a packet has been commited.
    #[allow(clippy::too_many_arguments)]
    fn verify_packet_acknowledgement(
        &self,
        ctx: &dyn ChannelReader,
        height: Height,
        connection_end: &ConnectionEnd,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
        ack: AcknowledgementCommitment,
    ) -> Result<(), Error> {
        match self {
            AnyClientState::Tendermint(tm_state) => tm_state.verify_packet_acknowledgement(
                ctx,
                height,
                connection_end,
                proof,
                root,
                port_id,
                channel_id,
                sequence,
                ack,
            ),

            #[cfg(test)]
            AnyClientState::Mock(mock_state) => mock_state.verify_packet_acknowledgement(
                ctx,
                height,
                connection_end,
                proof,
                root,
                port_id,
                channel_id,
                sequence,
                ack,
            ),
        }
    }

    /// Verify a `proof` that of the next_seq_received.
    #[allow(clippy::too_many_arguments)]
    fn verify_next_sequence_recv(
        &self,
        ctx: &dyn ChannelReader,
        height: Height,
        connection_end: &ConnectionEnd,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Result<(), Error> {
        match self {
            AnyClientState::Tendermint(tm_state) => tm_state.verify_next_sequence_recv(
                ctx,
                height,
                connection_end,
                proof,
                root,
                port_id,
                channel_id,
                sequence,
            ),

            #[cfg(test)]
            AnyClientState::Mock(mock_state) => mock_state.verify_next_sequence_recv(
                ctx,
                height,
                connection_end,
                proof,
                root,
                port_id,
                channel_id,
                sequence,
            ),
        }
    }

    /// Verify a `proof` that a packet has not been received.
    #[allow(clippy::too_many_arguments)]
    fn verify_packet_receipt_absence(
        &self,
        ctx: &dyn ChannelReader,
        height: Height,
        connection_end: &ConnectionEnd,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Result<(), Error> {
        match self {
            AnyClientState::Tendermint(tm_state) => tm_state.verify_packet_receipt_absence(
                ctx,
                height,
                connection_end,
                proof,
                root,
                port_id,
                channel_id,
                sequence,
            ),

            #[cfg(test)]
            AnyClientState::Mock(mock_state) => mock_state.verify_packet_receipt_absence(
                ctx,
                height,
                connection_end,
                proof,
                root,
                port_id,
                channel_id,
                sequence,
            ),
        }
    }
}

impl From<TmClientState> for AnyClientState {
    fn from(cs: TmClientState) -> Self {
        Self::Tendermint(cs)
    }
}

#[cfg(test)]
impl From<MockClientState> for AnyClientState {
    fn from(cs: MockClientState) -> Self {
        Self::Mock(cs)
    }
}

impl From<&dyn ClientState> for AnyClientState {
    fn from(client_state: &dyn ClientState) -> Self {
        #[cfg(test)]
        if let Some(cs) = downcast_client_state::<MockClientState>(client_state) {
            return AnyClientState::from(*cs);
        }

        if let Some(cs) = downcast_client_state::<TmClientState>(client_state) {
            AnyClientState::from(cs.clone())
        } else {
            unreachable!()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct IdentifiedAnyClientState {
    pub client_id: ClientId,
    pub client_state: AnyClientState,
}

impl IdentifiedAnyClientState {
    pub fn new(client_id: ClientId, client_state: AnyClientState) -> Self {
        IdentifiedAnyClientState {
            client_id,
            client_state,
        }
    }
}

impl Protobuf<IdentifiedClientState> for IdentifiedAnyClientState {}

impl TryFrom<IdentifiedClientState> for IdentifiedAnyClientState {
    type Error = Error;

    fn try_from(raw: IdentifiedClientState) -> Result<Self, Self::Error> {
        Ok(IdentifiedAnyClientState {
            client_id: raw.client_id.parse().map_err(|e: ValidationError| {
                Error::invalid_raw_client_id(raw.client_id.clone(), e)
            })?,
            client_state: raw
                .client_state
                .ok_or_else(Error::missing_raw_client_state)?
                .try_into()?,
        })
    }
}

impl From<IdentifiedAnyClientState> for IdentifiedClientState {
    fn from(value: IdentifiedAnyClientState) -> Self {
        IdentifiedClientState {
            client_id: value.client_id.to_string(),
            client_state: Some(value.client_state.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use ibc::clients::ics07_tendermint::client_state::test_util::get_dummy_tendermint_client_state;
    use ibc::clients::ics07_tendermint::header::test_util::get_dummy_tendermint_header;
    use ibc_proto::google::protobuf::Any;
    use test_log::test;

    use super::AnyClientState;

    #[test]
    fn any_client_state_serialization() {
        let tm_client_state: AnyClientState =
            get_dummy_tendermint_client_state(get_dummy_tendermint_header()).into();

        let raw: Any = tm_client_state.clone().into();
        let tm_client_state_back = AnyClientState::try_from(raw).unwrap();
        assert_eq!(tm_client_state, tm_client_state_back);
    }
}
