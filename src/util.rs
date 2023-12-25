use std::time::SystemTime;

use crate::{
    db::{
        model::User,
        schema::{sessions, users},
    },
    error::AppError,
    AppState,
};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;
use diesel::prelude::*;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use rand::{distributions::Alphanumeric, Rng};
use uuid::Uuid;

pub async fn authorization(
    State(state): State<AppState>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let val = jar
        .get("session_id")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .value()
        .trim();

    let session_id = Uuid::parse_str(val).unwrap();

    let mut conn = state.pool.get().await?;

    // Get the user from the session_id
    let user = users::table
        .inner_join(sessions::table)
        .filter(
            sessions::id
                .eq(session_id)
                .and(sessions::expires_at.gt(SystemTime::now())),
        )
        .select(User::as_select())
        .first::<User>(&mut conn)
        .await
        .optional()?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}

pub fn generate_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}
