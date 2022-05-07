use serde::{Deserialize, Serialize};

pub struct Config {
    pub port: u16,
    pub db_url: String,
    pub secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub email: String,
    pub exp: u64,
}
