use axum::{Router, routing::get};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::routes::{health::health, index::index};

mod api;
mod health;
mod index;

pub fn build_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .nest("/api", api::router())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}
