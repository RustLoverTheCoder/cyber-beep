use axum::extract::Extension;
use axum::Json;
use sea_orm::DbConn;
use validator::Validate;

use crate::domain::dto::InitInput;
use crate::error::ApiResult;

/// Init cyber-beep
pub async fn initialize(Json(input): Json<InitInput>, Extension(_): Extension<DbConn>) -> ApiResult<()> {
    input.validate()?;
    // Check
    Ok(())
}