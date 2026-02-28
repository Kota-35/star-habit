use axum::{Router, routing::get};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::{
    config::env_vars,
    routes::{health::health, index::index},
};

mod api;
mod health;
mod index;

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}

pub async fn build_router() -> Result<Router, sqlx::Error> {
    let db_pool = PgPool::connect(&env_vars().database_url).await?;

    let state = AppState { db_pool };

    let router = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .nest("/api", api::router())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(state);

    Ok(router)
}
