use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FirebaseClaims {
    pub aud: String,
    pub iss: String,
    /// uid
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub auth_time: i64,
}
