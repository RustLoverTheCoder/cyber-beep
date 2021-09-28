#[macro_use]
extern crate serde;

use std::borrow::Cow;
use std::convert::Infallible;
use std::time::Duration;

use axum::handler::{get, post};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::BoxRoute;
use axum::{AddExtensionLayer, BoxError, Router};
use sea_orm::DbConn;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::handler::{
    index::{index, ping},
    users::initialize,
};

pub mod config;
mod domain;
mod error;
pub mod handler;
mod utils;

pub fn app(db: DbConn) -> Router<BoxRoute> {
    // Build middleware stack
    let middleware_stack = ServiceBuilder::new()
        .timeout(Duration::from_secs(10))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(db))
        .into_inner();
    // Build our application by composing routes
    Router::new()
        .route("/", get(index))
        .route("/ping", get(ping))
        .route("/init", post(initialize))
        .layer(middleware_stack)
        .handle_error(handle_error)
        .boxed()
}

fn handle_error(error: BoxError) -> Result<impl IntoResponse, Infallible> {
    if error.is::<tower::timeout::error::Elapsed>() {
        return Ok((StatusCode::REQUEST_TIMEOUT, Cow::from("Request Timeout")));
    }

    Ok((
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from("Internal Server Error"),
    ))
}
