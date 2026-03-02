use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use once_cell::sync::Lazy;
use tokio::sync::RwLock;

static KEY_CACHE: Lazy<RwLock<KeyCache>> =
    Lazy::new(|| RwLock::new(KeyCache::empty()));

#[derive(Debug, Clone)]
struct KeyCache {
    /// kid -> PEM cert string
    keys: HashMap<String, String>,
    expires_at: Instant,
}

impl KeyCache {
    fn empty() -> Self {
        Self {
            keys: HashMap::new(),
            expires_at: Instant::now(), // すぐ期限切れ扱いにする
        }
    }

    fn is_fresh(&self) -> bool {
        Instant::now() < self.expires_at && !self.keys.is_empty()
    }
}

pub(super) async fn fetch_firebase_public_keys()
-> Result<(HashMap<String, String>, Duration), reqwest::Error> {
    // Firebase の公開鍵エンドポイント
    const URL: &str = "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";

    let resp = reqwest::Client::new()
        .get(URL)
        .send()
        .await?
        .error_for_status()?;

    // Cache-Control: public, max-age=XXXX
    let max_age = resp
        .headers()
        .get(reqwest::header::CACHE_CONTROL)
        .and_then(|v| v.to_str().ok())
        .and_then(parse_max_age_seconds)
        .unwrap_or(3600); // 取れなければ控えめに1hにする

    let keys: HashMap<String, String> = resp.json().await?;
    Ok((keys, Duration::from_secs(max_age)))
}

fn parse_max_age_seconds(cache_control: &str) -> Option<u64> {
    cache_control
        .split(',')
        .map(|s| s.trim())
        .find_map(|part| part.strip_prefix("max-age="))
        .and_then(|v| v.parse::<u64>().ok())
}

pub(crate) async fn get_cached_keys()
-> Result<HashMap<String, String>, reqwest::Error> {
    {
        let cache = KEY_CACHE.read().await;
        if cache.is_fresh() {
            return Ok(cache.keys.clone());
        }
    }

    // 期限切れ -> 取得して更新
    let (keys, ttl) = fetch_firebase_public_keys().await?;
    let mut cache = KEY_CACHE.write().await;

    // TTLより少し早めに切らす
    let safety_margin = Duration::from_secs(30);
    let ttl = ttl.saturating_sub(safety_margin);

    cache.keys = keys.clone();
    cache.expires_at = Instant::now() + ttl;
    Ok(keys)
}
