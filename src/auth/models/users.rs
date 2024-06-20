use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use validator_derive::Validate;
use crate::schema::*;
use validator::{Validate, ValidationError};


#[derive(Queryable, Serialize, Deserialize)]
pub struct User {                                   //The main Model
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub last_login: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Validate)]
#[diesel(table_name=users)]
pub struct NewUser {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    pub username: String,
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 6, message = "Username must be at least  characters long"))]
    pub password: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=users)]
pub struct UpdateUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub last_login: Option<chrono::NaiveDateTime>,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            created_at: user.created_at,
            last_login: user.last_login,
            is_active: user.is_active,
            is_staff: user.is_staff,
            is_superuser: user.is_superuser,
        }
    }
}