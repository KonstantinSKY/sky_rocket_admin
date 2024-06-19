use rocket::serde::json::Json;
use rocket::{get, State};
use sea_orm::{entity::*, DatabaseConnection};
use serde::{Deserialize, Serialize};
use rocket::http::Status;
use super::super::models::user;
use super::super::services::crypto;
use chrono::{NaiveDateTime, Utc};
use crate::db::{select, insert, delete};
use validator::Validate;
use crate::project::{responses, validators};

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub last_login: Option<NaiveDateTime>,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub created_at: NaiveDateTime,
}
impl From<user::Model> for UserResponse {
    fn from(user: user::Model) -> Self {
        UserResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            last_login: user.last_login,
            is_active: user.is_active,
            is_staff: user.is_staff,
            is_superuser: user.is_superuser,
            created_at: user.created_at,
        }
    }
}


#[get("/auth/users")]
pub async fn get_all_users(db: &State<DatabaseConnection>) -> Result<Json<Vec<UserResponse>>, rocket::http::Status> {
    let result = select::select_all::<user::Entity>(db).await;
    responses::handle_selection_result_by_response_struct(result) 
}


// Define the NewUser struct
#[derive(Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 1, max = 15))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    #[validate(length(min = 1, max = 20))]
    pub first_name: Option<String>,
    #[validate(length(min = 1, max = 20))]
    pub last_name: Option<String>,
}

// Handler to add a new user
#[post("/auth/users", data = "<new_user>")]
pub async fn add_user(db: &State<DatabaseConnection>, new_user: Json<NewUser>) -> Result<Json<UserResponse>, rocket::http::Status> {
    
    if let Err(validation_errors) = new_user.validate() {
        let errors = validators::validation_errors_to_response(validation_errors);
        return Err(Status::BadRequest);
    }

    let active_user = user::ActiveModel {
        name: Set(new_user.name.clone()),
        email: Set(new_user.email.clone()),
        password: Set(crypto::hash_password(new_user.password.clone())), // Assuming you have a function to hash passwords
        first_name: Set(new_user.first_name.clone()),
        last_name: Set(new_user.last_name.clone()),
        created_at: Set(Utc::now().naive_utc()),
        is_active: Set(true), // Default values for new users
        is_staff: Set(false),
        is_superuser: Set(false),
        last_login: Set(None),
        ..Default::default()
    };

    let insert_result = insert::insert::<user::Entity, _>(db, active_user).await;
    responses::handle_insertion_result_by_response_struct(insert_result)

}


// Handler to delete a user
#[delete("/auth/users/<user_id>")]
pub async fn delete_user(db: &State<DatabaseConnection>, user_id: i32) -> Result<Status, Status> {
    let result = user::Entity::delete_by_id(user_id).exec(db.inner()).await;    // Correct
    responses::handle_deletion_result(result)
}

