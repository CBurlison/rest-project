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
    test_bool: bool,
    test_bool2: bool
}

pub async fn hello() -> impl Responder {
    let html = String::from(r#"
        <!DOCTYPE html>
        <meta charset="utf-8">
        <title>Hello, world!</title>
        <h2>
            @value:name;
            @if:test_bool;{
            <br/>this is displaying!
            }
            @if:test_bool2;{
            <br/>this should not be here!
            }
            <br/>/@value:name;
        </h2>
    "#);

    let user = User {
        id: Uuid::new_v4().to_string(),
        name: String::from("Test Name"),
        email: String::from(""),
        password: String::from(""),
        ip: String::from(""),
        session: String::from(""),
        test_bool: true,
        test_bool2: false
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
