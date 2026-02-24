use crate::application::port::repost_repository::RepostRepository;
use crate::domain::entity::repost::Repost;
use crate::domain::vo::Id;

pub struct RepostToggle {
    repost_repo: Box<dyn RepostRepository + Send + Sync>,
}

impl RepostToggle {
    pub fn new(repost_repo: Box<dyn RepostRepository + Send + Sync>) -> Self {
        RepostToggle { repost_repo }
    }

    /// Returns true if the post is now reposted, false if now un-reposted.
    pub async fn execute(&self, post_id: &Id, user_id: &Id) -> Result<bool, String> {
        let exists = self.repost_repo.exists(post_id, user_id).await?;
        if exists {
            self.repost_repo.delete(post_id, user_id).await?;
            Ok(false)
        } else {
            let repost = Repost {
                id: Id::new_v4(),
                post_id: post_id.clone(),
                user_id: user_id.clone(),
                deleted: false,
                created_at: None,
            };
            self.repost_repo.save(&repost).await?;
            Ok(true)
        }
    }
}
