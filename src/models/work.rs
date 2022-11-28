use crate::schema::works;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct Work {
    pub id: u64,
    pub user_id: u64,
    pub stamp_type_id: u64,
    pub stamp: chrono::NaiveDateTime,
    pub enabled_flag: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "works"]
pub struct NewWork<'a> {
    pub user_id: &'a u64,
    pub stamp_type_id: &'a u64,
}

#[derive(Insertable)]
#[table_name = "works"]
pub struct UpdateWork<'a> {
    pub user_id: &'a u64,
    pub stamp_type_id: &'a u64,
    pub stamp: &'a chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "works"]
pub struct DeleteWork<'a> {
    pub user_id: &'a u64,
    pub stamp_type_id: &'a u64,
}