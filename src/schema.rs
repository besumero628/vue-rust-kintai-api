// @generated automatically by Diesel CLI.

diesel::table! {
    organization (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
        prescribed_working_hours -> Nullable<Float>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
