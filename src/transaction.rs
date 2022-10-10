use serde::de::DeserializeOwned;

use crate::{
    command::{Statement, StatementKind},
    db::Queryable,
    protocol::{BeginRequest, CommitRequest, QueryCommand, RollbackRequest},
    ArcadeDBError, Database, ErrorResponse,
};

pub struct Transaction {
    session_id: String,
    db: Database,
}

impl Transaction {
    pub(crate) async fn begin(db: Database) -> Result<Transaction, ArcadeDBError<ErrorResponse>> {
        let response = db
            .client
            .request_no_response(BeginRequest::new(db.name()))
            .await?;

        Ok(Transaction {
            session_id: response
                .metadata
                .get("arcadedb-session-id")
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Failed to fetch sesionId metadata"))?,
            db,
        })
    }

    pub fn query<'a, 'b>(&'a self, stmt: &'b str) -> Statement<'a, 'b, Transaction> {
        Statement::new(self, stmt, StatementKind::Query)
    }
    pub fn command<'a, 'b>(&'a self, stmt: &'b str) -> Statement<'a, 'b, Transaction> {
        Statement::new(self, stmt, StatementKind::Command)
    }

    pub async fn commit(self) -> Result<(), ArcadeDBError<ErrorResponse>> {
        self.db
            .client
            .request_no_response(CommitRequest::new(self.db.name(), &self.session_id))
            .await
            .map(|response| response.payload)
    }
    pub async fn rollback(self) -> Result<(), ArcadeDBError<ErrorResponse>> {
        self.db
            .client
            .request_no_response(RollbackRequest::new(self.db.name(), &self.session_id))
            .await
            .map(|response| response.payload)
    }
}

#[async_trait::async_trait]
impl Queryable for Transaction {
    async fn send<'a, 'b, T: DeserializeOwned + Send + Sync, Q: Queryable + Send + Sync>(
        &self,
        cmd: Statement<'a, 'b, Q>,
    ) -> Result<Vec<T>, ArcadeDBError<ErrorResponse>> {
        self.db
            .client
            .request(QueryCommand::with_session_id(cmd, &self.session_id))
            .await
            .map(|response| response.payload.result)
    }

    fn name(&self) -> &str {
        self.db.name()
    }
}
