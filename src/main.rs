use mongodb::bson::{doc, Document};
use mongodb::{Client, Collection};
use mongodb::error::Error;
use mongodb::options::ClientOptions;
use crate::insert::insert;
use crate::search::search;

mod insert;
mod search;
mod model;

#[tokio::main]
async fn main() -> Result<(), mongodb::error::Error> {
    // Free Text Search (Mongo-DB)

    let client_options = ClientOptions
    ::parse("mongodb://localhost:27017")
        .await?;

    let client = Client::with_options(client_options)?;
    let db = client.database("test");
    db.drop(None).await?;
    let collection : Collection<Document> = db
        .collection("books");

    insert(collection).await?;

    println!("INSERTED DATA SUCCESSFULLY...!");

    let query = doc! {
        "$text": {
            "$search" : "romance novel featuring Elizabeth"
        }
    };

    search(db, query).await
}