use serde::Deserialize;

/// Firebase ID トークン内の `firebase` クレーム（sign_in_provider 等）
#[derive(Debug, Clone, Deserialize)]
pub struct FirebasePayload {
    #[serde(rename = "sign_in_provider")]
    pub sign_in_provider: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirebaseClaims {
    pub aud: String,
    pub iss: String,
    /// uid
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub auth_time: i64,
    /// Firebase 固有クレーム（sign_in_provider 等）。トークンに含まれない場合もある
    #[serde(default)]
    pub firebase: Option<FirebasePayload>,
}
