use std::fmt::Display;

use serde::{de::DeserializeOwned, Deserialize};

#[derive(Debug)]
pub enum ArcadeDBError<T: DeserializeOwned> {
    Error(T),
    Generic(anyhow::Error),
}

impl<T: DeserializeOwned> From<anyhow::Error> for ArcadeDBError<T> {
    fn from(err: anyhow::Error) -> Self {
        ArcadeDBError::Generic(err)
    }
}

impl From<ErrorResponse> for ArcadeDBError<ErrorResponse> {
    fn from(err: ErrorResponse) -> Self {
        ArcadeDBError::Error(err)
    }
}

#[derive(Deserialize, Debug, thiserror::Error)]
pub struct ErrorResponse {
    pub error: String,
    pub detail: Option<String>,
    pub exception: Option<String>,
}
impl Display for ErrorResponse {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("ErrorResponse")
            .field("session_id", &self.error)
            .finish()
    }
}
