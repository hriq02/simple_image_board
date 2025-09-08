use async_graphql::SimpleObject;

#[derive(SimpleObject, sqlx::FromRow, Debug)]
pub struct Post {
    pub id: i32,
    pub uploader: String,
    pub artist: Option<String>,
    pub tags: Vec<String>,
}