mod endpoints;
pub mod error;
pub mod user;

use axum::routing::{delete, get, post, put};
use axum::{http, middleware, Router};
use lazy_static::lazy_static;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::error::Error;
use axum::http::{HeaderValue, Method};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::cors::{any, CorsLayer};

lazy_static! {
    pub static ref MAIL_CLIENT: AsyncSmtpTransport<Tokio1Executor> = {
        let creds = Credentials::new(
            env::var("SMTP_USER").unwrap(),
            env::var("SMTP_PASS").unwrap(),
        );

        AsyncSmtpTransport::<Tokio1Executor>::relay(&env::var("SMTP_HOST").unwrap())
            .unwrap()
            .credentials(creds)
            .build()
    };
}

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

    let serve_dir_from_assets = ServeDir::new("assets");

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers([http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE])
        .allow_credentials(true)
        .allow_origin(
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
        );

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
        .route("/places/new", post(endpoints::place::new_place))
        .route(
            "/snippets/:id/vote",
            post(endpoints::story_snippet::vote_snippet),
        )
        .route(
            "/snippets/:id/vote",
            delete(endpoints::story_snippet::remove_vote),
        )
        .route("/comments/:id", put(endpoints::comments::edit_comment))
        .route(
            "/notifications/:id/mark",
            post(endpoints::notifications::mark_notification),
        )
        .route(
            "/notifications/mark",
            post(endpoints::notifications::mark_all_notifications),
        )
        .route(
            "/notifications/",
            get(endpoints::notifications::get_notifications),
        )
        .route(
            "/shop/badges/:id/buy",
            post(endpoints::profile_badges::buy_badge),
        )
        .route(
            "/place/tag/new",
            post(endpoints::place::new_place_tag)
        )
        .route(
            "/place/transfer",
            post(endpoints::place::transfer_ownership)
        )
        .route("/resend", post(endpoints::email::send_confirmation_email))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            endpoints::auth::auth_middleware::<axum::body::Body>,
        ))
        .route("/login", post(endpoints::login::login_handler))
        .route("/register", post(endpoints::register::register_handler))
        .route("/confirm/:id", post(endpoints::email::confirm_email))
        .route(
            "/profile/:username/badges",
            get(endpoints::profile_badges::get_user_badges),
        )
        .route(
            "/profile/:username",
            get(endpoints::profile::get_user_profile),
        )
        .route(
            "/shop/badges",
            get(endpoints::profile_badges::get_shop_badges),
        )
        .with_state(state)
        .layer(cors)
        .route("/", get(sample_response_handler))
        .nest_service("/assets", serve_dir_from_assets);

    let listener = TcpListener::bind("127.0.0.1:5431")
        .await
        .expect("Cannot start server");
    println!("Storytime backend running.");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
