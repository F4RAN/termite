use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordHash {
    value: String,
}

impl PasswordHash {
    /// Password hash: non-empty, max 255 chars (e.g. bcrypt/argon2).
    pub fn new(value: String) -> Result<Self, String> {
        let value = value.trim();
        if value.is_empty() {
            return Err("Password hash cannot be empty".into());
        }
        if value.len() > 255 {
            return Err("Password hash must be at most 255 characters".into());
        }
        Ok(PasswordHash { value: value.to_string() })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}
