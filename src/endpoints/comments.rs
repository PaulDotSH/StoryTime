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
pub struct PostComment {
    body: String,
}

pub async fn new_comment (
    Path(snippet_id): Path<Uuid>,
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<PostComment>,
) -> Result<Response, AppError> {
    let username = get_username_from_header(&headers); // This cannot fail since we set it in middleware
    let role = get_role_from_header(&headers); // This cannot fail since we set it in middleware

    if role < User {
        return Err(AppError(anyhow!("You do not have permission to post a story snippet, make sure you confirmed your email and are not banned!")));
    }

    let id: Uuid = query_scalar!(
        r#"
        INSERT INTO comments(body, snippet, writer) VALUES ($1, $2, $3) RETURNING id;
        "#,
        payload.body,
        snippet_id,
        username
    )
        .fetch_one(&state.postgres)
        .await?;

    Ok(id.to_string().into_response())
}

#[derive(Serialize, Deserialize)]
pub struct EditComment {
    body: String,
}

//TODO: Low prio, maybe macro this to get rid of duplication
pub async fn edit_comment(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<EditComment>,
) -> Result<(), AppError> {
    let role = get_role_from_header(&headers);
    println!("{}", role);
    if role < User {
        return Err(AppError(anyhow!(
            "You do not have permission to edit this comment!"
        )));
    }

    if role == User {
        let username = get_username_from_header(&headers);
        println!("{}", username);
        let resp = sqlx::query!(
            "UPDATE comments SET body = $1, modified = $2 WHERE id = $3 AND writer = $4",
            payload.body,
            Utc::now().naive_utc(),
            id,
            username
        )
            .execute(&state.postgres)
            .await?;
        if resp.rows_affected() == 0 {
            return Err(AppError(anyhow!(
                "You do not have permission to edit this comment or it does not exist"
            )));
        }
    } else {
        sqlx::query!(
            "UPDATE comments SET body = $1, modified = $2 WHERE id = $3",
            payload.body,
            Utc::now().naive_utc(),
            id
        )
            .execute(&state.postgres)
            .await?;
    }

    Ok(())
}



#[derive(Deserialize)]
pub struct CommentPagination {
    last_score: i32,
}
impl Default for CommentPagination {
    fn default() -> Self {
        Self {
            last_score: i32::MAX,
        }
    }
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Comment {
    id: Uuid,
    writer: String,
    body: String,
    created: NaiveDateTime,
    modified: Option<NaiveDateTime>,
    score: i32
}

pub async fn get_story_comments(
    Path(snippet_id): Path<Uuid>,
    pagination: Option<Query<CommentPagination>>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Comment>>, AppError> {
    let Query(pagination) = pagination.unwrap_or_default();

    let comments = query_as!(
        Comment,
        r#"
        SELECT id, writer, body, created, modified, score FROM comments WHERE snippet = $1 AND score < $2
        ORDER BY score DESC, id
        LIMIT 25;
        "#,
        snippet_id,
        pagination.last_score
    ).fetch_all(&state.postgres).await?;

    Ok(Json(comments))
}