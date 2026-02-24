use async_trait::async_trait;
use sqlx::{PgPool, Row};

use crate::application::port::post_media_repository::PostMediaRepository;
use crate::domain::entity::post_media::PostMedia;
use crate::domain::vo::Id;

pub struct PgPostMediaRepository {
    pool: PgPool,
}

impl PgPostMediaRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostMediaRepository for PgPostMediaRepository {
    async fn save(&self, post_media: &PostMedia) -> Result<PostMedia, String> {
        sqlx::query(
            r#"INSERT INTO post_media (id, post_id, media_id) VALUES ($1, $2, $3)"#,
        )
        .bind(post_media.id.as_uuid())
        .bind(post_media.post_id.as_uuid())
        .bind(post_media.media_id.as_ref().map(|x| x.as_uuid()))
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(post_media.clone())
    }

    async fn find_by_id(&self, id: &Id) -> Result<Option<PostMedia>, String> {
        let row = sqlx::query(r#"SELECT id, post_id, media_id FROM post_media WHERE id = $1"#)
            .bind(id.as_uuid())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        row.map(|r| row_to_post_media(&r)).transpose()
    }

    async fn find_by_post_id(&self, post_id: &Id) -> Result<Vec<PostMedia>, String> {
        let rows = sqlx::query(
            r#"SELECT id, post_id, media_id FROM post_media WHERE post_id = $1"#,
        )
        .bind(post_id.as_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        rows.iter().map(row_to_post_media).collect()
    }

    async fn delete_by_post_id(&self, post_id: &Id) -> Result<(), String> {
        sqlx::query(r#"DELETE FROM post_media WHERE post_id = $1"#)
            .bind(post_id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

fn row_to_post_media(row: &sqlx::postgres::PgRow) -> Result<PostMedia, String> {
    Ok(PostMedia {
        id: Id::from_uuid(row.get("id")),
        post_id: Id::from_uuid(row.get("post_id")),
        media_id: row.get::<Option<uuid::Uuid>, _>("media_id").map(Id::from_uuid),
    })
}
