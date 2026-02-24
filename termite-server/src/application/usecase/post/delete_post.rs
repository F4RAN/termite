use crate::application::port::post_repository::PostRepository;
use crate::domain::vo::Id;

pub struct DeletePost {
    post_repo: Box<dyn PostRepository + Send + Sync>,
}

impl DeletePost {
    pub fn new(post_repo: Box<dyn PostRepository + Send + Sync>) -> Self {
        DeletePost { post_repo }
    }

    pub async fn execute(&self, post_id: &Id, user_id: &Id) -> Result<(), String> {
        let post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or("Post not found")?;
        if post.user_id.as_uuid_string() != user_id.as_uuid_string() {
            return Err("Not authorized to delete this post".into());
        }
        self.post_repo.delete(post_id).await
    }
}
