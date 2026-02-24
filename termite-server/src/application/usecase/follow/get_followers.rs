use crate::application::port::follow_repository::FollowRepository;
use crate::domain::entity::follow::Follow;
use crate::domain::vo::Id;

pub struct GetFollowers {
    follow_repo: Box<dyn FollowRepository + Send + Sync>,
}

impl GetFollowers {
    pub fn new(follow_repo: Box<dyn FollowRepository + Send + Sync>) -> Self {
        GetFollowers { follow_repo }
    }

    pub async fn execute(&self, user_id: &Id) -> Result<Vec<Follow>, String> {
        self.follow_repo.find_followers(user_id).await
    }
}
