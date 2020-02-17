#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use self::models::{NewUser, User, Appointment, NewAppointment};

pub mod models;
pub mod schema;

#[cfg(not(test))]
pub fn establish_db_connection() -> PgConnection {
    println!("dev connection");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
pub fn establish_db_connection() -> PgConnection {
    println!("test connection");
    dotenv().ok();

    let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_user<'a>(username: &'a str, password: &'a str, user_type: bool) -> User {
    use schema::users;

    let conn = establish_db_connection();
    let new_user = NewUser {
        username: username,
        password: password,
        user_type: user_type
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&conn)
        .expect("Error saving new user")
}

pub fn insert_appointment<'a>(username: &'a str, doctor: &'a str, date: &'a str) -> Appointment {
    use schema::appointments;

    let conn = establish_db_connection();
    let new_app = NewAppointment {
        username: username,
        doctor: doctor,
        time_of_app: date
    };

    diesel::insert_into(appointments::table)
        .values(&new_app)
        .get_result(&conn)
        .expect("Error saving new user")
}


pub fn get_user_type(username: &str) -> Result<bool, &str>{
    use schema::users;

    let conn = establish_db_connection();

    let results: Vec<User> = users::table
        .filter(users::username.eq(username.to_string()))
        .load(&conn)
        .expect("Error loading users");

    println!("{:?}", results);

    if results.is_empty() {
        return Err("Nope");
    }

    return Ok(results[0].user_type);
}

pub fn check_if_username_and_password_in_db(username: &str, password: &str) -> bool {
    use schema::users;

    let conn = establish_db_connection();

    let results: Vec<User> = users::table
        .filter(users::username.eq(username.to_string()))
        .filter(users::password.eq(password.to_string()))
        .load(&conn)
        .expect("Error loading users");

    if results.is_empty() {
        return false;
    }
    return true;
}

pub fn get_appointments_for_doctor(doctor_name: &str) -> String {
    use schema::appointments;

    let conn = establish_db_connection();

    let results: Vec<Appointment> = appointments::table
        .filter(appointments::doctor.eq(doctor_name.to_string()))
        .load(&conn)
        .expect("Error loading appointments");
    
    let html_string = &mut String::from("<form action=\"/appointments\" method=\"GET\">");
    for el in &results {
        let new_string = format!("<input type=\"radio\" id={:?} value={:?} name=\"app\">
        <label for={:?}>{}</label><br>", el.time_of_app,el.time_of_app,el.time_of_app,el.time_of_app);
        html_string.push_str(&new_string.to_string());
    }
    html_string.push_str(&"</form>".to_string());

    return html_string.to_string();
}   



pub fn get_username_from_appointments(doctor_name: &str, date: &str) -> Result<String, String> {
    use schema::appointments;

    let conn = establish_db_connection();

    let results: Vec<Appointment> = appointments::table
        .filter(appointments::time_of_app.eq(date.to_string()))
        .filter(appointments::doctor.eq(doctor_name.to_string()))
        .load(&conn)
        .expect("Error loading appointments");

    if results.is_empty() {
        return Err("Nope".to_string());
    }
    

    let a: &str = &results[0].username;

    return Ok(a.to_string());
}

pub fn update_appointment(username: &str, doctor_name: &str, date: &str) {
    use schema::appointments;
    
    let conn = establish_db_connection();
    diesel::update(appointments::table
        .filter(appointments::time_of_app.eq(date.to_string()))
        .filter(appointments::doctor.eq(doctor_name.to_string())))
        .set(appointments::username.eq(username.to_string()))
        .execute(&conn)
        .expect("Error updating appointment");

    return;
}
