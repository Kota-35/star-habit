use axum::Router;

use crate::routes::AppState;

mod auth;

pub fn router() -> Router<AppState> {
    Router::new().nest("/auth", auth::router())
}
