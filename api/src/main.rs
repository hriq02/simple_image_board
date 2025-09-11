use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, routing::{get,post}, Router};
use routes::graphql::{graphql_handler, graphql_playground, health};
use tower_http::cors::{Any, CorsLayer};
use model::file_schema::QueryRoot;
use tokio::net::TcpListener;

mod routes;
mod model;
mod services;
mod logger;
mod errors;

#[tokio::main]
async fn main() {
    
    print!("\x1B[2J\x1B[1;1H"); //clear terminal
    logger::log("API Started on http://localhost:8000/", logger::LogLevel::Info);
    
    match run().await{
        Ok(_) => println!("API Stopped"),
        Err(e) => logger::log_err("Error: {}", e, logger::LogLevel::Error)
    };
}


async fn run() -> Result<(), std::io::Error>{
    let pool = match services::sql_service::setup_db().await {
        Ok(pool) => pool,
        Err(e) => {
            logger::log_err("Error setting up database", e, logger::LogLevel::Error);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database setup failed"));
        }
    };
    axum::serve(
        TcpListener::bind("0.0.0.0:8000").await?,
        Router::new()
        .route("/upload",post(routes::upload::upload))
        .route("/new_tag",post(routes::new_tag::new_tag))
        .route("/playground", 
            get(graphql_playground)
            .post(graphql_handler)
        )
        .route("/query", post(graphql_handler))
        .route("/health", get(health))
        .layer(Extension(pool.clone()))
        .layer(Extension(
            Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
                .data(pool)
                .finish()
        ))
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_origin(Any)
                .allow_headers(Any)
        )
    ).await?;

    Ok(())

}