// Generic

struct Attribute {
    key: String,
    value: String,
}

impl Attribute {
    fn new(key: String, value: String) -> Self {}
}

enum EventType {
    Message,
    Custom(String),
}

struct Event {
    tpe: EventType,
    attributes: Vec<Attribute>,
}

impl Event {
    fn new(tpe: EventType, attributes: &[(String, String)]) -> Self {
        // ...
    }
}

type HandlerResult<T> = Result<HandlerOutput, Error>;

struct HandlerOutput<T> {
    result: T,
    events: Vec<Event>,
}

impl HandlerResult<T> {
    fn new(result: T) -> Self {
        Self {
            result,
            events: vec![],
        }
    }

    fn emit(mut self, event: impl Into<Event>) -> Self {
        self.events.push(event.into());
        self
    }
}

struct Context<C: Chain> {
    // ...
}

// Module specific

trait ClientContext<C>
where
    C: Chain,
{
    fn get_client_type(&self, client_id: ClientIdentifier) -> Option<C::ClientType>;
    fn get_client_state(&self, client_id: ClientIdentifier) -> Option<C::ClientState>;
}

impl<C> ClientContext<C> for Context<C>
where
    C: Chain,
{
    // ...
}

trait ClientKeeper<C: Chain> {
    fn store_client_type(client_type: C::ClientType) -> Result<(), Error>;
    fn store_client_state(client_state: C::ClientState) -> Result<(), Error>;
}

impl<C> ClientKeeper for Context<C>
where
    C: Chain,
{
    // ...
}

struct MsgCreateClient<C: Chain> {
    // ...
}

struct CreateClientResult<C: Chain> {
    // ...
}

enum ClientEvent {
    ClientCreated(ClientIdentifier),
}

impl From<ClientEvent> for Event {
    fn from(ce: ClientEvent) -> Event {
        match ce {
            ClientEvent::ClientCreated(client_id) => Event::new(
                EventType::Custom("ClientCreated".to_string()),
                vec![("client_id", client_id.to_string())],
            ),
        }
    }
}

fn create_client<C>(
    ctx: impl ClientContext<C>,
    msg: MsgCreateClient<C>,
) -> HandlerResult<CreateClientResult> {
    // ...

    let output = HandlerOutput::new(CreateClientResult {
        client_type,
        client_state,
    });

    output.emit(ClientEvent::ClientCreated(client_id))
}

fn create_client_keep<C>(
    ctx: impl ClientKeeper<C>,
    data: CreateClientResult<C>,
) -> Result<(), Error>
where
    C: Chain,
{
    keeper.store_client_state(data.client_state)?;
    keeper.store_client_type(data.client_type)?;

    Ok(())
}

enum ClientMsg<C> {
    CreateClient(MsgCreateClient<C>),
}

fn handler<C>(ctx: Context<C>, msg: ClientMsg<C>) -> HandlerResult<Vec<Event>>
where
    C: Chain,
{
    match msg {
        ClientMsg::CreateClient(msg) => {
            let HandlerResult { output, event } = create_client(ctx, msg)?;
            create_client_keep(ctx, result)?;
            Ok(events)
        } // TODO: What to do about potential data to return? Return as a byte array?
    }
}

