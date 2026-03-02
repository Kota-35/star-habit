use serde::Serialize;
use uuid::Uuid;

/// 認証プロバイダ。PostgreSQL の auth_provider_id ENUM と対応。
#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize)]
#[sqlx(type_name = "auth_provider_id")]
pub enum AuthProviderId {
    #[sqlx(rename = "password")]
    #[serde(rename = "password")]
    Password,
    #[sqlx(rename = "phone")]
    #[serde(rename = "phone")]
    Phone,
    #[sqlx(rename = "google.com")]
    #[serde(rename = "google.com")]
    GoogleCom,
    #[sqlx(rename = "facebook.com")]
    #[serde(rename = "facebook.com")]
    FacebookCom,
}

impl AuthProviderId {
    /// Firebase ID トークンの `firebase.sign_in_provider` を DB 用の enum に変換する。
    /// 未対応のプロバイダは `None`。
    pub fn from_firebase_sign_in_provider(s: &str) -> Option<Self> {
        match s {
            "password" => Some(Self::Password),
            "phone" => Some(Self::Phone),
            "google.com" => Some(Self::GoogleCom),
            "facebook.com" => Some(Self::FacebookCom),
            _ => None,
        }
    }
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct AuthMethod {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider_id: AuthProviderId,
    pub provider_uid: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
