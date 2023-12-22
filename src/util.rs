use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};
use axum_extra::extract::CookieJar;

use uuid::Uuid;

use crate::{error::AppError, AppState};

pub async fn authorization(
    State(state): State<AppState>,
    jar: CookieJar,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let val = jar
        .get("session_id")
        .ok_or(AppError::Status(StatusCode::UNAUTHORIZED))?
        .value()
        .trim();

    let _session_id = Uuid::parse_str(val).unwrap();

    let mut _conn = state.pool.get().await.map_err(|_| AppError::Deadpool)?;

    // need to finish off

    Ok(next.run(request).await)
}
