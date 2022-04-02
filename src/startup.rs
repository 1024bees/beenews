use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};

use super::routes::{health_check, subscribe};
use tower_http::trace::TraceLayer;

use tower_request_id::{RequestId, RequestIdLayer};

use axum::body::Body;
use axum::http::Request;
use sqlx::PgPool;
use std::future::Future;
use std::net::TcpListener;
use std::sync::Arc;
pub fn run(
    listener: TcpListener,
    connection: PgPool,
) -> impl Future<Output = Result<(), hyper::Error>> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(
            // Let's create a tracing span for each request
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                // We get the request id from the extensions
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "unknown".into());
                // And then we put it along with other information into the `request` span
                tracing::info_span!(
                    "request",
                    id = %request_id,
                    method = %request.method(),
                    uri = %request.uri(),
                )
            }),
        )
        // This layer creates a new id for each request and puts it into the request extensions.
        // Note that it should be added after the Trace layer.
        .layer(RequestIdLayer)
        .layer(Extension(Arc::new(connection)));

    axum::Server::from_tcp(listener)
        .expect("Spawning server from listener failed")
        .serve(app.into_make_service())
}
