use axum::extract::Extension;
use axum::Json;

use sea_orm::{DbConn, EntityTrait};
use validator::Validate;

use crate::domain::dto::InitInput;
use crate::domain::entity::prelude::Users;

use crate::domain::payload::Payload;
use crate::error::{ApiResult, ServerError};

/// Init cyber-beep
pub async fn initialize(
    Json(mut input): Json<InitInput>,
    Extension(conn): Extension<DbConn>,
) -> ApiResult<String> {
    input.validate()?;

    let count = Users::find().count(&conn).await?;

    if count != 0 {
        return Err(ServerError::ReInitializedError);
    }

    input.username.make_ascii_lowercase();
    input.email.make_ascii_lowercase();

    // todo password hash

    Ok(Payload::success(None))
}
