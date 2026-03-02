use jsonwebtoken::TokenData;

use super::claims::Claims;

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
