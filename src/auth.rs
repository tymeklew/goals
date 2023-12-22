use crate::db::model::{Reset, Session};
use crate::db::schema::{resets, sessions, users};
use crate::error::AppError;
use crate::util;
use crate::validate::validate_password;
use crate::{db::model::User, AppState};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{response::IntoResponse, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::OptionalExtension;
use diesel_async::RunQueryDsl;
use lettre::message::header::ContentType;
use lettre::{AsyncTransport, Message};
use log2::info;
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

    info!("New user id : {}", user.id);

    diesel::insert_into(users::table)
        .values(user)
        .execute(&mut conn)
        .await
        .map_err(AppError::Diesel)?;

    Ok(StatusCode::CREATED)
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
    let session_id = session.id;

    // Save session
    diesel::insert_into(sessions::table)
        .values(session)
        .execute(&mut conn)
        .await
        .map_err(AppError::Diesel)?;

    let mut cookie = Cookie::new("session_id", session_id.to_string());
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_domain(state.config.domain);

    Ok(jar.add(cookie))
}

// Make a new reset request
// /auth/reset/:email
pub async fn new_reset(
    Path(email): Path<String>,
    State(state): State<AppState>,
) -> Result<StatusCode, AppError> {
    info!("Resetting {email}");

    let mut conn = state.pool.get().await.map_err(|_| AppError::Deadpool)?;

    let user_id = users::table
        .filter(users::email.eq(email.clone()))
        .select(users::id)
        .first::<Uuid>(&mut conn)
        .await
        .optional()
        .map_err(AppError::Diesel)?
        .ok_or(AppError::Status(StatusCode::NOT_FOUND))?;

    let reset = Reset {
        id: Uuid::new_v4(),
        token: util::generate_token(),
        user_id,
    };

    diesel::insert_into(resets::table)
        .values(reset.clone())
        .execute(&mut conn)
        .await
        .map_err(AppError::Diesel)?;

    let mail = Message::builder()
        .from(state.config.email.into())
        .to(email
            .parse()
            .map_err(|_| AppError::Status(StatusCode::BAD_REQUEST))?)
        .subject("Password reset")
        .header(ContentType::TEXT_PLAIN)
        .body(format!(
            "{}/reset?token={}&reset_id={}",
            state.config.domain, reset.token, reset.id
        ))
        .map_err(|_| AppError::Status(StatusCode::BAD_REQUEST))?;

    state.mailer.send(mail).await.map_err(AppError::Lettre)?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct ResetForm {
    token: String,
    reset_id: Uuid,
    password: String,
}
// /auth/reset?token=<token>&reset_id=<reset_id>
// Json body with passwd
pub async fn reset(
    State(state): State<AppState>,
    Json(form): Json<ResetForm>,
) -> Result<StatusCode, AppError> {
    let mut conn = state.pool.get().await.map_err(|_| AppError::Deadpool)?;

    let reset = resets::table
        .filter(resets::id.eq(form.reset_id))
        .first::<Reset>(&mut conn)
        .await
        .map_err(AppError::Diesel)?;

    if reset.token != form.token {
        return Err(AppError::Status(StatusCode::UNAUTHORIZED));
    }

    let hashed = hash(form.password, DEFAULT_COST).map_err(AppError::Bcrypt)?;

    // Update the user password with the hashed updated password
    diesel::update(users::table)
        .set(users::password.eq(hashed))
        .execute(&mut conn)
        .await
        .map_err(AppError::Diesel)?;

    // Delete reset in database
    diesel::delete(resets::table)
        .filter(resets::id.eq(reset.id))
        .execute(&mut conn)
        .await
        .map_err(AppError::Diesel)?;

    Ok(StatusCode::OK)
}
