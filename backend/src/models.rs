use serde::Deserialize;
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
