
use axum::{
    routing::get,
    Router
};
use sqlx::{Pool, Postgres};
use crate::presentation::http::auth::auth_user::register_user_controller;
use crate::application::usecase::user::register_user::RegisterUser

pub fn create_router(db: Pool<Postgres>) -> Router {
    // TODO: 
    // 1- Create Repo
    // 2- Create Usecase
    // 3- Create Controller



    return Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/register", post(register_user_controller(register_user_usecase: RegisterUser)));
}