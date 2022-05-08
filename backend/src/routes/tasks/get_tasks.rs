use axum::{extract::Extension, response::IntoResponse, Json};
use fizz::models::Task;
use sqlx::{query, types::Uuid, Pool, Postgres};
use std::sync::Arc;

pub async fn get_tasks(
    Extension(uuid): Extension<Uuid>,
    Extension(db): Extension<Arc<Pool<Postgres>>>,
) -> impl IntoResponse {
    let db = &*db;

    let user_query = query!("SELECT * FROM tasks WHERE user_id = $1", uuid)
        .fetch_all(db)
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
