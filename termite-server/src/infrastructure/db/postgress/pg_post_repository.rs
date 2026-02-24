use async_trait::async_trait;
use chrono::Utc;
use sqlx::{PgPool, Row};

use crate::application::port::post_repository::PostRepository;
use crate::domain::entity::post::Post;
use crate::domain::vo::{Id, PostBody, PostStatus};

pub struct PgPostRepository {
    pool: PgPool,
}

impl PgPostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostRepository for PgPostRepository {
    async fn save(&self, post: &Post) -> Result<Post, String> {
        sqlx::query(
            r#"INSERT INTO post (id, body, user_id, reply_to_post_id, quote_to_post_id, status, deleted, edited_at, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
        )
        .bind(post.id.as_uuid())
        .bind(post.body.as_str())
        .bind(post.user_id.as_uuid())
        .bind(post.reply_to_post_id.as_ref().map(|x| x.as_uuid()))
        .bind(post.quote_to_post_id.as_ref().map(|x| x.as_uuid()))
        .bind(post.status.as_db_str())
        .bind(post.deleted)
        .bind(post.edited_at)
        .bind(post.created_at.unwrap_or_else(Utc::now))
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Post {
            created_at: Some(post.created_at.unwrap_or_else(Utc::now)),
            ..post.clone()
        })
    }

    async fn find_by_id(&self, id: &Id) -> Result<Option<Post>, String> {
        let row = sqlx::query(
            r#"SELECT id, body, user_id, reply_to_post_id, quote_to_post_id, status, deleted, edited_at, created_at
               FROM post WHERE id = $1 AND deleted = false"#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        row.map(|r| row_to_post(&r)).transpose()
    }

    async fn update(&self, post: &Post) -> Result<Post, String> {
        let r = sqlx::query(
            r#"UPDATE post SET body = $2, status = $3, deleted = $4, edited_at = $5
               WHERE id = $1"#,
        )
        .bind(post.id.as_uuid())
        .bind(post.body.as_str())
        .bind(post.status.as_db_str())
        .bind(post.deleted)
        .bind(post.edited_at)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        if r.rows_affected() == 0 {
            return Err("Post not found".into());
        }
        Ok(post.clone())
    }

    async fn delete(&self, id: &Id) -> Result<(), String> {
        let r = sqlx::query(r#"UPDATE post SET deleted = true WHERE id = $1"#)
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        if r.rows_affected() == 0 {
            return Err("Post not found".into());
        }
        Ok(())
    }

    async fn find_by_user_id(
        &self,
        user_id: &Id,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Post>, String> {
        let rows = sqlx::query(
            r#"SELECT id, body, user_id, reply_to_post_id, quote_to_post_id, status, deleted, edited_at, created_at
               FROM post WHERE user_id = $1 AND deleted = false
               ORDER BY created_at DESC NULLS LAST LIMIT $2 OFFSET $3"#,
        )
        .bind(user_id.as_uuid())
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        rows.iter().map(row_to_post).collect()
    }

    async fn find_feed(
        &self,
        user_id: &Id,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Post>, String> {
        // Feed: posts from users that user_id follows, plus own posts, ordered by created_at desc
        let rows = sqlx::query(
            r#"SELECT p.id, p.body, p.user_id, p.reply_to_post_id, p.quote_to_post_id, p.status, p.deleted, p.edited_at, p.created_at
               FROM post p
               WHERE p.deleted = false
                 AND (p.user_id = $1 OR p.user_id IN (SELECT followed_user_id FROM follow WHERE following_user_id = $1 AND deleted = false))
                 AND p.reply_to_post_id IS NULL
               ORDER BY p.created_at DESC NULLS LAST
               LIMIT $2 OFFSET $3"#,
        )
        .bind(user_id.as_uuid())
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        rows.iter().map(row_to_post).collect()
    }

    async fn find_replies(
        &self,
        post_id: &Id,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Post>, String> {
        let rows = sqlx::query(
            r#"SELECT id, body, user_id, reply_to_post_id, quote_to_post_id, status, deleted, edited_at, created_at
               FROM post WHERE reply_to_post_id = $1 AND deleted = false
               ORDER BY created_at DESC NULLS LAST LIMIT $2 OFFSET $3"#,
        )
        .bind(post_id.as_uuid())
        .bind(limit as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        rows.iter().map(row_to_post).collect()
    }
}

fn row_to_post(row: &sqlx::postgres::PgRow) -> Result<Post, String> {
    let id = Id::from_uuid(row.get("id"));
    let body = PostBody::new(row.get::<String, _>("body")).map_err(|e| e.to_string())?;
    let user_id = Id::from_uuid(row.get("user_id"));
    let reply_to_post_id = row
        .get::<Option<uuid::Uuid>, _>("reply_to_post_id")
        .map(Id::from_uuid);
    let quote_to_post_id = row
        .get::<Option<uuid::Uuid>, _>("quote_to_post_id")
        .map(Id::from_uuid);
    let status = PostStatus::new(row.get::<String, _>("status")).map_err(|e| e.to_string())?;
    let deleted = row.get::<bool, _>("deleted");
    let edited_at = row.get::<Option<chrono::DateTime<Utc>>, _>("edited_at");
    let created_at = row.get::<Option<chrono::DateTime<Utc>>, _>("created_at");
    Ok(Post {
        id,
        body,
        user_id,
        reply_to_post_id,
        quote_to_post_id,
        status,
        deleted,
        edited_at,
        created_at,
    })
}
