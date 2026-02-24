use crate::domain::entity::post_media::PostMedia;
use crate::domain::vo::Id;
use async_trait::async_trait;

#[async_trait]
pub trait PostMediaRepository {
    async fn save(&self, post_media: &PostMedia) -> Result<PostMedia, String>;
    async fn find_by_id(&self, id: &Id) -> Result<Option<PostMedia>, String>;
    async fn find_by_post_id(&self, post_id: &Id) -> Result<Vec<PostMedia>, String>;
    async fn delete_by_post_id(&self, post_id: &Id) -> Result<(), String>;
}
