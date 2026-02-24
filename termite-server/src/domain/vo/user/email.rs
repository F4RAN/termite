use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Email {
    value: String,
}

impl Email {
    /// Email: local@domain.tld, standard format.
    pub fn new(value: Option<String>) -> Result<Option<Self>, String> {
        if value.is_none() {
            return Ok(None);
        }
        let value = value.as_ref().unwrap().trim();
        if value.is_empty() {
            return Ok(None);
        }
        let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if re.is_match(value) {
            Ok(Some(Email { value: value.to_string() }))
        } else {
            Err("Invalid email format".into())
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}
