use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Wrong username or password")]
    UsernameOrPasswordError,

    #[error(transparent)]
    AxumTypedHeaderError(#[from] axum::extract::rejection::TypedHeaderRejection),

    #[error(transparent)]
    AxumExtensionError(#[from] axum::extract::rejection::ExtensionRejection),

    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
}

pub type ApiResult<T> = anyhow::Result<T, ServerError>;

impl IntoResponse for ServerError {
    type Body = Body;
    type BodyError = <Self::Body as axum::body::HttpBody>::Error;

    fn into_response(self) -> Response<Self::Body> {
        let (status_code, body) = match self {
            ServerError::UsernameOrPasswordError => {
                tracing::warn!("UsernameOrPasswordError");
                (StatusCode::UNAUTHORIZED, Body::from(self.to_string()))
            }
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{}]", self).replace("\n", ", ");
                tracing::debug!("{}", message);
                (StatusCode::BAD_REQUEST, Body::from(message))
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Body::from("Internal Server Error"))
        };

        Response::builder().status(status_code).body(body).unwrap()
    }
}