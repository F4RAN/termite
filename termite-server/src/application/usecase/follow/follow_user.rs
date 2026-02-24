use crate::application::port::follow_repository::FollowRepository;
use crate::domain::entity::follow::Follow;
use crate::domain::vo::Id;

pub struct FollowUser {
    follow_repo: Box<dyn FollowRepository + Send + Sync>,
}

impl FollowUser {
    pub fn new(follow_repo: Box<dyn FollowRepository + Send + Sync>) -> Self {
        FollowUser { follow_repo }
    }

    pub async fn execute(
        &self,
        following_user_id: &Id,
        followed_user_id: &Id,
    ) -> Result<Follow, String> {
        if following_user_id.as_uuid_string() == followed_user_id.as_uuid_string() {
            return Err("Cannot follow yourself".into());
        }
        if self.follow_repo.exists(following_user_id, followed_user_id).await? {
            return Err("Already following".into());
        }
        let follow = Follow {
            id: Id::new_v4(),
            following_user_id: following_user_id.clone(),
            followed_user_id: followed_user_id.clone(),
            deleted: false,
            created_at: None,
        };
        self.follow_repo.save(&follow).await
    }
}
