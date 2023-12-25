use axum::{extract::State, http::StatusCode, Extension, Json};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use validator::Validate;

use crate::{
    db::{model::User, schema::users},
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



