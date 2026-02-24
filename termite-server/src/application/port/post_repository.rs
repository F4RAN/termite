use crate::domain::entity::post::Post;
use crate::domain::vo::Id;
use async_trait::async_trait;

#[async_trait]
pub trait PostRepository {
    async fn save(&self, post: &Post) -> Result<Post, String>;
    async fn find_by_id(&self, id: &Id) -> Result<Option<Post>, String>;
    async fn update(&self, post: &Post) -> Result<Post, String>;
    async fn delete(&self, id: &Id) -> Result<(), String>;
    async fn find_by_user_id(&self, user_id: &Id, limit: u32, offset: u32) -> Result<Vec<Post>, String>;
    async fn find_feed(&self, user_id: &Id, limit: u32, offset: u32) -> Result<Vec<Post>, String>;
    async fn find_replies(&self, post_id: &Id, limit: u32, offset: u32) -> Result<Vec<Post>, String>;
}
