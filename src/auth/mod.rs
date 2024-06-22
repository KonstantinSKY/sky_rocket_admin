pub mod routes;
pub mod models;
pub mod repositories;
pub mod services;

use crate::project::AppSettings;
use rocket::routes;
use routes as R;

// Setting app application HERE
pub fn get_app_settings() -> AppSettings {   
    AppSettings { 
        name : "auth", 
        verbose_name: "Authentication",
        routes: routes![
          R::hi_json::hi_json,
          R::hello::hello,
          R::users::get_all_users,
          R::users::create_user,
          R::users::delete_user,
          R::users::update_user,
          R::groups::get_all_groups,
          R::groups::create_group,
          R::groups::delete_group,
          R::groups::update_group,
          R::authorization::login,
          // R::user_group::get_all,
          // R::user_group::add_one,
          ],
  } 
}