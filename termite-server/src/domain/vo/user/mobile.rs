use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mobile {
    value: String,
}

impl Mobile {
    /// Mobile: digits only, length 10–15 (E.164 style).
    pub fn new(value: String) -> Result<Self, String> {
        let value = value.trim();
        if value.is_empty() {
            return Err("Mobile cannot be empty".into());
        }
        if !value.chars().all(|c| c.is_ascii_digit()) {
            return Err("Mobile must contain only digits".into());
        }
        if value.len() < 10 || value.len() > 15 {
            return Err("Mobile must be 10–15 digits".into());
        }
        Ok(Mobile { value: value.to_string() })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}