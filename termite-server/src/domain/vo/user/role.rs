use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Moderator,
    User,
    Guest,
}

impl Role {
    /// Role: must be one of admin, moderator, user, guest (case-insensitive).
    pub fn new(value: String) -> Result<Self, String> {
        match value.trim().to_lowercase().as_str() {
            "admin" => Ok(Role::Admin),
            "moderator" => Ok(Role::Moderator),
            "user" => Ok(Role::User),
            "guest" => Ok(Role::Guest),
            _ => Err("Role must be admin, moderator, user, or guest".into()),
        }
    }

    /// Lowercase string for DB storage.
    pub fn as_db_str(&self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::Moderator => "moderator",
            Role::User => "user",
            Role::Guest => "guest",
        }
    }
}