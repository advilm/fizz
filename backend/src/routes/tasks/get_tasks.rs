use axum::{
    extract::{Extension, Query},
    response::IntoResponse,
    Json,
};
use fizz::models::Task;
use serde::Deserialize;
use sqlx::{query, types::Uuid, Pool, Postgres};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct Options {
    completed: Option<bool>,
}

pub async fn get_tasks(
    Extension(uuid): Extension<Uuid>,
    Extension(db): Extension<Arc<Pool<Postgres>>>,
    query: Query<Options>,
) -> impl IntoResponse {
    let user_query = query!(
        "SELECT * FROM tasks WHERE user_id = $1 AND completed = $2",
        uuid,
        query.completed.unwrap_or(false)
    )
    .fetch_all(db.as_ref())
    .await
    .unwrap();

    let tasks = user_query
        .iter()
        .map(|task| Task {
            id: task.id,
            title: task.title.clone(),
            description: task.description.clone(),
            priority: task.priority,
            time_estimate: task.time_estimate,
            due: task.due,
            completed: task.completed,
            color: task.color,
        })
        .collect::<Vec<Task>>();

    Json(tasks)
}
