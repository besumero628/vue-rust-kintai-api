// @generated automatically by Diesel CLI.

diesel::table! {
    stamp_types (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        uid -> Varchar,
        family_name -> Varchar,
        given_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    works (id) {
        id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        stamp_type_id -> Unsigned<Bigint>,
        stamp -> Datetime,
        enabled_flag -> Bool,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::joinable!(works -> stamp_types (stamp_type_id));
diesel::joinable!(works -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    stamp_types,
    users,
    works,
);
