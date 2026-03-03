use axum::{Router, routing::get};

use crate::{auth::middleware::RequireAuthLayer, routes::AppState};

pub mod auth;
pub mod me;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router())
        .route("/me", get(me::me))
        .layer(RequireAuthLayer)
}
