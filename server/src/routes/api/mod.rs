use axum::Router;

use crate::routes::AppState;

pub mod auth;

pub fn router() -> Router<AppState> {
    Router::new().nest("/auth", auth::router())
}
