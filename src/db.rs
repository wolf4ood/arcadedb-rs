use std::sync::Arc;

use serde::de::DeserializeOwned;

use crate::{
    command::{Statement, StatementKind},
    error::{ArcadeDBError, ErrorResponse},
    protocol::{GenericResponse, QueryCommand, ServerCommand, ServerCommandRequest},
    transaction::Transaction,
    ArcadeDB,
};

#[derive(Clone)]
pub struct Database {
    pub(crate) client: ArcadeDB,
    name: Arc<String>,
}

#[async_trait::async_trait]
pub trait Queryable {
    async fn send<'a, 'b, T: DeserializeOwned + Send + Sync, Q: Queryable + Send + Sync>(
        &self,
        cmd: Statement<'a, 'b, Q>,
    ) -> Result<Vec<T>, ArcadeDBError<ErrorResponse>>;

    fn name(&self) -> &str;

    fn metadata(&self) -> &[(&str, &str)] {
        &[]
    }
}

#[async_trait::async_trait]
impl Queryable for Database {
    async fn send<'a, 'b, T: DeserializeOwned + Send + Sync, Q: Queryable + Send + Sync>(
        &self,
        cmd: Statement<'a, 'b, Q>,
    ) -> Result<Vec<T>, ArcadeDBError<ErrorResponse>> {
        self.client
            .request(QueryCommand::new(cmd))
            .await
            .map(|response| response.payload.result)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Database {
    pub(crate) fn new(client: ArcadeDB, name: String) -> Self {
        Self {
            client,
            name: Arc::new(name),
        }
    }

    pub async fn exists(&self) -> Result<bool, ArcadeDBError<ErrorResponse>> {
        self.client
            .databases()
            .await
            .map(|response| response.result.contains(&self.name))
    }

    pub async fn drop(&self) -> Result<GenericResponse, ArcadeDBError<ErrorResponse>> {
        self.client
            .request(ServerCommandRequest::<GenericResponse>::new(
                ServerCommand::drop_db(&self.name),
            ))
            .await
            .map(|response| response.payload)
    }
    pub async fn create(&self) -> Result<GenericResponse, ArcadeDBError<ErrorResponse>> {
        self.client
            .request(ServerCommandRequest::<GenericResponse>::new(
                ServerCommand::create_db(&self.name),
            ))
            .await
            .map(|response| response.payload)
    }

    pub async fn tx(&self) -> Result<Transaction, ArcadeDBError<ErrorResponse>> {
        Transaction::begin(self.clone()).await
    }

    pub fn query<'a, 'b>(&'a self, stmt: &'b str) -> Statement<'a, 'b, Database> {
        Statement::new(self, stmt, StatementKind::Query)
    }
    pub fn command<'a, 'b>(&'a self, stmt: &'b str) -> Statement<'a, 'b, Database> {
        Statement::new(self, stmt, StatementKind::Command)
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
