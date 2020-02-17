#![allow(unused)]
#![allow(clippy::all)]

use crate::schema::{users, appointments};
use chrono::DateTime;
use chrono::offset::Utc;

#[derive(Queryable, Debug)]
pub struct Appointment {
    pub id: i32,
    pub username: String,
    pub doctor: String,
    pub time_of_app: String,
}

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub user_type: bool,
}

#[derive(Insertable, Debug)]
#[table_name="appointments"]
pub struct NewAppointment<'a>{
    pub username: &'a str,
    pub doctor: &'a str,
    pub time_of_app: &'a str
}

#[derive(Insertable, Debug)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub user_type: bool,
}