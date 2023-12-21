use super::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Selectable, Queryable, Insertable, PartialEq, Eq, Clone)]
#[diesel(table_name=users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name=goals)]
pub struct Goal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, Selectable, Queryable, Insertable, Associations)]
#[diesel(table_name=sessions)]
#[diesel(belongs_to(User))]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
}
