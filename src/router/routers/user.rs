use iron::Request;
use rustc_serialize::Decodable;

use crate::SQL::users::structs::User;

fn decode_json_field(field_name: &str) {}

impl Decodable for User {
    fn decode<D: rustc_serialize::Decoder>(to_decode: &mut D) -> Result<Self, D::Error> {
        to_decode.read_struct("JsonRequest", 3, |json| {
            let title = json.read_struct_field("title", 1, |title| title.read_str())?;
            let message = json.read_struct_field("message", 0, |message| message.read_str())?;
            Ok(User {
                id,
                last_name,
                name,
            })
        })
    }
}

pub fn create_user_route(request: Request) {}
