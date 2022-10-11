use std::{collections::HashMap, fmt::Display};

use crate::{
    options::ArcadeDBOptions,
    protocol::{Method, Request},
    ArcadeDBError, Auth,
};
use anyhow::Result;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;

use super::{ArcadeResponse, Transport};

pub struct ReqwestTransport {
    opts: ArcadeDBOptions,
    client: Client,
}

#[async_trait::async_trait]
impl Transport for ReqwestTransport {
    async fn send<T>(
        &self,
        request: T,
    ) -> Result<ArcadeResponse<T::Response>, ArcadeDBError<T::ResponseError>>
    where
        T: Request + Send + Sync,
        T::Payload: Send + Sync,
        T::ResponseError: DeserializeOwned + Display,
    {
        self.prepare(request)
            .send_with_response::<T::Response, T::ResponseError>()
            .await?
            .into_result()
            .map_err(ArcadeDBError::Error)
    }

    async fn send_no_response<T>(
        &self,
        request: T,
    ) -> Result<ArcadeResponse<()>, ArcadeDBError<T::ResponseError>>
    where
        T: Request + Send + Sync,
        T::Payload: Send + Sync,
        T::ResponseError: DeserializeOwned + Display,
    {
        self.prepare(request)
            .send_without_response::<T::ResponseError>()
            .await
    }

    fn new(opts: ArcadeDBOptions) -> Self {
        ReqwestTransport {
            opts,
            client: Client::new(),
        }
    }
}

impl ReqwestTransport {
    fn prepare<T>(&self, request: T) -> RequestBuilder
    where
        T: Request + Send + Sync,
        T::Payload: Send + Sync,
        T::ResponseError: DeserializeOwned,
    {
        let url = format!("{}{}", self.opts.url, request.path());
        let builder = match request.method() {
            Method::Get => self.client.get(url),
            Method::Post => self.client.post(url),
        };
        builder
            .authenticated(&self.opts.auth)
            .with_custom_headers(request.metadata())
            .json(request.payload())
    }
}

#[async_trait::async_trait]
trait BuilderExt {
    fn authenticated(self, auth: &Auth) -> Self;
    async fn send_with_response<OK: DeserializeOwned, ERR: DeserializeOwned + Display>(
        self,
    ) -> Result<Either<ArcadeResponse<OK>, ERR>>;

    fn with_custom_headers(self, metadata: HashMap<String, String>) -> Self;
    async fn send_without_response<ERR: DeserializeOwned + Display>(
        self,
    ) -> Result<ArcadeResponse<()>, ArcadeDBError<ERR>>;
}

#[async_trait::async_trait]
impl BuilderExt for RequestBuilder {
    fn authenticated(self, auth: &Auth) -> Self {
        match auth {
            Auth::NoAuth => self,
            Auth::Basic(credentials) => {
                self.basic_auth(&credentials.username, Some(&credentials.password))
            }
        }
    }

    fn with_custom_headers(self, metadata: HashMap<String, String>) -> Self {
        let mut this = self;
        for (k, v) in metadata.iter() {
            this = this.header(k, v);
        }
        this
    }
    async fn send_with_response<OK: DeserializeOwned, ERR: DeserializeOwned + Display>(
        self,
    ) -> Result<Either<ArcadeResponse<OK>, ERR>> {
        let response = self.send().await?;

        let headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_str().unwrap().to_string()))
            .collect();

        if response.status().is_success() {
            Ok(Either::Left(ArcadeResponse::new(
                response.json().await?,
                headers,
            )))
        } else {
            Ok(Either::Right(response.json().await?))
        }
    }
    async fn send_without_response<ERR: DeserializeOwned + Display>(
        self,
    ) -> Result<ArcadeResponse<()>, ArcadeDBError<ERR>> {
        let response = self.send().await.map_err(anyhow::Error::from)?;

        let headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_str().unwrap().to_string()))
            .collect();

        if response.status().is_success() {
            Ok(ArcadeResponse::new((), headers))
        } else {
            Err(ArcadeDBError::Error(
                response.json().await.map_err(anyhow::Error::from)?,
            ))
        }
    }
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    fn into_result(self) -> Result<L, R> {
        match self {
            Either::Left(l) => Ok(l),
            Either::Right(r) => Err(r),
        }
    }
}
