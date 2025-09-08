use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router, Server};
use model::file_schema::QueryRoot;

mod routes;
mod model;
mod services;

#[derive(Clone)]
pub struct AppState {
    pool: sqlx::Pool<sqlx::Postgres>,
}


#[tokio::main]
async fn main() {
    let pool = services::sql_service::setup_db().await;
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).data(pool.clone()).finish();

    let state = AppState{
        pool
    };

    print!("\x1B[2J\x1B[1;1H"); //clear terminal
    println!("Listening on http://localhost:8000/");
    
    let app = Router::new()
        .with_state(state)
        .route("/", get(routes::file_route::graphql_playground).post(routes::file_route::graphql_handler))
        .route("/health", get(routes::file_route::health))
        .layer(Extension(schema));

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
