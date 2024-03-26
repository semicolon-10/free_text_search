use mongodb::bson::Document;
use mongodb::{Collection, Database};
use mongodb::options::FindOptions;

pub async fn search(db: Database, query: Document)
 -> Result<(), mongodb::error::Error> {
    let collection : Collection<Document> = db
        .collection("books");

    let options = FindOptions::builder()
        .projection(None)
        .build();

    let mut cursor = collection.find(query, options)
        .await?;

    while let Ok(result) = cursor.advance().await {
        match result {
            true => {
                match cursor.deserialize_current() {
                    Ok(doc) => println!("{:?}",doc),
                    Err(e) => eprintln!("Error deserializing {:?}",e)
                }
            }
            false => {
                println!("End of Docs");
                break
            }
        }
    }

    Ok(())
}