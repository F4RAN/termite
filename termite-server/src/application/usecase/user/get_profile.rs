use crate::application::port::user_repository::UserRepository;
use crate::domain::entity::user::User;

pub struct GetProfile {
    user_repo: Box<dyn UserRepository + Send + Sync>,
}

impl GetProfile {
    pub fn new(user_repo: Box<dyn UserRepository + Send + Sync>) -> Self {
        GetProfile { user_repo }
    }

    pub async fn execute(&self, username: &str) -> Result<Option<User>, String> {
        self.user_repo.find_by_username(username).await
    }
}
