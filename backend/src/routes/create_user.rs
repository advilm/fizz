use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{Duration, Utc};
use fizz::models::{Config, Token, User};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::Rng;
use sqlx::{postgres::Postgres, query, Pool};
use std::sync::Arc;
use validator::Validate;

pub async fn create_user(
    Json(payload): Json<User>,
    state: Extension<Arc<Pool<Postgres>>>,
    config: Extension<Arc<Config>>,
) -> impl IntoResponse {
    let state = &*state.0;

    if payload.validate().is_err() {
        return (StatusCode::BAD_REQUEST, "Validation Error".to_string());
    }

    let user_query = query("SELECT 1 FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(state)
        .await;

    if user_query.is_err() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "Database error".to_string(),
        );
    } else if let Some(_) = user_query.unwrap() {
        return (StatusCode::CONFLICT, "User conflict".to_string());
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
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "Database error".to_string(),
        );
    }

    let token = Token {
        email: payload.email,
        exp: Utc::now()
            .checked_add_signed(Duration::weeks(4))
            .unwrap()
            .timestamp_millis(),
    };

    let token = encode(
        &Header::default(),
        &token,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .unwrap();

    (StatusCode::CREATED, token)
}
