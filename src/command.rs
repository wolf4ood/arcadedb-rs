use std::collections::HashMap;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::{db::Queryable, ArcadeDBError, ErrorResponse};

#[derive(Serialize)]
pub struct Statement<'a, 'b, T: Queryable> {
    #[serde(skip_serializing)]
    pub(crate) queryable: &'a T,
    #[serde(skip_serializing)]
    pub(crate) kind: StatementKind,
    command: &'b str,
    language: Language,
    params: HashMap<&'b str, Value>,
}

pub enum StatementKind {
    Query,
    Command,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Language {
    SQL,
    Cypher,
}

impl<'a, 'b, Q: Queryable + Send + Sync> Statement<'a, 'b, Q> {
    pub(crate) fn new(queryable: &'a Q, command: &'b str, kind: StatementKind) -> Self {
        Statement {
            queryable,
            command,
            params: HashMap::new(),
            language: Language::SQL,
            kind,
        }
    }

    pub fn param(mut self, name: &'b str, value: impl Into<Value>) -> Self {
        self.params.insert(name, value.into());
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }

    pub fn params(mut self, params: &[(&'b str, &dyn IntoValue)]) -> Self {
        let new_params: HashMap<&str, Value> =
            params.iter().map(|&(k, v)| (k, v.to_value())).collect();
        self.params.extend(new_params);
        self
    }

    pub async fn send<T: DeserializeOwned + Send + Sync>(
        self,
    ) -> Result<Vec<T>, ArcadeDBError<ErrorResponse>> {
        self.queryable.send(self).await
    }
}

pub trait IntoValue {
    fn to_value(&self) -> Value;
}

impl<T: Into<Value> + Clone> IntoValue for T {
    fn to_value(&self) -> Value {
        self.clone().into()
    }
}
