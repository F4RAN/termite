use crate::application::port::post_repository::PostRepository;
use crate::domain::entity::post::Post;
use crate::domain::vo::Id;

pub struct GetFeed {
    post_repo: Box<dyn PostRepository + Send + Sync>,
}

impl GetFeed {
    pub fn new(post_repo: Box<dyn PostRepository + Send + Sync>) -> Self {
        GetFeed { post_repo }
    }

    pub async fn execute(
        &self,
        user_id: &Id,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Post>, String> {
        self.post_repo.find_feed(user_id, limit, offset).await
    }
}
