use axum::{http::StatusCode, Extension, Json};
use sqlx::PgPool;
use crate::{model::entities::Tag, services::sql_service};

pub async fn new_tag(
    Extension(pool): Extension<PgPool>,
    Json(tag): Json<Tag>
) -> (StatusCode, Json<String>)  {
    
    sql_service::insert_tag(&pool, &tag).await;

    (StatusCode::OK, Json(
        tag.name
    ))
}
