pub mod hello;
pub mod hi_json;
pub mod users;
pub mod groups;
// pub mod user_group;
pub mod authorization;

use rocket_db_pools::Connection;
use crate::project::database::DbConn; 

type Conn = Connection<DbConn>;