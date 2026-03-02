use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;

use crate::{
    auth::{
        firebase::verify_firebase_id_token,
        jwt::{JwtConfig, generate_access_token, generate_refresh_token},
    },
    config::env_vars,
    models::{
        auth_method::{AuthMethod, AuthProviderId},
        profile::Profile,
        user::User,
    },
    routes::AppState,
};

#[derive(Deserialize, ToSchema)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    /// Firebase Auth で発行された ID トークン（ボディで送信）
    #[serde(rename = "idToken")]
    pub id_token: String,
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
        (status = 400, description = "Bad Request (unsupported or missing sign_in_provider in Firebase ID token)"),
        (status = 401, description = "Unauthorized (missing or invalid idToken in request body)"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn signup(
    State(ctx): State<AppState>,
    Json(input): Json<SignupRequest>,
) -> (StatusCode, Json<Option<SignupResponse>>) {
    let claims = match verify_firebase_id_token(
        &input.id_token,
        &env_vars().firebase_project_id,
    )
    .await
    {
        Ok(c) => c,
        Err(_error) => {
            // 詳細は verify_firebase_id_token 内で既に error レベルでログ済み
            return (StatusCode::UNAUTHORIZED, Json(None));
        }
    };

    let provider_uid = claims.sub.clone();
    let provider_id = match claims
        .firebase
        .as_ref()
        .and_then(|f| f.sign_in_provider.as_deref())
        .and_then(AuthProviderId::from_firebase_sign_in_provider)
    {
        Some(id) => id,
        None => {
            tracing::warn!(
                "[signup] unsupported or missing sign_in_provider in token (firebase.sign_in_provider)"
            );
            return (StatusCode::BAD_REQUEST, Json(None));
        }
    };

    let user = {
        let mut tx = match ctx.db_pool.begin().await {
            Ok(t) => t,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
            }
        };

        let user = match sqlx::query_as::<_, User>(
            "INSERT INTO users DEFAULT VALUES RETURNING *",
        )
        .fetch_one(&mut *tx)
        .await
        {
            Ok(u) => u,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
            }
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

        let _auth_method = match sqlx::query_as::<_, AuthMethod>(
            "INSERT INTO auth_methods (user_id, provider_id, provider_uid) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(&user.id)
        .bind(provider_id)
        .bind(&provider_uid)
        .fetch_one(&mut *tx)
        .await
        {
            Ok(m) => m,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
            }
        };

        if tx.commit().await.is_err() {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
        }

        user
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
        Json(Some(SignupResponse {
            access_token,
            refresh_token,
        })),
    )
}
