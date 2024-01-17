use crate::{user::Role, AppState};
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderValue, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use chrono::Utc;

#[derive(sqlx::FromRow, Debug)]
struct User {
    username: String,
    // pw_changed: chrono::NaiveDateTime,
    tok_expire: chrono::NaiveDateTime,
    perm: Role,
}

pub async fn auth_middleware<B>(
    State(state): State<AppState>,
    mut request: Request<Body>, // insert the username and role headers in the following requests in case they are needed so we don't hit the database again
    next: Next,                 // So we can forward the request
) -> Response {
    println!("{:?}", request);
    let headers = request.headers_mut();

    let Some(cookie) = headers.get("cookie").cloned() else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let Ok(cookie_str) = cookie.to_str() else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let cookie_pairs: Vec<&str> = cookie_str.split(';').collect();

    for pair in cookie_pairs {
        if let Some(token) = pair.trim().strip_prefix("TOKEN=") {
            let Ok(user) = sqlx::query_as!(
                User,
                "SELECT username, tok_expire, perm FROM users WHERE token = $1",
                token
            )
            .fetch_one(&state.postgres)
            .await
            else {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Invalid token").into_response();
            };

            if user.tok_expire < Utc::now().naive_utc() {
                sqlx::query!(
                    "UPDATE users SET token = NULL WHERE username = $1",
                    user.username
                )
                .execute(&state.postgres)
                .await
                .unwrap(); // Shouldn't fail
                return StatusCode::UNAUTHORIZED.into_response();
            }

            headers.insert("id", HeaderValue::from_str(&user.username).unwrap());
            headers.insert("role", HeaderValue::from(user.perm as i16));
        }
    }

    next.run(request).await
}
