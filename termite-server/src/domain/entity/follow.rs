use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::vo::Id;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Follow {
    pub id: Id,
    pub following_user_id: Id,
    pub followed_user_id: Id,
    pub deleted: bool,
    pub created_at: Option<DateTime<Utc>>,
}
