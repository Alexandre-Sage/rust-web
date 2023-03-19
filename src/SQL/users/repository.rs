
use super::structs::{User, UserColumns, UserRepository};
use crate::{schema::users, SQL::db_connection::DbConnection};
use diesel::{prelude::*};

impl UserRepository {
    pub fn new() -> Self {
        Self {
            connection: DbConnection("./database.sqlite"),
            user_table: users::table,
            columns: UserColumns {
                id: users::id,
                name: users::name,
                last_name: users::last_name,
            },
        }
    }
    pub async fn create(&mut self, user: User) {
        match diesel::insert_into(self.user_table)
            .values(&user)
            .execute(&mut self.connection)
        {
            Ok(data) => data,
            Err(e) => panic!("Diesel err {}", e),
        };
    }
    pub async fn get_all(&mut self) -> Vec<User> /* -> Result<Vec<User>, diesel::result::Error> */ {
        match self
            .user_table
            .select(users::all_columns)
            .load::<User>(&mut self.connection)
        {
            Ok(data) => data,
            Err(err) => panic!("Get err {}", err),
        }
    }
    pub async fn get_by_id(&mut self, id: i32) -> User {
        match self
            .user_table
            .filter(self.columns.id.eq(id))
            .select(users::all_columns)
            .first::<User>(&mut self.connection)
        {
            Ok(data) => data,
            Err(err) => panic!("{}", err),
        }
    }
}


