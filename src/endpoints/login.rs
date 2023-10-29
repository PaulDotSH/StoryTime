use anyhow::anyhow;
use crate::{error::AppError, user::Role, AppState};
use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
use axum::http::{header, HeaderMap};
use axum::response::{IntoResponse, Redirect, Response};
use axum::{extract::State, Json};
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_scalar};
use tower_http::classify::ServerErrorsFailureClass::StatusCode;
use crate::endpoints::common::{FORMAT, generate_token};
use chrono::{Utc, Duration, NaiveDate, NaiveDateTime};

#[derive(Serialize, Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<Login>,
) -> Result<Response, AppError> {

    let pw: String = query_scalar!(
        r#"
        SELECT pw from users where username = $1
        "#,
        payload.username,
    )
        .fetch_one(&state.postgres)
        .await?;

    let Ok(parsed_hash) = PasswordHash::new(&pw) else {
        // Case where user is banned, TODO: implement user banning endpoint and temporary banning using another field in the database
        // if let Ok(ban_expire) = NaiveDateTime::parse_from_str(&pw, FORMAT) {
        return Err(AppError(anyhow!("User was permanently banned")));
    };

    if Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_err() {
        return Err(AppError(anyhow!("Incorrect Password")));
    }

    let token = generate_token(128);

    sqlx::query!(
        "UPDATE users SET token = $1, tok_expire = $2 WHERE username = $3",
        token,
        (Utc::now() + Duration::days(7)).naive_utc(),
        payload.username
    )
        .execute(&state.postgres)
        .await?;

    let cookie = format!("TOKEN={}; Path=/; Max-Age=604800", &token);

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    Ok((headers, Redirect::to("/")).into_response())
}
