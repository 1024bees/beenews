use std::sync::Arc;

use axum::extract::{Extension, Form};

use axum::{http::StatusCode, response::IntoResponse};
use sqlx::PgConnection;

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    Form(form): Form<FormData>,
    Extension(connection): Extension<Arc<PgConnection>>,
) -> impl IntoResponse {
    println!("form is {:#?}", form);
    StatusCode::OK
}
