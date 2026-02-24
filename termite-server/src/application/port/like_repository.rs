use crate::domain::entity::like::Like;
use crate::domain::vo::Id;
use async_trait::async_trait;

#[async_trait]
pub trait LikeRepository {
    async fn save(&self, like: &Like) -> Result<Like, String>;
    async fn delete(&self, post_id: &Id, user_id: &Id) -> Result<(), String>;
    async fn find_by_id(&self, id: &Id) -> Result<Option<Like>, String>;
    async fn find_by_post_and_user(&self, post_id: &Id, user_id: &Id) -> Result<Option<Like>, String>;
    async fn exists(&self, post_id: &Id, user_id: &Id) -> Result<bool, String>;
    async fn count_by_post(&self, post_id: &Id) -> Result<u64, String>;
}
