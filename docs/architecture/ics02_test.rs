pub struct MsgCreateClient<C: Chain> {
    client_id: ClientIdentifier,
    client_type: C::ClientType,
    consensus_state: C::ConsensusState,
}

trait ClientHandler<C>
where
    C: Chain,
{
    fn get_client_type(&self, client_identifier: ClientIdentifier) -> Option<C::ClientType>;
}

pub struct MockClientHandler<C: Chain> {
    client_state: Option<C::ClientState>,
    client_type: Option<C::ClientType>,
}

impl<C> ClientHandler<C> for MockClientHandler<C>
where
    C: Chain,
{
    fn get_client_state(&self, client_identifier: ClientIdentifier) -> Option<C::ClientState> {
        self.client_state.clone()
    }

    fn get_client_type(&self, client_identifier: ClientIdentifier) -> Option<C::ClientType> {
        self.client_type.clone()
    }
}

struct CreateClientResult<C> {
    client_type: C::ClientType,
    client_state: C::ClientState,
}

fn create_client<C>(
    msg: MsgCreateClient<C>,
    handler: impl ClientHandler<C>,
) -> Result<CreateClientResult, Error>
where
    C: Chain,
{
    if handler.get_client_state(msg.client_id).is_some() {
        bail!(ErrorKind::ClientStateAlreadyExists);
    }

    if handler.get_client_type(msg.client_id).is_some() {
        bail!(ErrorKind::ClientTypeAlreadyExists);
    }

    let client_state = msg.client_type.initialize(msg.consensus_state)?;

    Ok(CreateClientResult {
        client_type,
        client_state,
    })
}

fn test_create_client() {
    let mock = MockClientHandler {
        client_type: Some(todo!()),
        client_state: Some(todo!()),
    };

    let msg = MsgCreateClient {
        client_id: todo!(),
        client_type: todo!(),
        consensus_state: todo!(),
    };

    let result = create_client(msg, mock);

    match result {
        Ok(res) => {
            assert_eq!(res.client_type, todo!());
            assert_eq!(res.client_state, todo!());
        }
        Err(err) => {
            panic!("unexpected error: {}", err);
        }
    }
}

pub struct MsgUpdateClient<C: Chain> {
    client_id: ClientIdentifier,
    header: C::Header,
}

pub struct UpdateClientResult<C: Chain> {
    client_state: C::ClientState,
    consensus_state: C::ConsensusState,
}

fn update_client<C>(msg: MsgUpdateClient<C>, handler: impl ClientHandler<C>) -> Result<(), Error>
where
    C: Chain,
{
    let client_type = handler
        .get_client_type(msg.client_id)
        .ok_or_else(|| ErrorKind::ClientTypeNotFound(msg.client_id))?;

    let client_state = handler
        .get_client_state(msg.client_id)
        .ok_or_else(|| ErrorKind::ClientStateNotFound(msg.client_id))?;

    let consensus_state = handler
        .query_consensus_state(msg.client_id, client_state.latest_height())
        .ok_or_else(|| {
            ErrorKind::ConsensusStateNotFound(msg.client_id, client_state.latest_height())
        })?;

    client_type.check_validity_and_update_state(
        &mut client_state,
        &mut consensus_state,
        &msg.header,
    )?;

    Ok(UpdateClientResult {
        client_state,
        consensus_state,
    })
}

