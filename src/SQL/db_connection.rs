use diesel::{Connection, SqliteConnection};
#[allow(non_snake_case)]
pub fn DbConnection(string: &str) -> SqliteConnection {
    match SqliteConnection::establish(string) {
        Ok(ok) => ok,
        Err(_err) => panic!("Db err"),
    }
}
