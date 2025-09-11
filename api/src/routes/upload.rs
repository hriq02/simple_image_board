use axum::{http::StatusCode, Extension, Json};
use sqlx::PgPool;
use crate::{logger::{ LogLevel,log_err}, model::entities::{Post, PostUpload}, services::sql_service};

pub async fn upload(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<PostUpload>
) -> (StatusCode, Json<Post>)  {

    let id : i32 = match sql_service::insert_post(&pool, &payload).await.try_into() {
        Ok(id) => id,
        Err(e) => {
            log_err("Failed to create post", e, LogLevel::Error);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(Post::default()));
        }
    };

    (StatusCode::OK, Json(
        Post{
            id,
            uploader: payload.uploader,
            artist: Some(payload.artist),
            tags: payload.tags.iter().map(|t| t.name.clone()).collect()
        }
    ))
}
