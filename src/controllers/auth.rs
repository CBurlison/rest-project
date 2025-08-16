use std::{str::FromStr};
use actix_web::{
    self, HttpRequest, HttpResponse, Responder
};
use super::super::helpers::http_helpers;
use serde::{
    Deserialize, 
    Serialize
};
use serde_json::Value;
use html5ever::{
    interface::TreeSink, 
    parse_document, 
    tendril::{
        TendrilSink,
        StrTendril
    }, 
    tree_builder::TreeBuilderOpts,
    ParseOpts,
    serialize::{
        SerializeOpts, 
        serialize
    }
};
use rcdom::{
    Handle, 
    NodeData, 
    RcDom,
    SerializableHandle
};

pub async fn hello() -> impl Responder {
    let html = r#"
        <!DOCTYPE html>
        <meta charset="utf-8">
        <title>Hello, world!</title>
        <h1 class="foo">Hello, <i>world!</i></h1>
        <h2>
            <html-modal type="value" value="name" />
        </h2>
    "#.to_string();

    let user = User {
        name: "Cory".to_string(),
        password: "dummy".to_string(),
        email: "test@gmail.com".to_string(),
        ip: "127.0.0.1".to_string(),
        session: "this is the session ID".to_string(),
        test_data: vec!["test_val_1".to_string(), "test_val_2".to_string(), "test_val_3".to_string()],
        test_bool: true
    };
    
    let opts = ParseOpts {
            tree_builder: TreeBuilderOpts {
                drop_doctype: true,
                ..Default::default()
            },
        ..Default::default()
    };
    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();
    
    process_document(&dom, &user);
    
    let mut bytes = vec![];
    let document_clone: SerializableHandle = dom.document.clone().into();
    serialize(&mut bytes, &document_clone, SerializeOpts::default()).unwrap();
    let result = String::from_utf8(bytes).unwrap();

    HttpResponse::Ok().body(result)
}

pub fn process_document<T>(
    dom: &RcDom,
    modal: &T)
    where
        T: serde::ser::Serialize
    {
        let json_value: Value = serde_json::to_value(&modal).unwrap();
        let document = &dom.document;

        inner_process_document(&dom, 0, document, &json_value, &Value::Null);
    }

fn inner_process_document(
    dom: &RcDom, 
    indent: usize, 
    handle: &Handle, 
    modal: &Value, 
    foreach_value: &Value)
    {
    match handle.data {
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            match name.local.to_string().as_str() {
                "html-modal" => {
                    assert!(name.ns == ns!(html));

                    // let mut attr_key = String::new();
                    let mut attr_val = String::new();

                    for attr in attrs.borrow().iter() {
                        assert!(attr.name.ns == ns!());
                        
                        match attr.name.local.to_string().to_lowercase().as_str() {
                            "type" => {
                                // attr_key = attr.value.to_string();
                            }
                            "value" => {
                                attr_val = attr.value.to_string();
                            }
                            _ => {}
                        }
                    }

                    let val_split = attr_val.split(".");

                    let mut disp_val = modal;
                    for val in val_split {
                            disp_val = &modal[val];
                    }
                    
                    
                    let ten = StrTendril::from_str(disp_val.as_str().unwrap()).unwrap();
                    dom.append(handle, html5ever::tree_builder::AppendText(ten));
                }
                _ => {}
            }
        },

        NodeData::ProcessingInstruction { .. } => unreachable!(),

        _ => {}
    }
    
    for child in handle.children.borrow().iter() {
        inner_process_document(dom, indent + 4, child, modal, foreach_value);
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

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    password: String,
    ip: String,
    session: String,
    test_data: Vec<String>,
    test_bool: bool
}