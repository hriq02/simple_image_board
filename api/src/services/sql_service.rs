




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
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS posts (
            id SERIAL PRIMARY KEY,
            uploader VARCHAR(20) NOT NULL,
            artist VARCHAR(100),
            tags TEXT[] NOT NULL
        );
        "#
    )
    .execute(pool)
    .await
    .expect("Failed to create posts table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            username VARCHAR(40) PRIMARY KEY,
            password VARCHAR(50) NOT NULL,
            email VARCHAR(100)
        );
        "#
    )
    .execute(pool)
    .await
    .expect("Failed to create users table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tags (
            name text PRIMARY KEY
        );
        "#
    )
    .execute(pool)
    .await
    .expect("Failed to create tags table");
}