use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::routes::AppState;

#[derive(Deserialize)]
pub struct Signup {
    username: String,

    email: String,

    #[serde(rename = "firebaseUid")]
    firebase_uid: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub firebase_uid: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Profile {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: Uuid,
}

pub async fn signup(
    State(ctx): State<AppState>,
    Json(input): Json<Signup>,
) -> (StatusCode, Json<Option<User>>) {
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
    (StatusCode::CREATED, Json(Some(user)))
}
