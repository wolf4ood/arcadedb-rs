use arcadedb_rs::{ArcadeDB, Auth, RecordID};
use serde::Deserialize;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arcadedb = ArcadeDB::builder()
        .auth(Auth::basic("root", "playwithdata"))
        .build("http://localhost:2480")
        .await?;

    let db = arcadedb.db("command_example");

    if !db.exists().await? {
        db.create().await?;
    }

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Person {
        #[serde(rename = "@rid")]
        id: RecordID,
        name: String,
        surname: String,
    }

    db.command("create vertex type Person if not exists")
        .send::<Value>()
        .await?;

    db.command("insert into Person set name = :name, surname = :surname")
        .param("name", "Paul")
        .param("surname", "Rust")
        .send::<Person>()
        .await?;

    let results = db.query("select * from Person").send::<Person>().await?;

    println!("Got {:#?}", results);

    db.drop().await?;
    Ok(())
}
