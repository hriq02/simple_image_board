use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, routing::{get,post}, Router};
use routes::graphql::{graphql_handler, graphql_playground, health};
use tower_http::cors::{Any, CorsLayer};
use model::file_schema::QueryRoot;
use tokio::net::TcpListener;

mod routes;
mod model;
mod services;


#[tokio::main]
async fn main() {
    let pool = services::sql_service::setup_db().await;
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).data(pool.clone()).finish();

    print!("\x1B[2J\x1B[1;1H"); //clear terminal
    println!("API Started on http://localhost:8000/");

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);
    
    let app = Router::new()
        .route("/playground", 
            get(graphql_playground)
            .post(graphql_handler)
        )
        .route("/query", post(graphql_handler))
        .route("/health", get(health))
        .layer(Extension(schema))
        .layer(cors);

    

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
