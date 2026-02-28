use server::{
    config::{env_vars, init_env},
    observability::telemetry::init_tracing,
    routes::build_router,
};

#[tokio::main]
async fn main() {
    init_env().expect("failed to load config from .env");

    init_tracing();

    let app = build_router().await.expect("Failed to build router");

    let addr: std::net::SocketAddr =
        (std::net::Ipv4Addr::UNSPECIFIED, env_vars().port).into();

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind failed");

    axum::serve(listener, app).await.expect("serve failed");
}
