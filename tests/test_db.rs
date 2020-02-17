use hospital_manager::*;

#[test]
fn test_insert_user() {
    use diesel::prelude::*;

    let conn = establish_db_connection();

    insert_user("test_abc", "test_pass", false);

    let test_results: Vec<models::User> = schema::users::table
        .filter(schema::users::username.eq("test_abc".to_string()))
        .filter(schema::users::password.eq("test_pass".to_string()))
        .load(&conn)
        .expect("Error loading users");
    
    assert_eq!(test_results.is_empty(), false);
}

#[test]
fn test_get_user_type() {
    use diesel::prelude::*;

    let conn = establish_db_connection();

    insert_user("test_abc", "test_pass", false);
    
    assert_eq!(get_user_type("test_abc").unwrap(), false);
}

#[test]
fn test_check_if_username_and_password_in_db() {
    use diesel::prelude::*;

    let conn = establish_db_connection();

    insert_user("test_abc", "test_pass", false);

    assert_eq!(check_if_username_and_password_in_db("test_abc", "test_pass"), true);
}
