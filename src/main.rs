use zero2bees::configuration::get_configuration;
use zero2bees::startup::run;

#[tokio::main]
async fn main() {
    let config = get_configuration().expect("Failed to read configuration");
    let addr = format!("127.0.0.1:{}", config.application_port);
    let listener = std::net::TcpListener::bind(addr).expect("could not bind addr");
    run(listener).await.expect("Starting app failed");
}
