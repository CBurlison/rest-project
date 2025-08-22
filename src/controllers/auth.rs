use actix_web::{
    self, HttpRequest, HttpResponse, Responder
};
use super::super::helpers::http_helpers;
use serde::Serialize;
use super::super::html_modal::html_modal;

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
    let html = r#"
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
    "#.to_string();

    let user = User {
        name: "Cory".to_string(),
        number: 10,
        password: "dummy".to_string(),
        email: "test@gmail.com".to_string(),
        ip: "127.0.0.1".to_string(),
        session: "this is the session ID".to_string(),
        test_data: vec!["test_val_1".to_string(), "test_val_2".to_string(), "test_val_3".to_string()],
        test_true: true,
        test_false: false
    };
    
    let result = html_modal::process_string(&html, &user, None);
    
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
