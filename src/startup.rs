use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};

use tower_http::trace::TraceLayer;
use super::routes::{health_check, subscribe};


use sqlx::PgPool;
use std::future::Future;
use std::net::TcpListener;
use std::sync::Arc;
pub fn run(
    listener: TcpListener,
    connection: PgPool,
) -> impl Future<Output = Result<(), hyper::Error>> {
    //env_logger::Builder::from_env(Env:: default () . default_filter_or("info")) . init() ;
    //
    //
    //
    //
    
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(Arc::new(connection)));

    axum::Server::from_tcp(listener)
        .expect("Spawning server from listener failed")
        .serve(app.into_make_service())
}
