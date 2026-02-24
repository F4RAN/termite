use async_trait::async_trait;
use chrono::Utc;
use sqlx::{PgPool, Row};

use crate::application::port::follow_repository::FollowRepository;
use crate::domain::entity::follow::Follow;
use crate::domain::vo::Id;

pub struct PgFollowRepository {
    pool: PgPool,
}

impl PgFollowRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FollowRepository for PgFollowRepository {
    async fn save(&self, follow: &Follow) -> Result<Follow, String> {
        sqlx::query(
            r#"INSERT INTO follow (id, following_user_id, followed_user_id, deleted, created_at)
               VALUES ($1, $2, $3, $4, $5)"#,
        )
        .bind(follow.id.as_uuid())
        .bind(follow.following_user_id.as_uuid())
        .bind(follow.followed_user_id.as_uuid())
        .bind(follow.deleted)
        .bind(follow.created_at.unwrap_or_else(Utc::now))
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Follow {
            created_at: Some(follow.created_at.unwrap_or_else(Utc::now)),
            ..follow.clone()
        })
    }

    async fn delete(&self, following_user_id: &Id, followed_user_id: &Id) -> Result<(), String> {
        let r = sqlx::query(
            r#"UPDATE follow SET deleted = true
               WHERE following_user_id = $1 AND followed_user_id = $2"#,
        )
        .bind(following_user_id.as_uuid())
        .bind(followed_user_id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        if r.rows_affected() == 0 {
            return Err("Follow not found".into());
        }
        Ok(())
    }

    async fn find_by_id(&self, id: &Id) -> Result<Option<Follow>, String> {
        let row = sqlx::query(
            r#"SELECT id, following_user_id, followed_user_id, deleted, created_at
               FROM follow WHERE id = $1 AND deleted = false"#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        row.map(|r| row_to_follow(&r)).transpose()
    }

    async fn find_followers(&self, user_id: &Id) -> Result<Vec<Follow>, String> {
        let rows = sqlx::query(
            r#"SELECT id, following_user_id, followed_user_id, deleted, created_at
               FROM follow WHERE followed_user_id = $1 AND deleted = false
               ORDER BY created_at DESC"#,
        )
        .bind(user_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        rows.iter().map(row_to_follow).collect()
    }

    async fn find_following(&self, user_id: &Id) -> Result<Vec<Follow>, String> {
        let rows = sqlx::query(
            r#"SELECT id, following_user_id, followed_user_id, deleted, created_at
               FROM follow WHERE following_user_id = $1 AND deleted = false
               ORDER BY created_at DESC"#,
        )
        .bind(user_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        rows.iter().map(row_to_follow).collect()
    }

    async fn exists(&self, following_user_id: &Id, followed_user_id: &Id) -> Result<bool, String> {
        let row: (bool,) = sqlx::query_as(
            r#"SELECT EXISTS(
               SELECT 1 FROM follow
               WHERE following_user_id = $1 AND followed_user_id = $2 AND deleted = false)"#,
        )
        .bind(following_user_id.as_uuid())
        .bind(followed_user_id.as_uuid())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(row.0)
    }
}

fn row_to_follow(row: &sqlx::postgres::PgRow) -> Result<Follow, String> {
    Ok(Follow {
        id: Id::from_uuid(row.get("id")),
        following_user_id: Id::from_uuid(row.get("following_user_id")),
        followed_user_id: Id::from_uuid(row.get("followed_user_id")),
        deleted: row.get::<bool, _>("deleted"),
        created_at: row.get::<Option<chrono::DateTime<Utc>>, _>("created_at"),
    })
}
