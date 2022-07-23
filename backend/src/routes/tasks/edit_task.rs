use axum::{extract::Extension, response::IntoResponse, Json};
use http::StatusCode;
use serde::Deserialize;
use sqlx::{query, types::Uuid, Pool, Postgres};
use std::sync::Arc;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Payload {
    id: i32,
    #[validate(length(min = 1, max = 300))]
    title: String,
    description: String,
    #[validate(range(min = 1, max = 100))]
    priority: i32,
    #[validate(range(min = 0))]
    time_estimate: i32,
    #[validate(range(min = 0))]
    due: i64,
    #[validate(range(min = 0))]
    recurring: i16,
    completed: bool,
    #[validate(range(min = 0, max = 16777215))]
    color: i32,
}

pub async fn edit_task(
    Json(payload): Json<Payload>,
    Extension(uuid): Extension<Uuid>,
    Extension(db): Extension<Arc<Pool<Postgres>>>,
) -> impl IntoResponse {
    if payload.validate().is_err() {
        return (StatusCode::BAD_REQUEST, "Validation Error");
    }

    query!(
        r#"UPDATE tasks SET title = $1, description = $2, priority = $3,
        time_estimate = $4, due = $5, completed = $6, color = $7, recurring = $8
        WHERE id = $9 AND user_id = $10"#,
        payload.title,
        payload.description,
        payload.priority,
        payload.time_estimate,
        payload.due,
        payload.completed,
        payload.color,
        payload.recurring,
        payload.id,
        uuid,
    )
    .execute(db.as_ref())
    .await
    .unwrap();

    (StatusCode::OK, "Task edited")
}
