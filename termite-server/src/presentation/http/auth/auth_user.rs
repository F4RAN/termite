use std::sync::Arc;

use crate::application::usecase::user::register_user::RegisterUser;
use crate::domain::entity::user::User;
use crate::domain::vo::{Email, Mobile, Username, Nickname, Password, PasswordHash};
use serde::Deserialize;
use axum::{
    Json,
    http::StatusCode,
};

#[derive(Deserialize)]
pub struct RegisterUserRequest{
    mobile: Option<String>,
    email: Option<String>,
    username: String,
    nickname: String,
    password: String
}


pub fn register_user_controller(register_user_usecase: RegisterUser) 
-> impl Fn(Json<RegisterUserRequest>) 
-> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Json<User>, StatusCode>> + Send>> + Clone {
    
    let register_user = Arc::new(register_user_usecase);
    move |Json(body): Json<RegisterUserRequest>| {
        let register_user = Arc::clone(&register_user);
        Box::pin(async move {
            let mobile = Mobile::new(body.mobile)
            .map_err(|_| StatusCode::BAD_REQUEST);
            let email = Email::new(body.email).
            map_err(|_| StatusCode::BAD_REQUEST);
            let username = Username::new(body.username).
            map_err(|_| StatusCode::BAD_REQUEST);
            let nickname = Nickname::new(body.nickname).
            map_err(|_| StatusCode::BAD_REQUEST);
            let password = Password::new(body.password).
            map_err(|_| StatusCode::BAD_REQUEST);
            let password_hash = Password::hash_password(password.unwrap())
            .map_err(|_| StatusCode::BAD_REQUEST);
            
            let password_hash = PasswordHash::new(password_hash.unwrap()).unwrap();
            
            let user = register_user
            .execute(username.unwrap(), password_hash, email.unwrap(), mobile.unwrap(), nickname.unwrap()).await;
            match user {
                Ok(user) => Ok(Json(user)),
                Err(e) => Err(StatusCode::BAD_REQUEST),
            }

        })
    }
}