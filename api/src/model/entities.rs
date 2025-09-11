use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, sqlx::FromRow, Debug,Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub uploader: String,
    pub artist: Option<String>,
    pub tags: Vec<String>,
}


#[derive(SimpleObject, sqlx::FromRow, Debug)]
pub struct Tag {
    pub name: String,
}


#[derive(SimpleObject, sqlx::FromRow, Debug)]
pub struct Query{
    pub tags : Vec<Tag>,
    pub posts : Vec<Post>,
}