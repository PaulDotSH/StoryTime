mod endpoints;
pub mod error;
pub mod user;

use axum::routing::{get, post};
use axum::{middleware, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::error::Error;
use std::net::SocketAddr;

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
        .layer(middleware::from_fn_with_state(
            state.clone(),
            endpoints::auth::auth_middleware,
        ))
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
