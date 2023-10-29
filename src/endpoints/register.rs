use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
use serde::{Deserialize, Serialize};
use axum::{extract::{Path, Query, State}, Json};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{Response, IntoResponse, Redirect};
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use sqlx::query;
use crate::{AppState, error::AppError, user::Role};

#[derive(Serialize, Deserialize)]
pub struct CreateAccount {
    email: String,
    username: String,
    password: String,
}

//TODO: Auth middleware
//TODO: Regex email verification
//TODO: Email verification
//TODO: Login handler

fn generate_token(length: usize) -> String {
    Alphanumeric.sample_string(&mut thread_rng(), length)
}

pub async fn register_handler(State(state): State<AppState>, Json(payload): Json<CreateAccount>) -> Result<Response, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let password = Argon2::default().hash_password(payload.password.as_bytes(), &salt).unwrap();

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

    let cookie = format!("TOKEN={}; Path=/; Max-Age=300", &token);

    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    Ok((headers, Redirect::to("/")).into_response())
}