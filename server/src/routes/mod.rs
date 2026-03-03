use axum::Router;
use axum::routing::get;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    config::env_vars,
    routes::{health::health, index::index, openapi::ApiDoc},
};

mod api;
mod health;
mod index;
pub mod openapi;

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
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .layer(CorsLayer::permissive())
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            ),
        )
        .with_state(state);

    Ok(router)
}
