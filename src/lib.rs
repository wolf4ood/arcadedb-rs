//! Experimental Rust client for ArcadeDB.
//!
//!
//! You can use arcadedb-rs this lines in your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! arcadedb-rs = "*"
//! ```
//!
//! Query Example:
//!
//! ```rust,no_run
//!
//! use arcadedb_rs::{ArcadeDB, Auth, RecordID};
//! use serde::Deserialize;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let arcadedb = ArcadeDB::builder()
//!         .auth(Auth::basic("root", "playwithdata"))
//!         .build("http://localhost:2480")
//!         .await?;
//!
//!     let db = arcadedb.db("movies");
//!
//!     #[derive(Deserialize, Debug)]
//!     #[allow(dead_code)]
//!     struct Movie {
//!         #[serde(rename = "@rid")]
//!         id: RecordID,
//!         title: String,
//!         tagline: String,
//!         released: i32,
//!     }
//!
//!     let results = db
//!         .query("select * from Movie where title = :title")
//!         .param("title", "The Matrix")
//!         .send::<Movie>()
//!         .await?;
//!
//!     println!("Got {:#?}", results);
//!     Ok(())
//! }
//! ```
//!
//!
//!

mod client;
mod command;
mod db;
mod document;
mod error;
mod options;
mod protocol;
mod transaction;
mod transport;
mod types;

pub use client::ArcadeDB;
pub use command::Language;
pub use db::Database;
pub use error::{ArcadeDBError, ErrorResponse};
pub use options::Auth;
pub use types::rid::RecordID;
