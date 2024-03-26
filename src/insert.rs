use std::fs;
use mongodb::bson::{Bson, doc, Document, to_bson};
use mongodb::{Collection, IndexModel};
use mongodb::results::CreateIndexResult;
use crate::model::Book;

pub async fn insert(collection: Collection<Document>) ->
    Result<(), mongodb::error::Error> {
    match fs::read_to_string("./data.json") {
        Ok(data) => {
            let dataset : Vec<Book> = serde_json::from_str(&data)
                .expect("Unable to parse");

            for doc in dataset {
                let bson_obj = to_bson(&doc)?;

                if let Bson::Document(document) = bson_obj {
                    collection.insert_one(document,None).await?;
                }
            }
        }
        Err(e) => eprintln!("{:?}",e)
    }
    add_index(collection).await?;
    Ok(())
}

async fn add_index(collection: Collection<Document>) -> mongodb::error::Result<CreateIndexResult> {
    let index_model = IndexModel::builder()
        .keys(doc! {"summary": "text"})
        .build();

    collection.create_index(index_model, None).await
}