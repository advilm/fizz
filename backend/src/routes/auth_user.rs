use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{Duration, Utc};
use fizz::models::{Config, LoginUser, Token};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::{postgres::Postgres, query, Pool};
use std::sync::Arc;

pub async fn auth_user(
    Json(payload): Json<LoginUser>,
    state: Extension<Arc<Pool<Postgres>>>,
    config: Extension<Arc<Config>>,
) -> impl IntoResponse {
    let state = &*state.0;

    let user_query = query!(
        r"SELECT username,hash FROM users WHERE email = $1",
        &payload.email
    )
    .fetch_optional(state)
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

            return (StatusCode::OK, token);
        } else {
            return (StatusCode::UNAUTHORIZED, "Invalid password".to_string());
        }
    }

    (StatusCode::NOT_FOUND, "User not found".to_string())
}
