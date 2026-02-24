use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostBody {
    value: String,
}

impl PostBody {
    /// Post body: text, 1–10_000 chars.
    pub fn new(value: String) -> Result<Self, String> {
        let value = value.trim();
        if value.is_empty() {
            return Err("Post body cannot be empty".into());
        }
        if value.len() > 10_000 {
            return Err("Post body must be at most 10_000 characters".into());
        }
        Ok(PostBody { value: value.to_string() })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}
