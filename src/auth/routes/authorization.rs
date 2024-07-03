use crate::auth::services::jwt::get_jwt_token;
use crate::project::ProjectSettings;

use super::super::repositories::users::UserRepository;
use super::Conn; //Import from mod.rs
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use rocket::State;
use super::super::services;


#[derive(serde::Deserialize, Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}


#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(mut db: Conn, credentials: Json<Credentials>,  state: &State<ProjectSettings>) -> Result<Value, Custom<Value>> {
    println!("Credentials: {credentials:?}");

    let user = match UserRepository::find_by_username(&mut db, &credentials.username).await {
        Ok(user) => user,
        Err(diesel::result::Error::NotFound) => return Err(Custom(Status::Unauthorized, json!("Wrong credentials"))),
        Err(_) => return Err(Custom(Status::InternalServerError, json!("Server Error"))),
    };

    if services::hash::verify_password(&credentials.password, &user.password).is_err() {
        return Err(Custom(Status::Unauthorized, json!("Wrong credentials")));
    }

    match get_jwt_token(user.id, &user.username, &user.email, 3600, state) {
        Ok(token) => Ok(json!({ "token": token })),
        Err(_) => Err(Custom(Status::InternalServerError, json!("Server Token Error")))
    }
}

