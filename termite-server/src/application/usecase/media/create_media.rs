use crate::application::port::media_repository::MediaRepository;
use crate::domain::entity::media::Media;
use crate::domain::vo::{Id, MediaPath, MediaType};

pub struct CreateMedia {
    media_repo: Box<dyn MediaRepository + Send + Sync>,
}

impl CreateMedia {
    pub fn new(media_repo: Box<dyn MediaRepository + Send + Sync>) -> Self {
        CreateMedia { media_repo }
    }

    pub async fn execute(&self, media_type: MediaType, path: MediaPath) -> Result<Media, String> {
        let media = Media {
            id: Id::new_v4(),
            media_type,
            path,
        };
        self.media_repo.save(&media).await
    }
}
