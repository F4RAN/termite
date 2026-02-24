use crate::domain::entity::media::Media;
use crate::domain::vo::Id;
use async_trait::async_trait;

#[async_trait]
pub trait MediaRepository {
    async fn save(&self, media: &Media) -> Result<Media, String>;
    async fn find_by_id(&self, id: &Id) -> Result<Option<Media>, String>;
    async fn delete(&self, id: &Id) -> Result<(), String>;
}
