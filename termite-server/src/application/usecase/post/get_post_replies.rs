use crate::application::port::post_repository::PostRepository;
use crate::domain::entity::post::Post;
use crate::domain::vo::Id;

pub struct GetPostReplies {
    post_repo: Box<dyn PostRepository + Send + Sync>,
}

impl GetPostReplies {
    pub fn new(post_repo: Box<dyn PostRepository + Send + Sync>) -> Self {
        GetPostReplies { post_repo }
    }

    pub async fn execute(
        &self,
        post_id: &Id,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Post>, String> {
        self.post_repo.find_replies(post_id, limit, offset).await
    }
}
