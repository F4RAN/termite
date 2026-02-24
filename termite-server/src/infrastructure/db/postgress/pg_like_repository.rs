use async_trait::async_trait;
use chrono::Utc;
use sqlx::{PgPool, Row};

use crate::application::port::like_repository::LikeRepository;
use crate::domain::entity::like::Like;
use crate::domain::vo::Id;

pub struct PgLikeRepository {
    pool: PgPool,
}

impl PgLikeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LikeRepository for PgLikeRepository {
    async fn save(&self, like: &Like) -> Result<Like, String> {
        sqlx::query(
            r#"INSERT INTO "like" (id, post_id, user_id, deleted, created_at)
               VALUES ($1, $2, $3, $4, $5)"#,
        )
        .bind(like.id.as_uuid())
        .bind(like.post_id.as_uuid())
        .bind(like.user_id.as_uuid())
        .bind(like.deleted)
        .bind(like.created_at.unwrap_or_else(Utc::now))
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Like {
            created_at: Some(like.created_at.unwrap_or_else(Utc::now)),
            ..like.clone()
        })
    }

    async fn delete(&self, post_id: &Id, user_id: &Id) -> Result<(), String> {
        let r = sqlx::query(
            r#"UPDATE "like" SET deleted = true WHERE post_id = $1 AND user_id = $2"#,
        )
        .bind(post_id.as_uuid())
        .bind(user_id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        if r.rows_affected() == 0 {
            return Err("Like not found".into());
        }
        Ok(())
    }

    async fn find_by_id(&self, id: &Id) -> Result<Option<Like>, String> {
        let row = sqlx::query(
            r#"SELECT id, post_id, user_id, deleted, created_at
               FROM "like" WHERE id = $1 AND deleted = false"#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        row.map(|r| row_to_like(&r)).transpose()
    }

    async fn find_by_post_and_user(
        &self,
        post_id: &Id,
        user_id: &Id,
    ) -> Result<Option<Like>, String> {
        let row = sqlx::query(
            r#"SELECT id, post_id, user_id, deleted, created_at
               FROM "like" WHERE post_id = $1 AND user_id = $2 AND deleted = false"#,
        )
        .bind(post_id.as_uuid())
        .bind(user_id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        row.map(|r| row_to_like(&r)).transpose()
    }

    async fn exists(&self, post_id: &Id, user_id: &Id) -> Result<bool, String> {
        let row: (bool,) = sqlx::query_as(
            r#"SELECT EXISTS(
               SELECT 1 FROM "like"
               WHERE post_id = $1 AND user_id = $2 AND deleted = false)"#,
        )
        .bind(post_id.as_uuid())
        .bind(user_id.as_uuid())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(row.0)
    }

    async fn count_by_post(&self, post_id: &Id) -> Result<u64, String> {
        let row: (i64,) = sqlx::query_as(
            r#"SELECT COUNT(*)::bigint FROM "like" WHERE post_id = $1 AND deleted = false"#,
        )
        .bind(post_id.as_uuid())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(row.0 as u64)
    }
}

fn row_to_like(row: &sqlx::postgres::PgRow) -> Result<Like, String> {
    Ok(Like {
        id: Id::from_uuid(row.get("id")),
        post_id: Id::from_uuid(row.get("post_id")),
        user_id: Id::from_uuid(row.get("user_id")),
        deleted: row.get::<bool, _>("deleted"),
        created_at: row.get::<Option<chrono::DateTime<Utc>>, _>("created_at"),
    })
}
