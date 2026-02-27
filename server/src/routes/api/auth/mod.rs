use axum::{Router, routing::get};

use crate::routes::api::auth::signup::signup;

mod signup;

pub fn router() -> Router {
    Router::new().route("/signup", get(signup))
}
