use axum::extract::State;
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
use tokio::sync::mpsc::{channel, Sender};
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
    // Sender
    sender: Sender<Messsage>,
}

impl AppState {
    async fn new() -> AppState {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
            var("DB_URL").expect("Failed to load DB_URL"),
        );

        let pool = Pool::builder(config).build().unwrap();

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

        let (tx, mut rx) = channel::<Messsage>(100);

        tokio::spawn(async move {
            loop {
                println!("Meow");
                if let Ok(i) = rx.try_recv() {
                    println!("got = {:?}", i);
                }
            }
        });

        AppState {
            pool,
            mailer,
            config: Config::load(),
            sender: tx,
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

    // Logger setup
    log2::open("log.txt")
        .module(true)
        .level("info")
        .tee(true)
        .start()
        .set_level("info");

    let state = AppState::new().await;

    let app = Router::new()
        .route("/api/goals/create", post(goals::create))
        .route("/api/goals/view", get(goals::view_all))
        .route("/api/goals/view/:id", get(goals::view_one))
        // Everything above requirest authentication via session_id cookies
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            util::authorization,
        ))
        .route("/api/testing", get(testing))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/reset/:email", post(auth::new_reset))
        .route("/api/auth/reset", post(auth::reset))
        .nest_service(
            "/",
            ServeDir::new("./client/dist")
                .not_found_service(ServeFile::new("./client/dist/index.html")),
        )
        .route_layer(middleware::from_fn(util::logging))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(var("PORT").expect("Failed to load PORT"))
        .await
        .expect("Failed to bind to listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve app");
}

async fn testing(State(state): State<AppState>) -> &'static str {
    state.sender.send(Messsage::Testing).await.unwrap();
    "testing"
}
