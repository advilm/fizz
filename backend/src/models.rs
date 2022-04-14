use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct User {
    #[validate(email, length(max = 50))]
    pub email: String,

    #[validate(length(min = 5, max = 32))]
    pub username: String,

    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginUser {
    #[validate(email, length(max = 50))]
    pub email: String,

    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

pub struct Config {
    pub port: u16,
    pub db_url: String,
    pub secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub email: String,
    pub exp: i64,
}
