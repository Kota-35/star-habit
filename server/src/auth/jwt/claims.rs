use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
