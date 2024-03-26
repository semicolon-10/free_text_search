use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub genre: String,
    pub summary: String
}