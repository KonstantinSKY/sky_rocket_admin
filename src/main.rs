use rocket_db_pools::Database;
use project::database::DbConn;

mod schema;
mod project;
mod settings;
mod auth;

#[rocket::main]
async fn main() {
    let sets = project::ProjectSettings::new(); 
    let _ = rocket::custom(sets.figment)
        .mount("/", sets.routes)                                  //Add 
        .attach(DbConn::init())
        .launch()
        .await;
}
