use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use diesel::prelude::*;
use diesel::OptionalExtension;
use diesel_async::RunQueryDsl;

use crate::{
    db::{
        model::User,
        schema::{sessions, users},
    },
    error::AppError,
    AppState,
};

pub async fn authorization(
    State(state): State<AppState>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let session_id = jar
        .get("session_id")
        .ok_or(AppError::Status(StatusCode::UNAUTHORIZED))?
        .value()
        .trim();

    let mut conn = state.pool.get().await.map_err(|_| AppError::Deadpool)?;

    let user: User = sessions::table
        .inner_join(users::table)
        .filter(sessions::id.eq(uuid::Uuid::parse_str(session_id).unwrap()))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .await
        .optional()
        .map_err(|_| AppError::Diesel)?
        .ok_or(AppError::Status(StatusCode::NOT_FOUND))?;

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}
