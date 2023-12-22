use axum::{extract::State, http::StatusCode, Extension, Json};
// Checkup Calender thing idea cool colours
// Make ti looks cool
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    db::{
        model::{Goal, User},
        schema::goals,
    },
    error::AppError,
    AppState,
};

#[derive(Deserialize)]
pub struct CreateGoalForm {
    pub title: String,
    pub description: String,
}

pub async fn create(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(form): Json<CreateGoalForm>,
) -> Result<StatusCode, AppError> {
    let goal = Goal {
        id: Uuid::new_v4(),
        user_id: user.id,
        title: form.title,
        description: form.description,
    };

    let mut conn = state.pool.get().await.map_err(|_| AppError::Deadpool)?;

    diesel::insert_into(goals::table)
        .values(goal)
        .execute(&mut conn)
        .await
        .map_err(AppError::Diesel)?;

    Ok(StatusCode::CREATED)
}
