use std::time::SystemTime;

use super::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Deserialize,
    Debug,
    Serialize,
    Identifiable,
    Selectable,
    Queryable,
    Insertable,
    PartialEq,
    Eq,
    Clone,
)]
#[diesel(table_name=users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub admin: bool,
}

#[derive(Deserialize, Serialize, Debug, Associations, Identifiable, Queryable, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name=goals)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Goal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
}

#[derive(
    Deserialize,
    Debug,
    Serialize,
    Selectable,
    Identifiable,
    Queryable,
    Insertable,
    Associations,
    PartialEq,
)]
#[diesel(belongs_to(User))]
#[diesel(table_name=sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: SystemTime,
}

#[derive(
    Deserialize, Clone, Debug, Selectable, Identifiable, Queryable, Insertable, Associations,
)]
#[diesel(table_name=resets)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Reset {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
}
