use crate::model::entities::Post;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use sqlx::PgPool;

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
}