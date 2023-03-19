#[cfg(test)]
mod user_repo_test_suite {
    use crate::schema::users;
    use crate::SQL::db_connection::DbConnection;
    use crate::SQL::users::__test__::fixtures::user_repo_fixture::{
        after_test, before_test, instanciate_user,
    };
    use crate::SQL::users::structs::{User, UserRepository};
    use diesel::prelude::*;
    use rand::random;
    const CONN: &str = "./database.sqlite";
    #[async_std::test]
    async fn create_user_test() {
        let mut repository: UserRepository = UserRepository::new();
        let id: i32 = random::<i32>();
        let new_user = instanciate_user(id);
        repository.create(new_user.clone()).await;
        let user_from_db = match users::table
            .filter(users::id.eq(id))
            .select(users::all_columns)
            .first::<User>(&mut DbConnection(CONN))
        {
            Ok(data) => data,
            Err(err) => panic!("ALALALLALALA {}", err),
        };
        assert_eq!(user_from_db, new_user);
        after_test().await;
    }
    #[async_std::test]
    async fn get_all_users_test() {
        before_test().await;
        let mut repository: UserRepository = UserRepository::new();
        let users_from_db = repository.get_all().await;
        assert_eq!(users_from_db.len(), 4);
        after_test().await;
    }
}
