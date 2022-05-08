use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use fizz::models::Config;
use http::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use sqlx::{postgres::Postgres, Pool};
use std::{env, net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

mod routes;
use routes::*;

mod middleware;
use middleware::*;

#[tokio::main]
async fn main() -> fizz::Res<()> {
    dotenv().ok();

    let config: Config = Config {
        port: env::var("PORT").unwrap().parse::<u16>().unwrap(),
        db_url: env::var("DATABASE_URL").unwrap(),
        secret: env::var("SECRET").unwrap(),
    };

    let pool = Pool::<Postgres>::connect(&config.db_url).await?;

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::POST, Method::GET])
        .allow_origin(Any)
        .allow_headers(vec![CONTENT_TYPE, AUTHORIZATION]);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    let app = Router::new()
        .route("/tasks/add", post(add_task))
        .route("/tasks/fetch", get(get_tasks))
        .layer(axum::middleware::from_fn(auth))
        .route("/users/register", post(register_user))
        .route("/users/login", post(login_user))
        .layer(Extension(Arc::new(config)))
        .layer(Extension(Arc::new(pool)))
        .layer(cors);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
