#[macro_use]
extern crate rouille;

use hospital_manager::{insert_user, get_user_type, get_appointments_for_doctor, get_username_from_appointments, update_appointment, check_if_username_and_password_in_db};
use hospital_manager::models::{Appointment};

use std::collections::HashMap;
use std::io;
use std::sync::Mutex;

use rouille::{Request, Response};

#[derive(Debug, Clone)]
struct SessionData { login: String, user_type: bool }

fn main() {
    let sessions_storage: Mutex<HashMap<String, SessionData>> = Mutex::new(HashMap::new());

    rouille::start_server("localhost:8000", move |request| {
        rouille::log(&request, io::stdout(), || {
            rouille::session::session(request, "SID", 3600, |session| {
                let mut session_data = if session.client_has_sid() {
                    if let Some(data) = sessions_storage.lock().unwrap().get(session.id()) {
                        Some(data.clone())
                    } else {
                        None
                    }
                } else {
                    None
                };

                let response = handle_route(&request, &mut session_data);

                if let Some(d) = session_data {
                    sessions_storage.lock().unwrap().insert(session.id().to_owned(), d);

                } else if session.client_has_sid() {
                    sessions_storage.lock().unwrap().remove(session.id());
                }
                
                response
            })
        })
    });
}

fn handle_route(request: &Request, session_data: &mut Option<SessionData>) -> Response {
    router!(request,
        (POST) (/) => {
            if let Some(session_data) = session_data.as_ref() { 
                    println!("HERE");
                    return Response::redirect_303("/");
            }
        
            else {
                println!("okay");
            }

            return Response::redirect_303("/")
        },

        (POST) (/appointments) => {
            if let Some(session_data) = session_data.as_ref() { 
                if session_data.user_type == false {
                    let data = try_or_400!(post_input!(request, {
                        doctor: String
                    }));
                    return Response::html(format!("{}", get_appointments_for_doctor(&data.doctor)));
                }
            }
        
            else {
                println!("okay");
            }
            
        },

        (POST) (/registered_in) => {
            handle_route_register(request);

            // This is the route that is called when the user wants to log in.

            // In order to retreive what the user sent us through the <form>, we use the
            // `post_input!` macro. This macro returns an error (if a field is missing for example),
            // so we use the `try_or_400!` macro to handle any possible error.
            //
            // If the macro is successful, `data` is an instance of a struct that has one member
            // for each field that we indicated in the macro.
            let data = try_or_400!(post_input!(request, {
                username: String,
                password: String,
                is_doctor: bool
            }));

            // Just a small debug message for this example. You could also output something in the
            // logs in a real application.
            println!("Register attempt with login {:?} and password {:?} and type {:?}", data.username, data.password, data.is_doctor);

            // In this example all login attempts are successful in the password starts with the
            // letter 'b'. Of course in a real website you should check the credentials in a proper
            // way.
            
            insert_user(&data.username, &data.password, data.is_doctor);
            *session_data = Some(SessionData { login: data.username, user_type: data.is_doctor });
            println!("Success");
            return Response::redirect_303("/logged_in")
        },

        (POST) (/logged_in) => {
            let data = try_or_400!(post_input!(request, {
                username: String,
                password: String
            }));

            println!("Login attempt with login {:?} and password {:?}", data.username, data.password);

            if !check_if_username_and_password_in_db(&data.username, &data.password) {
                return Response::redirect_303("/")
            }
            let user_type = get_user_type(&data.username).unwrap();

            *session_data = Some(SessionData { login: data.username, user_type: user_type });
            println!("Success");
            return Response::redirect_303("/logged_in")
        },

        (POST) (/logout) => {
            // This route is called when the user wants to log out.
            // We do so by simply erasing the content of `session_data`, which deletes the session.
            *session_data = None;

            // We return a dummy response to indicate what happened. In a real application you
            // should probably use some sort of HTML templating instead.
            return Response::html(r#"Logout successful.
                                     <a href="/">Click here to go to the home</a>"#);
        },

        _ => ()
    );

    if let Some(session_data) = session_data.as_ref() {
        if session_data.user_type == false {
            handle_route_logged_patient(request, session_data)
        }
        else {
            handle_route_logged_doctor(request, session_data)
        }
    } else {
        router!(request,
            (GET) (/) => {
                Response::html(r#"
                    <p>Greetings, would You like to register or login into the Ferris Hospital?</p>
                    <form action="/register" method="GET">
                    <button action="/register" type="submit">Register</button></form>
                    <form action="/login" method="GET">
                    <button action="/login" type="submit">Login</button></form>
                "#)
            },

            (GET) (/login) => {
                handle_route_log(request)
            },

            (GET) (/register) => {
                handle_route_register(request)
            },

            _ => {
                Response::redirect_303("/")
            }
        )
    }
}

fn handle_route_logged_patient(request: &Request, session_data: &SessionData) -> Response {
    router!(request,
        (GET) (/) => {
            Response::html(format!(r#"<p>Greetings {}, would You like to make an appointment at the Ferris Hospital?</p>
            <form action="/appointments" method="POST">
            <input type="text" name="doctor" placeholder="Doctor's name"><br>
            <input type="submit" value="Submit">
            </form>
            <form action="/logout" method="POST">
            <button action="/" type="submit">Logout</button></form>
            "#, session_data.login))
        },

        (GET) (/appointments) => {
            Response::html("<p>Here</p>")
        },

        (GET) (/logged_in) => {
            return Response::redirect_303("/");

        },

        _ => Response::empty_404()
    )
}

fn handle_route_logged_doctor(request: &Request, session_data: &SessionData) -> Response {
    router!(request,
        (GET) (/) => {
            Response::html(format!(r#"<p>Greetings, Dr.{}, would You like to log out of the Ferris Hospital?</p>
            <form action="/logout" method="POST">
            <button action="/" type="submit">Logout</button></form>
            "#, session_data.login))
        },

        (GET) (/logged_in) => {
            Response::html(r#"You are now logged in. If you close your tab and open it again,
                              you will still be logged in.<br />
                              <form action="/logout" method="POST">
                              <button>Logout</button></form>"#)
        },

        _ => Response::empty_404()
    )
}

fn handle_route_log(request: &Request) -> Response {
    router!(request,
        (GET) (/login) => {
            // Show some greetings with a dummy response.
            Response::html(r#"
            <form action="/logged_in" method="POST">
                <input type="text" name="username" placeholder="Username" />
                <input type="password" name="password" placeholder="Password" />
                <button type="submit">Go</button>
            </form>
        "#)
        },
        _ => Response::empty_404()
    )
}

fn handle_route_register(request: &Request) -> Response {
    router!(request,
        (GET) (/register) => {
            // Show some greetings with a dummy response.
            Response::html(r#"
            <form action="/registered_in" method="POST">
                <input type="text" name="username" placeholder="Username" />
                <input type="password" name="password" placeholder="Password" />
                <input type="checkbox" name="is_doctor" placeholder="Is Doctor?" />
                <button type="submit">Go</button>
            </form>
        "#)
        },
        _ => Response::empty_404()
    )
}
