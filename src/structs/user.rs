use core::error::Error;

use crate::schema::users;
use diesel::prelude::*;
use diesel::SqliteConnection;
use rustc_serialize::Decodable;
use rustc_serialize::Decoder;
use rustc_serialize::Encodable;
#[derive(Queryable, Insertable, Selectable, Debug, PartialEq, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub last_name: String,
}

pub struct DecodeStruct {}

impl DecodeStruct {
    pub fn new<T: Decoder>(
        data: &mut T,
        struct_name: &str,
        index: usize,
        cb: impl FnOnce(&mut T) -> Result<T, <T as Decoder>::Error>,
    ) {
        data.read_struct(struct_name, index, cb);
    }

    pub fn decode_json_string_field<Type: rustc_serialize::Decoder>(
        field_name: &str,
        index: usize,
        json: &mut Type,
    ) -> String {
        match json.read_struct_field(field_name, index, |field| field.read_str()) {
            Ok(value) => value,
            Err(err) => "Error".to_string(),
        }
    }

    pub fn decode_json_i32_field<Type: rustc_serialize::Decoder>(
        field_name: &str,
        index: usize,
        json: &mut Type,
    ) -> i32 {
        match json.read_struct_field(field_name, 1, |field| field.read_i32()) {
            Ok(value) => value,
            Err(err) => panic!("JSON DECODE I32 ERROR"),
        }
    }
}

/* impl Decodable for User {
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
} */
