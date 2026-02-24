use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordHash {
    value: String,
}

impl PasswordHash {
    /// Password hash: non-empty, max 255 chars (e.g. bcrypt/argon2).
    pub fn new(value: String) -> Result<Option<Self>, String> {
        let value = value.as_str();
        if value.is_empty() {
            return Ok(None);
        }
        if value.len() > 255 {
            return Err("Password hash must be at most 255 characters".into());
        }
        Ok(Some(PasswordHash { value: value.to_string() }))
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

