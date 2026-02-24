use crate::application::port::user_repository::UserRepository;
use crate::domain::entity::user::User;
use crate::domain::vo::Id;

pub struct DeleteAccount {
    user_repo: Box<dyn UserRepository + Send + Sync>,
}

impl DeleteAccount {
    pub fn new(user_repo: Box<dyn UserRepository + Send + Sync>) -> Self {
        DeleteAccount { user_repo }
    }

    pub async fn execute(&self, user_id: &Id) -> Result<User, String> {
        let mut user = self.user_repo.find_by_id(user_id).await?;
        user.deleted = true;
        self.user_repo.save(&user).await
    }
}
