use axum::{Router, routing::get};

use crate::{auth::middleware::RequireAuthLayer, routes::AppState};

pub mod auth;
pub mod me;

pub fn router() -> Router<AppState> {
    let public = Router::new().nest("/auth", auth::router());
    let protected = Router::new()
        .route("/me", get(me::me))
        .layer(RequireAuthLayer);
    public.merge(protected)
}
