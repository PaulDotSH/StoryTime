use crate::endpoints::common::*;
use crate::user::Role::User;
use crate::{error::AppError, AppState};
use anyhow::anyhow;
use axum::http::{HeaderMap, StatusCode};
use axum::{extract::State, Json};
use axum::extract::Path;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_scalar};

//Create
#[derive(Serialize, Deserialize)]
pub struct NewPlace {
    name: String,
    description: String,
    rules: String,
}

pub async fn new_place(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<NewPlace>,
) -> Result<StatusCode, AppError> {
    let role = get_role_from_header(&headers);

    if role < User {
        return Err(AppError(anyhow!("Make sure you confirmed your email and are not banned!")));
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

#[derive(Serialize, Deserialize)]
pub struct NewPlaceTag {
    name: String,
    place: String,
}

pub async fn new_place_tag(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<NewPlaceTag>,
) -> Result<(), AppError> {
    //TODO: Check if the user is place owner or moderator

    sqlx::query!(
        r#"
            INSERT INTO place_tags(name, place) VALUES ($1, $2);
        "#,
        payload.name,
        payload.place
    )
        .execute(&state.postgres)
        .await?;
    Ok(())
}

//Delete tag with CASCADE to also remove it from the link table

//Add tag to snippet

//Remove tag from snippet

//Basic read with pagination

//Transfer ownership
#[derive(Serialize, Deserialize)]
pub struct TransferPlace {
    place: String,
    new_owner: String
}

pub async fn transfer_ownership(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<TransferPlace>,
) -> Result<(), AppError> {
    let username = get_username_from_header(&headers);
    let place = sqlx::query!(
        r#"
            SELECT name FROM places WHERE name = $1 AND owner = $2;
        "#,
        payload.place,
        username
    )
        .fetch_one(&state.postgres)
        .await?;


    sqlx::query!(
        r#"
            UPDATE places SET owner = $1 WHERE name = $2;
        "#, //UPDATE users SET token = $1, tok_expire = $2 WHERE username = $3
        payload.new_owner,
        payload.place
    )
        .execute(&state.postgres)
        .await?;
    Ok(())
}

//Update rules

//Update description

//Upload photo

//TODO: After all of these also return the tags a snippet has