use envy;

use serde::Deserialize;
use std::{fmt::Display, sync::OnceLock};

/// アプリの実行環境。環境変数 APP_ENV の値で指定する。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppEnv {
    Development,
    Staging,
    Production,
}

impl Display for AppEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppEnv::Development => write!(f, "development"),
            AppEnv::Staging => write!(f, "staging"),
            AppEnv::Production => write!(f, "production"),
        }
    }
}

fn default_port() -> u16 {
    4000
}

/// 環境変数から読み込む設定。.env のキーと対応させる。
#[derive(Debug, Deserialize)]
pub struct Env {
    #[serde(default = "default_port")]
    pub port: u16,

    /// Firebase Project ID（ID トークン検証に使用）
    pub firebase_project_id: String,

    pub app_env: AppEnv,

    pub database_url: String,
}

static ENV: OnceLock<Env> = OnceLock::new();

/// 設定を初期化する。main の最初で 1 回だけ呼ぶ。
/// .env を読み、envy で Env に deserialize してから CONFIG に格納する。
pub fn init_env() -> Result<&'static Env, envy::Error> {
    dotenvy::dotenv().ok();

    let env = envy::from_env::<Env>()?;
    let _ = ENV.set(env);
    Ok(ENV.get().unwrap())
}

/// どこからでも参照する用。init_config() を先に呼んでおくこと。
pub fn env_vars() -> &'static Env {
    ENV.get().expect(
        "config not initialized: call env::init_config() at startup",
    )
}
