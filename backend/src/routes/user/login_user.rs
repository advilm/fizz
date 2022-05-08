use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Duration;
use fizz::models::{Config, Token};
use jsonwebtoken::{encode, get_current_timestamp, EncodingKey, Header};
use serde::Deserialize;
use sqlx::{postgres::Postgres, query, Pool};
use std::sync::Arc;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Payload {
    #[validate(length(min = 5, max = 32))]
    username: String,

    #[validate(length(min = 8, max = 128))]
    password: String,
}

pub async fn login_user(
    Json(payload): Json<Payload>,
    Extension(db): Extension<Arc<Pool<Postgres>>>,
    Extension(config): Extension<Arc<Config>>,
) -> impl IntoResponse {
    if payload.validate().is_err() {
        return (StatusCode::BAD_REQUEST, "Validation Error".to_string());
    }

    let user_query = query!(
        "SELECT id,username,hash FROM users WHERE username = $1",
        &payload.username
    )
    .fetch_optional(db.as_ref())
    .await;

    if user_query.is_err() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "Database error".to_string(),
        );
    }

    if let Some(user) = user_query.unwrap() {
        let password = payload.password.as_bytes();

        if argon2::verify_encoded(&user.hash, password).unwrap() {
            let month = Duration::weeks(4).num_milliseconds() as u64;
            let token = Token {
                uuid: user.id.as_u128(),
                exp: get_current_timestamp() + month,
            };

            let token = encode(
                &Header::default(),
                &token,
                &EncodingKey::from_secret(config.secret.as_bytes()),
            )
            .unwrap();

            return (StatusCode::OK, token);
        } else {
            return (StatusCode::UNAUTHORIZED, "Invalid password".to_string());
        }
    }

    (StatusCode::NOT_FOUND, "User not found".to_string())
}
