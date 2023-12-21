use axum::Router;
use axum::{middleware, routing::post};
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use std::env::var;

mod auth;
mod db;
mod error;
mod goals;
mod util;

#[derive(Clone)]
pub struct AppState {
    pool: Pool<AsyncPgConnection>,
}

impl AppState {
    fn new() -> AppState {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(
            var("DB_URL").expect("Failed to get db url"),
        );

        let pool = Pool::builder(config).build().unwrap();

        return AppState { pool };
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load env variables");

    let state = AppState::new();

    let app = Router::new()
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            util::authorization,
        ))
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind to listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve app");
}
