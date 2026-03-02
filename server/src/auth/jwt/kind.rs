use super::claims::TokenType;
use super::config::JwtConfig;

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
