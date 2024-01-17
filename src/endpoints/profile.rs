use crate::{error::AppError, user::Role, AppState};
use axum::extract::Path;
use axum::{extract::State, Json};
use axum::http::HeaderMap;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use uuid::Uuid;
use crate::endpoints::common::get_username_from_header;

#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    perm: Role,
    score: i32,
    comments: Vec<UserProfileComment>,
    snippets: Vec<UserProfileSnippet>,
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

#[derive(Serialize, Deserialize)]
pub struct CurrentUserProfile {
    username: String,
    email: String,
    perm: Role,
    score: i32,
    pp: i32
}

pub async fn get_current_user_profile(
    State(state): State<AppState>,
    headers: HeaderMap
) -> Result<Json<CurrentUserProfile>, AppError> {
    let username = get_username_from_header(&headers); // This cannot fail since we set it in middleware
    let user_row = query!(
        r#"
        SELECT email, perm, score, pp FROM users WHERE username = $1;
        "#,
        username,
    )
        .fetch_one(&state.postgres)
        .await?;

    Ok(Json(CurrentUserProfile{
        username: username.to_string(),
        email: user_row.email,
        perm: Role::from(user_row.perm),
        score: user_row.score,
        pp: user_row.pp,
    }))
}

pub async fn get_user_profile(
    Path(username): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<UserProfile>, AppError> {
    let user_row = query!(
        r#"
        SELECT perm, score FROM users WHERE username = $1;
        "#,
        username,
    )
        .fetch_one(&state.postgres)
        .await?;

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
