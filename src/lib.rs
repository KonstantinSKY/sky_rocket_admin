#![warn(clippy::pedantic)]  

pub use rocket_db_pools::Database;
pub use project::database::DbConn;

mod schema;
pub mod project;
mod settings;
pub mod auth;


