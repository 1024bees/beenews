use std::sync::Arc;

use axum::extract::{Extension, Form};

use axum::{http::StatusCode, response::IntoResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
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
    let request_id = Uuid::new_v4();
    // Spans, like logs, have an associated level
    // `info_span` creates a span at the info-level
    let request_span = tracing::info_span!(
    "Adding a new subscriber WOOHOO! ." ,
        % request_id ,
        subscriber_email = % form . email ,
        subscriber_name = % form . name
    );
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("Saving new subscriber details in the database",);

    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgConnection`
    // wrapped by `web::Data`.
    .execute(connection.as_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("req_id {}: Successfully added user details!", request_id);
            StatusCode::OK
        }
        Err(e) => {
            tracing::error!("req_id {}: Failed to execute query: {:?}", request_id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
