use std::{
    cell::{ RefMut, RefCell }, rc::Rc
};
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
    SerializableHandle,
    Node
};

pub async fn hello() -> impl Responder {
    let html = r#"
        <!DOCTYPE html>
        <meta charset="utf-8">
        <title>Hello, world!</title>
        <h1 class="foo">Hello, <i>world!</i></h1>
        <h2>
            <html-modal type="value" value="name" />
            <html-modal type="if" value="test_true"></br>
                test_true is displaying properly.
            </html-modal>
            <html-modal type="if" value="test_false"></br>
                test_false id displaying. You messed up.
            </html-modal>
            <ul>
            <html-modal type="foreach" value="test_data">
                <li><html-modal type="foreach-value" value="" /></li>
            </html-modal>
            </ul>
            <br/>Bottom text.
        </h2>
    "#.to_string();

    let user = User {
        name: "Cory".to_string(),
        password: "dummy".to_string(),
        email: "test@gmail.com".to_string(),
        ip: "127.0.0.1".to_string(),
        session: "this is the session ID".to_string(),
        test_data: vec!["test_val_1".to_string(), "test_val_2".to_string(), "test_val_3".to_string()],
        test_true: true,
        test_false: false
    };
    
    let opts = ParseOpts {
            tree_builder: TreeBuilderOpts {
                drop_doctype: true,
                ..Default::default()
            },
        ..Default::default()
    };
    let mut dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();
    
    process_document(&mut dom, &user);
    
    let mut bytes = vec![];
    let document_clone: SerializableHandle = dom.document.clone().into();
    serialize(&mut bytes, &document_clone, SerializeOpts::default()).unwrap();
    let result = String::from_utf8(bytes).unwrap();

    HttpResponse::Ok().body(result)
}

pub fn process_document<T>(
    dom: &mut RcDom,
    modal: &T)
    where
        T: serde::ser::Serialize
    {
        let json_value: Value = serde_json::to_value(&modal).unwrap();
        let document = &dom.document;

        inner_process_document(&dom, 0, document, None, &json_value, &Value::Null);
    }

fn inner_process_document(
    dom: &RcDom, 
    indent: usize, 
    handle: &Handle,
    parent_children: Option<&mut RefMut<'_, Vec<Rc<Node>>>>,
    modal: &Value, 
    foreach_value: &Value)
    {
    match handle.data {
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            if name.local.to_string().as_str() == "html-modal" {
                assert!(name.ns == ns!(html));

                let attr_info = get_attr_info(attrs);

                let unwrapped_children = parent_children.unwrap();

                match attr_info.name.to_lowercase().as_str() {
                    "value" => {
                        match_value(handle, modal, &attr_info.value, unwrapped_children);
                    }
                    "foreach-value" => {
                        match_foreach_value(handle, foreach_value, &attr_info.value, unwrapped_children);
                    }
                    "if" => {
                        match_if(handle, modal, &attr_info.value, unwrapped_children);
                    }
                    "foreach-if" => {
                        match_foreach_if(handle, foreach_value, &attr_info.value, unwrapped_children);
                    }
                    "foreach" => {
                        let val_split = attr_info.value.split(".");
                        let mut disp_val = modal;
                        for val in val_split {
                            disp_val = &disp_val[val];
                        }
                        
                        for val in disp_val.as_array().unwrap() {
                            let mut children = handle.children.borrow_mut();
                            for child in children.clone().iter() {
                                inner_process_document(dom, indent + 4, child, Some(&mut children), modal, val);
                            }
                        }
                        return;
                    }
                    _ => {}
                }
            }
        },

        NodeData::ProcessingInstruction { .. } => unreachable!(),

        _ => {}
    }
    
    let mut children = handle.children.borrow_mut();
    for child in children.clone().iter() {
        inner_process_document(dom, indent + 4, child, Some(&mut children), modal, foreach_value);
    }
}

fn get_attr_info(attrs: &RefCell<Vec<html5ever::Attribute>>) -> AttrInfo {
    let mut attr_info: AttrInfo = AttrInfo { name: "".to_string(), value: "".to_string() };

    for attr in attrs.borrow().iter() {
        assert!(attr.name.ns == ns!());
    
        match attr.name.local.to_string().to_lowercase().as_str() {
            "type" => {
                attr_info.name = attr.value.to_string();
            }
            "value" => {
                attr_info.value = attr.value.to_string();
            }
            _ => {}
        }
    }
    attr_info
}

fn match_value(handle: &Rc<Node>, modal: &Value, attr_val: &String, unwrapped_children: &mut RefMut<'_, Vec<Rc<Node>>>) {
    let val_split = attr_val.split(".");
    let mut disp_val = modal;

    for val in val_split {
        disp_val = &disp_val[val];
    }

    if let Some(pos) = unwrapped_children.iter().position(|child| Rc::ptr_eq(child, handle)) {
        let mut text = StrTendril::new();
        let text_handle = text.try_push_bytes(disp_val.as_str().unwrap().as_bytes());

        match text_handle {
            Ok(_) => {
                unwrapped_children.insert(pos, Node::new(NodeData::Text { contents: RefCell::new(text) }));
            }
            Err(_) => {}
        }
    }
}

fn match_foreach_value(handle: &Rc<Node>, foreach_value: &Value, attr_val: &String, unwrapped_children: &mut RefMut<'_, Vec<Rc<Node>>>) {
    let val_split = attr_val.split(".");
    let mut disp_val = foreach_value;
    println!("foreach in: {}", disp_val.to_string());

    for val in val_split {
        if disp_val.is_string() {
            break;
        }
        
        disp_val = &disp_val[val];
    }
    println!("foreach out: {}", disp_val.to_string());

    if let Some(pos) = unwrapped_children.iter().position(|child| Rc::ptr_eq(child, handle)) {
        let mut text = StrTendril::new();
        let text_handle = text.try_push_bytes(disp_val.as_str().unwrap().as_bytes());

        match text_handle {
            Ok(_) => {
                unwrapped_children.insert(pos, Node::new(NodeData::Text { contents: RefCell::new(text) }));
            }
            Err(_) => {}
        }
    }
}

fn match_if(handle: &Rc<Node>, modal: &Value, attr_val: &String, unwrapped_children: &mut RefMut<'_, Vec<Rc<Node>>>) {
    let val_split = attr_val.split(".");
    let mut disp_val = modal;
                        
    for val in val_split {
        disp_val = &disp_val[val];
    }
                        
    if !disp_val.as_bool().unwrap() {
        if let Some(pos) = unwrapped_children.iter().position(|child| Rc::ptr_eq(child, handle)) {
            unwrapped_children.remove(pos);
        }
    }
}

fn match_foreach_if(handle: &Rc<Node>, foreach_value: &Value, attr_val: &String, unwrapped_children: &mut RefMut<'_, Vec<Rc<Node>>>) {
    let val_split = attr_val.split(".");
    let mut disp_val = foreach_value;

    for val in val_split {
        disp_val = &disp_val[val];
    }

    if !disp_val.as_bool().unwrap() {
        if let Some(pos) = unwrapped_children.iter().position(|child| Rc::ptr_eq(child, handle)) {
            unwrapped_children.remove(pos);
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

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    password: String,
    ip: String,
    session: String,
    test_data: Vec<String>,
    test_true: bool,
    test_false: bool
}

struct AttrInfo {
    name: String,
    value: String
}