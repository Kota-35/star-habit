use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};

use super::claims::FirebaseClaims;
use super::error::AuthError;
use super::keys;

pub async fn verify_firebase_id_token(
    token: &str,
    project_id: &str,
) -> Result<FirebaseClaims, AuthError> {
    // 1) header 取得
    let header =
        decode_header(token).map_err(|_| AuthError::InvalidHeader)?;

    if header.alg != Algorithm::RS256 {
        return Err(AuthError::UnsupportedAlg);
    }

    let kid = header.kid.ok_or(AuthError::MissingKid)?;

    // 2) kid に対応する cert を取る（キャッシュ）
    let keys = keys::get_cached_keys()
        .await
        .map_err(|_| AuthError::KeyFetchFailed)?;
    let pem_cert = keys.get(&kid).ok_or(AuthError::UnknownKid)?;

    // 3) Validation 設定（aud / iss / exp）
    let issuer = format!("https://securetoken.google.com/{}", project_id);

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[project_id]);
    validation.set_issuer(&[issuer.as_str()]);
    validation.validate_exp = true;
    validation.leeway = 60; // 時計ズレ許容（秒）

    // 4) verify
    let decoding_key = DecodingKey::from_rsa_pem(pem_cert.as_bytes())
        .map_err(|_| AuthError::JwtVerifyFailed)?;

    let data = decode::<FirebaseClaims>(token, &decoding_key, &validation)
        .map_err(|_| AuthError::JwtVerifyFailed)?;

    // 5) 追加チェック（iat/auth_time/sub）
    let now = chrono::Utc::now().timestamp();

    if data.claims.sub.is_empty() || data.claims.sub.len() > 128 {
        return Err(AuthError::ClaimsInvalid("sub must be 1..=128 chars"));
    }
    if data.claims.iat > now + 60 {
        return Err(AuthError::ClaimsInvalid("iat must be in the past"));
    }
    if data.claims.auth_time > now + 60 {
        return Err(AuthError::ClaimsInvalid(
            "auth_time must be in the past",
        ));
    }

    Ok(data.claims)
}
