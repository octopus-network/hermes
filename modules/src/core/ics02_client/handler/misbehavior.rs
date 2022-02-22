//! Protocol logic specific to processing ICS2 messages of type `MsgCreateAnyClient`.

use crate::events::IbcEvent;
use crate::handler::{HandlerOutput, HandlerResult};
use crate::core::ics02_client::client_consensus::AnyConsensusState;
use crate::core::ics02_client::client_def::{AnyClient, ClientDef};
use crate::core::ics02_client::client_state::AnyClientState;
use crate::core::ics02_client::client_type::ClientType;
use crate::core::ics02_client::context::ClientReader;
use crate::core::ics02_client::error::Error;
use crate::core::ics02_client::events::Attributes;
use crate::core::ics02_client::handler::ClientResult;
use crate::core::ics02_client::msgs::misbehavior::MsgSubmitAnyMisbehaviour;
use crate::clients::ics10_grandpa::client_state::ClientState;
use crate::core::ics24_host::identifier::ClientId;

/// The result following the successful processing of a `MsgCreateAnyClient` message. Preferably
/// this data type should be used with a qualified name `create_client::Result` to avoid ambiguity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Result {
    pub client_id: ClientId,
    pub client_state: AnyClientState,
    pub consensus_state: AnyConsensusState,
}

pub fn process(
    ctx: &dyn ClientReader,
    msg: MsgSubmitAnyMisbehaviour,
) -> HandlerResult<ClientResult, Error> {
    let mut output = HandlerOutput::builder();
    let MsgSubmitAnyMisbehaviour {
        client_id,
        misbehaviour,
        signer: _,
    } = msg;

    // Read client type from the host chain store. The client should already exist.
    let client_type = ctx.client_type(&client_id)?;

    let client_def = AnyClient::from_client_type(client_type);

    // Read client state from the host chain store.
    let client_state = ctx.client_state(&client_id)?;

    tracing::info!(
        "In misbehaviour : [process] >> client_state: {:?}",
        client_state
    );

    let latest_height = client_state.latest_height();
    let consensus_state = ctx.consensus_state(&client_id, latest_height)?;

    // // Use client_state to validate the new header against the latest consensus_state.
    // // This function will return the new client_state (its latest_height changed) and a
    // // consensus_state obtained from header. These will be later persisted by the keeper.
    // let (new_client_state, new_consensus_state) = client_def
    //     .check_header_and_update_state(client_state, header.clone())
    //     .map_err(|e| Error::header_verification_failure(e.to_string()))?;

    let result = ClientResult::Misbehaviour(Result {
        client_id: client_id.clone(),
        client_state,
        consensus_state,
    });
    tracing::info!("in ics02_client: [misbehaviour] >> result : {:?}", result);

    let event_attributes = Attributes {
        // height: header.clone().height(),
        client_id,
        client_type: ClientType::Grandpa,
        ..Default::default()
    };

    output.emit(IbcEvent::ClientMisbehaviour(event_attributes.into()));

    Ok(output.with_result(result))
}
