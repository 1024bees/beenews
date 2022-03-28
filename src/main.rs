use zero2bees::startup::run;

#[tokio::main]
async fn main() {
    let listener =std::net::TcpListener::bind("127.0.0.1:3000").expect("could not bind to port 3000");
    run(listener).await.expect("Starting app failed");
}
