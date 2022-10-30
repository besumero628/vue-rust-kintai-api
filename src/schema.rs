// @generated automatically by Diesel CLI.

diesel::table! {
    organizations (id) {
        id -> Integer,
        name -> Varchar,
        prescribed_working_hours -> Nullable<Float>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    stamp_types (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        family_name -> Varchar,
        given_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        organization_id -> Nullable<Integer>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    works (id) {
        id -> Integer,
        user_id -> Integer,
        stamp_type_id -> Integer,
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
    users,
    works,
);
