use crate::application::port::user_repository::UserRepository;
use crate::domain::entity::user::User;
use crate::domain::vo::{Email, Id};

pub struct UpdateEmail {
    user_repo: Box<dyn UserRepository + Send + Sync>,
}

impl UpdateEmail {
    pub fn new(user_repo: Box<dyn UserRepository + Send + Sync>) -> Self {
        UpdateEmail { user_repo }
    }

    pub async fn execute(&self, user_id: &Id, email: Email) -> Result<User, String> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        user.email = Some(email);
        self.user_repo.save(&user).await
    }
}
