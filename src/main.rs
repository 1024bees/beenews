use sqlx::postgres::PgPoolOptions;
use zero2bees::configuration::get_configuration;
use zero2bees::startup::run;
use zero2bees::telemetry::{get_subscriber, init_subscriber};
#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let config = get_configuration().expect("Failed to read configuration");
    let addr = format!("{}:{}", config.application.host, config.application.port);
    tracing::info!("starting app at {}", addr);
    let listener = std::net::TcpListener::bind(addr).expect("could not bind addr");
    run(listener, connection)
        .await
        .expect("Starting app failed");
}
