use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};

use super::routes::health_check;
use super::routes::subscribe;

use std::future::Future;
use std::sync::Arc;

use sqlx::PgPool;
use std::net::TcpListener;
pub fn run(
    listener: TcpListener,
    connection: PgPool,
) -> impl Future<Output = Result<(), hyper::Error>> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(Extension(Arc::new(connection)));
    axum::Server::from_tcp(listener)
        .expect("Spawning server from listener failed")
        .serve(app.into_make_service())
}
