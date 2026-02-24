
use axum::{
    routing::get,
    Router
};
use sqlx::{Pool, Postgres};


pub fn create_router(db: Pool<Postgres>) -> Router {
    // TODO: 
    // 1- Create Repo
    // 2- Create Usecase
    // 3- Create Controller
    return Router::new().route("/", get(|| async { "Hello, World!" }))
}