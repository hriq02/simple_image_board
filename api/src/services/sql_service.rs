use crate::{logger, model::entities::{PostUpload, Tag}};
use sqlx::Row;


pub async fn setup_db() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    let pool = setup_conn().await?;
    setup_tables(&pool).await;    
    Ok(pool)
}

pub async fn setup_conn() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://admin:123456@localhost:5000/database?sslmode=disable")
        .await
}


async fn query( pool: &sqlx::Pool<sqlx::Postgres>,query: &str, error : &str) {
    match sqlx::query(query).execute(pool).await{
        Ok(_) => (),
        Err(e) => 
            logger::log_err(
                error, 
                e, 
                logger::LogLevel::Error
            )
    }
}

pub async fn setup_tables(pool: &sqlx::Pool<sqlx::Postgres>) {
    query(pool,
        r#"
        CREATE TABLE IF NOT EXISTS posts (
            id SERIAL PRIMARY KEY,
            uploader VARCHAR(20) NOT NULL,
            artist VARCHAR(100),
            tags TEXT[] NOT NULL
        );
        "#,
        "Failed to create posts table"
    ).await;


    query(pool,
        r#"
        CREATE TABLE IF NOT EXISTS users (
            username VARCHAR(40) PRIMARY KEY,
            password VARCHAR(50) NOT NULL,
            email VARCHAR(100)
        );
        "#,
        "Failed to create users table"
    ).await;

    query(pool,
        r#"
        CREATE TABLE IF NOT EXISTS tags (
            name text PRIMARY KEY,
            tag_type CHAR(1) NOT NULL
        );
        "#, 
        "Failed to create tags table"
    )
    .await;

    query(pool,
        "CREATE EXTENSION IF NOT EXISTS pg_trgm;",
    "Failed to create pg_trgm extension"
    ).await;
    
}



pub async fn insert_tag(pool: &sqlx::Pool<sqlx::Postgres>, tag : &Tag){
    let name = tag.name.to_lowercase().replace(" ", "_");
    match sqlx::query(
        "INSERT INTO tags (name) VALUES ($1) ON CONFLICT DO NOTHING"
    )
    .bind(name)
    .execute(pool)
    .await
    {
        Ok(_) => (),
        Err(e) => 
                logger::log_err(
                    "Failed to create tag", 
                    e, 
                    logger::LogLevel::Error
                )  
    }
}


pub async fn remove_post(
    pool: &sqlx::Pool<sqlx::Postgres>, 
    id : &i64
){

    match sqlx::query(
        "DELETE FROM posts WHERE id = $1"
    )
    .bind(id)
    .execute(pool)
    .await
    {
        Ok(_) => (),
        Err(e) => logger::log_err(
                "Failed to remove post", 
                e,
                logger::LogLevel::Error
        )  
    }

    query(
        pool,
        "SELECT setval('posts_id_seq', (SELECT COALESCE(MAX(id), 0) FROM posts))", 
        "could not update posts_id_seq"
    ).await;

}


pub async fn insert_post(
    pool: &sqlx::Pool<sqlx::Postgres>, 
    post : &PostUpload
){
    let tags_str : Vec<String> = 
                    post.tags.iter()
                            .map(|t| t.name.clone())
                            .collect();
    match sqlx::query(
        "INSERT INTO posts (uploader, artist, tags) 
            VALUES ($1, $2, $3)"
        )
        .bind(post.uploader.clone())
        .bind(post.artist.clone())
        .bind(tags_str)
        .execute(pool)
    .await
    {
        Ok(_) => (),
        Err(e) => logger::log_err(
                "Failed to create post", 
                e,
                logger::LogLevel::Error
        )  
    }
}


pub async fn get_next_post_serial(
    pool: &sqlx::Pool<sqlx::Postgres>
) -> i64{
    return match sqlx::query("SELECT last_value + 1 FROM posts_id_seq")
    .fetch_one(pool)
    .await
    {
        Ok(row) => row.get::<i64,_>(0),
        Err(e) => {
            logger::log_err(
                "Failed to get next serial", 
                e, 
                logger::LogLevel::Error
            );
            -1
        }
    }
}