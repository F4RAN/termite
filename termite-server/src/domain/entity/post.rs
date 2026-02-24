use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::vo::{Id, PostBody, PostStatus};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Id,
    pub body: PostBody,
    pub user_id: Id,
    pub reply_to_post_id: Option<Id>,
    pub quote_to_post_id: Option<Id>,
    pub status: PostStatus,
    pub deleted: bool,
    pub edited_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}
