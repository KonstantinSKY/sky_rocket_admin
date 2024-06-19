use rocket_db_pools::{Connection, Database};
use std::env;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

mod settings;

#[rocket::main]
async fn main() {
    dotenv::dotenv().ok();
      // Print DATABASE_URL to verify it is loaded
      println!("DATABASE_URL: {:?}", env::var("DATABASE_URL"));
    let _ = rocket::build()
        .mount("/", rocket::routes![
        ])
        .attach(DbConn::init())
        .launch()
        .await;
}
