use axum;
mod domain;
mod application;
mod router;
mod presentation;
use termite_core::{get_api_url, get_db_url};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    let db_url= get_db_url();
    let pgsql_client = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url).await.unwrap();
    
    let app = router::create_router(pgsql_client);
    let api_url = get_api_url();

    let listener = tokio::net::TcpListener::bind(&api_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}