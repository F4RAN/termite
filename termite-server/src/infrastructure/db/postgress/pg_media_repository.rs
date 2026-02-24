use async_trait::async_trait;
use sqlx::{PgPool, Row};

use crate::application::port::media_repository::MediaRepository;
use crate::domain::entity::media::Media;
use crate::domain::vo::{Id, MediaPath, MediaType};

pub struct PgMediaRepository {
    pool: PgPool,
}

impl PgMediaRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MediaRepository for PgMediaRepository {
    async fn save(&self, media: &Media) -> Result<Media, String> {
        sqlx::query(
            r#"INSERT INTO media (id, "type", path) VALUES ($1, $2, $3)"#,
        )
        .bind(media.id.as_uuid())
        .bind(media.media_type.as_db_str())
        .bind(media.path.as_str())
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(media.clone())
    }

    async fn find_by_id(&self, id: &Id) -> Result<Option<Media>, String> {
        let row = sqlx::query(r#"SELECT id, "type", path FROM media WHERE id = $1"#)
            .bind(id.as_uuid())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        row.map(|r| row_to_media(&r)).transpose()
    }

    async fn delete(&self, id: &Id) -> Result<(), String> {
        let r = sqlx::query(r#"DELETE FROM media WHERE id = $1"#)
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        if r.rows_affected() == 0 {
            return Err("Media not found".into());
        }
        Ok(())
    }
}

fn row_to_media(row: &sqlx::postgres::PgRow) -> Result<Media, String> {
    let id = Id::from_uuid(row.get("id"));
    let type_str = row
        .get::<Option<String>, _>("type")
        .ok_or("Media type is null")?;
    let path_str = row
        .get::<Option<String>, _>("path")
        .ok_or("Media path is null")?;
    let media_type = MediaType::new(type_str).map_err(|e| e.to_string())?;
    let path = MediaPath::new(path_str).map_err(|e| e.to_string())?;
    Ok(Media {
        id,
        media_type,
        path,
    })
}
