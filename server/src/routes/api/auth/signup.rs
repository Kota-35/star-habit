use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

use crate::{
    auth::jwt::{
        JwtConfig, generate_access_token, generate_refresh_token,
    },
    config::env_vars,
    models::{profile::Profile, user::User},
    routes::AppState,
};

#[derive(Deserialize, ToSchema)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    #[serde(rename = "firebaseUid")]
    pub firebase_uid: String,
}

/// Signup API のレスポンス用 DTO
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct SignupResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,

    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

/// 新規ユーザーとプロフィールを登録する
#[utoipa::path(
    post,
    path = "/api/auth/signup",
    request_body = SignupRequest,
    responses(
        (status = 201, description = "Created", body = SignupResponse),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn signup(
    State(ctx): State<AppState>,
    Json(input): Json<SignupRequest>,
) -> (StatusCode, Json<Option<SignupResponse>>) {
    let mut tx = match ctx.db_pool.begin().await {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    };

    let user = match sqlx::query_as::<_, User>(
        "INSERT INTO users (firebase_uid) VALUES ($1) RETURNING *",
    )
    .bind(&input.firebase_uid)
    .fetch_one(&mut *tx)
    .await
    {
        Ok(u) => u,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    };

    let profile = sqlx::query_as::<_, Profile>(
        "INSERT INTO profiles (username, email, user_id) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(&input.username)
    .bind(&input.email)
    .bind(&user.id)
    .fetch_one(&mut *tx)
    .await;

    if profile.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
    };

    if tx.commit().await.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
    }

    let jwt_config = JwtConfig {
        secret: env_vars().jwt_secret.clone(),
        issuer: env_vars().jwt_issuer.clone(),
        audience: env_vars().jwt_audience.clone(),
        access_ttl_secs: 60 * 15,            // 15分
        refresh_ttl_secs: 60 * 60 * 24 * 30, // 30日
    };

    let access_token = match generate_access_token(&user, &jwt_config) {
        Ok(token) => token,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    };

    let refresh_token = match generate_refresh_token(&user, &jwt_config) {
        Ok(token) => token,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    };

    (
        StatusCode::CREATED,
        Json(Some(SignupResponse {
            access_token,
            refresh_token,
        })),
    )
}
