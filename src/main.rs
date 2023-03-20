pub mod SQL;
pub mod modules;
pub mod router;
pub mod schema;
pub mod structs;
extern crate iron;
use crate::router::router::router;
use iron::Iron;
use SQL::users::structs::UserRepository;

const PORT: i32 = 3000;
const HOST: &str = "localhost:3000";
#[macro_use]
extern crate mime;
extern crate rustc_serialize;
fn main() {
    println!("Server is runing on {}", PORT);
    Iron::new(router()).http(HOST).unwrap();
}
