extern crate sky_rocket_admin as app;

use app::{project, Database};

#[rocket::main]
async fn main() {
    let sets = project::GlobalSettings::new(); 
    let _ = rocket::build()
        .manage(sets.project_settings)
        .mount("/", sets.routes)                                  
        .attach(app::DbConn::init())
        .launch()
        .await;
}