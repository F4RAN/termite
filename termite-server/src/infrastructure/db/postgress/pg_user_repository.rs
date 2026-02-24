use async_trait::async_trait;
use chrono::Utc;
use sqlx::{PgPool, Row};

use crate::application::port::user_repository::UserRepository;
use crate::domain::entity::user::User;
use crate::domain::vo::{
    Email, Id, Mobile, Nickname, PasswordHash, Role, Username,
};

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn save(&self, user: &User) -> Result<User, String> {
        sqlx::query(
            r#"INSERT INTO "user" (id, username, email, mobile, password_hash, nickname, avatar, header, role, deleted, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"#,
        )
        .bind(user.id.as_uuid())
        .bind(user.username.as_str())
        .bind(user.email.as_ref().map(|e| e.as_str()))
        .bind(user.mobile.as_ref().map(|m| m.as_str()))
        .bind(user.password_hash.as_str())
        .bind(user.nickname.as_ref().map(|n| n.as_str()))
        .bind(user.avatar.as_ref().map(|a| a.as_uuid()))
        .bind(user.header.as_ref().map(|h| h.as_uuid()))
        .bind(user.role.as_db_str())
        .bind(user.deleted)
        .bind(user.created_at.unwrap_or_else(Utc::now))
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(User {
            created_at: Some(user.created_at.unwrap_or_else(Utc::now)),
            ..user.clone()
        })
    }

    async fn find_by_id(&self, id: &Id) -> Result<User, String> {
        let row = sqlx::query(
            r#"SELECT id, username, email, mobile, password_hash, nickname, avatar, header, role, deleted, created_at
               FROM "user" WHERE id = $1 AND deleted = false"#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let row = row.ok_or("User not found")?;
        row_to_user(&row)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, String> {
        let row = sqlx::query(
            r#"SELECT id, username, email, mobile, password_hash, nickname, avatar, header, role, deleted, created_at
               FROM "user" WHERE username = $1 AND deleted = false"#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        match row {
            Some(r) => row_to_user(&r).map(Some),
            None => Ok(None),
        }
    }
}

fn row_to_user(row: &sqlx::postgres::PgRow) -> Result<User, String> {
    let id = Id::from_uuid(row.get("id"));
    let username = Username::new(row.get::<String, _>("username"))
        .map_err(|e| e.to_string())?;
    let email = row
        .get::<Option<String>, _>("email")
        .and_then(|s| Email::new(s).ok());
    let mobile = row
        .get::<Option<String>, _>("mobile")
        .and_then(|s| Mobile::new(s).ok());
    let password_hash = PasswordHash::new(row.get("password_hash"))
        .map_err(|e| e.to_string())?;
    let nickname = row
        .get::<Option<String>, _>("nickname")
        .and_then(|s| Nickname::new(s).ok());
    let avatar = row.get::<Option<uuid::Uuid>, _>("avatar").map(Id::from_uuid);
    let header = row.get::<Option<uuid::Uuid>, _>("header").map(Id::from_uuid);
    let role = Role::new(row.get::<String, _>("role")).map_err(|e| e.to_string())?;
    let deleted = row.get::<bool, _>("deleted");
    let created_at = row.get::<Option<chrono::DateTime<Utc>>, _>("created_at");

    Ok(User {
        id,
        username,
        email,
        mobile,
        password_hash,
        nickname,
        avatar,
        header,
        role,
        deleted,
        created_at,
    })
}
