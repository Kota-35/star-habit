//! 自前 JWT（access / refresh トークン）の生成・検証モジュール。
//!
//! - [claims]: クレーム型・トークン種別
//! - [config]: JWT 設定
//! - [kind]: Access/Refresh マーカーと TokenKind
//! - [verified]: 検証済みトークンラッパー
//! - [generate]: トークン生成
//! - [verify]: トークン検証

mod claims;
mod config;
mod generate;
mod kind;
mod verified;
mod verify;

pub use claims::{Claims, TokenType};
pub use config::JwtConfig;
pub use generate::{generate_access_token, generate_refresh_token};
pub use verified::{VerifiedAccess, VerifiedRefresh};
pub use verify::{verify_access_token, verify_refresh_token};
