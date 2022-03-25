use axum::{
    extract,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

use std::net::SocketAddr;

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new().route("/health_check", get(health_check));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Honestly no idea what the expectation is here");
}
