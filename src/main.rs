use axum::routing::delete;
use axum::Router;
use axum::{
    middleware,
    routing::{get, post},
};
use bb8::Pool;
use config::Config;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use std::env::var;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

mod account;
mod auth;
mod config;
mod db;
mod error;
mod goals;
mod util;
mod validate;

#[derive(Clone)]
pub struct AppState {
    // Db connection pool
    pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    // Smtp mailer to send emails
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    // Config for some constants and stuf
    config: Config,
}

impl AppState {
    async fn new() -> AppState {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
            var("DB_URL").expect("Failed to load DB_URL"),
        );

        let pool = Pool::builder().build(config).await.unwrap();

        let creds = Credentials::new(
            var("SMTP_USERNAME").expect("Failed to load SMTP_USERNAME"),
            var("SMTP_PASSWORD").expect("Failed to load SMTP_PASSWORD"),
        );

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(
            &var("SMTP_RELAY").expect("Failed to load SMTP_RELAY"),
        )
        .unwrap()
        .credentials(creds)
        .build();

        AppState {
            pool,
            mailer,
            config: Config::load(),
        }
    }
}

#[derive(Debug)]
pub enum Messsage {
    Testing,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load env variables");

    tracing_subscriber::fmt()
        .with_target(false)
        .with_writer(std::io::stderr)
        .init();

    let state = AppState::new().await;

    let app = Router::new()
        .route("/api/goals/create", post(goals::create))
        .route("/api/goals/view", get(goals::view_all))
        .route("/api/goals/view/:id", get(goals::view_one))
        .route("/api/account/update", post(account::update))
        .route("/api/account/delete", delete(account::delete))
        // Everything above requirest authentication via session_id cookies
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            util::authorization,
        ))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/reset/:email", post(auth::new_reset))
        .route("/api/auth/reset", post(auth::reset))
        .nest_service(
            "/",
            ServeDir::new("./client/dist")
                .not_found_service(ServeFile::new("./client/dist/index.html")),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
                .on_request(trace::DefaultOnRequest::new().level(Level::INFO)),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(var("PORT").expect("Failed to load PORT"))
        .await
        .expect("Failed to bind to listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve app");
}
