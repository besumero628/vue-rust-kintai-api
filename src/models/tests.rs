use crate::schema::tests;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct Test {
    pub id: u64,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "tests"]
pub struct NewTest<'a> {
    pub name: &'a str,
}
