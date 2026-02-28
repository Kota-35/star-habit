use axum::{Router, routing::post};

use crate::routes::api::auth::signup::signup;
use crate::routes::AppState;

mod signup;

pub fn router() -> Router<AppState> {
    Router::new().route("/signup", post(signup))
}
