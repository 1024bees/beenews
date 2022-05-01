
use zero2bees::configuration::get_configuration;

use zero2bees::startup::{Application};
use zero2bees::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let subscriber = get_subscriber("zero2bees".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let app = Application::build(configuration)
        .await
        .expect("App build failed");
    app.run_until_stopped().await?;
    Ok(())
}
