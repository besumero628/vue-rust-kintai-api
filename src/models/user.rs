use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub uid: String,
    pub family_name: String,
    pub given_name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub uid: &'a str,
    pub family_name: &'a str,
    pub given_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug ,Deserialize ,Insertable)]
#[table_name = "users"]
pub struct UpdateUser<'a> {
    pub family_name: &'a str,
    pub given_name: &'a str,
    pub updated_at: chrono::NaiveDateTime
}
