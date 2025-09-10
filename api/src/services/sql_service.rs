


pub async fn setup_db() -> sqlx::Pool<sqlx::Postgres> {
    let pool = setup_conn().await;
    setup_tables(&pool).await;    
    pool
}

pub async fn setup_conn() -> sqlx::Pool<sqlx::Postgres> {
    let database_url = "postgresql://admin:123456@localhost:5000/database?sslmode=disable";

    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect(&format!("Database connection error"))
}




pub async fn setup_tables(pool: &sqlx::Pool<sqlx::Postgres>) {
    async fn query( pool: &sqlx::Pool<sqlx::Postgres>,query: &str, error : &str) {
        sqlx::query(query)
        .execute(pool)
        .await
        .expect(error);
    }

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
            name text PRIMARY KEY
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