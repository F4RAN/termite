use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::vo::Id;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repost {
    pub id: Id,
    pub post_id: Id,
    pub user_id: Id,
    pub deleted: bool,
    pub created_at: Option<DateTime<Utc>>,
}
