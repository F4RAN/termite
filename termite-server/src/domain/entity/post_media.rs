use serde::{Deserialize, Serialize};

use crate::domain::vo::Id;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostMedia {
    pub id: Id,
    pub post_id: Id,
    pub media_id: Option<Id>,
}
