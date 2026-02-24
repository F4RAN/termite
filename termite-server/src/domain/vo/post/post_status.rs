use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PostStatus {
    Published,
    Draft,
    Archived,
}

impl PostStatus {
    /// Post status: published, draft, or archived (case-insensitive). Default in DB is 'published'.
    pub fn new(value: String) -> Result<Self, String> {
        match value.trim().to_lowercase().as_str() {
            "published" => Ok(PostStatus::Published),
            "draft" => Ok(PostStatus::Draft),
            "archived" => Ok(PostStatus::Archived),
            _ => Err("Post status must be published, draft, or archived".into()),
        }
    }
}
