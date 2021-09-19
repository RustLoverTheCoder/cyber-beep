use axum::handler::get;
use axum::Router;
use axum::routing::BoxRoute;

mod index;

pub fn routes() -> Router<BoxRoute> {
  Router::new()
      .route("/", get(index::index)).boxed()
      .route("/ping", get(index::ping)).boxed()
}