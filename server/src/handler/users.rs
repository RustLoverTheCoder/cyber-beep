use axum::extract::Extension;

use sea_orm::{DbConn, EntityTrait};

use crate::domain::dto::InitInput;
use crate::domain::entity::prelude::Users;

use crate::domain::payload::Payload;
use crate::error::{ApiResult, ServerError};
use crate::extract::ValidatedJson;

/// Init cyber-beep
pub async fn initialize(
    ValidatedJson(mut input): ValidatedJson<InitInput>,
    Extension(conn): Extension<DbConn>,
) -> ApiResult<String> {
    let count = Users::find().count(&conn).await?;

    if count != 0 {
        return Err(ServerError::ReInitializedError);
    }

    input.username.make_ascii_lowercase();
    input.email.make_ascii_lowercase();

    // todo password hash

    Ok(Payload::success(None))
}
