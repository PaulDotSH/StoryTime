use crate::endpoints::common::*;
use crate::user::Role::User;
use crate::{error::AppError, AppState};
use anyhow::anyhow;
use axum::http::{HeaderMap, StatusCode};
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_scalar};

//Create
#[derive(Serialize, Deserialize)]
pub struct NewPlace {
    name: String,
    description: String,
    rules: String,
}

#[axum::debug_handler]
pub async fn new_place(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<NewPlace>,
) -> Result<StatusCode, AppError> {
    let role = get_role_from_header(&headers);

    if role < User {
        return Err(AppError(anyhow!("You do not have permission to post a story snippet, make sure you confirmed your email and are not banned!")));
    }

    let username = get_username_from_header(&headers);

    let score: i32 = query_scalar!(
        r#"
        SELECT score FROM users WHERE username = $1;
        "#,
        username
    )
    .fetch_one(&state.postgres)
    .await?;

    if score < MIN_PLACE_CREATION_SCORE {
        return Err(AppError(anyhow!(
            "You don't have enough score to create a place"
        )));
    }

    query!(
        "INSERT INTO places (name, description, rules, owner) VALUES ($1, $2, $3, $4)",
        payload.name,
        payload.description,
        payload.rules,
        username
    )
    .execute(&state.postgres)
    .await?;

    Ok(StatusCode::OK)
}

//Basic read with pagination

//Transfer ownership

//Update rules

//Update description

//Upload photo
