use iron::{request::Request, Plugin};
use rustc_serialize::{json, Decodable, Decoder};

use crate::{
    modules::http::http_response::HttpResponse,
    structs::user::User,
    SQL::users::structs::{UserColumns, UserRepository},
};

fn decode_json_string_field<Type: rustc_serialize::Decoder>(
    field_name: &str,
    index: usize,
    json: &mut Type,
) -> String {
    match json.read_struct_field(field_name, index, |field| field.read_str()) {
        Ok(value) => value,
        Err(err) => "Error".to_string(),
    }
}

fn decode_json_i32_field<Type: rustc_serialize::Decoder>(
    field_name: &str,
    index: usize,
    json: &mut Type,
) -> i32 {
    match json.read_struct_field(field_name, 1, |field| field.read_i32()) {
        Ok(value) => value,
        Err(err) => panic!("JSON DECODE I32 ERROR"),
    }
}

impl Decodable for User {
    fn decode<D: Decoder>(to_decode: &mut D) -> Result<Self, D::Error> {
        to_decode.read_struct("User", 3, |json| {
            let last_name = decode_json_string_field("last_name", 2, json);
            let name = decode_json_string_field("name", 1, json);
            let id = decode_json_i32_field("id", 0, json);
            Ok(User {
                id,
                last_name,
                name,
            })
        })
    }
}

pub fn create_user_route(request: &mut Request) -> iron::Response {
    let mut user_repository = UserRepository::new();
    let request_body = match request.get_ref::<bodyparser::Json>() {
        Ok(json) => json.clone().unwrap().to_string(),
        Err(err) => panic!("{}", err),
    };
    let request_body = match json::decode::<User>(&request_body) {
        Ok(res) => res,
        Err(err) => panic!("{}", err),
    };
    user_repository.create(request_body.clone());
    HttpResponse::new()
        .set_json_headers()
        .set_response_body("ok")
        .get_instance()
        .unwrap()
}
