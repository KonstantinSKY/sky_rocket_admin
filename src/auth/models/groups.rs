use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;
use validator_derive::Validate;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Validate)]
#[diesel(table_name=groups)]
pub struct NewGroup {
    #[validate(length(min = 3, max = 100, message = "Username must be at least 3 characters long"))]
    pub name: String,
    pub description: Option<String>,
}
