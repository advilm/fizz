use axum::{extract::Extension, response::IntoResponse, Json};
use http::StatusCode;
use serde::Deserialize;
use sqlx::{query, types::Uuid, Pool, Postgres};
use std::sync::Arc;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Payload {
    #[validate(length(min = 1, max = 300))]
    title: String,
    description: String,
    #[validate(range(min = 1, max = 100))]
    priority: i32,
    #[validate(range(min = 0))]
    time_estimate: i32,
    #[validate(range(min = 0))]
    due: i64,
    #[validate(range(min = 0, max = 16777215))]
    color: i32,
}

pub async fn add_task(
    Json(payload): Json<Payload>,
    Extension(uuid): Extension<Uuid>,
    Extension(db): Extension<Arc<Pool<Postgres>>>,
) -> impl IntoResponse {
    if payload.validate().is_err() {
        return (StatusCode::BAD_REQUEST, "Validation Error");
    }

    query!(
        r#"INSERT INTO tasks (user_id, title, description, priority, time_estimate, due, completed, color)
        values ($1, $2, $3, $4, $5, $6, $7, $8)"#,
        uuid,
        payload.title,
        payload.description,
        payload.priority,
        payload.time_estimate,
        payload.due,
        false,
        payload.color
    )
    .execute(db.as_ref())
    .await
    .unwrap();

    (StatusCode::OK, "Task added")
}
