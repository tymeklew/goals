use axum::{extract::State, http::StatusCode, response::Redirect, Extension, Json};
use bcrypt::verify;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use validator::Validate;

use crate::{
    db::{
        model::User,
        schema::{resets, sessions, users},
    },
    error::AppError,
    AppState,
};

#[derive(Deserialize, AsChangeset, Validate)]
#[diesel(table_name=users)]
pub struct UpdateForm {
    #[validate(email)]
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

// /api/account/update
// {
//  "field" : "email || username || fname || lname",
//  "value" : "value to upadte"
// }
pub async fn update(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(form): Json<UpdateForm>,
) -> Result<StatusCode, AppError> {
    let mut conn = state.pool.get().await?;

    diesel::update(users::table.filter(users::id.eq(user.id)))
        .set(&form)
        .execute(&mut conn)
        .await?;

    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct DeleteForm {
    password: String,
}
// /api/account/delete
// {
//  §password : "password",
// }
pub async fn delete(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(form): Json<DeleteForm>,
) -> Result<Redirect, AppError> {
    let mut conn = state.pool.get().await?;

    let user = users::table
        .filter(users::id.eq(user.id))
        .first::<User>(&mut conn)
        .await
        .optional()?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Make sure the password matches
    if !verify(form.password, &user.password)? {
        return Err(StatusCode::UNAUTHORIZED.into());
    }

    // Delete everything related to the user
    diesel::delete(sessions::table)
        .filter(sessions::user_id.eq(user.id))
        .execute(&mut conn)
        .await?;

    diesel::delete(resets::table)
        .filter(resets::user_id.eq(user.id))
        .execute(&mut conn)
        .await?;

    diesel::delete(users::table)
        .filter(users::id.eq(user.id))
        .execute(&mut conn)
        .await?;

    Ok(Redirect::to("/"))
}
