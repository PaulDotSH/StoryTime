use crate::endpoints::common::{generate_token, get_role_from_header, get_username_from_header};
use crate::{error::AppError, user::Role, AppState, MAIL_CLIENT};
use anyhow::anyhow;
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::{extract::State, Json};
use chrono::{Duration, Utc};
use lettre::message::header::ContentType;
use lettre::{AsyncTransport, Message};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_scalar};

pub async fn confirm_email(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<StatusCode, AppError> {
    let result = query!(
        r#"
        SELECT expire, email from email_confirmation where code = $1
        "#,
        code,
    )
    .fetch_one(&state.postgres)
    .await?; // TODO: Better error message when code doesnt exist

    if result.expire < Utc::now().naive_utc() {
        return Err(AppError(anyhow!("Code expired")));
    }

    query!(
        r#"
        UPDATE users SET perm = $1 WHERE email = $2
        "#,
        Role::User as i16,
        result.email
    )
    .execute(&state.postgres)
    .await?;

    Ok(StatusCode::OK)
}

// TODO: Better error message when account email doesn't exist
pub async fn send_confirmation_email(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<StatusCode, AppError> {
    let role = get_role_from_header(&headers);
    if role != Role::UnconfirmedMail {
        return Err(AppError(anyhow!("Email already verified")));
    }
    let username = get_username_from_header(&headers);

    let email = query_scalar!(
        r#"
        SELECT email from users where username = $1
        "#,
        username,
    ).fetch_one(&state.postgres).await?;

    // If there exists an unexpired code, don't create a new one
    let result = query!(
        r#"
        SELECT expire, code from email_confirmation where email = $1
        "#,
        email,
    )
    .fetch_one(&state.postgres)
    .await;

    // If code exists in db
    if let Ok(r) = result {
        // If code didn't expire
        if r.expire > Utc::now().naive_utc() {
            // Send code that already exists
            send_email_verification(&email, &r.code).await?;
        } else {
            // If code expired
            let code = generate_token(128);
            query!(
                r#"
                UPDATE email_confirmation SET code = $1, expire = $2 WHERE email = $3
                "#,
                &code,
                (Utc::now() + Duration::minutes(5)).naive_utc(),
                email,
            )
            .execute(&state.postgres)
            .await?;
            send_email_verification(&email, &code).await?;
        }
        // If code doesn't exist
    } else {
        let code = generate_token(128);
        query!(
            r#"
                INSERT INTO email_confirmation (email, code)
                VALUES ($1, $2)
                "#,
            email,
            &code
        )
        .execute(&state.postgres)
        .await?;
        send_email_verification(&email, &code).await?;
    }

    Ok(StatusCode::OK)
}

pub async fn send_email_verification(
    email: &str,
    code: &str,
) -> Result<(), lettre::transport::smtp::Error> {
    let email = Message::builder()
        .from("Storytime <storytime@best.app>".parse().unwrap())
        .to(email.parse().unwrap())
        // .to(format!("<{}>", email).parse().unwrap())
        .subject("Email confirmation")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(format!(
            "Acesta este codul dvs de verificare: {}",
            code
        )))
        .unwrap();

    MAIL_CLIENT.send(email).await?;

    Ok(())
}
