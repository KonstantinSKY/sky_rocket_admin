pub use rocket_db_pools::Database;
pub use project::database::DbConn;

mod schema;
pub mod project;
pub mod commands;
mod settings;
pub mod auth;


