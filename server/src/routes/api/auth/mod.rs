use axum::{Router, routing::post};

use crate::routes::AppState;
use crate::routes::api::auth::signin::signin;
use crate::routes::api::auth::signup::signup;

pub mod signin;
pub mod signup;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/signup", post(signup))
        .route("/signin", post(signin))
}
