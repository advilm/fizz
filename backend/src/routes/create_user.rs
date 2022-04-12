use rand::Rng;

use sqlx::query;

use axum::http::StatusCode;

use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
};
use fizz::models;
use sqlx::{postgres::Postgres, Pool};
use std::sync::Arc;
use validator::Validate;

pub async fn create_user(
    Json(payload): Json<models::User>,
    state: Extension<Arc<Pool<Postgres>>>,
) -> impl IntoResponse {
    let state = &*state.0;

    if payload.validate().is_err() {
        return (StatusCode::BAD_REQUEST, "Validation Error");
    }

    let user_query = query("SELECT 1 FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(state)
        .await;

    if user_query.is_err() {
        return (StatusCode::SERVICE_UNAVAILABLE, "Database error");
    }

    if let Some(_) = user_query.unwrap() {
        return (StatusCode::CONFLICT, "User conflict");
    }

    let password = payload.password.as_bytes();
    let salt = &rand::thread_rng().gen::<[u8; 16]>();
    let a_config = argon2::Config::default();
    let hash = argon2::hash_encoded(password, salt, &a_config).unwrap();

    if query("INSERT INTO users (email, username, hash) VALUES ($1, $2, $3);")
        .bind(&payload.email)
        .bind(&payload.username)
        .bind(&hash)
        .execute(state)
        .await
        .is_err()
    {
        return (StatusCode::SERVICE_UNAVAILABLE, "Database error");
    }

    (StatusCode::CREATED, "User created")
}
