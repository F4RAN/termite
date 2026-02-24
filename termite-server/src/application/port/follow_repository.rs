use crate::domain::entity::follow::Follow;
use crate::domain::vo::Id;
use async_trait::async_trait;

#[async_trait]
pub trait FollowRepository {
    async fn save(&self, follow: &Follow) -> Result<Follow, String>;
    async fn delete(&self, following_user_id: &Id, followed_user_id: &Id) -> Result<(), String>;
    async fn find_by_id(&self, id: &Id) -> Result<Option<Follow>, String>;
    async fn find_followers(&self, user_id: &Id) -> Result<Vec<Follow>, String>;
    async fn find_following(&self, user_id: &Id) -> Result<Vec<Follow>, String>;
    async fn exists(&self, following_user_id: &Id, followed_user_id: &Id) -> Result<bool, String>;
}
