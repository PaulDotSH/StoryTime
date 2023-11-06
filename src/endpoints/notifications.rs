
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, query_as};
use crate::endpoints::common::{generate_token, get_username_from_header};
use crate::{error::AppError, user::Role, AppState, MAIL_CLIENT};
use anyhow::anyhow;
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::{extract::State, Json};
use chrono::{NaiveDateTime, Utc};
use sqlx::query;
use uuid::Uuid;

#[repr(i16)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Reply = 0,
    Canon = 1,
    NewLogin = 2,
    // Etc, to be added when needed
}

impl From<i16> for Kind {
    fn from(value: i16) -> Self {
        match value {
            0 => Kind::Reply,
            1 => Kind::Canon,
            2 => Kind::NewLogin,
            _ => Kind::Reply,
        }
    }
}

//TODO: Keep at most 50 notifications per user
pub async fn create_notification(user: &str, db: &Pool<Postgres>, kind: Kind, data: serde_json::Value) -> Result<(), AppError> {
        query!(
            r#"
                INSERT INTO notifications (users, kind, data)
                VALUES ($1, $2, $3)
                "#,
            user,
            kind as i16,
            data
        )
        .execute(db)
        .await?;

    Ok(())
}

pub async fn mark_notification(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(payload): Json<bool>,
) -> Result<StatusCode, AppError> {
    let username = get_username_from_header(&headers);

    query!(
        r#"
        UPDATE notifications SET read = $1 WHERE id = $2 AND users = $3
        "#,
        payload,
        id,
        username
    )
        .execute(&state.postgres)
        .await?;

    Ok(StatusCode::OK)
}

pub async fn mark_all_notifications(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<bool>,
) -> Result<StatusCode, AppError> {
    let username = get_username_from_header(&headers);

    query!(
        r#"
        UPDATE notifications SET read = $1 WHERE users = $2
        "#,
        payload,
        username
    )
        .execute(&state.postgres)
        .await?;

    Ok(StatusCode::OK)
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Notification {
    id: Uuid,
    kind: Kind,
    data: serde_json::Value,
    created: NaiveDateTime,
    read: bool
}

//TODO: Maybe add pagination based on timestamp?
pub async fn get_notifications(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Notification>, AppError> {
    let username = get_username_from_header(&headers);

    let notifications = query_as!(
        Notification,
        r#"
        SELECT id, kind, data, created, read FROM notifications WHERE users = $1 ORDER BY created DESC
        "#,
        username,
    ).fetch_one(&state.postgres).await?;

    Ok(Json(notifications))
}