use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use uuid::Uuid;

use crate::models::user::User;

use super::claims::Claims;
use super::config::JwtConfig;
use super::kind::{Access, Refresh, TokenKind};

fn generate_token<K: TokenKind>(
    user: &User,
    cfg: &JwtConfig,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let iat = now.timestamp();
    let exp = (now + Duration::seconds(K::ttl_secs(cfg))).timestamp();

    let claims = Claims {
        id: user.id,
        firebase_uid: user.firebase_uid.clone(),
        token_type: K::token_type(),
        iss: cfg.issuer.clone(),
        aud: cfg.audience.clone(),
        sub: user.id,
        iat,
        exp,
        jti: Uuid::new_v4(),
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(cfg.secret.as_bytes()),
    )
}

pub fn generate_access_token(
    user: &User,
    cfg: &JwtConfig,
) -> Result<String, jsonwebtoken::errors::Error> {
    generate_token::<Access>(user, cfg)
}

pub fn generate_refresh_token(
    user: &User,
    cfg: &JwtConfig,
) -> Result<String, jsonwebtoken::errors::Error> {
    generate_token::<Refresh>(user, cfg)
}
