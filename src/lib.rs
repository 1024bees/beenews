use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

use std::future::Future;

use std::net::TcpListener;
async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub fn run(listener: TcpListener) -> impl Future<Output = Result<(), hyper::Error>> {
    let app = Router::new().route("/health_check", get(health_check));
    axum::Server::from_tcp(listener)
        .expect("Spawning server from listener failed")
        .serve(app.into_make_service())
}
