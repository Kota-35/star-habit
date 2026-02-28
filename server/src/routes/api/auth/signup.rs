use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
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
    pub id: Uuid,
    #[serde(rename = "firebaseUid")]
    pub firebase_uid: String,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<User> for SignupResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            firebase_uid: user.firebase_uid,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
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

    let profile = match profile {
        Ok(p) => p,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(None)),
    };

    if tx.commit().await.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
    }

    info!("[signup] user created \n{:#?}\n{:#?}", user, profile);
    (StatusCode::CREATED, Json(Some(SignupResponse::from(user))))
}
