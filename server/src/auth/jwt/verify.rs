use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};

use super::claims::Claims;
use super::config::JwtConfig;
use super::kind::{Access, Refresh, TokenKind};
use super::verified::{VerifiedAccess, VerifiedRefresh};

fn verify_token<K: TokenKind>(
    token: &str,
    cfg: &JwtConfig,
) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
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
