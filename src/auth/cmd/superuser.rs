use diesel_async::{AsyncPgConnection, AsyncConnection};
use dotenv::dotenv;
use validator::Validate;


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
    println!("Username: {username}");
    println!("Email: {email}");

    let new_user = NewSuperUser{
        username,
        email,
        password,
        is_staff: true,
        is_superuser: true,
    };

    if let Err(validation_errors) = new_user.validate() {
        for (field, errors) in validation_errors.field_errors() {
            for error in errors {
                if let Some(message) = &error.message {
                    println!("Validation error on {field}: {message}");
                } else {
                    println!("Validation error on {field}: {error:?}");
                }
            }
        }
        return;
    }

    let new_user = match new_user.add_hashed_password() {
        Ok(user) => user,
        Err(e) => {
            println!("Hashing error {e}");
            return;
        }
    };
// 
    let _ = UserRepository::create_superuser(&mut conn, new_user)
        .await
        .map(|user| println!("Created Superuser: {0}  with email: {1}", user.username, user.email))
        .map_err(|e| println!("Superuser Not Created,  Error: {e}"));

}
