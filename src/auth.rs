use crate::db::model::Session;
use crate::db::schema::{sessions, users};
use crate::error::AppError;
use crate::{db::model::User, AppState};
use anyhow::Result;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use diesel::prelude::*;
use diesel::OptionalExtension;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(form): Json<RegisterForm>,
) -> Result<impl IntoResponse, AppError> {
    let user = User {
        id: Uuid::new_v4(),
        email: form.email,
        first_name: form.first_name,
        last_name: form.last_name,
        password: form.password,
    };

    let mut conn = state.pool.get().await.map_err(|_| AppError::Deadpool)?;

    diesel::insert_into(users::table)
        .values(user)
        .execute(&mut conn)
        .await
        .map_err(|_| AppError::Diesel)?;

    return Ok(StatusCode::CREATED);
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(form): Json<RegisterForm>,
) -> Result<CookieJar, AppError> {
    let mut conn = state.pool.get().await.map_err(|_| AppError::Deadpool)?;

    let user = match users::table
        .filter(users::email.eq(form.email))
        .get_result::<User>(&mut conn)
        .await
        .optional()
        .map_err(|_| AppError::Diesel)?
    {
        Some(user) => user,
        None => return Err(AppError::Status(StatusCode::NOT_FOUND)),
    };

    if user.password != form.password {
        return Err(AppError::Status(StatusCode::UNAUTHORIZED));
    }

    let session = Session {
        id: Uuid::new_v4(),
        user_id: user.id,
    };

    let session_id = session.id.clone();

    // Save session
    diesel::insert_into(sessions::table)
        .values(session)
        .execute(&mut conn)
        .await
        .map_err(|_| AppError::Diesel)?;

    Ok(jar.add(Cookie::new("session_id", session_id.to_string())))
}
