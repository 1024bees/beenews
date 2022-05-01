use super::routes::{health_check, subscribe};
use crate::email_client::EmailClient;
use axum::body::Body;
use axum::http::Request;
use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::future::Future;
use std::net::TcpListener;

use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tower_request_id::{RequestId, RequestIdLayer};

use crate::configuration::{get_configuration, DatabaseSettings, Settings};
use sqlx::postgres::PgPoolOptions;

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub async fn build(configuration: Settings) -> impl Future<Output = Result<(), hyper::Error>> {
    let connection = get_connection_pool(&configuration.database);
    // Build an `EmailClient` using `configuration`
    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender email address.");
    let timeout = configuration.email_client.timeout();
    let email_client = EmailClient::new(
        configuration.email_client.base_url,
        sender_email,
        configuration.email_client.authorization_token,
        timeout,
    );

    let config = get_configuration().expect("Failed to read configuration");
    let addr = format!("{}:{}", config.application.host, config.application.port);
    tracing::info!("starting app at {}", addr);
    let listener = std::net::TcpListener::bind(addr).expect("could not bind addr");
    run(listener, connection, email_client)
}

pub fn run(
    listener: TcpListener,
    connection: PgPool,
    email_client: EmailClient,
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
        .layer(Extension(Arc::new(connection)))
        .layer(Extension(Arc::new(email_client)));

    axum::Server::from_tcp(listener)
        .expect("Spawning server from listener failed")
        .serve(app.into_make_service())
}

// A new type to hold the newly built server and its port
pub struct Application {
    port: u16,
    listener: TcpListener,
    connection_pool: PgPool,
    email_client: EmailClient,
}
impl Application {
    // We have converted the `build` function into a constructor for
    // `Application`.
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address.");
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
        );
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        // We "save" the bound port in one of `Application`'s fields
        Ok(Self {
            listener,
            connection_pool,
            email_client,
            port,
        })
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), hyper::Error> {
        run(self.listener, self.connection_pool, self.email_client).await
    }
}
