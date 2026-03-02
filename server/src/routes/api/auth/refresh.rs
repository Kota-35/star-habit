use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

use crate::{
    auth::jwt::{
        JwtConfig, VerifiedRefresh, generate_access_token,
        generate_refresh_token, verify_refresh_token,
    },
    config::env_vars,
    models::user::User,
    routes::AppState,
};

#[derive(Deserialize, ToSchema)]
pub struct RefreshRequest {
    /// 再発行に使う refresh トークン（ボディで送信）
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct RefreshResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,

    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

/// refresh トークンで新しい access/refresh トークンのペアを再発行する
#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "OK", body = RefreshResponse),
        (status = 401, description = "Unauthorized (invalid or expired refreshToken, or user not found)"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn refresh(
    State(ctx): State<AppState>,
    Json(input): Json<RefreshRequest>,
) -> (StatusCode, Json<Option<RefreshResponse>>) {
    let cfg = JwtConfig {
        secret: env_vars().jwt_secret.clone(),
        issuer: env_vars().jwt_issuer.clone(),
        audience: env_vars().jwt_audience.clone(),
        access_ttl_secs: 60 * 15,
        refresh_ttl_secs: 60 * 60 * 24 * 30,
    };

    let VerifiedRefresh(token_data) =
        match verify_refresh_token(&input.refresh_token, &cfg) {
            Ok(v) => v,
            Err(_) => return (StatusCode::UNAUTHORIZED, Json(None)),
        };

    let user_id = token_data.claims.id;

    let user = match sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1",
    )
    .bind(&user_id)
    .fetch_one(&ctx.db_pool)
    .await
    {
        Ok(u) => u,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(None)),
    };

    let access_token = match generate_access_token(&user, &cfg) {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    };
    let refresh_token = match generate_refresh_token(&user, &cfg) {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    };

    (
        StatusCode::OK,
        Json(Some(RefreshResponse {
            access_token,
            refresh_token,
        })),
    )
}
