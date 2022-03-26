use axum::{http::StatusCode, response::IntoResponse, routing::get, Router, Server};

use std::{future::Future, net::SocketAddr};

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub fn run() -> impl Future<Output = Result<(), hyper::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new().route("/health_check", get(health_check));
    axum::Server::bind(&addr).serve(app.into_make_service())
}
