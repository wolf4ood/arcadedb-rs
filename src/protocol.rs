use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, marker::PhantomData};

use crate::{
    command::{Statement, StatementKind},
    db::Queryable,
    error::ErrorResponse,
};

const SESSION_HEADER: &str = "arcadedb-session-id";

type EmptyResponse = serde_json::Value;

pub trait Request {
    type Payload: Serialize;
    type Response: DeserializeOwned;
    type ResponseError: DeserializeOwned + Display;

    fn path(&self) -> String;

    fn method(&self) -> Method;

    fn payload(&self) -> Option<&Self::Payload>;

    fn metadata(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

pub enum Method {
    Get,
    Post,
}

pub struct ServerCommandRequest<'a, T> {
    phantom: PhantomData<T>,
    command: Command<'a>,
}

#[derive(Serialize)]
pub struct Command<'a> {
    command: ServerCommand<'a>,
}

impl<'a, T> ServerCommandRequest<'a, T> {
    pub fn new(command: ServerCommand<'a>) -> ServerCommandRequest<'a, GenericResponse> {
        ServerCommandRequest {
            phantom: PhantomData,
            command: Command { command },
        }
    }
}

pub enum ServerCommand<'a> {
    CreateDatabase(&'a str),
    DropDatabase(&'a str),
}

impl<'a> ServerCommand<'a> {
    pub fn create_db(db: &'a str) -> Self {
        Self::CreateDatabase(db)
    }
    pub fn drop_db(db: &'a str) -> Self {
        Self::DropDatabase(db)
    }
}

impl<'a> Serialize for ServerCommand<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ServerCommand::CreateDatabase(db) => {
                serializer.serialize_str(&format!("CREATE DATABASE {}", db))
            }
            ServerCommand::DropDatabase(db) => {
                serializer.serialize_str(&format!("DROP DATABASE {}", db))
            }
        }
    }
}

impl<'a, T: DeserializeOwned> Request for ServerCommandRequest<'a, T> {
    type Payload = Command<'a>;

    type Response = T;

    type ResponseError = ErrorResponse;

    fn path(&self) -> String {
        "/api/v1/server".to_owned()
    }

    fn method(&self) -> Method {
        Method::Post
    }

    fn payload(&self) -> Option<&Self::Payload> {
        Some(&self.command)
    }
}

#[derive(Deserialize, Debug)]
pub struct GenericResponse {
    pub result: String,
}

pub struct GetDatabasesRequest;

impl Request for GetDatabasesRequest {
    type Payload = ();

    type Response = DatabasesResponse;

    type ResponseError = ErrorResponse;

    fn path(&self) -> String {
        String::from("/api/v1/databases")
    }

    fn method(&self) -> Method {
        Method::Get
    }

    fn payload(&self) -> Option<&Self::Payload> {
        None
    }
}

#[derive(Deserialize, Debug)]
pub struct DatabasesResponse {
    pub result: Vec<String>,
    pub user: String,
    pub version: String,
}

pub struct QueryCommand<'a, 'b, T: DeserializeOwned, Q: Queryable> {
    payload: Statement<'a, 'b, Q>,
    session_id: Option<&'a str>,
    response: PhantomData<T>,
}

#[derive(Deserialize)]
pub struct ResultWrapper<T> {
    pub result: Vec<T>,
}

impl<'a, 'b, T: DeserializeOwned, Q: Queryable> QueryCommand<'a, 'b, T, Q> {
    pub fn new(cmd: Statement<'a, 'b, Q>) -> Self {
        QueryCommand {
            payload: cmd,
            response: PhantomData,
            session_id: None,
        }
    }
    pub fn with_session_id(cmd: Statement<'a, 'b, Q>, session_id: &'a str) -> Self {
        QueryCommand {
            payload: cmd,
            response: PhantomData,
            session_id: Some(session_id),
        }
    }
}

impl<'a, 'b, T: DeserializeOwned, Q: Queryable> Request for QueryCommand<'a, 'b, T, Q> {
    type Payload = Statement<'a, 'b, Q>;

    type Response = ResultWrapper<T>;

    type ResponseError = ErrorResponse;

    fn path(&self) -> String {
        match self.payload.kind {
            StatementKind::Query => {
                format!("/api/v1/query/{}", self.payload.queryable.name())
            }
            StatementKind::Command => {
                format!("/api/v1/command/{}", self.payload.queryable.name())
            }
        }
    }

    fn method(&self) -> Method {
        Method::Post
    }

    fn payload(&self) -> Option<&Self::Payload> {
        Some(&self.payload)
    }
    fn metadata(&self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        if let Some(session_id) = self.session_id {
            metadata.insert(SESSION_HEADER.to_string(), session_id.to_string());
        }
        metadata
    }
}

pub struct BeginRequest<'a> {
    name: &'a str,
}

impl<'a> BeginRequest<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

impl<'a> Request for BeginRequest<'a> {
    type Payload = EmptyResponse;

    type Response = ();

    type ResponseError = ErrorResponse;

    fn path(&self) -> String {
        format!("/api/v1/begin/{}", self.name)
    }

    fn method(&self) -> Method {
        Method::Post
    }

    fn payload(&self) -> Option<&Self::Payload> {
        None
    }
}

pub struct CommitRequest<'a> {
    name: &'a str,
    session_id: &'a str,
}

impl<'a> CommitRequest<'a> {
    pub fn new(name: &'a str, session_id: &'a str) -> Self {
        Self { name, session_id }
    }
}

impl<'a> Request for CommitRequest<'a> {
    type Payload = ();

    type Response = ();

    type ResponseError = ErrorResponse;

    fn path(&self) -> String {
        format!("/api/v1/commit/{}", self.name)
    }

    fn method(&self) -> Method {
        Method::Post
    }

    fn payload(&self) -> Option<&Self::Payload> {
        None
    }

    fn metadata(&self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert(SESSION_HEADER.to_string(), self.session_id.to_string());
        metadata
    }
}

pub struct RollbackRequest<'a> {
    name: &'a str,
    session_id: &'a str,
}

impl<'a> RollbackRequest<'a> {
    pub fn new(name: &'a str, session_id: &'a str) -> Self {
        Self { name, session_id }
    }
}

impl<'a> Request for RollbackRequest<'a> {
    type Payload = ();

    type Response = ();

    type ResponseError = ErrorResponse;

    fn path(&self) -> String {
        format!("/api/v1/rollback/{}", self.name)
    }

    fn method(&self) -> Method {
        Method::Post
    }

    fn payload(&self) -> Option<&Self::Payload> {
        None
    }

    fn metadata(&self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert(SESSION_HEADER.to_string(), self.session_id.to_string());
        metadata
    }
}
