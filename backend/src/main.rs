extern crate argon2;

use rand::Rng;

use sqlx::postgres::Postgres;
use sqlx::{query, Pool};

use fizz::models;

use axum::{
    extract::{Extension, Json},
    handler::post,
    http::StatusCode,
    response::IntoResponse,
    AddExtensionLayer, Router,
};
use http::{header::CONTENT_TYPE, Method};
use tower_http::cors::{Any, CorsLayer};

use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use validator::Validate;

#[tokio::main]
async fn main() -> fizz::Res<()> {
    let contents = std::fs::read_to_string("config.toml").expect("Failed to read config.toml");
    let config: models::Config =
        toml::from_str(&contents).expect("Failed to parse config.toml contents");

    let pool = Pool::<Postgres>::connect(config.postgres_url).await?;

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::POST])
        .allow_origin(Any)
        .allow_credentials(false)
        .allow_headers(vec![CONTENT_TYPE]);

    let app = Router::new()
        .route("/users/create", post(create_user))
        .layer(AddExtensionLayer::new(Arc::new(pool)))
        .layer(cors);

    let addr = SocketAddr::from((
        [0, 0, 0, 0],
        env::var("PORT")
            .unwrap_or(config.port)
            .parse::<u16>()
            .unwrap(),
    ));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn create_user(
    Json(payload): Json<models::User>,
    state: Extension<Arc<Pool<Postgres>>>,
) -> impl IntoResponse {
    let state = &*state.0;

    if payload.validate().is_err() {
        return (StatusCode::BAD_REQUEST, "Validation Error");
    }

    let user_query = query("SELECT 1 FROM users WHERE email = $1 OR username = $2")
        .bind(&payload.email)
        .bind(&payload.username)
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

    if query("INSERT INTO users VALUES ($1, $2, $3);")
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
