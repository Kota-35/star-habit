use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::auth::middleware::AuthenticatedUser;
use crate::routes::AppState;

/// /me のレスポンス。users.id と profiles の username / email を返す。
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MeResponse {
    #[serde(rename = "userId")]
    pub user_id: Uuid,

    pub username: String,

    pub email: String,
}

/// 認証中のユーザー自身の情報を返す。要 Bearer トークン（access token）。
#[utoipa::path(
    get,
    path = "/api/me",
    responses(
        (status = 200, description = "OK", body = MeResponse),
        (status = 401, description = "Unauthorized (missing or invalid access token)"),
        (status = 404, description = "Not Found (profile not found)"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn me(
    user: AuthenticatedUser,
    State(ctx): State<AppState>,
) -> Response {
    let row = sqlx::query_as::<_, (Uuid, String, String)>(
        "SELECT user_id, username, email FROM profiles WHERE user_id = $1",
    )
    .bind(user.user_id)
    .fetch_optional(&ctx.db_pool)
    .await;

    match row {
        Ok(Some((user_id, username, email))) => Json(MeResponse {
            user_id,
            username,
            email,
        })
        .into_response(),
        Ok(None) => {
            (StatusCode::NOT_FOUND, "Profile not found").into_response()
        }
        Err(e) => {
            tracing::error!("[me] {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response()
        }
    }
}
