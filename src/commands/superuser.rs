use diesel_async::{AsyncPgConnection, AsyncConnection};
use dotenv::dotenv;

async fn load_db_connection() -> AsyncPgConnection {
    dotenv.ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("Cannot load DB url from environment");
    AsyncPgConnection::establish(&database_url).await
        .expect("Cannot connect to Postgres")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection().await;

    let password_hash = auth::hash_password(password).unwrap();
    let new_user = NewUser { username, password: password_hash };
    let role_enums = role_codes.iter().map(|v| RoleCode::from_str(v.as_str()).unwrap()).collect();
    let user = UserRepository::create(&mut c, new_user, role_enums).await.unwrap();
    println!("User created {:?}", user);
    let roles = RoleRepository::find_by_user(&mut c, &user).await.unwrap();
    println!("Roles assigned {:?}", roles);
}
