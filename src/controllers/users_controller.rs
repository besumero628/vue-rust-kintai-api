use crate::models::user::{NewUser, UpdateUser, User};
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::util::establish_connection;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use diesel::{prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub uid: String,
    pub family_name: String,
    pub given_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserData {
    pub family_name: String,
    pub given_name: String,
}

#[derive(Deserialize)]
pub struct ArticleQuery {
    uid: String,
}

pub async fn show(query: web::Query<ArticleQuery>) -> impl Responder {
    let connection = &mut establish_connection();
    let new_user_uid = &(query.uid);
    let result = users
        .filter(uid.eq(new_user_uid))
        .first::<User>(connection)
        .expect("Error loading user");

    HttpResponse::Ok().json(result)
}

pub async fn create(new_user_data: web::Json<UserData>) -> impl Responder {
    let connection = &mut establish_connection();
    let new_user = NewUser {
        uid: &(new_user_data.uid),
        family_name: &(new_user_data.family_name),
        given_name: &(new_user_data.given_name),
        email: &(new_user_data.email),
        password: &(new_user_data.password),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new user");

    let result = users::dsl::users
        .order(id.desc())
        .first::<User>(connection)
        .expect("Error finding users");

    HttpResponse::Ok().json(result)
}

pub async fn edit(user_id: web::Path<u64>) -> impl Responder {
    let connection = &mut establish_connection();
    let user_id = user_id.to_owned();
    let result = users::dsl::users
        .find(user_id)
        .first::<User>(connection)
        .expect("Error loading user");

    HttpResponse::Ok().json(result)
}

pub async fn update(
    user_id: web::Path<u64>,
    new_user_data: web::Json<UpdateUserData>,
) -> impl Responder {
    let user_id = user_id.to_owned();

    let connection = &mut establish_connection();
    let target = users::dsl::users.find(user_id);

    let update_user = UpdateUser {
        family_name: &(new_user_data.family_name),
        given_name: &(new_user_data.given_name),
        updated_at: Utc::now().naive_local(),
    };

    diesel::update(target)
        .set((
            family_name.eq(update_user.family_name),
            given_name.eq(update_user.given_name),
            updated_at.eq(update_user.updated_at),
        ))
        .execute(connection)
        .expect("Error updating user");

    HttpResponse::Ok().finish()
}
