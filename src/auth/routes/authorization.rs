use super::super::repositories::users::UserRepository;
use bcrypt::verify;
use rocket::serde::json::{json, Json, Value};
use rocket::response::status::Custom;
use rocket::http::Status;
use super::Conn; //Import from mod.rs

#[derive(serde::Deserialize, Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(mut db: Conn, credentials: Json<Credentials>) -> Result<Value, Custom<Value>> {
    println!("Credentials: {:?}", credentials);
    
    match UserRepository::find_by_username(&mut db, &credentials.username).await {
        Ok(user) => {
            match verify(&credentials.password, &user.password) {
                Ok(true) => Ok(json!("Success")),
                Ok(false) | Err(_) => Err(Custom(Status::Unauthorized, json!("Wrong credentials"))),
            }
        }
        Err(diesel::result::Error::NotFound) => Err(Custom(Status::Unauthorized, json!("Wrong credentials"))),
        Err(_) => Err(Custom(Status::InternalServerError, json!("Server Error"))),
    }
}

