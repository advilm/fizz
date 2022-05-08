use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Duration;
use fizz::models::{Config, Token};
use jsonwebtoken::{encode, get_current_timestamp, EncodingKey, Header};
use rand::Rng;
use serde::Deserialize;
use sqlx::{postgres::Postgres, query, Pool};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Payload {
    #[validate(length(min = 5, max = 32))]
    username: String,

    #[validate(length(min = 8, max = 128))]
    password: String,
}

pub async fn register_user(
    Json(payload): Json<Payload>,
    Extension(db): Extension<Arc<Pool<Postgres>>>,
    Extension(config): Extension<Arc<Config>>,
) -> impl IntoResponse {
    let db = db.as_ref();

    if payload.validate().is_err() {
        return (StatusCode::BAD_REQUEST, "Validation Error".to_string());
    }

    let user_query = query("SELECT 1 FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(db)
        .await;

    if user_query.is_err() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "Database error".to_string(),
        );
    } else if user_query.unwrap().is_some() {
        return (StatusCode::CONFLICT, "User conflict".to_string());
    }

    let password = payload.password.as_bytes();
    let salt = &rand::thread_rng().gen::<[u8; 16]>();
    let a_config = argon2::Config::default();
    let hash = argon2::hash_encoded(password, salt, &a_config).unwrap();

    let uuid = Uuid::new_v4().as_u128();

    if query!(
        "INSERT INTO users (id, username, hash) VALUES ($1, $2, $3)",
        sqlx::types::Uuid::from_u128(uuid),
        &payload.username,
        &hash
    )
    .execute(db)
    .await
    .is_err()
    {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "Database error".to_string(),
        );
    }

    let month = Duration::weeks(4).num_milliseconds() as u64;
    let token = Token {
        uuid,
        exp: get_current_timestamp() + month,
    };

    let token = encode(
        &Header::default(),
        &token,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .unwrap();

    (StatusCode::CREATED, token)
}
