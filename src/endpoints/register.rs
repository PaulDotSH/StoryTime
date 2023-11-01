use crate::endpoints::common::generate_token;
use crate::{error::AppError, user::Role, AppState};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use axum::http::{header, HeaderMap};
use axum::response::{IntoResponse, Redirect, Response};
use axum::{extract::State, Json};
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use sqlx::query;

#[derive(Serialize, Deserialize)]
pub struct CreateAccount {
    email: String,
    username: String,
    password: String,
}

//TODO: Regex email verification
//TODO: Email verification
//TODO: Login handler

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateAccount>,
) -> Result<Response, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let password = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .unwrap();

    let token = generate_token(128);
    query!(
        r#"
        INSERT INTO users (username, email, pw, perm, token)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        payload.username,
        payload.email,
        password.to_string(),
        Role::UnconfirmedMail as i16,
        &token
    )
    .execute(&state.postgres)
    .await?;

    // 24 * 7 * 3600
    let cookie = format!("TOKEN={}; Path=/; Max-Age=604800", &token);

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    Ok((headers, Redirect::to("/")).into_response())
}
