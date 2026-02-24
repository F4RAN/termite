use crate::application::port::user_repository::UserRepository;
use crate::domain::entity::user::User;
use crate::domain::vo::{Id, PasswordHash};

pub struct UpdatePassword {
    user_repo: Box<dyn UserRepository + Send + Sync>,
}

impl UpdatePassword {
    pub fn new(user_repo: Box<dyn UserRepository + Send + Sync>) -> Self {
        UpdatePassword { user_repo }
    }

    pub async fn execute(&self, user_id: &Id, password_hash: PasswordHash) -> Result<User, String> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        user.password_hash = password_hash;
        self.user_repo.save(&user).await
    }
}
