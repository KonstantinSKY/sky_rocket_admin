use diesel::result::DatabaseErrorKind;
use diesel_async::{AsyncPgConnection, AsyncConnection};
use dotenv::dotenv;
use validator::Validate;

use crate::auth::cmd::superuser;

use super::super::services::auth;
use super::super::repositories::users::UserRepository;
use super::super::models::users::NewSuperUser;
//TODO move to mod.rs
pub async fn load_db_connection() -> AsyncPgConnection {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("Cannot load DB url from environment");
    AsyncPgConnection::establish(&database_url).await
        .expect("Cannot connect to Postgres")
}

pub async fn create(username: String, email: String, password: String) {
    let mut conn = load_db_connection().await;
    println!("Username: {}", username);
    println!("Email: {}", email);
    // println!("Password: {}", password);
    

    // match UserRepository::find_by_username(&mut conn, &username).await {
    //     Ok(User) => {
    //         println!("User with username {} already exists", username);
    //         println!()
    //         return;
    //     }
    //     Err(diesel::result::Error::NotFound) => {
    //         println!("Creating user with username: {}", username);
    //     }
    //     Err(others) => {
    //         println!("An error occurred: {:?}", others);
    //         return;
    //     }
    // }

    // let password_hash = auth::hash_password(password);
    // println!("Hashed Password: {}", password_hash);
    let new_user = NewSuperUser{
        username,
        email,
        password,
        is_staff: true,
        is_superuser: true,
    };
    match  new_user.validate(){
        Ok(_) => {
            println!("Validation successful");
            match UserRepository::create_superuser(&mut conn, new_user).await {
                    Ok(user) => {
                        println!("Added Super User with username: {}", user.username);
                    }
                    Err(e) => {
                        println!("An error occurred: {:?}", e);
                    }
                }
        }
        Err(errors) => {
            for (field, errors) in errors.field_errors() {
                for error in errors {
                    if let Some(message) = &error.message {
                        println!("Validation error on {}: {}", field, message);
                    } else {
                        println!("Validation error on {}: {:?}", field, error);
                    }
                }
            }
        }
    };
    // println!("new use {:?}",new_user);
    // match UserRepository::create_superuser(&mut conn, new_user).await {
    //     Ok(user) => {
    //         println!("User with username {} added", user.username);
    //     }
    //     Err(e) => {
    //         println!("An error occurred: {:?}", e);
    //     }
    // }
    
}
