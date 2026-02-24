use serde::{Deserialize, Serialize};
use bcrypt;

/// Plain password (before hashing). Use for registration/update; never persist.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Password {
    value: String,
}

impl Password {
    /// Password: at least 8 chars, at least one letter and one digit.
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err("Password cannot be empty".into());
        }
        if value.len() < 8 {
            return Err("Password must be at least 8 characters".into());
        }
        let has_letter = value.chars().any(|c| c.is_ascii_alphabetic());
        let has_digit = value.chars().any(|c| c.is_ascii_digit());
        if !has_letter || !has_digit {
            return Err("Password must contain at least one letter and one digit".into());
        }
        Ok(Password { value })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
    pub fn hash_password(password: Password) -> Result<String, String> {
        let password = password.as_str();
        let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|e| e.to_string())?;
        Ok(password_hash)
    }
}
