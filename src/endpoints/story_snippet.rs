use crate::endpoints::common::*;
use crate::user::Role::User;
use crate::{error::AppError, AppState};
use anyhow::anyhow;
use axum::extract::{Path, Query};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{extract::State, Json};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, query_scalar};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct PostStorySnippet {
    body: String,
    place: String,
}

//TODO: Make a way to continue a story snippet
//TODO: Place systems needs implementation here too
//TODO: Think of a way to implement the story snippet reading api
//Idea atm, frontend queries the post, you show all the snippets related to it, and add filtering at a later stage
//Comments and replies will be added later and loaded separately
pub async fn new_story_snippet(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<PostStorySnippet>,
) -> Result<Response, AppError> {
    let username = get_username_from_header(&headers); // This cannot fail since we set it in middleware
    let role = get_role_from_header(&headers); // This cannot fail since we set it in middleware

    if role < User {
        return Err(AppError(anyhow!("You do not have permission to post a story snippet, make sure you confirmed your email and are not banned!")));
    }

    let id: Uuid = query_scalar!(
        r#"
        INSERT INTO story_parts(body, writer, child_cannon_time, place) VALUES ($1, $2, NOW() + INTERVAL '24 hours', $3) RETURNING id;
        "#,
        payload.body,
        username,
        payload.place
    )
    .fetch_one(&state.postgres)
    .await?;

    Ok(id.to_string().into_response())
}

#[derive(Serialize, Deserialize)]
pub struct GetStory {
    body: String,
}

#[derive(Deserialize)]
pub struct StoryPagination {
    last_score: i32,
}
impl Default for StoryPagination {
    fn default() -> Self {
        Self {
            last_score: i32::MAX,
        }
    }
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct StorySnippet {
    id: Uuid,
    writer: String,
    body: String,
    created: NaiveDateTime,
    modified: Option<NaiveDateTime>,
    child_cannon_time: Option<NaiveDateTime>,
    parent: Option<Uuid>,
    score: i32,
    is_final: bool,
    child: Option<Uuid>,
}

//TODO: Get comments too when the system is implemented
pub async fn get_story(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<StorySnippet>, AppError> {
    let parent = query_as!(
        StorySnippet,
        r#"
        SELECT id, writer, body, created, modified, child_cannon_time, parent, child, score, is_final FROM story_parts WHERE id = $1
        "#,
        id,
    ).fetch_one(&state.postgres).await?;

    Ok(Json(parent))
}

//TODO: Get comments too when the system is implemented
pub async fn get_story_children(
    Path(id): Path<Uuid>,
    pagination: Option<Query<StoryPagination>>,
    State(state): State<AppState>,
) -> Result<Json<Vec<StorySnippet>>, AppError> {
    let Query(pagination) = pagination.unwrap_or_default();

    let stories = query_as!(
        StorySnippet,
        r#"
        SELECT id, writer, body, created, modified, child_cannon_time, parent, child, score, is_final FROM story_parts WHERE parent = $1 AND score < $2
        ORDER BY score DESC, id
        LIMIT 5;
        "#,
        id,
        pagination.last_score
    ).fetch_all(&state.postgres).await?;

    Ok(Json(stories))
}

#[repr(i16)]
#[derive(Deserialize, Serialize, Debug)]
pub enum Vote {
    Up = 1,
    Down = -1,
}

pub async fn vote_snippet(
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(payload): Json<Vote>,
) -> Result<StatusCode, AppError> {
    let role = get_role_from_header(&headers);
    if role < User {
        return Err(AppError(anyhow!("You do not have permission")));
    }

    let username = get_username_from_header(&headers); // This cannot fail since we set it in middleware

    sqlx::query!(
        "INSERT INTO snippet_votes (users, snippet, vote_type) VALUES ($1, $2, $3)",
        username,
        id,
        payload as i16
    )
    .execute(&state.postgres)
    .await?;

    Ok(StatusCode::OK)
}

pub async fn remove_vote(
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<StatusCode, AppError> {
    let role = get_role_from_header(&headers);
    if role < User {
        return Err(AppError(anyhow!("You do not have permission")));
    }

    let username = get_username_from_header(&headers); // This cannot fail since we set it in middleware

    sqlx::query!(
        "DELETE FROM snippet_votes WHERE users = $1 AND snippet = $2",
        username,
        id
    )
    .execute(&state.postgres)
    .await?;

    Ok(StatusCode::OK)
}

#[derive(Serialize, Deserialize)]
pub struct PostStorySnippetContinuation {
    body: String,
    is_final: bool,
}

#[axum::debug_handler]
pub async fn new_story_snippet_continuation(
    Path(parent): Path<Uuid>,
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<PostStorySnippetContinuation>,
) -> Result<Response, AppError> {
    let username = get_username_from_header(&headers); // This cannot fail since we set it in middleware
    let role = get_role_from_header(&headers); // This cannot fail since we set it in middleware

    if role < User {
        return Err(AppError(anyhow!("You do not have permission to post a story snippet, make sure you confirmed your email and are not banned!")));
    }

    let parent_record = sqlx::query!(
        r#"
        SELECT index, is_final FROM story_parts WHERE id = $1;
        "#,
        parent
    )
    .fetch_one(&state.postgres)
    .await?;

    if parent_record.is_final {
        return Err(AppError(anyhow!("Parent is already final snippet.")));
    }

    let new_index = parent_record.index + 1;

    //TODO: When adding canonization, check if the snippet already has a cannon child
    //TODO: Let an user only add a continuation once
    let id: Uuid = query_scalar!(
        r#"
        INSERT INTO story_parts(body, parent, writer, index, is_final) VALUES ($1, $2, $3, $4, $5) RETURNING id;
        "#,
        payload.body,
        parent,
        username,
        new_index,
        if new_index == MAX_INDEX { true } else { if new_index < MIN_INDEX { false } else { payload.is_final } }
    )
    .fetch_one(&state.postgres)
    .await?;

    Ok(id.to_string().into_response())
}

#[derive(Serialize, Deserialize)]
pub struct EditStorySnippet {
    body: String,
}

pub async fn edit_story_snippet(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<EditStorySnippet>,
) -> Result<(), AppError> {
    let role = get_role_from_header(&headers); // This cannot fail since we set it in middleware

    if role < User {
        return Err(AppError(anyhow!(
            "You do not have permission to edit this story snippet!"
        )));
    }

    if role == User {
        let username = get_username_from_header(&headers); // This cannot fail since we set it in middleware
        let resp = sqlx::query!(
            "UPDATE story_parts SET body = $1, modified = $2 WHERE id = $3 AND writer = $4",
            payload.body,
            Utc::now().naive_utc(),
            id,
            username
        )
        .execute(&state.postgres)
        .await?;
        if resp.rows_affected() == 0 {
            return Err(AppError(anyhow!(
                "You do not have permission to edit this snippet or it does not exist"
            )));
        }
    } else {
        // The user is an admin or "higher"
        sqlx::query!(
            "UPDATE story_parts SET body = $1, modified = $2 WHERE id = $3",
            payload.body,
            Utc::now().naive_utc(),
            id
        )
        .execute(&state.postgres)
        .await?;
    }

    Ok(())
}
