use crate::endpoints::common::*;
use crate::user::Role::User;
use crate::{error::AppError, user::Role, AppState};
use anyhow::anyhow;
use axum::extract::{Path, Query};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect, Response};
use axum::{extract::State, Json};
use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, query_scalar};
use uuid::Uuid;
use crate::endpoints::story_snippet::PostStorySnippet;

#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    perm: Role,
    score: i32,
    comments: Vec<UserProfileComment>,
    snippets: Vec<UserProfileSnippet>
}

#[derive(Serialize, Deserialize)]
pub struct UserProfileComment {
    id: Uuid,
    body: String,
    created: NaiveDateTime,
    modified: Option<NaiveDateTime>,
    score: i32,
    snippet: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct UserProfileSnippet {
    id: Uuid,
    body: String,
    created: NaiveDateTime,
    modified: Option<NaiveDateTime>,
    is_final: bool,
    score: i32,
    parent: Option<Uuid>,
    place: String,
}


pub async fn get_user_profile(
    Path(username): Path<String>,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<UserProfile>, AppError> {
    let user_row = query!(
        r#"
        SELECT perm, score FROM users WHERE username = $1;
        "#,
        username,
    ).fetch_one(&state.postgres).await?;

    let comments = query_as!(
        UserProfileComment,
        r#"
        SELECT body, id, created, modified, score, snippet FROM comments WHERE writer = $1 ORDER BY created ASC LIMIT 5;
        "#,
        username,
    ).fetch_all(&state.postgres).await?;

    let snippets = query_as!(
        UserProfileSnippet,
        r#"
        SELECT id, body, created, modified, is_final, score, parent, place FROM story_parts WHERE writer = $1 ORDER BY created ASC LIMIT 5;
        "#,
        username,
    ).fetch_all(&state.postgres).await?;

    let user = UserProfile {
        perm: Role::from(user_row.perm),
        score: user_row.score,
        comments,
        snippets,
    };

    Ok(Json(user))
}