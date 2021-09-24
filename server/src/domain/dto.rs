use validator::Validate;

use crate::config::constants::{PASSWORD_RE, USERNAME_RE};

#[derive(Debug, Deserialize, Validate)]
pub struct InitInput {
    #[validate(regex(path = "USERNAME_RE"))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(regex(path = "PASSWORD_RE"))]
    pub password: String,
    #[validate(length(min = 3))]
    pub nickname: String,
}