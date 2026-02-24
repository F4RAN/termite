use serde::{Deserialize, Serialize};

use crate::domain::vo::{Id, MediaPath, MediaType};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Media {
    pub id: Id,
    pub media_type: MediaType,
    pub path: MediaPath,
}
