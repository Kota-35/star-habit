use tracing_subscriber::{
    EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::config::{AppEnv, env_vars};

/// トレースを初期化する。main の早い段階で 1 回だけ呼ぶ。
/// ログが出ない場合は .env の RUST_LOG を確認すること（未設定なら info 以上が有効）。
pub fn init_tracing() {
    let default_filter = "server=info,tower_http=info,info";
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| default_filter.into());

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

    tracing::info!("tracing initialized (server + tower_http)");
}
