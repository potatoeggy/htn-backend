// @generated automatically by Diesel CLI.

diesel::table! {
    skills (id) {
        id -> Integer,
        user_id -> Integer,
        name -> Text,
        rating -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        company -> Text,
        email -> Text,
        phone -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    skill_frequencies (name) {
        name -> Text,
        frequency -> Integer,
    }
}

diesel::joinable!(skills -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(skills, users,);
