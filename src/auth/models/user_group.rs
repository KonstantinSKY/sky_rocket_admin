use diesel::prelude::*;
use crate::schema::*;
// use serde::{Serialize, Deserialize};

#[derive(Queryable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
pub struct UserGroup {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32,
}
#[derive(Insertable)]
#[diesel(table_name=users_groups)]
pub struct NewUserGroup {
    pub user_id: i32,
    pub group_id: i32,
}