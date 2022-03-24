use axum::{extract, response::IntoResponse, routing::get, Router};

use std::net::SocketAddr;
async fn greeting(extract::Path(name): extract::Path<String>) -> String {
    format!("Hello {}", name)
}

async fn basic_greeting() -> String {
    format!("Hello World")
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        .route("/", get(basic_greeting))
        .route("/:name", get(greeting));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Honestly no idea what the expectation is here");
}
