use axum::extract::Form;
use axum::routing::post;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

use std::future::Future;

use std::net::TcpListener;
async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

#[derive(serde::Deserialize, Debug)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(Form(form): Form<FormData>) -> impl IntoResponse {
    println!("form is {:#?}", form);
    StatusCode::OK
}

pub fn run(listener: TcpListener) -> impl Future<Output = Result<(), hyper::Error>> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));
    axum::Server::from_tcp(listener)
        .expect("Spawning server from listener failed")
        .serve(app.into_make_service())
}
