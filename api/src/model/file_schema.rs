use crate::model::entities::Post;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use sqlx::PgPool;

use super::entities::{Query, Tag};

pub(crate) struct QueryRoot;
pub(crate) type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;
#[Object]
impl QueryRoot {
    async fn posts(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Post>> {
        let pool = ctx.data::<PgPool>()?;
        let posts = sqlx::query_as::<_, Post>(
            r#"
            SELECT id, uploader, artist, tags 
            FROM posts
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(posts)
    }

    async fn post(&self, ctx: &Context<'_>, id: i32) -> async_graphql::Result<Option<Post>> {
        let pool = ctx.data::<PgPool>()?;
        let post = sqlx::query_as::<_, Post>(
            r#"
            SELECT id, uploader, artist, tags 
            FROM posts 
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(post)
    }
    async fn post_by_tag(&self, ctx: &Context<'_>, tag: String) -> async_graphql::Result<Vec<Post>> {
        let pool = ctx.data::<PgPool>()?;

        let posts = sqlx::query_as::<_, Post>(
            r#"
            SELECT id, uploader, artist, tags
            FROM posts
            WHERE tags @> $1
            "#
        )
        .bind(vec![tag])
        .fetch_all(pool)
        .await?;

        Ok(posts)
    }

    async fn get_near_tag(&self, ctx: &Context<'_>, near_tag: String) -> async_graphql::Result<Vec<Tag>> {
        let pool = ctx.data::<PgPool>()?;

        let tags = sqlx::query_as::<_, Tag>(
            r#"
            SELECT name
            FROM tags
            ORDER BY similarity(name, $1) DESC
            LIMIT 10
            "#
        )
        .bind(near_tag)
        .fetch_all(pool)
        .await?;

        Ok(tags)
    }

    async fn query_posts(
        &self,
        ctx: &Context<'_>,
        tags: Vec<String>,
        page: i32
    ) -> async_graphql::Result<Option<Query>> {
        let pool = ctx.data::<PgPool>()?;
        let limit = 20;
        let offset = ((page - 1).max(0) * limit) as i64;
    
        // Se não houver tags, pega todos os posts
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
            .await?
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
            .bind(&tags[..])
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };
    
        let all_tags = posts.iter()
            .flat_map(|post| post.tags.clone())
            .map(|tag| Tag { name: tag })
            .collect();
    
        Ok(Some(Query { posts, tags: all_tags }))
    }
    
}