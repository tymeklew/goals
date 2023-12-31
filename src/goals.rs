use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
// Checkup Calender thing idea cool colours
// Make ti looks cool
use diesel::BelongingToDsl;
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
use diesel::prelude::*;

#[derive(Deserialize)]
pub struct CreateGoalForm {
    pub title: String,
    pub description: String,
}

pub async fn create(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(form): Json<CreateGoalForm>,
) -> Result<(StatusCode, String), AppError> {
    let goal = Goal {
        id: Uuid::new_v4(),
        user_id: user.id,
        title: form.title,
        description: form.description,
    };

    let goal_id = goal.id;

    let mut conn = state.pool.get().await?;

    diesel::insert_into(goals::table)
        .values(goal)
        .execute(&mut conn)
        .await?;

    Ok((StatusCode::CREATED, goal_id.to_string()))
}

pub async fn view_all(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Goal>>, AppError> {
    let mut conn = state.pool.get().await?;

    let goals: Vec<Goal> = Goal::belonging_to(&user).load::<Goal>(&mut conn).await?;

    Ok(Json(goals))
}

pub async fn view_one(
    Extension(user): Extension<User>,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Goal>, AppError> {
    let mut conn = state.pool.get().await?;

    let goal = Goal::belonging_to(&user)
        .filter(goals::id.eq(id))
        .first::<Goal>(&mut conn)
        .await
        .optional()?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(goal))
}
