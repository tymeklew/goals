use axum::Router;
use axum::{
    middleware,
    routing::{get, post},
};
use config::Config;
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use std::env::var;
use tower_http::services::{ServeDir, ServeFile};

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
    pool: Pool<AsyncPgConnection>,
    // Smtp mailer to send emails
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    // Config for some constants and stuf
    config: Config,
}

impl AppState {
    async fn new() -> AppState {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
            var("DB_URL").expect("Failed to get db url"),
        );

        let pool = Pool::builder(config).build().unwrap();

        let creds = Credentials::new(
            var("SMTP_USERNAME").expect("Failed to load smtp username"),
            var("SMTP_PASSWORD").expect("Failed to load smtp password"),
        );

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(
            &var("SMTP_RELAY").expect("Failed to get relay env var"),
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

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load env variables");

    // Logger setup
    let _log2 = log2::open("log.txt").tee(true).start();

    let state = AppState::new().await;

    let app = Router::new()
        .route("/goals/create", post(goals::create))
        .route("/goals/view", get(goals::view_all))
        .route("/goals/view/:id", get(goals::view_one))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            util::authorization,
        ))
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .route("/auth/reset/:email", post(auth::new_reset))
        .route("/auth/reset", post(auth::reset))
        .nest_service(
            "/",
            ServeDir::new("./client/dist")
                .not_found_service(ServeFile::new("./client/dist/index.html")),
        )
        .with_state(state);

    let listener =
        tokio::net::TcpListener::bind(var("PORT").expect("Failed to find port in enviroment"))
            .await
            .expect("Failed to bind to listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve app");
}
