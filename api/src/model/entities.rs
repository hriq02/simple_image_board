use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, sqlx::FromRow, Debug,Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub uploader: String,
    pub artist: Option<String>,
    pub tags: Vec<String>,
}


#[derive(SimpleObject, sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
    pub tag_type: String,
}


#[derive(SimpleObject, sqlx::FromRow, Debug)]
pub struct Query{
    pub tags : Vec<Tag>,
    pub posts : Vec<Post>,
}


#[derive(SimpleObject, sqlx::FromRow, Debug,Deserialize, Serialize)]
pub struct PostUpload{
    pub uploader: String,
    pub artist: String,
    pub tags: Vec<Tag>,
}


impl Default for Post {
    fn default() -> Self {
        Post {
            id: 0,
            uploader: String::new(),
            artist: None,
            tags: Vec::new(),
        }
    }
}