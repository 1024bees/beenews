use axum::extract::Form;

use axum::{http::StatusCode, response::IntoResponse};

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(Form(form): Form<FormData>) -> impl IntoResponse {
    println!("form is {:#?}", form);
    StatusCode::OK
}
