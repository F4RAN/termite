use async_trait::async_trait;
use chrono::Utc;
use sqlx::{PgPool, Row};

use crate::application::port::repost_repository::RepostRepository;
use crate::domain::entity::repost::Repost;
use crate::domain::vo::Id;

pub struct PgRepostRepository {
    pool: PgPool,
}

impl PgRepostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RepostRepository for PgRepostRepository {
    async fn save(&self, repost: &Repost) -> Result<Repost, String> {
        sqlx::query(
            r#"INSERT INTO repost (id, post_id, user_id, deleted, created_at)
               VALUES ($1, $2, $3, $4, $5)"#,
        )
        .bind(repost.id.as_uuid())
        .bind(repost.post_id.as_uuid())
        .bind(repost.user_id.as_uuid())
        .bind(repost.deleted)
        .bind(repost.created_at.unwrap_or_else(Utc::now))
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Repost {
            created_at: Some(repost.created_at.unwrap_or_else(Utc::now)),
            ..repost.clone()
        })
    }

    async fn delete(&self, post_id: &Id, user_id: &Id) -> Result<(), String> {
        let r = sqlx::query(
            r#"UPDATE repost SET deleted = true WHERE post_id = $1 AND user_id = $2"#,
        )
        .bind(post_id.as_uuid())
        .bind(user_id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        if r.rows_affected() == 0 {
            return Err("Repost not found".into());
        }
        Ok(())
    }

    async fn find_by_id(&self, id: &Id) -> Result<Option<Repost>, String> {
        let row = sqlx::query(
            r#"SELECT id, post_id, user_id, deleted, created_at
               FROM repost WHERE id = $1 AND deleted = false"#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        row.map(|r| row_to_repost(&r)).transpose()
    }

    async fn find_by_post_and_user(
        &self,
        post_id: &Id,
        user_id: &Id,
    ) -> Result<Option<Repost>, String> {
        let row = sqlx::query(
            r#"SELECT id, post_id, user_id, deleted, created_at
               FROM repost WHERE post_id = $1 AND user_id = $2 AND deleted = false"#,
        )
        .bind(post_id.as_uuid())
        .bind(user_id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        row.map(|r| row_to_repost(&r)).transpose()
    }

    async fn exists(&self, post_id: &Id, user_id: &Id) -> Result<bool, String> {
        let row: (bool,) = sqlx::query_as(
            r#"SELECT EXISTS(
               SELECT 1 FROM repost
               WHERE post_id = $1 AND user_id = $2 AND deleted = false)"#,
        )
        .bind(post_id.as_uuid())
        .bind(user_id.as_uuid())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(row.0)
    }
}

fn row_to_repost(row: &sqlx::postgres::PgRow) -> Result<Repost, String> {
    Ok(Repost {
        id: Id::from_uuid(row.get("id")),
        post_id: Id::from_uuid(row.get("post_id")),
        user_id: Id::from_uuid(row.get("user_id")),
        deleted: row.get::<bool, _>("deleted"),
        created_at: row.get::<Option<chrono::DateTime<Utc>>, _>("created_at"),
    })
}
