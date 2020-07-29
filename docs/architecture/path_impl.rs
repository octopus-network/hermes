pub enum ICSPath {
    ClientType(path::ClientType),
    ClientState(path::ClientState),
    ConsensusState(path::ConsensusState),
    ClientConnections(path::ClientConnections),
    Connections(path::Connections),
    Ports(path::Ports),
    ChannelEnds(path::ChannelEnds),
    SeqSends(path::SeqSends),
    SeqRecvs(path::SeqRecvs),
    SeqAcks(path::SeqAcks),
    Commitments(path::Commitments),
    Acks(path::Acks),
}

impl ICSPath {
    fn is_provable(&self) -> bool {
        match self {
            Self::ClientType(path) => path.is_provable(),
            Self::ClientState(path) => path.is_provable(),
            Self::ConsensusState(path) => path.is_provable(),
            Self::ClientConnections(path) => path.is_provable(),
            Self::Connections(path) => path.is_provable(),
            Self::Ports(path) => path.is_provable(),
            Self::ChannelEnds(path) => path.is_provable(),
            Self::SeqSends(path) => path.is_provable(),
            Self::SeqRecvs(path) => path.is_provable(),
            Self::SeqAcks(path) => path.is_provable(),
            Self::Commitments(path) => path.is_provable(),
            Self::Acks(path) => path.is_provable(),
        }
    }

    fn into_bytes(&self) -> Vec<u8> {
        match self {
            Self::ClientType(path) => path.into_bytes(),
            Self::ClientState(path) => path.into_bytes(),
            Self::ConsensusState(path) => path.into_bytes(),
            Self::ClientConnections(path) => path.into_bytes(),
            Self::Connections(path) => path.into_bytes(),
            Self::Ports(path) => path.into_bytes(),
            Self::ChannelEnds(path) => path.into_bytes(),
            Self::SeqSends(path) => path.into_bytes(),
            Self::SeqRecvs(path) => path.into_bytes(),
            Self::SeqAcks(path) => path.into_bytes(),
            Self::Commitments(path) => path.into_bytes(),
            Self::Acks(path) => path.into_bytes(),
        }
    }
}

trait Path: Display {
    type Value;

    fn is_provable(&self) -> bool {
        true
    }

    fn into_bytes(self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

pub mod path {
    use derive_more::Display;

    #[derive(Display)]
    #[display(fmt = "clients/{}/clientType", _0)]
    pub struct ClientType(ClientId);

    impl Path for ClientType {
        type Value = ics02::ClientType;
    }

    #[derive(Display)]
    #[display(fmt = "clients/{}/clientState", _0)]
    pub struct ClientState(ClientId);

    impl Path for ClientState {
        type Value = ics02::ClientState;

        fn is_provable(&self) -> bool {
            false
        }
    }

    pub struct ConsensusState(ClientId, u64);
    pub struct ClientConnections(ClientId);
    pub struct Connections(ConnectionId);
    pub struct Ports(PortId);
    pub struct ChannelEnds(PortId, ChannelId);
    pub struct SeqSends(PortId, ChannelId);
    pub struct SeqRecvs(PortId, ChannelId);
    pub struct SeqAcks(PortId, ChannelId);
    pub struct Commitments(PortId, ChannelId, u64);
    pub struct Acks(PortId, ChannelId, u64);
}

