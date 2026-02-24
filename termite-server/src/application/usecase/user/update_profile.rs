use crate::application::port::user_repository::UserRepository;
use crate::domain::entity::user::User;
use crate::domain::vo::{Id, Nickname};

pub struct UpdateProfile {
    user_repo: Box<dyn UserRepository + Send + Sync>,
}

impl UpdateProfile {
    pub fn new(user_repo: Box<dyn UserRepository + Send + Sync>) -> Self {
        UpdateProfile { user_repo }
    }

    pub async fn execute(&self, user_id: &Id, nickname: Option<Nickname>) -> Result<User, String> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        if let Some(n) = nickname {
            user.nickname = Some(n);
        }
        self.user_repo.save(&user).await
    }
}
