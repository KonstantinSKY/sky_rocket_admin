use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncPgConnection as Conn};

use super::super::models::users::*;
use crate::schema::*;

pub struct UserRepository;

impl UserRepository {
    pub async fn select_all(conn: &mut Conn) -> QueryResult<Vec<User>> {
        users::table.load::<User>(conn).await
    }

    // pub async fn select_all(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
    //     users::table.find(id).get_result(conn).await
    // }
    // pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
    //     rustaceans::table.limit(limit).load(c).await
    // }

    pub async fn create(conn: &mut Conn, new_user: NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(conn)
            .await
    }

    pub async fn update(conn: &mut Conn, id: i32, user: User) -> QueryResult<User> {
        diesel::update(users::table.find(id))
            .set((
                users::username.eq(user.username),
                users::email.eq(user.email),
                users::first_name.eq(user.first_name),
            ))
            .get_result(conn)
            .await
            .map_err(|e| {
                eprintln!("Database update error: {:?}", e);
                e
            })
    }

    pub async fn delete(conn: &mut Conn, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(conn).await
    }
}

