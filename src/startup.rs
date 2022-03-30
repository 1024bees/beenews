use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use super::routes::{health_check, subscribe};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};

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
    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        // Output the formatted spans to stdout.
        std::io::stdout,
    );
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .init();

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(Arc::new(connection)));

    axum::Server::from_tcp(listener)
        .expect("Spawning server from listener failed")
        .serve(app.into_make_service())
}
