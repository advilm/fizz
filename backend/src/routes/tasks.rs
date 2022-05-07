use axum::{extract::Extension, response::IntoResponse};

pub async fn get_tasks(Extension(_email): Extension<String>) -> impl IntoResponse {
    "todo"
}
