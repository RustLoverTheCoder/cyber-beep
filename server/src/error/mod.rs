use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use thiserror::Error;

use crate::domain::payload::Payload;
use axum::http::header::CONTENT_TYPE;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Wrong username or password")]
    UsernameOrPasswordError,

    #[error("Cannot be re-initialized")]
    ReInitializedError,

    #[error(transparent)]
    AxumTypedHeaderError(#[from] axum::extract::rejection::TypedHeaderRejection),

    #[error(transparent)]
    AxumExtensionError(#[from] axum::extract::rejection::ExtensionRejection),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    DatabaseError(#[from] sea_orm::DbErr),

    #[error(transparent)]
    InternalServerError(#[from] anyhow::Error),
}

pub type ApiResult<T> = anyhow::Result<Json<Payload<T>>, ServerError>;

impl IntoResponse for ServerError {
    type Body = Body;
    type BodyError = <Self::Body as axum::body::HttpBody>::Error;

    fn into_response(self) -> Response<Self::Body> {
        let (status_code, message) = match self {
            ServerError::UsernameOrPasswordError => {
                tracing::warn!("UsernameOrPasswordError");
                (StatusCode::UNAUTHORIZED, self.to_string())
            }
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{}]", self).replace("\n", ", ");
                tracing::debug!("{}", message);
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::ReInitializedError => (StatusCode::FORBIDDEN, self.to_string()),
            _ => {
                tracing::error!("Internal Server Error: [{}]", self.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
        };

        let payload = json!({ "error": message }).to_string();
        Response::builder()
            .header(CONTENT_TYPE, "application/json")
            .status(status_code)
            .body(Body::from(payload))
            .unwrap()
    }
}
