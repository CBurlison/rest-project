use actix_web::{
    self, HttpRequest, HttpResponse, Responder
};
use super::super::helpers::http_helpers;
use serde::Serialize;
use super::super::html_modal::html_modal;
use uuid::Uuid;
use std::{
    fs::read_to_string,
    env::current_dir
};
// use std::time::{Duration, SystemTime};

#[derive(Serialize)]
struct User {
    id: String,
    name: String,
    email: String,
    password: String,
    ip: String,
    session: String,
    test_true: bool,
    test_false: bool,
    str_vec: Vec<String>,
    user_vec: Vec<User>,
    vec_vec: Vec<Vec<u64>>,
    test_f64: f64
}

pub async fn auth() -> impl Responder {
    let read = read_to_string("web/auth/auth.html");
    match read {
        Ok(html) => {
                let user = User {
                id: Uuid::new_v4().to_string(),
                name: String::from("Test Name"),
                email: String::from(""),
                password: String::from(""),
                ip: String::from(""),
                session: String::from(""),
                test_true: true,
                test_false: false,
                str_vec: vec![String::from("str 1"), String::from("str 2"), String::from("str 3")],
                vec_vec: vec![vec![0,1,2], vec![3,4,5]],
                test_f64: 1.23,
                user_vec: vec![User {
                        id: Uuid::new_v4().to_string(),
                        name: String::from("Test Name 2"),
                        email: String::from(""),
                        password: String::from(""),
                        ip: String::from(""),
                        session: String::from(""),
                        test_true: true,
                        test_false: false,
                        str_vec: vec![String::from("str 4"), String::from("str 5"), String::from("str 6")],
                        vec_vec: vec![],
                        test_f64: 1.23,
                        user_vec: vec![User {
                            id: Uuid::new_v4().to_string(),
                            name: String::from("Test Name 4"),
                            email: String::from(""),
                            password: String::from(""),
                            ip: String::from(""),
                            session: String::from(""),
                            test_true: true,
                            test_false: false,
                            str_vec: vec![String::from("str 4"), String::from("str 5"), String::from("str 6")],
                            vec_vec: vec![],
                            test_f64: 1.23,
                            user_vec: vec![]
                        }]
                    },
                    User {
                        id: Uuid::new_v4().to_string(),
                        name: String::from("Test Name 3"),
                        email: String::from(""),
                        password: String::from(""),
                        ip: String::from(""),
                        session: String::from(""),
                        test_true: true,
                        test_false: false,
                        str_vec: vec![String::from("str 1"), String::from("str 1"), String::from("str 1")],
                        vec_vec: vec![],
                        test_f64: 1.23,
                        user_vec: vec![]
                    }]
            };

            // let now = SystemTime::now();
            
            let result = html_modal::process_string(&html, &user);

            // match now.elapsed() {
            //     Ok(elapsed) => {
            //         println!("{} ms", elapsed.as_nanos() as f64 / 1000000f64);
            //     }
            //     Err(_) => {}
            // }
            
            HttpResponse::Ok().body(result)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("{}\nCurrent dir: {}", e, current_dir().unwrap().to_str().unwrap()))
        }
    }
}

pub async fn echo(req: HttpRequest, req_body: String) -> impl Responder {
    let test = http_helpers::get_header_value(req, "test_header");
    HttpResponse::Ok().body(format!("{test}: {req_body}"))
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// pub async fn logon(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }
