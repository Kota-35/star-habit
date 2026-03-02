use chrono::{Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
    decode, encode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::user::User;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    // Custom Claims
    pub id: uuid::Uuid,
    pub firebase_uid: String,
    pub token_type: TokenType,

    // --- registered (standard) claims ---
    pub iss: String,
    pub aud: String,
    pub sub: uuid::Uuid,
    pub iat: i64,
    pub exp: i64,
    pub jti: Uuid,
}

pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub audience: String,
    pub access_ttl_secs: i64,
    pub refresh_ttl_secs: i64,
}

#[derive(Clone, Copy, Debug)]
pub struct Access;

#[derive(Clone, Copy, Debug)]
pub struct Refresh;

pub trait TokenKind {
    fn token_type() -> TokenType;
    fn ttl_secs(cfg: &JwtConfig) -> i64;
}

impl TokenKind for Access {
    fn token_type() -> TokenType {
        TokenType::Access
    }
    fn ttl_secs(cfg: &JwtConfig) -> i64 {
        cfg.access_ttl_secs
    }
}

impl TokenKind for Refresh {
    fn token_type() -> TokenType {
        TokenType::Refresh
    }
    fn ttl_secs(cfg: &JwtConfig) -> i64 {
        cfg.refresh_ttl_secs
    }
}

/// access 用に検証されたトークン
#[derive(Debug)]
pub struct VerifiedAccess(pub TokenData<Claims>);

impl VerifiedAccess {
    pub fn claims(&self) -> &Claims {
        &self.0.claims
    }
}

/// refresh 用に検証されたトークン
#[derive(Debug)]
pub struct VerifiedRefresh(pub TokenData<Claims>);

impl VerifiedRefresh {
    pub fn claims(&self) -> &Claims {
        &self.0.claims
    }
}

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

fn verify_token<K: TokenKind>(
    token: &str,
    cfg: &JwtConfig,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[cfg.issuer.clone()]);
    validation.set_audience(&[cfg.audience.clone()]);

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(cfg.secret.as_bytes()),
        &validation,
    )?;

    if data.claims.token_type != K::token_type() {
        return Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken,
        ));
    }

    Ok(data)
}

pub fn verify_access_token(
    token: &str,
    cfg: &JwtConfig,
) -> Result<VerifiedAccess, jsonwebtoken::errors::Error> {
    verify_token::<Access>(token, cfg).map(VerifiedAccess)
}

pub fn verify_refresh_token(
    token: &str,
    cfg: &JwtConfig,
) -> Result<VerifiedRefresh, jsonwebtoken::errors::Error> {
    verify_token::<Refresh>(token, cfg).map(VerifiedRefresh)
}
