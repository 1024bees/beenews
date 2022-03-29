use std::sync::Arc;

use axum::extract::{Extension, Form};

use axum::{http::StatusCode, response::IntoResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    Form(form): Form<FormData>,
    Extension(connection): Extension<Arc<PgPool>>,
) -> impl IntoResponse {
    match sqlx::query!(
        r#"
INSERT INTO subscriptions (id, email, name, subscribed_at)
VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(connection.as_ref())
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            log::warn!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
