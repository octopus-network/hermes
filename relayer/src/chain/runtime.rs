use std::{sync::Arc, thread};

use crossbeam_channel as channel;
use tokio::runtime::Runtime as TokioRuntime;
use tracing::error;

use ibc::{
    events::IbcEvent,
    ics02_client::{
        client_consensus::{AnyConsensusState, AnyConsensusStateWithHeight, ConsensusState},
        client_state::{AnyClientState, ClientState, IdentifiedAnyClientState},
        events::UpdateClient,
        header::{AnyHeader, Header},
        misbehaviour::MisbehaviourEvidence,
    },
    ics03_connection::{
        connection::{ConnectionEnd, IdentifiedConnectionEnd},
        version::Version,
    },
    ics04_channel::{
        channel::{ChannelEnd, IdentifiedChannelEnd},
        packet::{PacketMsgType, Sequence},
    },
    ics23_commitment::commitment::CommitmentPrefix,
    ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
    proofs::Proofs,
    query::QueryTxRequest,
    signer::Signer,
    Height,
};
use ibc_proto::ibc::core::{
    channel::v1::{
        PacketState, QueryChannelClientStateRequest, QueryChannelsRequest,
        QueryConnectionChannelsRequest, QueryNextSequenceReceiveRequest,
        QueryPacketAcknowledgementsRequest, QueryPacketCommitmentsRequest,
        QueryUnreceivedAcksRequest, QueryUnreceivedPacketsRequest,
    },
    client::v1::{QueryClientStatesRequest, QueryConsensusStatesRequest},
    commitment::v1::MerkleProof,
    connection::v1::{QueryClientConnectionsRequest, QueryConnectionsRequest},
};

use crate::{
    config::ChainConfig,
    connection::ConnectionMsgType,
    error::Error,
    event::{
        bus::EventBus,
        monitor::{EventBatch, EventReceiver, MonitorCmd, Result as MonitorResult, TxMonitorCmd},
    },
    keyring::KeyEntry,
    light_client::LightClient,
};

use super::{
    handle::{ChainHandle, ChainRequest, ProdChainHandle, ReplyTo, Subscription},
    Chain,
};

pub struct Threads {
    pub chain_runtime: thread::JoinHandle<()>,
    pub event_monitor: Option<thread::JoinHandle<()>>,
}

pub struct ChainRuntime<C: Chain> {
    /// The specific chain this runtime runs against
    chain: C,

    /// The sender side of a channel to this runtime. Any `ChainHandle` can use this to send
    /// chain requests to this runtime
    request_sender: channel::Sender<ChainRequest>,

    /// The receiving side of a channel to this runtime. The runtime consumes chain requests coming
    /// in through this channel.
    request_receiver: channel::Receiver<ChainRequest>,

    /// An event bus, for broadcasting events that this runtime receives (via `event_receiver`) to subscribers
    event_bus: EventBus<Arc<MonitorResult<EventBatch>>>,

    /// Receiver channel from the event bus
    event_receiver: EventReceiver,

    /// Sender channel to terminate the event monitor
    tx_monitor_cmd: TxMonitorCmd,

    /// A handle to the light client
    light_client: Box<dyn LightClient<C>>,

    #[allow(dead_code)]
    rt: Arc<TokioRuntime>, // Making this future-proof, so we keep the runtime around.
}

impl<C: Chain + Send + 'static> ChainRuntime<C> {
    /// Spawns a new runtime for a specific Chain implementation.
    pub fn spawn(
        config: ChainConfig,
        rt: Arc<TokioRuntime>,
    ) -> Result<Box<dyn ChainHandle>, Error> {
        tracing::info!("in runtime: [spawn]");

        // Similar to `from_config`.
        let chain = C::bootstrap(config, rt.clone())?;

        // Start the light client
        let light_client = chain.init_light_client()?;

        // Start the event monitor
        let (event_batch_rx, tx_monitor_cmd) = chain.init_event_monitor(rt.clone())?;

        // Instantiate & spawn the runtime
        let (handle, _) = Self::init(chain, light_client, event_batch_rx, tx_monitor_cmd, rt);

        Ok(handle)
    }

    /// Initializes a runtime for a given chain, and spawns the associated thread
    fn init(
        chain: C,
        light_client: Box<dyn LightClient<C>>,
        event_receiver: EventReceiver,
        tx_monitor_cmd: TxMonitorCmd,
        rt: Arc<TokioRuntime>,
    ) -> (Box<dyn ChainHandle>, thread::JoinHandle<()>) {
        tracing::info!("in runtime: [init]");

        let chain_runtime = Self::new(chain, light_client, event_receiver, tx_monitor_cmd, rt);

        // Get a handle to the runtime
        let handle = chain_runtime.handle();

        // Spawn the runtime & return
        let id = handle.id();
        tracing::info!("in runtime: [init] >> id: {}", id);
        let thread = thread::spawn(move || {
            if let Err(e) = chain_runtime.run() {
                error!("failed to start runtime for chain '{}': {}", id, e);
            }
        });

        (handle, thread)
    }

    /// Basic constructor
    fn new(
        chain: C,
        light_client: Box<dyn LightClient<C>>,
        event_receiver: EventReceiver,
        tx_monitor_cmd: TxMonitorCmd,
        rt: Arc<TokioRuntime>,
    ) -> Self {
        tracing::info!("in runtime: [new]");

        let (request_sender, request_receiver) = channel::unbounded::<ChainRequest>();

        Self {
            rt,
            chain,
            request_sender,
            request_receiver,
            event_bus: EventBus::new(),
            event_receiver,
            tx_monitor_cmd,
            light_client,
        }
    }

    pub fn handle(&self) -> Box<dyn ChainHandle> {
        let chain_id = self.chain.id().clone();
        let sender = self.request_sender.clone();

        Box::new(ProdChainHandle::new(chain_id, sender))
    }

    fn run(mut self) -> Result<(), Error> {
        tracing::info!("in runtime: [run]");
        loop {
            channel::select! {
                recv(self.event_receiver) -> event_batch => {
                    match event_batch {
                        Ok(event_batch) => {
                            self.event_bus
                                .broadcast(Arc::new(event_batch));
                        },
                        Err(e) => {
                            error!("received error via event bus: {}", e);
                            return Err(Error::channel_receive(e));
                        },
                    }
                },
                recv(self.request_receiver) -> event => {
                    match event {
                        Ok(ChainRequest::Shutdown { reply_to }) => {
                            self.tx_monitor_cmd.send(MonitorCmd::Shutdown)
                                .map_err(Error::send)?;

                            let res = self.chain.shutdown();
                            reply_to.send(res)
                                .map_err(Error::send)?;

                            break;
                        }

                        Ok(ChainRequest::Subscribe { reply_to }) => {
                            self.subscribe(reply_to)?
                        },

                        Ok(ChainRequest::SendMsgs { proto_msgs, reply_to }) => {
                            self.send_msgs(proto_msgs, reply_to)?
                        },

                        Ok(ChainRequest::Signer { reply_to }) => {
                            tracing::info!("in Runtime: [Run] >> get_signer");
                            self.get_signer(reply_to)?
                        }

                        Ok(ChainRequest::Key { reply_to }) => {
                            self.get_key(reply_to)?
                        }

                        Ok(ChainRequest::ModuleVersion { port_id, reply_to }) => {
                            self.module_version(port_id, reply_to)?
                        }

                        Ok(ChainRequest::BuildHeader { trusted_height, target_height, client_state, reply_to }) => {
                            self.build_header(trusted_height, target_height, client_state, reply_to)?
                        }

                        Ok(ChainRequest::BuildClientState { height, reply_to }) => {
                            self.build_client_state(height, reply_to)?
                        }

                        Ok(ChainRequest::BuildConsensusState { trusted, target, client_state, reply_to }) => {
                            self.build_consensus_state(trusted, target, client_state, reply_to)?
                        }

                       Ok(ChainRequest::BuildMisbehaviour { client_state, update_event, reply_to }) => {
                            self.check_misbehaviour(update_event, client_state, reply_to)?
                        }

                        Ok(ChainRequest::BuildConnectionProofsAndClientState { message_type, connection_id, client_id, height, reply_to }) => {
                            self.build_connection_proofs_and_client_state(message_type, connection_id, client_id, height, reply_to)?
                        },

                        Ok(ChainRequest::BuildChannelProofs { port_id, channel_id, height, reply_to }) => {
                            self.build_channel_proofs(port_id, channel_id, height, reply_to)?
                        },

                        Ok(ChainRequest::QueryLatestHeight { reply_to }) => {
                            self.query_latest_height(reply_to)?
                        }

                        Ok(ChainRequest::QueryClients { request, reply_to }) => {
                            self.query_clients(request, reply_to)?
                        },

                        Ok(ChainRequest::QueryClientConnections { request, reply_to }) => {
                            self.query_client_connections(request, reply_to)?
                        },

                        Ok(ChainRequest::QueryClientState { client_id, height, reply_to }) => {
                            self.query_client_state(client_id, height, reply_to)?
                        },

                        Ok(ChainRequest::QueryConsensusStates { request, reply_to }) => {
                            self.query_consensus_states(request, reply_to)?
                        },

                        Ok(ChainRequest::QueryConsensusState { client_id, consensus_height, query_height, reply_to }) => {
                            self.query_consensus_state(client_id, consensus_height, query_height, reply_to)?
                        },

                        Ok(ChainRequest::QueryUpgradedClientState { height, reply_to }) => {
                            self.query_upgraded_client_state(height, reply_to)?
                        }

                       Ok(ChainRequest::QueryUpgradedConsensusState { height, reply_to }) => {
                            self.query_upgraded_consensus_state(height, reply_to)?
                        }

                        Ok(ChainRequest::QueryCommitmentPrefix { reply_to }) => {
                            self.query_commitment_prefix(reply_to)?
                        },

                        Ok(ChainRequest::QueryCompatibleVersions { reply_to }) => {
                            self.query_compatible_versions(reply_to)?
                        },

                        Ok(ChainRequest::QueryConnection { connection_id, height, reply_to }) => {
                            self.query_connection(connection_id, height, reply_to)?
                        },

                        Ok(ChainRequest::QueryConnections { request, reply_to }) => {
                            self.query_connections(request, reply_to)?
                        },

                        Ok(ChainRequest::QueryConnectionChannels { request, reply_to }) => {
                            self.query_connection_channels(request, reply_to)?
                        },

                        Ok(ChainRequest::QueryChannels { request, reply_to }) => {
                            self.query_channels(request, reply_to)?
                        },

                        Ok(ChainRequest::QueryChannel { port_id, channel_id, height, reply_to }) => {
                            self.query_channel(port_id, channel_id, height, reply_to)?
                        },

                        Ok(ChainRequest::QueryChannelClientState { request, reply_to }) => {
                            self.query_channel_client_state(request, reply_to)?
                        },

                        Ok(ChainRequest::ProvenClientState { client_id, height, reply_to }) => {
                            self.proven_client_state(client_id, height, reply_to)?
                        },

                        Ok(ChainRequest::ProvenConnection { connection_id, height, reply_to }) => {
                            self.proven_connection(connection_id, height, reply_to)?
                        },

                        Ok(ChainRequest::ProvenClientConsensus { client_id, consensus_height, height, reply_to }) => {
                            self.proven_client_consensus(client_id, consensus_height, height, reply_to)?
                        },

                        Ok(ChainRequest::BuildPacketProofs { packet_type, port_id, channel_id, sequence, height, reply_to }) => {
                            self.build_packet_proofs(packet_type, port_id, channel_id, sequence, height, reply_to)?
                        },

                        Ok(ChainRequest::QueryPacketCommitments { request, reply_to }) => {
                            self.query_packet_commitments(request, reply_to)?
                        },

                        Ok(ChainRequest::QueryUnreceivedPackets { request, reply_to }) => {
                            self.query_unreceived_packets(request, reply_to)?
                        },

                        Ok(ChainRequest::QueryPacketAcknowledgement { request, reply_to }) => {
                            self.query_packet_acknowledgements(request, reply_to)?
                        },

                        Ok(ChainRequest::QueryUnreceivedAcknowledgement { request, reply_to }) => {
                            self.query_unreceived_acknowledgement(request, reply_to)?
                        },

                        Ok(ChainRequest::QueryNextSequenceReceive { request, reply_to }) => {
                            self.query_next_sequence_receive(request, reply_to)?
                        },

                        Ok(ChainRequest::QueryPacketEventData { request, reply_to }) => {
                            self.query_txs(request, reply_to)?
                        },

                        Err(e) => error!("received error via chain request channel: {}", e),
                    }
                },
            }
        }

        Ok(())
    }

    fn subscribe(&mut self, reply_to: ReplyTo<Subscription>) -> Result<(), Error> {
        tracing::info!("In runtime: [subscribe]");
        let subscription = self.event_bus.subscribe();

        reply_to.send(Ok(subscription)).map_err(Error::send)?;

        Ok(())
    }

    fn send_msgs(
        &mut self,
        proto_msgs: Vec<prost_types::Any>,
        reply_to: ReplyTo<Vec<IbcEvent>>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [send_msgs]");
        let result = self.chain.send_msgs(proto_msgs);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn query_latest_height(&self, reply_to: ReplyTo<Height>) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_latest_height]");
        let latest_height = self.chain.query_latest_height();

        reply_to.send(latest_height).map_err(Error::send)?;

        Ok(())
    }

    fn get_signer(&mut self, reply_to: ReplyTo<Signer>) -> Result<(), Error> {
        tracing::info!("In Runtime: [get_signer]");
        let result = self.chain.get_signer();

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn get_key(&mut self, reply_to: ReplyTo<KeyEntry>) -> Result<(), Error> {
        tracing::info!("In Runtime: [get_key]");
        let result = self.chain.get_key();

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn module_version(&self, port_id: PortId, reply_to: ReplyTo<String>) -> Result<(), Error> {
        tracing::info!("In Runtime: [module_version]");
        let result = self.chain.query_module_version(&port_id);

        reply_to.send(Ok(result)).map_err(Error::send)?;

        Ok(())
    }

    fn build_header(
        &mut self,
        trusted_height: Height,
        target_height: Height,
        client_state: AnyClientState,
        reply_to: ReplyTo<(AnyHeader, Vec<AnyHeader>)>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [build_header]");
        let result = self
            .chain
            .build_header(
                trusted_height,
                target_height,
                &client_state,
                self.light_client.as_mut(),
            )
            .map(|(header, support)| {
                let header = header.wrap_any();
                let support = support.into_iter().map(|h| h.wrap_any()).collect();
                (header, support)
            });

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    /// Constructs a client state for the given height
    fn build_client_state(
        &self,
        height: Height,
        reply_to: ReplyTo<AnyClientState>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [build_client_state]");
        let client_state = self
            .chain
            .build_client_state(height)
            .map(|cs| cs.wrap_any());
        tracing::info!("In runtime: [build client state] >> client_state: [{:?}]", client_state);

        reply_to.send(client_state).map_err(Error::send)?;

        Ok(())
    }

    /// Constructs a consensus state for the given height
    fn build_consensus_state(
        &mut self,
        trusted: Height,
        target: Height,
        client_state: AnyClientState,
        reply_to: ReplyTo<AnyConsensusState>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [build_consensus_state]");
        let verified = self.light_client.verify(trusted, target, &client_state)?;

        let consensus_state = self
            .chain
            .build_consensus_state(verified.target)
            .map(|cs| cs.wrap_any());
        tracing::info!("In runtime: [build_conesnsus_state] >> consensus_state: [{:?}]", consensus_state);

        reply_to.send(consensus_state).map_err(Error::send)?;

        Ok(())
    }

    /// Constructs AnyMisbehaviour for the update event
    fn check_misbehaviour(
        &mut self,
        update_event: UpdateClient,
        client_state: AnyClientState,
        reply_to: ReplyTo<Option<MisbehaviourEvidence>>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [check_misbehaviour]");
        let misbehaviour = self
            .light_client
            .check_misbehaviour(update_event, &client_state);

        reply_to.send(misbehaviour).map_err(Error::send)?;

        Ok(())
    }

    fn build_connection_proofs_and_client_state(
        &self,
        message_type: ConnectionMsgType,
        connection_id: ConnectionId,
        client_id: ClientId,
        height: Height,
        reply_to: ReplyTo<(Option<AnyClientState>, Proofs)>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [build_connection_proofs_and_client_state]");
        let result = self.chain.build_connection_proofs_and_client_state(
            message_type,
            &connection_id,
            &client_id,
            height,
        );

        let result = result
            .map(|(opt_client_state, proofs)| (opt_client_state.map(|cs| cs.wrap_any()), proofs));

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn query_clients(
        &self,
        request: QueryClientStatesRequest,
        reply_to: ReplyTo<Vec<IdentifiedAnyClientState>>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_clients]");
        let result = self.chain.query_clients(request);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn query_client_connections(
        &self,
        request: QueryClientConnectionsRequest,
        reply_to: ReplyTo<Vec<ConnectionId>>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_client_connections]");
        let result = self.chain.query_client_connections(request);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn query_client_state(
        &self,
        client_id: ClientId,
        height: Height,
        reply_to: ReplyTo<AnyClientState>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_client_state]");
        let client_state = self
            .chain
            .query_client_state(&client_id, height)
            .map(|cs| cs.wrap_any());

        reply_to.send(client_state).map_err(Error::send)?;

        Ok(())
    }

    fn query_upgraded_client_state(
        &self,
        height: Height,
        reply_to: ReplyTo<(AnyClientState, MerkleProof)>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_upgraded_client_state]");
        let result = self
            .chain
            .query_upgraded_client_state(height)
            .map(|(cl, proof)| (cl.wrap_any(), proof));

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn query_consensus_states(
        &self,
        request: QueryConsensusStatesRequest,
        reply_to: ReplyTo<Vec<AnyConsensusStateWithHeight>>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_conesensus_states]");
        let consensus_states = self.chain.query_consensus_states(request);

        reply_to.send(consensus_states).map_err(Error::send)?;

        Ok(())
    }

    fn query_consensus_state(
        &self,
        client_id: ClientId,
        consensus_height: Height,
        query_height: Height,
        reply_to: ReplyTo<AnyConsensusState>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_consensus_state]");
        let consensus_state =
            self.chain
                .query_consensus_state(client_id, consensus_height, query_height);

        reply_to.send(consensus_state).map_err(Error::send)?;

        Ok(())
    }

    fn query_upgraded_consensus_state(
        &self,
        height: Height,
        reply_to: ReplyTo<(AnyConsensusState, MerkleProof)>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_upgraded_consensus_state]");
        let result = self
            .chain
            .query_upgraded_consensus_state(height)
            .map(|(cs, proof)| (cs.wrap_any(), proof));

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn query_commitment_prefix(&self, reply_to: ReplyTo<CommitmentPrefix>) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_commitment_prefix]");
        let prefix = self.chain.query_commitment_prefix();

        reply_to.send(prefix).map_err(Error::send)?;

        Ok(())
    }

    fn query_compatible_versions(&self, reply_to: ReplyTo<Vec<Version>>) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_compatible_versions]");
        let versions = self.chain.query_compatible_versions();

        reply_to.send(versions).map_err(Error::send)?;

        Ok(())
    }

    fn query_connection(
        &self,
        connection_id: ConnectionId,
        height: Height,
        reply_to: ReplyTo<ConnectionEnd>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_connection]");
        let connection_end = self.chain.query_connection(&connection_id, height);

        reply_to.send(connection_end).map_err(Error::send)?;

        Ok(())
    }

    fn query_connections(
        &self,
        request: QueryConnectionsRequest,
        reply_to: ReplyTo<Vec<IdentifiedConnectionEnd>>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_connections]");
        let result = self.chain.query_connections(request);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn query_connection_channels(
        &self,
        request: QueryConnectionChannelsRequest,
        reply_to: ReplyTo<Vec<IdentifiedChannelEnd>>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_connection_channels]");
        let result = self.chain.query_connection_channels(request);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn query_channels(
        &self,
        request: QueryChannelsRequest,
        reply_to: ReplyTo<Vec<IdentifiedChannelEnd>>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [qeury_channels]");
        let result = self.chain.query_channels(request);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn query_channel(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        height: Height,
        reply_to: ReplyTo<ChannelEnd>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_channel]");
        let result = self.chain.query_channel(&port_id, &channel_id, height);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn query_channel_client_state(
        &self,
        request: QueryChannelClientStateRequest,
        reply_to: ReplyTo<Option<IdentifiedAnyClientState>>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_channel_client_state]");
        let result = self.chain.query_channel_client_state(request);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn proven_client_state(
        &self,
        client_id: ClientId,
        height: Height,
        reply_to: ReplyTo<(AnyClientState, MerkleProof)>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_client_state]");
        let result = self
            .chain
            .proven_client_state(&client_id, height)
            .map(|(cs, mp)| (cs.wrap_any(), mp));

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn proven_connection(
        &self,
        connection_id: ConnectionId,
        height: Height,
        reply_to: ReplyTo<(ConnectionEnd, MerkleProof)>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [proven_connection]");
        let result = self.chain.proven_connection(&connection_id, height);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn proven_client_consensus(
        &self,
        client_id: ClientId,
        consensus_height: Height,
        height: Height,
        reply_to: ReplyTo<(AnyConsensusState, MerkleProof)>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [proven_client_consensus]");
        let result = self
            .chain
            .proven_client_consensus(&client_id, consensus_height, height)
            .map(|(cs, mp)| (cs.wrap_any(), mp));

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn build_channel_proofs(
        &self,
        port_id: PortId,
        channel_id: ChannelId,
        height: Height,
        reply_to: ReplyTo<Proofs>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [build_channel_proofs]");
        let result = self
            .chain
            .build_channel_proofs(&port_id, &channel_id, height);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn build_packet_proofs(
        &self,
        packet_type: PacketMsgType,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
        height: Height,
        reply_to: ReplyTo<(Vec<u8>, Proofs)>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [build_packet_proofs]");
        let result =
            self.chain
                .build_packet_proofs(packet_type, port_id, channel_id, sequence, height);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn query_packet_commitments(
        &self,
        request: QueryPacketCommitmentsRequest,
        reply_to: ReplyTo<(Vec<PacketState>, Height)>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_packet_commitments]");
        let result = self.chain.query_packet_commitments(request);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn query_unreceived_packets(
        &self,
        request: QueryUnreceivedPacketsRequest,
        reply_to: ReplyTo<Vec<u64>>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_unreceived_packets]");
        let result = self.chain.query_unreceived_packets(request);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn query_packet_acknowledgements(
        &self,
        request: QueryPacketAcknowledgementsRequest,
        reply_to: ReplyTo<(Vec<PacketState>, Height)>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [qeury_packet_acknowledgements]");
        let result = self.chain.query_packet_acknowledgements(request);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn query_unreceived_acknowledgement(
        &self,
        request: QueryUnreceivedAcksRequest,
        reply_to: ReplyTo<Vec<u64>>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [qeury_unreceived_acknowledgement]");
        let result = self.chain.query_unreceived_acknowledgements(request);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn query_next_sequence_receive(
        &self,
        request: QueryNextSequenceReceiveRequest,
        reply_to: ReplyTo<Sequence>,
    ) -> Result<(), Error> {
        tracing::info!("In Runtime: [query_next_sequence_receive]");
        let result = self.chain.query_next_sequence_receive(request);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }

    fn query_txs(
        &self,
        request: QueryTxRequest,
        reply_to: ReplyTo<Vec<IbcEvent>>,
    ) -> Result<(), Error> {
        tracing::info!("In runtime: [query_txs]");
        let result = self.chain.query_txs(request);

        reply_to.send(result).map_err(Error::send)?;

        Ok(())
    }
}
