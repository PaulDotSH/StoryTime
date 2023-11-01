mod endpoints;
pub mod error;
pub mod user;

use axum::routing::{get, post, put};
use axum::{middleware, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres, query};
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    postgres: Pool<Postgres>,
}

pub async fn sample_response_handler() -> String {
    "Home page example response".to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let pool = PgPoolOptions::new()
        .max_connections(8) // TODO: Check what number would be appropriate here
        .connect(
            &env::var("DATABASE_URL").expect("Your .env file is missing the DATABASE_URL variable"),
        )
        .await?;

    let state = AppState { postgres: pool }; // TODO: Maybe add redis here for caching queries

    let app = Router::new()
        .route("/auth-only", get(sample_response_handler))
        .route("/snippets/:id", get(endpoints::story_snippet::get_story))
        .route(
            "/snippets/:id/children",
            get(endpoints::story_snippet::get_story_children),
        )
        .route(
            "/snippets/new",
            post(endpoints::story_snippet::new_story_snippet),
        )
        .route(
            "/snippets/:id/new",
            post(endpoints::story_snippet::new_story_snippet_continuation),
        )
        .route(
            "/snippets/:id",
            put(endpoints::story_snippet::edit_story_snippet),
        )
        .route(
            "/snippets/:id/comments/new",
            post(endpoints::comments::new_comment),
        )
        .route(
            "/snippets/:id/comments",
            get(endpoints::comments::get_story_comments),
        )
        .route(
            "/comments/:id",
            put(endpoints::comments::edit_comment),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            endpoints::auth::auth_middleware,
        ))
        .route("/login", post(endpoints::login::login_handler))
        .route("/register", post(endpoints::register::register_handler))
        .with_state(state)
        .route("/", get(sample_response_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 5431));
    println!("Storytime backend running.");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
