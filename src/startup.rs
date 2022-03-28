use axum::{
    routing::{get, post},
    Router,
};

use super::routes::health_check;
use super::routes::subscribe;

use std::future::Future;

use std::net::TcpListener;

pub fn run(listener: TcpListener) -> impl Future<Output = Result<(), hyper::Error>> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));
    axum::Server::from_tcp(listener)
        .expect("Spawning server from listener failed")
        .serve(app.into_make_service())
}
