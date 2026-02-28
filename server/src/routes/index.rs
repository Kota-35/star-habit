use axum::{extract::State, response::Html};

use crate::routes::AppState;

pub async fn index(State(_): State<AppState>) -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
