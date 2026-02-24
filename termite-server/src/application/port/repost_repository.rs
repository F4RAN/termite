use crate::domain::entity::repost::Repost;
use crate::domain::vo::Id;
use async_trait::async_trait;

#[async_trait]
pub trait RepostRepository {
    async fn save(&self, repost: &Repost) -> Result<Repost, String>;
    async fn delete(&self, post_id: &Id, user_id: &Id) -> Result<(), String>;
    async fn find_by_id(&self, id: &Id) -> Result<Option<Repost>, String>;
    async fn find_by_post_and_user(&self, post_id: &Id, user_id: &Id) -> Result<Option<Repost>, String>;
    async fn exists(&self, post_id: &Id, user_id: &Id) -> Result<bool, String>;
}
