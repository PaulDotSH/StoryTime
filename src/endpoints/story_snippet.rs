use crate::endpoints::common::{
    generate_token, get_role_from_header, get_username_from_header, FORMAT,
};
use crate::user::Role::User;
use crate::{error::AppError, user::Role, AppState};
use anyhow::anyhow;
use axum::extract::{Path, Query};
use axum::http::{header, HeaderMap};
use axum::response::{IntoResponse, Redirect, Response};
use axum::{extract::State, Json};
use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{query, query_as, query_scalar};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct PostStorySnippet {
    body: String,
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
        INSERT INTO story_parts(body, writer) VALUES ($1, $2) RETURNING id;
        "#,
        payload.body,
        username
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
    child: Option<Uuid>,
    score: i32,
}

//TODO: Get comments too when the system is implemented
pub async fn get_story(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<StorySnippet>, AppError> {
    let parent = query_as!(
        StorySnippet,
        r#"
        SELECT id, writer, body, created, modified, child_cannon_time, parent, child, score FROM story_parts WHERE id = $1
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
        SELECT id, writer, body, created, modified, child_cannon_time, parent, child, score FROM story_parts WHERE parent = $1 AND score < $2
        ORDER BY score DESC, id
        LIMIT 5;
        "#,
        id,
        pagination.last_score
    ).fetch_all(&state.postgres).await?;

    Ok(Json(stories))
}

#[derive(Serialize, Deserialize)]
pub struct PostStorySnippetContinuation {
    body: String,
}

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

    //TODO: When adding canonization, check if the snippet already has a cannon child
    //TODO: Let an user only add a continuation once
    let id: Uuid = query_scalar!(
        r#"
        INSERT INTO story_parts(body, parent, writer) VALUES ($1, $2, $3) RETURNING id;
        "#,
        payload.body,
        parent,
        username
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
