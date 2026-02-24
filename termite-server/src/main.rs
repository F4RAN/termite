use axum;
mod router;
use termite_core::get_api_url;

#[tokio::main]
async fn main() {
    let app = router::create_router();
    let url = get_api_url();
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}