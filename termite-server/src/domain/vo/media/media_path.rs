use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaPath {
    value: String,
}

impl MediaPath {
    /// Media path: non-empty, max 512 chars.
    pub fn new(value: String) -> Result<Self, String> {
        let value = value.trim();
        if value.is_empty() {
            return Err("Media path cannot be empty".into());
        }
        if value.len() > 512 {
            return Err("Media path must be at most 512 characters".into());
        }
        Ok(MediaPath { value: value.to_string() })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}
