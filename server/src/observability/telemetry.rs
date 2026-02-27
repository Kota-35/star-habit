use tracing_subscriber::{
    EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::config::{AppEnv, env_vars};

pub fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,tower_http=info".into());

    let format = match env_vars().app_env == AppEnv::Development {
        true => fmt::layer()
            .compact()
            .with_timer(())
            .with_target(false)
            .boxed(),
        false => fmt::layer().json().with_timer(()).boxed(),
    };

    tracing_subscriber::registry()
        .with(env_filter)
        .with(format)
        .init();
}
