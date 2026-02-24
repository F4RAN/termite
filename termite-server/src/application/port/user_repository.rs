use crate::domain::entity::user::User;
use crate::domain::vo::Id;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: &User) -> Result<User, String>;
    async fn find_by_id(&self, id: &Id) -> Result<User, String>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, String>;
}