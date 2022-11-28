use crate::models::work::{DeleteWork, NewWork, UpdateWork, Work};
use crate::schema::works;
use crate::schema::works::dsl::*;
use crate::util::establish_connection;
use actix_web::{web, HttpResponse, Responder};
use chrono::{prelude::*, Duration};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetWorkData {
    pub user_id: u64,
    pub year: i32,
    pub month: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWorkDataQuery {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub min: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkData {
    pub user_id: u64,
    pub stamp_type_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWorkData {
    pub user_id: u64,
    pub stamp_type_id: u64,
}

pub async fn index(target_info: web::Query<TargetWorkData>) -> impl Responder {
    let connection = &mut establish_connection();
    let current_user_id = &(target_info.user_id);
    let start_year = target_info.year;
    let start_month = target_info.month;
    let mut _end_year = target_info.year;
    let mut _end_month = &(start_month) + 1;

    if start_month == 12 {
        _end_year = target_info.year + 1;
        _end_month = 1
    }

    let target_start_date = Utc.ymd(start_year, start_month, 1).and_hms(0, 0, 0);
    let target_end_date = Utc.ymd(_end_year, _end_month, 1).and_hms(0, 0, 0);
    let start_datetime = (target_start_date - Duration::hours(9)).naive_utc();
    let end_datetime = (target_end_date - Duration::hours(9) - Duration::seconds(1)).naive_utc();

    let result = works
        .filter(user_id.eq(current_user_id))
        .filter(stamp.gt(start_datetime))
        .filter(stamp.lt(end_datetime))
        .filter(enabled_flag.eq(true))
        .load::<Work>(connection)
        .expect("Error loading works");

    HttpResponse::Ok().json(result)
}

pub async fn create(target_info: web::Query<UpdateWorkDataQuery>, new_work_data: web::Json<WorkData>) -> impl Responder {
    let year = target_info.year;
    let month = target_info.month;
    let day = target_info.day;

    let target_start_date = Utc.ymd(year, month, day).and_hms(0, 0, 0);
    let target_end_date = Utc.ymd(year, month, day).and_hms(23, 59, 59);
    let start_datetime = (target_start_date - Duration::hours(9)).naive_utc();
    let end_datetime = (target_end_date - Duration::hours(9) - Duration::seconds(1)).naive_utc();
    
    let connection = &mut establish_connection();
    let new_work = NewWork {
        user_id: &(new_work_data.user_id),
        stamp_type_id: &(new_work_data.stamp_type_id),
    };
    

    let same_stamp_count = works
        .filter(user_id.eq(new_work.user_id))
        .filter(stamp_type_id.eq(new_work.stamp_type_id))
        .filter(stamp.gt(start_datetime))
        .filter(stamp.lt(end_datetime))
        .count()
        .get_result::<i64>(connection)
        .unwrap();

    if same_stamp_count != 0 {
        let previous_stamp = works
            .filter(user_id.eq(new_work.user_id))
            .filter(stamp_type_id.eq(new_work.stamp_type_id))
            .filter(stamp.gt(start_datetime))
            .filter(stamp.lt(end_datetime))
            .order(id.desc())
            .first::<Work>(connection)
            .expect("Error previous Work Search");

        let target = works.find(previous_stamp.id);

        diesel::update(target)
            .set(enabled_flag.eq(false))
            .execute(connection)
            .expect("Error updating work");
    }

    diesel::insert_into(works::table)
        .values(&new_work)
        .execute(connection)
        .expect("Error saving new work");

    let result = works::dsl::works
        .order(id.desc())
        .first::<Work>(connection)
        .expect("Error finding work");

    HttpResponse::Ok().json(result)
}

pub async fn update(target_info: web::Query<UpdateWorkDataQuery>, new_work_data: web::Json<UpdateWorkData>) -> impl Responder {
    println!("{:?}", target_info);
    
    let year = target_info.year;
    let month = target_info.month;
    let day = target_info.day;
    let hour = target_info.hour;
    let min = target_info.min;

    let work_stamp_utc = Utc.ymd(year, month, day).and_hms(hour, min, 0);
    let work_stamp_naive = (work_stamp_utc - Duration::hours(9)).naive_local();

    let target_start_date = Utc.ymd(year, month, day).and_hms(0, 0, 0);
    let target_end_date = Utc.ymd(year, month, day).and_hms(23, 59, 59);
    let start_datetime = (target_start_date - Duration::hours(9)).naive_utc();
    let end_datetime = (target_end_date - Duration::hours(9) - Duration::seconds(1)).naive_utc();

    let connection = &mut establish_connection();
    let new_work = UpdateWork {
        user_id: &(new_work_data.user_id),
        stamp_type_id: &(new_work_data.stamp_type_id),
        stamp: &work_stamp_naive
    };

    let same_stamp_count = works
        .filter(user_id.eq(new_work.user_id))
        .filter(stamp_type_id.eq(new_work.stamp_type_id))
        .filter(stamp.gt(start_datetime))
        .filter(stamp.lt(end_datetime))
        .count()
        .get_result::<i64>(connection)
        .unwrap();

    if same_stamp_count != 0 {
        let previous_stamp = works
            .filter(user_id.eq(new_work.user_id))
            .filter(stamp_type_id.eq(new_work.stamp_type_id))
            .filter(stamp.gt(start_datetime))
            .filter(stamp.lt(end_datetime))
            .order(id.desc())
            .first::<Work>(connection)
            .expect("Error previous Work Search");

        let target = works.find(previous_stamp.id);

        diesel::update(target)
            .set(enabled_flag.eq(false))
            .execute(connection)
            .expect("Error updating work");
    }

    diesel::insert_into(works::table)
        .values(&new_work)
        .execute(connection)
        .expect("Error saving new work");

    let result = works::dsl::works
        .order(id.desc())
        .first::<Work>(connection)
        .expect("Error finding work");

    HttpResponse::Ok().json(result)
}

pub async fn delete(params: web::Json<WorkData>) -> impl Responder {
    let delete_work = DeleteWork {
        user_id: &(params.user_id),
        stamp_type_id: &(params.stamp_type_id),
    };

    let connection = &mut establish_connection();

    let target_work = works
        .filter(user_id.eq(delete_work.user_id))
        .filter(stamp_type_id.eq(delete_work.stamp_type_id))
        .filter(enabled_flag.eq(true))
        .first::<Work>(connection)
        .expect("Error target delete work");

    let target = works.find(target_work.id);

    diesel::update(target)
        .set(enabled_flag.eq(false))
        .execute(connection)
        .expect("Error delete work");

    HttpResponse::Ok().finish()
}
