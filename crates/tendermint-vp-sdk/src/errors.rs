use ic_agent::AgentError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("agent error")]
    AgentError(#[from] AgentError),
}
