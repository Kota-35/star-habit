use std::error::Error;

use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use x509_parser::pem::parse_x509_pem;
use x509_parser::parse_x509_certificate;
use x509_parser::public_key::PublicKey;

use super::claims::FirebaseClaims;
use super::error::AuthError;
use super::keys;

/// X.509 証明書 PEM から RSA 公開鍵（modulus, exponent）を取得する。
/// jsonwebtoken の from_rsa_der は rust_crypto 時に PKCS#1 を期待するため、
/// SPKI のまま渡すと InvalidSignature になる。modulus/exponent で渡すと確実。
fn rsa_public_key_from_pem_cert(
    pem_cert: &str,
) -> Result<(Vec<u8>, Vec<u8>), AuthError> {
    let (_, pem) = parse_x509_pem(pem_cert.as_bytes()).map_err(|e| {
        tracing::debug!("[verify_firebase_id_token] PEM parse error: {:?}", e);
        AuthError::JwtVerifyFailed
    })?;
    let (_, cert) = parse_x509_certificate(&pem.contents).map_err(|e| {
        tracing::debug!("[verify_firebase_id_token] X.509 parse error: {:?}", e);
        AuthError::JwtVerifyFailed
    })?;
    let pk = cert.public_key().parsed().map_err(|e| {
        tracing::debug!("[verify_firebase_id_token] public_key parse error: {:?}", e);
        AuthError::JwtVerifyFailed
    })?;
    match pk {
        PublicKey::RSA(rsa) => Ok((rsa.modulus.to_vec(), rsa.exponent.to_vec())),
        _ => {
            tracing::debug!("[verify_firebase_id_token] not an RSA key");
            Err(AuthError::JwtVerifyFailed)
        }
    }
}

pub async fn verify_firebase_id_token(
    token: &str,
    project_id: &str,
) -> Result<FirebaseClaims, AuthError> {
    // 1) header 取得
    let header =
        decode_header(token).map_err(|e| {
            tracing::error!(
                error = %e,
                token_len = token.len(),
                "[verify_firebase_id_token] decode_header failed"
            );
            AuthError::InvalidHeader
        })?;

    if header.alg != Algorithm::RS256 {
        return Err(AuthError::UnsupportedAlg);
    }

    let kid = header.kid.ok_or(AuthError::MissingKid)?;

    // 2) kid に対応する cert を取る（キャッシュ）
    let keys = keys::get_cached_keys()
        .await
        .map_err(|e| {
            tracing::error!("[verify_firebase_id_token] key fetch failed: {}", e);
            AuthError::KeyFetchFailed
        })?;
    let pem_cert = keys.get(&kid).ok_or_else(|| {
        tracing::error!(
            kid = %kid,
            project_id = %project_id,
            available_kids = ?keys.keys().collect::<Vec<_>>(),
            "[verify_firebase_id_token] unknown kid (key not in Firebase JWKS)"
        );
        AuthError::UnknownKid
    })?;

    // 3) X.509 証明書から RSA 公開鍵（modulus, exponent）を取得
    let (modulus, exponent) = rsa_public_key_from_pem_cert(pem_cert)?;

    // 4) Validation 設定（aud / iss / exp）
    // 本番トークンの iss は https://securetoken.google.com/<project_id>
    let issuer = format!("https://securetoken.google.com/{}", project_id);

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[project_id]);
    validation.set_issuer(&[issuer.as_str()]);
    validation.validate_exp = true;
    validation.leeway = 60; // 時計ズレ許容（秒）

    // 5) verify（raw components で渡すと SPKI/PKCS#1 の解釈差で InvalidSignature にならない）
    let decoding_key = DecodingKey::from_rsa_raw_components(&modulus, &exponent);

    let data = decode::<FirebaseClaims>(token, &decoding_key, &validation).map_err(|e| {
        let kind = e.kind();
        let hint = match kind {
            jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                " (Firebase 本番とエミュレータの不一致・トークンの二重エンコード・鍵形式の不一致の可能性)"
            }
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => " (トークン有効期限切れ)",
            jsonwebtoken::errors::ErrorKind::InvalidAudience => " (project_id またはトークンの aud 不一致)",
            jsonwebtoken::errors::ErrorKind::InvalidIssuer => {
                " (トークンの iss が一致しません。サーバー FIREBASE_PROJECT_ID とクライアントの Firebase projectId を同じにしてください)"
            },
            _ => "",
        };
        let expected_iss = format!("https://securetoken.google.com/{}", project_id);
        tracing::error!(
            error_kind = ?kind,
            error_message = %e,
            project_id = %project_id,
            expected_iss = %expected_iss,
            kid = %kid,
            token_len = token.len(),
            hint = hint,
            "[verify_firebase_id_token] jwt verify failed"
        );
        if let Some(source) = e.source() {
            tracing::debug!(
                source = %source,
                "[verify_firebase_id_token] jwt verify cause"
            );
        }
        AuthError::JwtVerifyFailed
    })?;

    // 6) 追加チェック（iat/auth_time/sub）
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
