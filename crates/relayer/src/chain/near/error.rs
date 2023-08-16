use thiserror::Error;

#[derive(Error, Debug)]
pub enum NearError {
    #[error("Invalid account ID")]
    InvalidAccountId,
    #[error("dummy error")]
    DummyError,
    #[error("serde json failure")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("get signer failure")]
    GetSignerFailure(#[from] crate::error::Error),
    #[error("Parser InMemorySigner failure")]
    ParserInMemorySignerFailure(#[from] std::io::Error),
    #[error("Parser Near Account Id failure")]
    ParserNearAccountIdFailure(#[from] near_account_id::ParseAccountError),
    #[error("Custom error: {0}")]
    CustomError(String),
}
