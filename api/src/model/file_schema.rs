use std::collections::HashSet;
use crate::{logger, model::entities::Post};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use sqlx::PgPool;
use super::entities::{Query, Tag};

pub(crate) struct QueryRoot;
pub(crate) type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
#[Object]
impl QueryRoot {
        async fn query_tag(&self, ctx: &Context<'_>, near_tag: String) -> async_graphql::Result<Vec<Tag>> {
        Ok(
            sqlx::query_as::<_, Tag>(
                r#"
                SELECT name
                FROM tags
                ORDER BY similarity(name, $1) DESC
                LIMIT 10
                "#
            )
            .bind(near_tag)
            .fetch_all(ctx.data::<PgPool>()?)
            .await
            .map_err(|e| {
                let gql_error = async_graphql::ServerError::new(format!("Database error: {}", e), std::option::Option::None); // <- pos = None
                logger::log_err(
                    "Failed to query tags",
                    gql_error.clone(),
                    logger::LogLevel::Warn,
                );
                gql_error
            })?
        )
    }

    async fn query_posts(
        &self,
        ctx: &Context<'_>,
        tags: Vec<String>,
        page: i32
    ) -> async_graphql::Result<Query> {
        let pool = ctx.data::<PgPool>()?;
        let limit = 20;
        let offset = ((page - 1).max(0) * limit) as i64;
        let mut unique_tags : HashSet<String> = HashSet::new();

        let err = |e| {
            let gql_error = async_graphql::ServerError::new(format!("Database error: {}", e), std::option::Option::None); // <- pos = None
            logger::log_err(
                "Failed to query posts",
                gql_error.clone(),
                logger::LogLevel::Warn,
            );
            gql_error
        };
    
        let posts = if tags.is_empty() {
            sqlx::query_as::<_, Post>(
                r#"
                SELECT id, uploader, artist, tags
                FROM posts
                LIMIT $1
                OFFSET $2
                "#
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(err)?
        } else {
            sqlx::query_as::<_, Post>(
                r#"
                SELECT id, uploader, artist, tags
                FROM posts
                WHERE tags @> $1::text[]
                LIMIT $2
                OFFSET $3
                "#
            )
            .bind(tags.as_slice())
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(err)?
        };
    
        let all_tags = posts.iter()
            .flat_map(|post| post.tags.clone())
            .filter(|tag| unique_tags.insert(tag.clone()))
            .map(|tag| Tag { name: tag })
            .collect();
    
        Ok(Query { posts, tags: all_tags })
    }
    
}