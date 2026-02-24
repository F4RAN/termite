use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Nickname {
    value: String,
}

impl Nickname {
    /// Nickname: 1–30 chars, alphanumeric, underscore, hyphen.
    pub fn new(value: String) -> Result<Self, String> {
        let value = value.trim();
        if value.is_empty() {
            return Err("Nickname cannot be empty".into());
        }
        if value.len() > 30 {
            return Err("Nickname must be at most 30 characters".into());
        }
        let re = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
        if re.is_match(value) {
            Ok(Nickname { value: value.to_string() })
        } else {
            Err("Nickname may only contain letters, digits, underscore and hyphen".into())
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}
