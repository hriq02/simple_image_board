// routes/graphql.rs
use crate::model::file_schema::ServiceSchema;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use serde::Serialize;

#[derive(Serialize)] 
struct Health { 
    healthy: bool
}

pub(crate) async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(Health { healthy: true }))
}

pub(crate) async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/query").subscription_endpoint("/ws"),
    ))
}

pub(crate) async fn graphql_handler(
    Extension(schema): Extension<ServiceSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
