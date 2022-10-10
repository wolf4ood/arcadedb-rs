#![allow(dead_code)]

use std::{ops::Deref, thread::spawn};

use arcadedb_rs::{ArcadeDB, Database};
use uuid::Uuid;

pub fn db_name() -> String {
    Uuid::new_v4().to_string()
}

pub async fn arcadedb() -> ArcadeDB {
    ArcadeDB::builder()
        .auth(arcadedb_rs::Auth::basic("root", "playwithdata"))
        .build("http://localhost:2480")
        .await
        .unwrap()
}

pub async fn new_db(name: &str) -> TestDB {
    new_db_internal(name, true).await
}
pub async fn new_db_internal(name: &str, create: bool) -> TestDB {
    let arcade = arcadedb().await;

    let db = arcade.db(name);

    if create {
        let databases = arcade.databases().await.unwrap();

        if databases.result.contains(&name.to_string()) {
            let _ = db.drop().await;
        }
        db.create().await.unwrap();
    }
    TestDB {
        db,
        created: create,
    }
}

pub async fn existing_db(name: &str) -> TestDB {
    new_db_internal(name, false).await
}

pub struct TestDB {
    db: Database,
    created: bool,
}

impl Deref for TestDB {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

impl Drop for TestDB {
    fn drop(&mut self) {
        let cloned = self.db.clone();

        if self.created {
            spawn(move || {
                let rt = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                rt.block_on(cloned.drop())
            });
        }
    }
}
