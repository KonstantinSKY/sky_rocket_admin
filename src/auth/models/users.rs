use bcrypt::{BcryptError, DEFAULT_COST};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use validator_derive::Validate;
use crate::schema::*;
use super::super::services::auth;

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
    #[validate(length(min = 3, max = 100, message = "Username must be at least 3 characters long"))]
    pub username: String,
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 6, max = 100, message = "Username must be at least 6 characters long"))]
    pub password: String,
}
impl NewUser {
    pub fn with_hashed_password(self, ) -> Result<Self, BcryptError> {
        match auth::hash_password(self.password) {
            Ok(hashed_password) => Ok(Self {
                password: hashed_password,
                ..self
            }),
            Err(e) => Err(e),
        }
    }
    pub fn convert (self,user_json : Json<User>) -> Result<User, Custom<Value>> {
        match auth::hash_password(self.password) {
            Ok(hashed_password) => Ok(Self {
                password: hashed_password,
                ..self
            }),
            Err(e) => Err(e),
        }
    }
}


#[derive(Insertable, Deserialize, Validate, Debug)]
#[diesel(table_name=users)]
pub struct NewSuperUser {
    #[validate(length(min = 5, max = 150, message = "Username must be at least 3 characters long"))]
    pub username: String,
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: String,
    #[validate(length(min = 8, max = 150, message = "Username must be at least 6 characters long"))]
    pub password: String,
    pub is_staff: bool,
    pub is_superuser: bool,
}


#[derive(Deserialize, Validate)]
pub struct NewUserWithGroups {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub groups_ids: Vec<i32>,
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