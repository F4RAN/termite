use crate::application::port::user_repository::UserRepository;
use crate::domain::entity::user::User;
use crate::domain::vo::{Id, Mobile};

pub struct UpdateMobile {
    user_repo: Box<dyn UserRepository + Send + Sync>,
}

impl UpdateMobile {
    pub fn new(user_repo: Box<dyn UserRepository + Send + Sync>) -> Self {
        UpdateMobile { user_repo }
    }

    pub async fn execute(&self, user_id: &Id, mobile: Mobile) -> Result<User, String> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        user.mobile = Some(mobile);
        self.user_repo.save(&user).await
    }
}
