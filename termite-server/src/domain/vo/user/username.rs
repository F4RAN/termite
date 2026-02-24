use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Username {
    value: String,
}

impl Username {
    /// Username: 3–30 chars, alphanumeric, underscore, hyphen.
    pub fn new(value: String) -> Result<Self, String> {
        let value = value.trim();
        if value.is_empty() {
            return Err("Username cannot be empty".into());
        }
        if value.len() < 3 {
            return Err("Username must be at least 3 characters".into());
        }
        if value.len() > 30 {
            return Err("Username must be at most 30 characters".into());
        }
        let re = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
        if re.is_match(value) {
            Ok(Username { value: value.to_string() })
        } else {
            Err("Username may only contain letters, digits, underscore and hyphen".into())
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}
