use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};
use rocket::{get, post, State};
use sea_orm::{entity::*, Condition, DatabaseConnection, QueryFilter};
use super::super::models::user_group;
use crate::db::{select, insert};
use crate::project::responses;

#[get("/auth/users_groups")]
pub async fn get_all(db: &State<DatabaseConnection>) -> Result<Json<Vec<user_group::Model>>, Status> {
    let result = select::select_all::<user_group::Entity>(db).await;
    responses::handle_selection_result(result)
}

// Post New User group
#[derive(Deserialize)]
pub struct NewUserGroup {
    pub user_id: i32,
    pub group_id: i32,
}
// Handler to add a new group
#[post("/auth/users_groups", data = "<new_user_group>")]
pub async fn add_one(db: &State<DatabaseConnection>, new_user_group: Json<NewUserGroup>) -> Result<Json<user_group::Model>, Status> {
    
    let active_user_group = user_group::ActiveModel {
        user_id: Set(new_user_group.user_id),
        group_id: Set(new_user_group.group_id),
        ..Default::default()
    };

    let insert_result = insert::insert::<user_group::Entity, _>(db, active_user_group).await;
    responses::handle_insertion_result(insert_result)
}
#[derive(Deserialize)]
pub struct UserGroupParams {
    pub user_id: i32,
    pub group_id: i32,
}
// Handler to delete a user group by user_id and group_id
#[delete("/auth/users_groups", data = "<params>")]
pub async fn delete_group(db: &State<DatabaseConnection>, params: Json<UserGroupParams>) -> Result<Status, Status> {
    let delete_result = user_group::Entity::delete_many()
        .filter(
            Condition::all()
                .add(user_group::Column::UserId.eq(params.user_id))
                .add(user_group::Column::GroupId.eq(params.group_id)),
        )
        .exec(db.inner())
        .await;

    responses::handle_deletion_result(delete_result)
}