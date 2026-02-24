use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::domain::vo::{Email, Id, Mobile, Nickname, PasswordHash, Role, Username};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Id,
    pub username: Username,
    pub email: Option<Email>,
    pub mobile: Option<Mobile>,
    pub password_hash: Option<PasswordHash>,
    pub nickname: Nickname,
    /// Reference to media.id (avatar image).
    pub avatar: Option<Id>,
    /// Reference to media.id (header/banner image).
    pub header: Option<Id>,
    pub role: Role,
    pub deleted: bool,
    pub created_at: DateTime<Utc>,
}
