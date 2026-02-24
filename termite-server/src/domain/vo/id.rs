use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Id {
    value: Uuid,
}

impl Id {
    /// Id: must be a valid UUID (any version).
    pub fn new(value: String) -> Result<Self, String> {
        let value = value.trim();
        if value.is_empty() {
            return Err("Id cannot be empty".into());
        }
        Uuid::parse_str(value).map_err(|e| format!("Invalid UUID: {}", e)).map(|value| Id { value })
    }

    /// Create a new random v4 Id.
    pub fn new_v4() -> Self {
        Id { value: Uuid::new_v4() }
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.value
    }

    pub fn as_uuid_string(&self) -> String {
        self.value.to_string()
    }
}
