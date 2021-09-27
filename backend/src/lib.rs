pub mod models;

pub type Res<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
