use actix_web::{
    self, HttpRequest, HttpResponse, Responder
};
use super::super::helpers::http_helpers;
use serde::Serialize;
use super::super::html_modal::html_modal::{ HtmlModalParser, ModalParser };

#[derive(Serialize)]
struct User {
    name: String,
    number: i32,
    email: String,
    password: String,
    ip: String,
    session: String,
    test_data: Vec<String>,
    test_true: bool,
    test_false: bool
}

pub async fn hello() -> impl Responder {
    let html = String::from(r#"
        <!DOCTYPE html>
        <meta charset="utf-8">
        <title>Hello, world!</title>
        <h1 class="foo">Hello, <i>world!</i></h1>
        <h2>
            <html-modal type="value" value="name"></html-modal>
            <html-modal type="value" value="number"></html-modal>
            <html-modal type="if" value="test_true"></br>
                test_true is displaying properly.
            </html-modal>
            <html-modal type="if" value="test_false"></br>
                test_false id displaying. You messed up.
            </html-modal>
            <ul>
            <html-modal type="foreach" value="test_data">
                <li><html-modal type="foreach-value" value=""></html-modal></li>
            </html-modal>
            </ul>
            <br/>Bottom text.
        </h2>
    "#);

    let user = User {
        name: String::from("Cory"),
        number: 10,
        password: String::from("dummy"),
        email: String::from("test@gmail.com"),
        ip: String::from("127.0.0.1"),
        session: String::from("this is the session ID"),
        test_data: vec![String::from("test_val_1"), String::from("test_val_2"), String::from("test_val_3")],
        test_true: true,
        test_false: false
    };
    
    let parser = HtmlModalParser { opts: Default::default() };
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
