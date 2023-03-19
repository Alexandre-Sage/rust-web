pub mod SQL;
pub mod modules;
pub mod router;
pub mod schema;
use crate::router::router::router;
use iron::Iron;
use SQL::users::structs::{User, UserRepository};
const PORT: i32 = 3000;
const HOST: &str = "localhost:3000";

fn main() {
    println!("Server is runing on {}", PORT);
    Iron::new(router()).http(HOST).unwrap();
}
