use axum::{extract::Extension, response::IntoResponse, Json};
use serde::Deserialize;
use sqlx::{query, Pool, Postgres};
use std::sync::Arc;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct NewTask {
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
    Json(task): Json<NewTask>,
    Extension(email): Extension<String>,
    Extension(db): Extension<Arc<Pool<Postgres>>>,
) -> impl IntoResponse {
    let db = &*db;

    query!(
        r#"INSERT INTO tasks (email, title, description, priority, time_estimate, due, completed, color)
        values ($1, $2, $3, $4, $5, $6, $7, $8)"#,
        email,
        task.title,
        task.description,
        task.priority,
        task.time_estimate,
        task.due,
        false,
        task.color
    )
    .execute(db)
    .await
    .unwrap();

    "Task added"
}
