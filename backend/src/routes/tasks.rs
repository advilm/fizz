use axum::{extract::Extension, response::IntoResponse, Json};
use serde::Serialize;
use sqlx::{postgres::types::PgInterval, query, Pool, Postgres};
use std::sync::Arc;

#[derive(Serialize)]
struct Task {
    id: i32,
    email: String,
    title: String,
    description: String,
    time_estimate: i64,
    due: i64,
    completed: bool,
    color: i32,
}

pub async fn get_tasks(
    Extension(email): Extension<String>,
    Extension(db): Extension<Arc<Pool<Postgres>>>,
) -> impl IntoResponse {
    let db = &*db;

    let user_query = query!("SELECT * FROM tasks WHERE email = $1", &email)
        .fetch_all(db)
        .await
        .unwrap();

    let tasks = user_query
        .iter()
        .map(|task| Task {
            id: task.id,
            email: task.email.clone(),
            title: task.title.clone(),
            description: task.description.clone(),
            time_estimate: pg_interval_to_ms(&task.time_estimate),
            due: task.due.assume_utc().unix_timestamp(),
            completed: task.completed,
            color: task.color,
        })
        .collect::<Vec<Task>>();

    Json(tasks)
}

fn pg_interval_to_ms(interval: &PgInterval) -> i64 {
    return interval.months as i64 * 30 * 24 * 60 * 60 * 1000
        + interval.days as i64 * 24 * 60 * 60 * 1000
        + interval.microseconds / 1000;
}
