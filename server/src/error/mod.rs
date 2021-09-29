use std::convert::Infallible;

use axum::body::{Bytes, Full};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use thiserror::Error;

use crate::domain::payload::Payload;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Cannot be re-initialized")]
    ReInitializedError,

    #[error(transparent)]
    AxumFormError(#[from] axum::extract::rejection::FormRejection),

    #[error(transparent)]
    AxumJsonError(#[from] axum::extract::rejection::JsonRejection),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    DatabaseError(#[from] sea_orm::DbErr),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

pub type ApiResult<T> = anyhow::Result<Json<Payload<T>>, ServerError>;

impl IntoResponse for ServerError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        let (status_code, message) = match self {
            Self::ValidationError(err) => {
                let message = format!("Input validation error: [{}]", err).replace("\n", ", ");
                tracing::debug!("{}", message);
                (StatusCode::BAD_REQUEST, message)
            }
            Self::AxumJsonError(err) => {
                let message = format!("{:?}", err);
                (err.into_response().status(), message)
            }
            Self::ReInitializedError => (StatusCode::FORBIDDEN, self.to_string()),
            _ => {
                tracing::error!("Internal Server Error: [{}]", self.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
            }
        };

        let payload = Json(json!({ "error": message }));
        (status_code, payload).into_response()
    }
}
