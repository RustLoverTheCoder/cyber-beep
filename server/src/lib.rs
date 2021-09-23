#[macro_use]
extern crate serde;

use std::borrow::Cow;
use std::convert::Infallible;
use std::time::Duration;

use axum::{AddExtensionLayer, BoxError, Router};
use axum::handler::get;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::BoxRoute;
use sea_orm::DbConn;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::handler::index::{index, ping};

pub mod config;
pub mod handler;
mod entity;
mod error;

pub fn app(db: DbConn) -> Router<BoxRoute> {
    let middleware_stack = ServiceBuilder::new()
        .timeout(Duration::from_secs(10))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(db))
        .into_inner();
    // Build our application by composing routes
    Router::new()
        .route("/", get(index))
        .route("/ping", get(ping))
        .layer(middleware_stack)
        .handle_error(handle_error)
        .boxed()
}

fn handle_error(error: BoxError) -> Result<impl IntoResponse, Infallible> {
    if error.is::<tower::timeout::error::Elapsed>() {
        return Ok((StatusCode::REQUEST_TIMEOUT, Cow::from("Request Timeout")));
    }

    Ok((StatusCode::INTERNAL_SERVER_ERROR, Cow::from("Internal Server Error")))
}