use crate::application::port::like_repository::LikeRepository;
use crate::domain::entity::like::Like;
use crate::domain::vo::Id;

pub struct LikePostToggle {
    like_repo: Box<dyn LikeRepository + Send + Sync>,
}

impl LikePostToggle {
    pub fn new(like_repo: Box<dyn LikeRepository + Send + Sync>) -> Self {
        LikePostToggle { like_repo }
    }

    /// Returns true if the post is now liked, false if now unliked.
    pub async fn execute(&self, post_id: &Id, user_id: &Id) -> Result<bool, String> {
        let exists = self.like_repo.exists(post_id, user_id).await?;
        if exists {
            self.like_repo.delete(post_id, user_id).await?;
            Ok(false)
        } else {
            let like = Like {
                id: Id::new_v4(),
                post_id: post_id.clone(),
                user_id: user_id.clone(),
                deleted: false,
                created_at: None,
            };
            self.like_repo.save(&like).await?;
            Ok(true)
        }
    }
}
