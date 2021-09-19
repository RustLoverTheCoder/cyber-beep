use std::borrow::Cow;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    AddExtensionLayer,
    handler::get,
    http::StatusCode,
    response::IntoResponse,
    Router,
    routing::BoxRoute
};
use sea_orm::DatabaseConnection;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;

use index::{index, ping};

mod index;

pub type SharedState = Arc<State>;

pub struct State {
   pub db: DatabaseConnection,
}

impl State {
    fn build(db: DatabaseConnection) -> State {
        State {
            db
        }
    }
}

pub fn routes(db: DatabaseConnection) -> Router<BoxRoute> {
    // Build our application by composing routes
    Router::new()
        .route("/", get(index))
        .route("/ping", get(ping))
        // Add middleware to all routes
        .layer(ServiceBuilder::new()
                   .timeout(Duration::from_secs(10))
                   .layer(TraceLayer::new_for_http())
                   .layer(AddExtensionLayer::new(Arc::new(State::build(db))))
                   .into_inner(),
        )
        // Handle errors from middleware
        .handle_error(handle_error)
        .check_infallible()
        .boxed()
}

fn handle_error(error: BoxError) -> Result<impl IntoResponse, Infallible> {
    if error.is::<tower::timeout::error::Elapsed>() {
        return Ok((StatusCode::REQUEST_TIMEOUT, Cow::from("Request Timeout")));
    }

    Ok((
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Internal Server Error: {}", error)),
    ))
}