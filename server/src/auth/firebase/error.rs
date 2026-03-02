use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("missing bearer token")]
    MissingToken,
    #[error("invalid token header")]
    InvalidHeader,
    #[error("unsupported alg")]
    UnsupportedAlg,
    #[error("missing kid")]
    MissingKid,
    #[error("unknown kid")]
    UnknownKid,
    #[error("failed to fetch public keys")]
    KeyFetchFailed,
    #[error("jwt verify failed")]
    JwtVerifyFailed,
    #[error("claims validation failed: {0}")]
    ClaimsInvalid(&'static str),
}
