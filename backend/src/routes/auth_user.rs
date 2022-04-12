use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
};
use fizz::models;
use sqlx::{postgres::Postgres, query, Pool};
use std::sync::Arc;

pub async fn auth_user(
    Json(payload): Json<models::LoginUser>,
    state: Extension<Arc<Pool<Postgres>>>,
) -> impl IntoResponse {
    let state = &*state.0;

    let user_query = query!(r"SELECT hash FROM users WHERE email = $1", &payload.email)
        .fetch_optional(state)
        .await;

    if user_query.is_err() {
        return (StatusCode::SERVICE_UNAVAILABLE, "Database error");
    }

    if let Some(user) = user_query.unwrap() {
        let password = payload.password.as_bytes();

        if argon2::verify_encoded(&user.hash, password).unwrap() {
            return (StatusCode::OK, "User authenticated");
        } else {
            return (StatusCode::UNAUTHORIZED, "Invalid password");
        }
    }

    (StatusCode::NOT_FOUND, "User not found")
}
