// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Integer,
        content -> Text,
        title -> Text,
        sending_date -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        last_name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    users,
);
