use crate::application::port::post_repository::PostRepository;
use crate::domain::entity::post::Post;
use crate::domain::vo::{Id, PostBody, PostStatus};

pub struct EditPost {
    post_repo: Box<dyn PostRepository + Send + Sync>,
}

impl EditPost {
    pub fn new(post_repo: Box<dyn PostRepository + Send + Sync>) -> Self {
        EditPost { post_repo }
    }

    pub async fn execute(
        &self,
        post_id: &Id,
        user_id: &Id,
        body: Option<PostBody>,
        status: Option<PostStatus>,
    ) -> Result<Post, String> {
        let mut post = self
            .post_repo
            .find_by_id(post_id)
            .await?
            .ok_or("Post not found")?;
        if post.user_id.as_uuid_string() != user_id.as_uuid_string() {
            return Err("Not authorized to edit this post".into());
        }
        if let Some(b) = body {
            post.body = b;
        }
        if let Some(s) = status {
            post.status = s;
        }
        self.post_repo.update(&post).await
    }
}
