use diesel::prelude::*;
use diesel_async::{AsyncPgConnection as Conn, RunQueryDsl};
// use users::email;
use super::super::models::users::{NewSuperUser, NewUser, UpdateUser, User};
use crate::schema::users;

pub struct UserRepository;

impl UserRepository {
    /// Selects all users from the database.
    /// 
    /// # Errors
    /// 
    /// This function will return a `QueryResult` error if there is an issue
    /// loading the users from the database.
    pub async fn select_all(conn: &mut Conn) -> QueryResult<Vec<User>> {
        users::table.load::<User>(conn).await
    }
    
    /// Finds a user by their username in the database.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to the database connection.
    /// * `username` - A reference to the username string to search for.
    ///
    /// # Errors
    /// This function will return a `QueryResult` error if there is an issue
    /// querying the database or if no user with the given username is found.
    pub async fn find_by_username(conn: &mut Conn, username: &String) -> QueryResult<User> {
        users::table
            .filter(users::username.eq(username))
            .get_result(conn)
            .await
    }

    // pub async fn get_user_token_data(conn: &mut Conn, username: &str) -> QueryResult<UserTokenData> {
    //     use crate::schema::users::dsl::{users, id, username as uname, email};
    //     users
    //         .filter(uname.eq(username))
    //         .select((id, uname, email))
    //         .first::<UserTokenData>(conn)
    //         .await
    // }

    /// Creates a new superuser in the database.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to the database connection.
    /// * `new_user` - The new user data to insert.
    ///
    /// # Returns
    /// * `QueryResult<User>` - The inserted user or an error if the insertion fails.
    ///
    /// # Errors
    /// This function will return a `QueryResult::Err` if the insertion into the database fails.
    pub async fn create_superuser(conn: &mut Conn, new_user: NewSuperUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(conn)
            .await
    }
    
    /// Creates a new user in the database.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to the database connection.
    /// * `new_user` - The new user data to insert.
    ///
    /// # Returns
    /// * `QueryResult<User>` - The inserted user or an error if the insertion fails.
    ///
    /// # Errors
    /// This function will return a `QueryResult::Err` if the insertion into the database fails.
    pub async fn create(conn: &mut Conn, new_user: NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(conn)
            .await
    }

    // pub async fn create_user_with_groups(conn: &mut Conn, new_user: NewUser, role_codes: Vec<i32>) -> QueryResult<User> {
    //     let user_with_hashed_password = NewUser {
    //         password: crypto::hash_password(new_user.password),
    //         ..new_user
    //     };

    //     let user = diesel::insert_into(users::table)
    //         .values(new_user)
    //         .get_result::<User>(conn)
    //         .await?;

    //     for group_code in group_codes {
    //         let new_user_role = {
    //             if let Ok(role) = RoleRepository::find_by_code(c, &role_code).await {
    //                 NewUserRole { user_id: user.id, role_id: role.id }
    //             } else {
    //                 let name = role_code.to_string();
    //                 let new_role = NewRole { code: role_code, name };
    //                 let role = RoleRepository::create(c, new_role).await?;
    //                 NewUserRole { user_id: user.id, role_id: role.id }
    //             }
    //         };

    //         diesel::insert_into(users_roles::table)
    //             .values(new_user_role)
    //             .get_result::<UserRole>(c)
    //             .await?;
    //     }

    //     Ok(user)
    // }
    
    /// Updates a user in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `id` - The ID of the user to update.
    /// * `user` - The user data to update.
    ///
    /// # Errors
    ///
    /// This function will return a `QueryResult` error if there is an issue
    /// updating the user in the database.
    pub async fn update(conn: &mut Conn, id: i32, user: UpdateUser) -> QueryResult<User> {
        diesel::update(users::table.find(id))
            .set((
                users::username.eq(user.username),
                users::email.eq(user.email),
                users::first_name.eq(user.first_name),
            ))
            .get_result(conn)
            .await
            .map_err(|e| {
                eprintln!("Database update error: {e:?}");
                e
            })
    }
    
    /// Deletes a user from the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    /// * `id` - The ID of the user to delete.
    ///
    /// # Errors
    ///
    /// This function will return a `QueryResult` error if there is an issue
    /// deleting the user from the database.
    pub async fn delete(conn: &mut Conn, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(conn).await
    }
}
