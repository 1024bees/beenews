use sqlx::PgPool;
use zero2bees::configuration::get_configuration;
use zero2bees::startup::run;
use zero2bees::telemetry::{get_subscriber,init_subscriber};
#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let config = get_configuration().expect("Failed to read configuration");
    let addr = format!("127.0.0.1:{}", config.application_port);
    let listener = std::net::TcpListener::bind(addr).expect("could not bind addr");
    run(listener, connection)
        .await
        .expect("Starting app failed");
}
