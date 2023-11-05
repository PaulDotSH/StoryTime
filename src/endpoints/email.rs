use crate::endpoints::common::generate_token;
use crate::{error::AppError, user::Role, AppState, MAIL_CLIENT};
use anyhow::anyhow;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use chrono::Utc;
use lettre::message::header::ContentType;
use lettre::{AsyncTransport, Message};
use serde::{Deserialize, Serialize};
use sqlx::query;

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

    if result.expire > Utc::now().naive_utc() {
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

#[derive(Serialize, Deserialize)]
pub struct SendConfirmationMail {
    email: String,
}

// TODO: Better error message when account email doesn't exist
pub async fn send_confirmation_email(
    State(state): State<AppState>,
    Json(payload): Json<SendConfirmationMail>,
) -> Result<StatusCode, AppError> {
    //TODO: Check if user already has verified email (Role > UnverifiedEmail)

    // If there exists an unexpired code, don't create a new one
    let result = query!(
        r#"
        SELECT expire, code from email_confirmation where email = $1
        "#,
        payload.email,
    )
    .fetch_one(&state.postgres)
    .await;

    // If code exists in db
    if let Ok(r) = result {
        // If code didn't expire
        if r.expire > Utc::now().naive_utc() {
            // Send code that already exists
            send_email_verification(&payload.email, &r.code).await?;
        } else {
            // If code expired
            let code = generate_token(128);
            query!(
                r#"
                UPDATE email_confirmation SET code = $1, expire = $2 WHERE email = $3
                "#,
                &code,
                Utc::now().naive_utc(),
                payload.email,
            )
            .execute(&state.postgres)
            .await?;
            send_email_verification(&payload.email, &code).await?;
        }
        // If code doesn't exist
    } else {
        let code = generate_token(128);
        query!(
            r#"
                INSERT INTO email_confirmation (email, code)
                VALUES ($1, $2)
                "#,
            payload.email,
            &code
        )
        .execute(&state.postgres)
        .await?;
        send_email_verification(&payload.email, &code).await?;
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
