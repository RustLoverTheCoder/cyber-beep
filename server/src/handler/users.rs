use axum::extract::Extension;
use axum::Json;
use sea_orm::DbConn;
use validator::Validate;

use crate::domain::dto::InitInput;
use crate::error::ApiResult;
use crate::error::Result;

fn validate_payload<T: Validate>(payload: &T) -> Result<()> {
    Ok(payload.validate()?)
}

/// Init cyber-beep
pub async fn initialize(
    Json(input): Json<InitInput>,
    Extension(_): Extension<DbConn>) -> ApiResult<()> {
    validate_payload(&input)?;
    Ok(())
}