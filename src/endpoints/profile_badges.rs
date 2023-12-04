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

//TODO: Add profile badge endpoint, also let the user upload the badge photo.
//If needed add an endpoint to get the price of a specific award

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct ShopBadge {
    id: i32,
    image: String,
    descr: String,
    color: String,
    price: i32,
}

pub async fn buy_badge(
    Path(award_id): Path<i32>,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<(), AppError> {
    let price = sqlx::query_scalar!(
        r#"
        SELECT price FROM profile_badges_shop WHERE award = $1;
        "#,
        award_id
    )
    .fetch_one(&state.postgres)
    .await?;
    let username = get_username_from_header(&headers);
    let user_pp = sqlx::query_scalar!(
        r#"
        SELECT pp FROM users WHERE username = $1;
        "#,
        username
    )
    .fetch_one(&state.postgres)
    .await?;
    if user_pp < price {
        return Err(AppError(anyhow!("You do not have enough PlotPoints to purchase this badge")));
    }

    sqlx::query!(
            "UPDATE users SET pp = pp - $1 WHERE username = $2",
            price,
            username
        )
        .execute(&state.postgres)
        .await?;

    sqlx::query!(
            "INSERT INTO profile_badges_link (users, award) VALUES ($1, $2)",
            username,
            award_id
        )
        .execute(&state.postgres)
        .await?;
    Ok(())
}

pub async fn get_shop_badges(
    State(state): State<AppState>,
) -> Result<Json<Vec<ShopBadge>>, AppError> {
    let badges = query_as!(
        ShopBadge,
        r#"
        SELECT pb.*, pbs.price
        FROM profile_badges pb
        JOIN profile_badges_shop pbs ON pb.id = pbs.award
        "#
    )
    .fetch_all(&state.postgres)
    .await?;

    Ok(Json(badges))
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct ProfileBadge {
    id: i32,
    image: String,
    descr: String,
    color: String,
    earned_at: NaiveDateTime,
}

pub async fn get_user_badges(
    Path(username): Path<String>,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<ProfileBadge>>, AppError> {
    let badges = query_as!(
        ProfileBadge,
        r#"
        SELECT pb.*, pbl.earned_at
        FROM profile_badges pb
        JOIN profile_badges_link pbl ON pb.id = pbl.award
        WHERE pbl.users = $1;
        "#,
        username
    )
    .fetch_all(&state.postgres)
    .await?;

    Ok(Json(badges))
}
