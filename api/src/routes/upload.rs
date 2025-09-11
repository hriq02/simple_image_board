use axum::{http::StatusCode, Json};
use crate::model::entities::Post;

pub async fn upload(Json(payload): Json<Post>) -> (StatusCode, Json<Post>)  {
    println!("Upload: {:#?}", payload);
    (StatusCode::OK, Json(payload))
}
