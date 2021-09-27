use validator::Validate;

use crate::config::constants::{PASSWORD_RE, USERNAME_RE};

#[derive(Debug, Deserialize, Validate)]
pub struct InitInput {
    #[validate(regex(path = "USERNAME_RE", message = "Username is invalid"))]
    pub username: String,
    #[validate(email(message = "Email is invalid"))]
    pub email: String,
    #[validate(regex(path = "PASSWORD_RE", message = "Password is invalid"))]
    pub password: String,
    #[validate(length(min = 3, message = "Nickname is invalid"))]
    pub nickname: String,
}