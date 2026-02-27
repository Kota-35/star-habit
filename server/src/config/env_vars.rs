use envy;

use serde::Deserialize;
use std::sync::OnceLock;

/// 環境変数から読み込む設定。.env のキーと対応させる。
#[derive(Debug, Deserialize)]
pub struct Env {
    #[serde(default = "default_port")]
    pub port: u16,

    /// Firebase Project ID（ID トークン検証に使用）
    pub firebase_project_id: String,
}

fn default_port() -> u16 {
    4000
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
