use crate::schema::users;
use diesel::prelude::*;
use diesel::SqliteConnection;
#[derive(Queryable, Insertable, Selectable, Debug, PartialEq, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub last_name: String,
}
pub struct UserColumns {
    pub(super) id: users::id,
    pub(super) name: users::name,
    pub(super) last_name: users::last_name,
}
pub struct UserRepository {
    pub(super) connection: SqliteConnection,
    pub(super) user_table: users::table,
    pub(super) columns: UserColumns,
}
