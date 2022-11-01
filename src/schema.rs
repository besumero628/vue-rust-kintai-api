// @generated automatically by Diesel CLI.

diesel::table! {
    organizations (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        prescribed_working_hours -> Nullable<Float>,
    }
}

diesel::table! {
    stamp_types (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    tests (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        family_name -> Varchar,
        given_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        organization_id -> Nullable<Unsigned<Bigint>>,
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
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::joinable!(users -> organizations (organization_id));
diesel::joinable!(works -> stamp_types (stamp_type_id));
diesel::joinable!(works -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    organizations,
    stamp_types,
    tests,
    users,
    works,
);
