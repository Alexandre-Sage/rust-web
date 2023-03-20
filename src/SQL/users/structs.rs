use crate::schema::users;
use diesel::SqliteConnection;

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
