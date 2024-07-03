// pub mod errors;
// pub mod responses;
// pub mod validators;
pub mod database;

use dotenv::dotenv;
use std::env;

use crate::{schema::groups::name, settings as project_setting};
use rocket::Route;

const PROJECT_NAME: &str = "Sky Rocket Admin";
const PROJECT_VERBOSE_NAME: &str = "Sky Rocket Admin System";

pub struct AppSettings {
    pub name: &'static str,
    pub verbose_name: &'static str,
    pub routes: Vec<Route>,
}

pub struct ProjectSettings {
    pub name: String, 
    pub verbose_name: String,
    pub secret_key: String,
}

pub struct GlobalSettings {
    pub project_settings: ProjectSettings, 
    pub apps_settings: Vec<AppSettings>,
    pub routes: Vec<Route>,
}

impl GlobalSettings {
    #[must_use]
    pub fn new() -> Self {
        dotenv().ok(); // Load environment variables

        //secret key
        let secret_key = env::var(project_setting::SECRET_KEY_NAME)
            .map_err(|_| "SECRET KEY must be set. Set as default".to_string())
            .unwrap_or_else(|_| "XXXX-DEFAULT_SECRET_KEY-YYYY".to_string());

        let project_settings: ProjectSettings = ProjectSettings {
            name: PROJECT_NAME.to_owned(),
            verbose_name: PROJECT_VERBOSE_NAME.to_owned(),
            secret_key,
        };

        //app settings
        let apps_settings = project_setting::app_setting_collection();

        // Collection app route
        let mut routes = Vec::<Route>::new();
        for app_settings in &apps_settings {
            routes.extend(app_settings.routes.clone());
        }

        Self {
            project_settings,
            apps_settings,
            routes,
        }
    }
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self::new()
    }
}
