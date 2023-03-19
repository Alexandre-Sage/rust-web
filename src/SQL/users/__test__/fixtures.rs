pub mod user_repo_fixture {
    use crate::schema::users;
    use crate::SQL::db_connection::DbConnection;
    use crate::SQL::users::structs::User;
    use diesel::prelude::*;
    use rand::random;
    const CONN: &str = "./database.sqlite";
    pub fn instanciate_user(id: i32) -> User {
        User {
            id,
            name: format!("Test-{}", id),
            last_name: format!("Test-{}", id),
        }
    }
    pub async fn before_test() {
        let ids: Vec<i32> = vec![
            random::<i32>(),
            random::<i32>(),
            random::<i32>(),
            random::<i32>(),
        ];
        let users: Vec<User> = ids.iter().map(|id| instanciate_user(*id)).collect();
        match diesel::insert_into(users::table)
            .values(users)
            .execute(&mut DbConnection(CONN))
        {
            Ok(data) => data,
            Err(err) => panic!("Err before, {}", err),
        };
    }
    pub async fn after_test() {
        match diesel::delete(users::table).execute(&mut DbConnection(CONN)) {
            Ok(d) => d,
            Err(e) => panic!("After {}", e),
        };
    }
}
