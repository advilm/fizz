use serde::{Deserialize, Serialize};

pub struct Config {
    pub port: u16,
    pub db_url: String,
    pub secret: String,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub uuid: u128,
    pub exp: u64,
}

#[derive(Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub priority: i32,
    pub time_estimate: i32,
    pub due: i64,
    pub completed: bool,
    pub color: i32,
}
