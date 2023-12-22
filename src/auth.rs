use crate::db::model::Session;
use crate::db::schema::{sessions, users};
use crate::error::AppError;
use crate::validate::validate_password;
use crate::{db::model::User, AppState};
use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::OptionalExtension;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterForm {
    #[validate(email)]
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    #[validate(custom = "validate_password")]
    pub password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(form): Json<RegisterForm>,
) -> Result<impl IntoResponse, AppError> {
    form.validate().map_err(AppError::Validate)?;

    // Db connection
    let mut conn = state.pool.get().await.map_err(|_| AppError::Deadpool)?;
    // Check for conflict before hashing because hashing takes a while
    if users::table
        .filter(users::email.eq(form.email.clone()))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(AppError::Diesel)?
        != 0
    {
        return Ok(StatusCode::CONFLICT);
    };

    let user = User {
        id: Uuid::new_v4(),
        email: form.email,
        first_name: form.first_name,
        last_name: form.last_name,
        password: hash(form.password, DEFAULT_COST).map_err(AppError::Bcrypt)?,
    };

    diesel::insert_into(users::table)
        .values(user)
        .execute(&mut conn)
        .await
        .map_err(AppError::Diesel)?;

    return Ok(StatusCode::CREATED);
}

#[derive(Deserialize, Validate)]
pub struct LoginForm {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(form): Json<LoginForm>,
) -> Result<CookieJar, AppError> {
    form.validate().map_err(AppError::Validate)?;

    let mut conn = state.pool.get().await.map_err(|_| AppError::Deadpool)?;

    let user = match users::table
        .filter(users::email.eq(form.email))
        .get_result::<User>(&mut conn)
        .await
        .optional()
        .map_err(AppError::Diesel)?
    {
        Some(user) => user,
        None => return Err(AppError::Status(StatusCode::UNAUTHORIZED)),
    };
    // Verify if the form password is equal to to hash stored on the database
    if !verify(form.password, &user.password).map_err(AppError::Bcrypt)? {
        return Err(AppError::Status(StatusCode::UNAUTHORIZED));
    }

    let session = Session {
        id: Uuid::new_v4(),
        user_id: user.id,
    };

    // Clone session.id because when inserting it will consume it and I need to use it later
    let session_id = session.id.clone();

    // Save session
    diesel::insert_into(sessions::table)
        .values(session)
        .execute(&mut conn)
        .await
        .map_err(AppError::Diesel)?;

    let mut cookie = Cookie::new("session_id", session_id.to_string());
    cookie.set_path("/");
    cookie.set_http_only(true);

    Ok(jar.add(cookie))
}

// Make a new reset request
pub async fn new_reset() {}
pub async fn reset() {}
