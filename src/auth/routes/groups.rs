use chrono::Utc;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};
use rocket::{get, post, State};
use sea_orm::{entity::*, DatabaseConnection};
use super::super::models::group;
use crate::db::{select, insert};
use crate::project::responses;

#[get("/auth/groups")]
pub async fn get_all_groups(db: &State<DatabaseConnection>) -> Result<Json<Vec<group::Model>>, Status> {
    let result = select::select_all::<group::Entity>(db).await;
    responses::handle_selection_result(result)
}

// Post New group
#[derive(Deserialize)]
pub struct NewGroup {
    pub name: String,
    pub description: Option<String>,
}
// Handler to add a new group
#[post("/auth/groups", data = "<new_group>")]
pub async fn add_group(db: &State<DatabaseConnection>, new_group: Json<NewGroup>) -> Result<Json<group::Model>, Status> {
    
    let active_group = group::ActiveModel {
        name: Set(new_group.name.clone()),
        description: Set(new_group.description.clone()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let insert_result = insert::insert::<group::Entity, _>(db, active_group).await;
    responses::handle_insertion_result(insert_result)
}

// Handler to delete a user
#[delete("/auth/groups/<group_id>")]
pub async fn delete_group(db: &State<DatabaseConnection>, group_id: i32) -> Result <Status, Status> {
    let result = group::Entity::delete_by_id(group_id).exec(db.inner()).await;    // Correct
    responses::handle_deletion_result(result)

}

