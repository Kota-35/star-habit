use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    auth::{
        firebase::verify_firebase_id_token,
        jwt::{JwtConfig, generate_access_token, generate_refresh_token},
    },
    config::env_vars,
    models::user::User,
    routes::AppState,
};

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct SigninRequest {
    /// Firebase Auth で発行された ID トークン（ボディで送信）
    #[serde(rename = "idToken")]
    pub id_token: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct SigninResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,

    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

/// 既存ユーザーでサインインし、JWT の access/refresh トークンを返す
#[utoipa::path(
    post,
    path = "/api/auth/signin",
    request_body = SigninRequest,
    responses(
        (status = 201, description = "Created", body = SigninResponse),
        (status = 401, description = "Unauthorized (invalid or expired idToken)"),
        (status = 404, description = "Not Found (user not registered; sign up first)"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn signin(
    State(ctx): State<AppState>,
    Json(input): Json<SigninRequest>,
) -> (StatusCode, Json<Option<SigninResponse>>) {
    let claims = match verify_firebase_id_token(
        &input.id_token,
        &env_vars().firebase_project_id,
    )
    .await
    {
        Ok(c) => c,
        Err(_) => {
            return (StatusCode::UNAUTHORIZED, Json(None));
        }
    };

    let provider_uid = claims.sub.clone();

    let user = match sqlx::query_as::<_, User>(
        r#"
            SELECT 
                users.*
            FROM users 
            INNER JOIN auth_methods 
                ON users.id = auth_methods.user_id
            WHERE
                auth_methods.provider_uid = $1
        "#,
    )
    .bind(&provider_uid)
    .fetch_one(&ctx.db_pool)
    .await
    {
        Ok(u) => u,
        Err(error) => {
            if let sqlx::Error::RowNotFound = error {
                return (StatusCode::NOT_FOUND, Json(None));
            }
            tracing::error!("[signin] {}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
        }
    };

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
        Json(Some(SigninResponse {
            access_token,
            refresh_token,
        })),
    )
}
