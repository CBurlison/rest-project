use actix_web::{
    self, HttpRequest, HttpResponse, Responder
};
use super::super::helpers::http_helpers;
use serde::Serialize;
use super::super::html_modal::html_modal::{ HtmlModalParser, ModalParser };
use uuid::Uuid;

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
    user_vec: Vec<User>
}

pub async fn hello() -> impl Responder {
    let html = String::from(r#"
        <!DOCTYPE html>
        <meta charset="utf-8">
        <title>Hello, world!</title>
        <h2>
            /@value:name; Example <br/>@value:name;
            <br/><br/>
            /@value:str_vec[2]; Example <br/>@value:str_vec[2];
            <br/><br/>
            /@if:test_true; Example
            <br/>
            @if:test_true;{
            this is displaying!
            }
            <br/><br/>
            /@if:test_false; Example
            <br/>
            @if:test_false;{
            <br/>
            this should not be here!
            }
            <br/><br/>
            /@foreach:str_vec; Example
            <ul>
            @foreach:str_vec;{
                <li>/@foreachvalue:0; = @foreachvalue:0;</li>
            }
            </ul>
            <br/><br/>
            /@foreach:user_vec; Example
            <ul>
            @foreach:user_vec;{
                <li>/@foreachvalue:0.name; = @foreachvalue:0.name;</li>
            
                <br/>/@foreachforeach:0.user_vec; Example
                @foreachforeach:0.user_vec;{
                    <li>/@foreachvalue:1.name; = @foreachvalue:1.name;</li>
                }
                <br/>
            }
            </ul>
        </h2>
    "#);

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
                user_vec: vec![]
            }]
    };

    let parser: HtmlModalParser = HtmlModalParser{};
    let result = parser.process_string(&html, &user);
    
    HttpResponse::Ok().body(result)
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
