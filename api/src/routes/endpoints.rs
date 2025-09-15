use axum::{extract::Path, http::StatusCode, Extension, Json};
use reqwest::Url;
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

pub async fn remove_post(
    Extension(pool): Extension<PgPool>,
    Path(file_id): Path<String>
) -> (StatusCode, Json<String>)  {
    
    let id : i64 = match file_id.parse(){
        Ok(vl) => vl,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json("Not a valid id".to_string()));
        }
    };

    sql_service::remove_post(&pool, &id).await;

    let rmv_url = match Url::parse(&format!("http://localhost:7000/remove/{}", id)){
        Ok(url) => url,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
    };

    match reqwest::get(rmv_url).await {
        Ok(_) => (),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))
    }

    (StatusCode::OK, Json(id.to_string()))
}