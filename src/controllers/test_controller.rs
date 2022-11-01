use crate::models::tests::{NewTest, Test};
use crate::schema::tests;
use crate::schema::tests::dsl::*;
use crate::util::establish_connection;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestData {
    name: String,
}

pub async fn index() -> impl Responder {
    let connection = &mut establish_connection();
    let results = tests.load::<Test>(connection).expect("Error loading tests");

    HttpResponse::Ok().json(results)
}

pub async fn show(info: web::Path<u64>) -> impl Responder {
    let connection = &mut establish_connection();
    let user_id = info.to_owned();
    let result = tests::dsl::tests
        .find(user_id)
        .load::<Test>(connection)
        .expect("Error loading test");

    HttpResponse::Ok().json(result)
}

pub async fn new() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn edit(info: web::Path<u64>) -> impl Responder {
    let connection = &mut establish_connection();
    let user_id = info.to_owned();
    let result = tests::dsl::tests
        .find(user_id)
        .load::<Test>(connection)
        .expect("Error loading test");

    HttpResponse::Ok().json(result)
}

pub async fn create(item: web::Json<TestData>) -> impl Responder {
    let connection = &mut establish_connection();
    let new_test = NewTest {
        name: &(item.name)
    };

    diesel::insert_into(tests::table)
        .values(&new_test)
        .execute(connection)
        .expect("Error saving new test");

    let result = tests::dsl::tests
        .order(id.desc())
        .first::<Test>(connection)
        .expect("Error finding tests");

    HttpResponse::Ok().json(result)
}

pub async fn update(info: web::Path<u64>) -> impl Responder {
    let user_id = info.to_owned();

    let connection = &mut establish_connection();
    let target = tests::dsl::tests.find(user_id);

    diesel::update(target)
        .set(tests::name.eq("update name"))
        .execute(connection)
        .expect("Error updating tests");

    HttpResponse::Ok().finish()
}

pub async fn delete(info: web::Path<u64>) -> impl Responder {
    let user_id = info.to_owned();

    let connection = &mut establish_connection();
    let target = tests::dsl::tests.find(user_id);

    diesel::delete(target)
        .execute(connection)
        .expect("Error deleteing tests");

    HttpResponse::Ok().finish()
}
