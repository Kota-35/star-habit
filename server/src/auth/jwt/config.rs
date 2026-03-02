pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub audience: String,
    pub access_ttl_secs: i64,
    pub refresh_ttl_secs: i64,
}
