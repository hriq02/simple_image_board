use axum::{http::StatusCode, Extension, Json};
use sqlx::PgPool;
use crate::{model::entities::{PostUpload, Tag}, services::sql_service};

pub async fn upload(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<PostUpload>
) -> (StatusCode, Json<String>)  {

    let next_id = sql_service::get_next_post_serial(&pool).await;
    sql_service::insert_post(&pool, &payload).await;

    (StatusCode::OK, Json(
        next_id.to_string()
    ))
}


pub async fn new_tag(
    Extension(pool): Extension<PgPool>,
    Json(tag): Json<Tag>
) -> (StatusCode, Json<String>)  {
    
    sql_service::insert_tag(&pool, &tag).await;

    (StatusCode::OK, Json(
        tag.name
    ))
}


pub async fn get_next_post_id(
    Extension(pool): Extension<PgPool>,
) -> (StatusCode, Json<i64>)  {
    
    let next_id = sql_service::get_next_post_serial(&pool).await;
    (StatusCode::OK, Json(next_id))

}