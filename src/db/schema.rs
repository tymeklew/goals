use diesel::prelude::*;

table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        first_name -> Text,
        last_name -> Text,
        password -> Text
    }
}

table! {
    goals (id) {
        id -> Uuid,
        user_id -> Uuid,
        title -> Text,
        description -> Text
    }
}

table! {
    sessions (id) {
        id -> Uuid,
        user_id -> Uuid
    }
}

diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(users, sessions,);
