use axum::Router;
use axum::{
    middleware,
    routing::{get, post},
};
use diesel_async::{
    pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use std::env::var;
use tower_http::services::{ServeDir, ServeFile};

mod auth;
mod db;
mod error;
mod goals;
mod util;
mod validate;

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
        .route("/goals/create", post(goals::create))
        .route("/goals/view", get(goals::view_all))
        .route("/goals/view/:id", get(goals::view_one))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            util::authorization,
        ))
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
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
