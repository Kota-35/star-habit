//! Firebase ID トークン検証モジュール。
//!
//! - [claims]: JWT クレーム型
//! - [error]: 認証エラー型
//! - [keys]: 公開鍵取得・キャッシュ
//! - [verify]: トークン検証 API

mod claims;
mod error;
mod keys;
mod verify;

pub use claims::FirebaseClaims;
pub use error::AuthError;
pub use verify::verify_firebase_id_token;
