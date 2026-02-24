use crate::application::port::post_repository::PostRepository;
use crate::domain::entity::post::Post;
use crate::domain::vo::Id;

pub struct GetPost {
    post_repo: Box<dyn PostRepository + Send + Sync>,
}

impl GetPost {
    pub fn new(post_repo: Box<dyn PostRepository + Send + Sync>) -> Self {
        GetPost { post_repo }
    }

    pub async fn execute(&self, post_id: &Id) -> Result<Option<Post>, String> {
        self.post_repo.find_by_id(post_id).await
    }
}
