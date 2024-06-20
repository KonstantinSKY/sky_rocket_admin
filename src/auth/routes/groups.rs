use rocket::response::status::{Custom, NoContent};
use rocket::{get, post, delete, put, http::Status};
use rocket::serde::json::{Json, json, Value};
use validator::Validate;

use super::super::{
    repositories::groups::GroupRepository,
    models::groups::*
};

use super::Conn;                                    // Connection<DbConn> from mod.rs

#[get("/auth/groups")]
pub async fn get_all_groups(mut db: Conn) -> Result<Value, Custom<Value>> {
    GroupRepository::select_all(&mut db).await
    .map(|groups | json!(groups))
    .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[post("/auth/groups", format="json", data="<new_group>")]
pub async fn create_group(mut db: Conn, new_group: Json<NewGroup> ) -> Result<Custom<Value>, Custom<Value>> {
    if new_group.validate().is_err() {
        return Err(Custom(Status::UnprocessableEntity, json!("Validation Error")));
    }
    GroupRepository::create(&mut db, new_group.into_inner()).await
        .map(|group| Custom(Status::Created, json!(group)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[delete("/auth/groups/<id>")]
pub async fn delete_group(mut db: Conn, id: i32) -> Result<NoContent, Custom<Value>> {
    GroupRepository::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[put("/auth/groups/<id>", format="json", data="<new_group>")]
pub async fn update_group(mut db: Conn, id: i32, new_group: Json<NewGroup>) -> Result<Value, Custom<Value>> {
    if new_group.validate().is_err() {
        return Err(Custom(Status::UnprocessableEntity, json!("Validation Error")));
    }
    GroupRepository::update(&mut db, id, new_group.into_inner()).await
        .map(|group| json!(group))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

