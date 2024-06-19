// pub mod errors;
// pub mod responses;
// pub mod validators;
pub mod database;

use std::env;
use dotenv::dotenv;

use rocket::{figment::Figment, Config, Route};
use crate::settings as project_setting;
// use rocket_okapi::settings;

pub struct AppSettings {
    pub name: &'static str,
    pub verbose_name: &'static str,
    pub routes: Vec<Route>,
}

pub struct ProjectSettings {
    pub secret_key: String,  
    pub figment: Figment,
    pub apps_settings: Vec<AppSettings>,
    pub routes: Vec<Route>,
}

impl ProjectSettings {
    pub fn new () -> Self {
        dotenv().ok();      // Load environment variables

        //secret key
        let secret_key = env::var(project_setting::SECRET_KEY_NAME).expect("SECRET KEY must be set");
        let figment = Config::figment()   
                                .merge(("secret_key", secret_key.clone()));
        
        //app settings
        let apps_settings = project_setting::app_setting_collection();

        // Collection app route        
        let mut routes = Vec::<Route>::new();
        for app_settings in &apps_settings{
            routes.extend(app_settings.routes.clone());
        }
        
        Self {
            secret_key,
            figment,
            apps_settings,
            routes,
        }        
    }
}