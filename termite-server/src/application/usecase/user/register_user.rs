use crate::application::port::user_repository::UserRepository;
use crate::domain::entity::user::User;
use crate::domain::vo::{Email, Id, Mobile, Nickname, PasswordHash, Role, Username};

pub struct RegisterUser {
    user_repo: Box<dyn UserRepository + Send + Sync>,
}

impl RegisterUser {
    pub fn new(user_repo: Box<dyn UserRepository + Send + Sync>) -> Self {
        RegisterUser { user_repo }
    }

    pub async fn execute(
        &self,
        username: Username,
        password_hash: PasswordHash,
        role: Role,
        email: Option<Email>,
        mobile: Option<Mobile>,
        nickname: Option<Nickname>,
    ) -> Result<User, String> {
        if mobile.is_none() && email.is_none() {
            return Err("Mobile number or Email Address is mandatory.".into());
        }
        let user = User {
            id: Id::new_v4(),
            username,
            email,
            mobile,
            password_hash,
            nickname,
            avatar: None,
            header: None,
            role,
            deleted: false,
            created_at: None,
        };
        self.user_repo.save(&user).await
    }
}
