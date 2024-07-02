use crate::auth::services::auth::get_jwt_token;

use super::super::repositories::users::UserRepository;
use super::Conn; //Import from mod.rs
use bcrypt::verify;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status::Custom;
use rocket::serde::json::{json, Json, Value};
use std::time::{SystemTime, UNIX_EPOCH};
use super::super::services;


#[derive(serde::Deserialize, Debug)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}



// #[rocket::post("/login", format="json", data="<credentials>")]
// pub async fn login(mut db: Conn, credentials: Json<Credentials>) -> Result<Value, Custom<Value>> {
//     println!("Credentials: {:?}", credentials);

//     match UserRepository::find_by_username(&mut db, &credentials.username).await {
//         Ok(user) => {
//             match verify(&credentials.password, &user.password) {
//                 Ok(true) => Ok(json!("Success")),
//                 Ok(false) | Err(_) => Err(Custom(Status::Unauthorized, json!("Wrong credentials"))),
//             }
//         }
//         Err(diesel::result::Error::NotFound) => Err(Custom(Status::Unauthorized, json!("Wrong credentials"))),
//         Err(_) => Err(Custom(Status::InternalServerError, json!("Server Error"))),
//     }
// }

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(mut db: Conn, credentials: Json<Credentials>) -> Result<Value, Custom<Value>> {
    println!("Credentials: {credentials:?}");

    let user = match UserRepository::find_by_username(&mut db, &credentials.username).await {
        Ok(user) => user,
        Err(diesel::result::Error::NotFound) => return Err(Custom(Status::Unauthorized, json!("Wrong credentials"))),
        Err(_) => return Err(Custom(Status::InternalServerError, json!("Server Error"))),
    };

    if services::auth::verify_password(&credentials.password, &user.password).is_err() {
        return Err(Custom(Status::Unauthorized, json!("Wrong credentials")));
    }

    match get_jwt_token(user.id, &user.username, &user.email, 3600) {
        Ok(token) => Ok(json!({ "token": token })),
        Err(_) => Err(Custom(Status::InternalServerError, json!("Server Token Error")))
    }
}

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Error((rocket::http::Status::Unauthorized, ()));
        }

        let token = keys[0].replace("Bearer ", "");
        let key = DecodingKey::from_secret("your_secret_key".as_ref());
        let validation = Validation::default();

        match decode::<Claims>(&token, &key, &validation) {
            Ok(token_data) => {
                let claims = token_data.claims;
                Outcome::Success(AuthenticatedUser {
                    username: claims.sub,
                })
            }
            Err(_) => Outcome::Error((rocket::http::Status::Unauthorized, ())),
        }
    }
}
