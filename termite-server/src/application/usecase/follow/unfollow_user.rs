use crate::application::port::follow_repository::FollowRepository;
use crate::domain::vo::Id;

pub struct UnfollowUser {
    follow_repo: Box<dyn FollowRepository + Send + Sync>,
}

impl UnfollowUser {
    pub fn new(follow_repo: Box<dyn FollowRepository + Send + Sync>) -> Self {
        UnfollowUser { follow_repo }
    }

    pub async fn execute(
        &self,
        following_user_id: &Id,
        followed_user_id: &Id,
    ) -> Result<(), String> {
        self.follow_repo
            .delete(following_user_id, followed_user_id)
            .await
    }
}
