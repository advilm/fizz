use axum::routing::post;

use sqlx::{postgres::Postgres, Pool};

use axum::{extract::Extension, Router};
use http::{header::CONTENT_TYPE, Method};
use tower_http::cors::{Any, CorsLayer};

use std::{env, net::SocketAddr, sync::Arc};

use dotenv::dotenv;

use fizz::models::Config;

mod routes;
use routes::{auth_user, create_user};

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
        .allow_methods(vec![Method::POST])
        .allow_origin(Any)
        .allow_credentials(false)
        .allow_headers(vec![CONTENT_TYPE]);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    let app = Router::new()
        .route("/users/create", post(create_user))
        .route("/users/auth", post(auth_user))
        .layer(Extension(Arc::new(pool)))
        .layer(Extension(Arc::new(config)))
        .layer(cors);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
