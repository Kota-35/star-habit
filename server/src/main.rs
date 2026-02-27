use axum::{Router, response::Html, routing::get};
use server::{
    config::{env_vars, init_env},
    observability::telemetry::init_tracing,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    init_env().expect("failed to load config from .env");

    init_tracing();

    let app = Router::new()
        .route("/", get(handler))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr: std::net::SocketAddr =
        (std::net::Ipv4Addr::UNSPECIFIED, env_vars().port).into();

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind failed");
    axum::serve(listener, app).await.expect("serve failed");
}

async fn handler() -> Html<&'static str> {
    info!("hello");
    Html("<h1>Hello, World!</h1>")
}
