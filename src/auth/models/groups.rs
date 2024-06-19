use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize}
use crate::schema::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=groups)]
pub struct NewGroup {
    pub name: String,
    pub description: Option<String>,
}
