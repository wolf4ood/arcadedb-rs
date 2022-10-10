mod test_utils;

use serde::Deserialize;
use serde_json::Value;
use test_utils::{existing_db, new_db};
use uuid::Uuid;

use arcadedb_rs::{ArcadeDBError, ErrorResponse, Language, RecordID};

#[tokio::test]
async fn should_run_simple_query() {
    let db = existing_db("movies").await;

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Record {
        uuid: Uuid,
    }
    let results = db
        .query("select uuid() as uuid")
        .send::<Record>()
        .await
        .unwrap();

    assert_eq!(1, results.len());
}

#[tokio::test]
async fn should_fail_to_run_non_idempotent_query() {
    let db = existing_db("movies").await;

    let error = db
        .query("create vertex type Person")
        .send::<()>()
        .await
        .unwrap_err();

    let match_error = "Query 'create vertex type Person' is not idempotent".to_string();
    assert!(
        matches!(
            &error,
            ArcadeDBError::Error(ErrorResponse {
                detail: Some(err),
                ..
            }) if err == &match_error
        ),
        "{:?}",
        error
    );
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Movie {
    #[serde(rename = "@rid")]
    id: RecordID,
    title: String,
    tagline: String,
    released: i32,
}

#[tokio::test]
async fn should_query_with_parameters() {
    let db = existing_db("movies").await;

    let results = db
        .query("select * from Movie where title = :title limit 1")
        .param("title", "The Matrix")
        .send::<Movie>()
        .await
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!("The Matrix", results[0].title);
    assert_eq!("Welcome to the Real World", results[0].tagline);
    assert_eq!(1999, results[0].released);
}

#[tokio::test]
async fn should_cypher_query_with_parameters() {
    let db = existing_db("movies").await;

    let results = db
        .query("match (m:Movie) where m.title = $title return m limit 1")
        .language(Language::Cypher)
        .param("title", "The Matrix")
        .send::<Movie>()
        .await
        .unwrap();

    assert_eq!(1, results.len());

    assert_eq!("The Matrix", results[0].title);
    assert_eq!("Welcome to the Real World", results[0].tagline);
    assert_eq!(1999, results[0].released);
}

#[tokio::test]
async fn should_exec_a_command_with_parameters() {
    let db = new_db("should_exec_a_command_with_parameters").await;
    use serde_json::Value;

    let results = db
        .command("create vertex type Person")
        .send::<Value>()
        .await
        .unwrap();

    assert_eq!(1, results.len());

    let results = db
        .command("insert into Person set name = :name")
        .param("name", "John")
        .send::<Value>()
        .await
        .unwrap();

    assert_eq!(1, results.len());
}

#[tokio::test]
async fn should_execute_an_isolated_transaction() {
    let db = new_db("should_execute_a_transaction").await;

    db.command("create vertex type Person")
        .send::<Value>()
        .await
        .unwrap();

    let tx = db.tx().await.unwrap();

    tx.command("insert into Person set name = 'John'")
        .send::<Value>()
        .await
        .unwrap();

    let results = tx
        .query("select * from Person")
        .send::<Value>()
        .await
        .unwrap();

    assert_eq!(1, results.len());

    let results = db
        .command("select * from Person")
        .send::<Value>()
        .await
        .unwrap();

    assert_eq!(0, results.len());

    tx.commit().await.unwrap();

    let results = db
        .query("select * from Person")
        .send::<Value>()
        .await
        .unwrap();

    assert_eq!(1, results.len());
}

#[tokio::test]
async fn should_rollback_a_transaction() {
    let db = new_db("should_rollback_a_transaction").await;

    db.command("create vertex type Person")
        .send::<Value>()
        .await
        .unwrap();

    let tx = db.tx().await.unwrap();

    tx.command("insert into Person set name = 'John'")
        .send::<Value>()
        .await
        .unwrap();

    tx.rollback().await.unwrap();

    let results = db
        .command("select * from Person")
        .send::<Value>()
        .await
        .unwrap();

    assert_eq!(0, results.len());
}
