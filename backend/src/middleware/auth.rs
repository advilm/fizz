use std::sync::Arc;

use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use fizz::models::{Config, Token};
use jsonwebtoken::{decode, errors::Error, Algorithm, DecodingKey, TokenData, Validation};
use sqlx::types::Uuid;

pub async fn auth<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let config: &Arc<Config> = req.extensions().get().unwrap();

    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Ok(user) = verify_token(auth_header, &config.secret).await {
        req.extensions_mut()
            .insert(Uuid::from_u128(user.claims.uuid));
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn verify_token(token: &str, secret: &String) -> Result<TokenData<Token>, Error> {
    decode::<Token>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
}
