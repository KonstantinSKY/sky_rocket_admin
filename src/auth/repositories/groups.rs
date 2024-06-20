// === Groups Repository ===
use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncPgConnection as Conn};
use super::super::models::groups::*;
use crate::schema::*;

pub struct GroupRepository;

impl GroupRepository {
    pub async fn select_all(conn: &mut Conn) -> QueryResult<Vec<Group>> {
        groups::table.load::<Group>(conn).await
    }

    pub async fn create(conn: &mut Conn, new_group: NewGroup) -> QueryResult<Group> {
        diesel::insert_into(groups::table)
            .values(new_group)
            .get_result(conn)
            .await
    }

    pub async fn update(conn: &mut Conn, id: i32, new_group: NewGroup) -> QueryResult<Group> {
        diesel::update(groups::table.find(id))
            .set((
                groups::name.eq(new_group.name),
                groups::description.eq(new_group.description),
            ))
            .get_result(conn)
            .await
            .map_err(|e| {
                eprintln!("Database update error: {:?}", e);
                e
            })
    }

    pub async fn delete(conn: &mut Conn, id: i32) -> QueryResult<usize> {
        diesel::delete(groups::table.find(id)).execute(conn).await
    }
}
