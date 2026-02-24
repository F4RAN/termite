use crate::application::port::post_media_repository::PostMediaRepository;
use crate::application::port::post_repository::PostRepository;
use crate::domain::entity::post::Post;
use crate::domain::entity::post_media::PostMedia;
use crate::domain::vo::{Id, PostBody, PostStatus};

pub struct CreatePost {
    post_repo: Box<dyn PostRepository + Send + Sync>,
    post_media_repo: Box<dyn PostMediaRepository + Send + Sync>,
}

impl CreatePost {
    pub fn new(
        post_repo: Box<dyn PostRepository + Send + Sync>,
        post_media_repo: Box<dyn PostMediaRepository + Send + Sync>,
    ) -> Self {
        CreatePost {
            post_repo,
            post_media_repo,
        }
    }

    pub async fn execute(
        &self,
        user_id: &Id,
        body: PostBody,
        reply_to_post_id: Option<Id>,
        quote_to_post_id: Option<Id>,
        media_ids: Vec<Id>,
    ) -> Result<Post, String> {
        let post = Post {
            id: Id::new_v4(),
            body,
            user_id: user_id.clone(),
            reply_to_post_id,
            quote_to_post_id,
            status: PostStatus::Published,
            deleted: false,
            edited_at: None,
            created_at: None,
        };
        let post = self.post_repo.save(&post).await?;
        for media_id in media_ids {
            let post_media = PostMedia {
                id: Id::new_v4(),
                post_id: post.id.clone(),
                media_id: Some(media_id),
            };
            let _ = self.post_media_repo.save(&post_media).await?;
        }
        Ok(post)
    }
}
