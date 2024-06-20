use rocket::response::status::{Custom, NoContent};
use rocket::{get, post, delete, put, http::Status};
use rocket::serde::json::{Json, json, Value};
use validator::Validate;

use super::super::repositories::users::UserRepository;
use super::super::models::users::*;

use super::Conn; //Import from mod.rs



#[get("/auth/users")]
pub async fn get_all_users(mut db: Conn) -> Result<Value, Custom<Value>> {
    UserRepository::select_all(&mut db).await
    .map(|users| {
        let user_responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
        json!(user_responses)
    })
    .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[post("/auth/users", format="json", data="<new_user>")]
pub async fn create_user(mut db: Conn, new_user: Json<NewUser> ) -> Result<Custom<Value>, Custom<Value>> {
     // Validate the incoming user data
    if new_user.validate().is_err() {
        return Err(Custom(Status::UnprocessableEntity, json!("Validation Error")));
    }

    UserRepository::create(&mut db, new_user.into_inner()).await
        .map(|user| Custom(Status::Created, json!(UserResponse::from(user))))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[post("/auth/users/create_with_roles", format = "json", data = "<user_with_groups>")]
pub async fn create_user_with_groups(mut db: Conn, user_with_groups: Json<NewUserWithGroups>) -> Result<Custom<Value>, Custom<Value>> {
    // Validate the incoming request data
    if let Err(errors) = user_with_groups.validate() {
        return Err(Custom(Status::UnprocessableEntity, json!({"errors": errors})));
    }
 // Destructure the payload to move values out directly
    let NewUserWithGroups { username, email, password, groups_ids } = user_with_groups.into_inner();
    
    let new_user = NewUser {
        username,
        email,
        password,
    };
// TODO
    UserRepository::create(&mut db, new_user).await
        .map(|user| Custom(Status::Created, json!(UserResponse::from(user))))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error creating user")))
    
}



#[delete("/auth/users/<id>")]
pub async fn delete_user(mut db: Conn, id: i32) -> Result<NoContent, Custom<Value>> {
    UserRepository::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[put("/auth/users/<id>", format="json", data="<user>")]
pub async fn update_user(mut db: Conn, id: i32, user: Json<UpdateUser>) -> Result<Value, Custom<Value>> {
    UserRepository::update(&mut db, id, user.into_inner()).await
        .map(|user| json!(UserResponse::from(user)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}




// #[rocket::get("/rustaceans")]
// pub async fn get_all_users(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
//     RustaceanRepository::find_multiple(&mut db, 100).await
//         .map(|rustaceans| json!(rustaceans))
//         .map_err(|e| server_error(e.into()))
// }


// // Define the NewUser struct
// #[derive(Deserialize, Validate)]
// pub struct NewUser {
//     #[validate(length(min = 1, max = 15))]
//     pub name: String,
//     #[validate(email)]
//     pub email: String,
//     #[validate(length(min = 6))]
//     pub password: String,
//     #[validate(length(min = 1, max = 20))]
//     pub first_name: Option<String>,
//     #[validate(length(min = 1, max = 20))]
//     pub last_name: Option<String>,
// }

// // Handler to add a new user
// #[post("/auth/users", data = "<new_user>")]
// pub async fn add_user(db: &State<DatabaseConnection>, new_user: Json<NewUser>) -> Result<Json<UserResponse>, rocket::http::Status> {
    
//     if let Err(validation_errors) = new_user.validate() {
//         let errors = validators::validation_errors_to_response(validation_errors);
//         return Err(Status::BadRequest);
//     }

//     let active_user = user::ActiveModel {
//         name: Set(new_user.name.clone()),
//         email: Set(new_user.email.clone()),
//         password: Set(crypto::hash_password(new_user.password.clone())), // Assuming you have a function to hash passwords
//         first_name: Set(new_user.first_name.clone()),
//         last_name: Set(new_user.last_name.clone()),
//         created_at: Set(Utc::now().naive_utc()),
//         is_active: Set(true), // Default values for new users
//         is_staff: Set(false),
//         is_superuser: Set(false),
//         last_login: Set(None),
//         ..Default::default()
//     };

//     let insert_result = insert::insert::<user::Entity, _>(db, active_user).await;
//     responses::handle_insertion_result_by_response_struct(insert_result)

// }


// // Handler to delete a user
// #[delete("/auth/users/<user_id>")]
// pub async fn delete_user(db: &State<DatabaseConnection>, user_id: i32) -> Result<Status, Status> {
//     let result = user::Entity::delete_by_id(user_id).exec(db.inner()).await;    // Correct
//     responses::handle_deletion_result(result)
// }

