mod test_utils;

use arcadedb_rs::{ArcadeDBError, ErrorResponse};
use test_utils::{arcadedb, db_name};

#[tokio::test]
async fn should_create_list_drop_db() {
    let arcade = arcadedb().await;

    let name = db_name();
    let db = arcade.db(&name);

    let create_result = db.create().await.unwrap();
    assert_eq!("ok", create_result.result);
    let dbs = arcade.databases().await.unwrap();

    assert!(dbs.result.contains(&name));

    let drop_result = db.drop().await.unwrap();
    assert_eq!("ok", drop_result.result);
    let dbs = arcade.databases().await.unwrap();
    assert!(!dbs.result.contains(&name));
}

#[tokio::test]
async fn create_should_on_existing_db() {
    let arcade = arcadedb().await;

    let name = db_name();
    let db = arcade.db(&name);

    let create_result = db.create().await.unwrap();
    assert_eq!("ok", create_result.result);

    let create_result = db.create().await.unwrap_err();

    let error_detail = format!("Database '{}' already exists", name);
    assert!(matches!(
        create_result,
        ArcadeDBError::Error(ErrorResponse { detail, .. }) if detail == Some(error_detail)
    ));

    db.drop().await.unwrap();
}
