use std::collections::HashMap;

use serde::de::DeserializeOwned;

use crate::{options::ArcadeDBOptions, protocol::Request, ArcadeDBError};

mod reqwest_impl;

pub use reqwest_impl::ReqwestTransport;

#[async_trait::async_trait]
pub trait Transport {
    fn new(opts: ArcadeDBOptions) -> Self;
    async fn send<T>(
        &self,
        request: T,
    ) -> Result<ArcadeResponse<T::Response>, ArcadeDBError<T::ResponseError>>
    where
        T: Request + Send + Sync,
        T::Payload: Send + Sync,
        T::ResponseError: DeserializeOwned;

    async fn send_no_response<T>(
        &self,
        request: T,
    ) -> Result<ArcadeResponse<()>, ArcadeDBError<T::ResponseError>>
    where
        T: Request + Send + Sync,
        T::Payload: Send + Sync,
        T::ResponseError: DeserializeOwned;
}

pub struct ArcadeResponse<T> {
    pub(crate) payload: T,
    pub(crate) metadata: HashMap<String, String>,
}

impl<T> ArcadeResponse<T> {
    pub fn new(payload: T, headers: HashMap<String, String>) -> Self {
        Self {
            payload,
            metadata: headers,
        }
    }
}
