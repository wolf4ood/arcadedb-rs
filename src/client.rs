use std::sync::Arc;

use anyhow::Result;
use serde::de::DeserializeOwned;

use crate::{
    db::Database,
    error::{ArcadeDBError, ErrorResponse},
    options::{ArcadeDBOptions, Auth},
    protocol::{DatabasesResponse, GetDatabasesRequest, Request},
    transport::{ArcadeResponse, ReqwestTransport, Transport},
};

#[derive(Clone)]
pub struct ArcadeDB(Arc<ArcadeDBInternal>);

struct ArcadeDBInternal {
    transport: ReqwestTransport,
}

impl ArcadeDB {
    pub fn builder() -> ArcadeDBBuilder {
        ArcadeDBBuilder(ArcadeDBOptions::default())
    }
    async fn connect(opts: ArcadeDBOptions) -> Result<ArcadeDB> {
        Ok(ArcadeDB(Arc::new(ArcadeDBInternal {
            transport: ReqwestTransport::new(opts),
        })))
    }

    pub async fn databases(&self) -> Result<DatabasesResponse, ArcadeDBError<ErrorResponse>> {
        self.request(GetDatabasesRequest)
            .await
            .map(|response| response.payload)
    }

    pub(crate) async fn request<T>(
        &self,
        request: T,
    ) -> Result<ArcadeResponse<T::Response>, ArcadeDBError<T::ResponseError>>
    where
        T: Request + Send + Sync,
        T::Payload: Send + Sync,
        T::ResponseError: DeserializeOwned,
    {
        self.0.transport.send(request).await
    }
    pub(crate) async fn request_no_response<T>(
        &self,
        request: T,
    ) -> Result<ArcadeResponse<()>, ArcadeDBError<T::ResponseError>>
    where
        T: Request + Send + Sync,
        T::Payload: Send + Sync,
        T::ResponseError: DeserializeOwned,
    {
        self.0.transport.send_no_response(request).await
    }
    pub fn db(&self, name: impl Into<String>) -> Database {
        Database::new(self.clone(), name.into())
    }
}

pub struct ArcadeDBBuilder(ArcadeDBOptions);

impl ArcadeDBBuilder {
    pub fn auth(mut self, auth: Auth) -> ArcadeDBBuilder {
        self.0.auth = auth;
        self
    }
    pub async fn build(mut self, url: impl Into<String>) -> Result<ArcadeDB> {
        self.0.url = url.into();

        ArcadeDB::connect(self.0).await
    }
}
