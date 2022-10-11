use arcadedb_rs::{ArcadeDB, Auth, RecordID};
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arcadedb = ArcadeDB::builder()
        .auth(Auth::basic("root", "playwithdata"))
        .build("http://localhost:2480")
        .await?;

    let db = arcadedb.db("movies");

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Movie {
        #[serde(rename = "@rid")]
        id: RecordID,
        title: String,
        tagline: String,
        released: i32,
    }

    let results = db
        .query("select * from Movie where title = :title")
        .param("title", "The Matrix")
        .send::<Movie>()
        .await?;

    println!("Got {:#?}", results);
    Ok(())
}
