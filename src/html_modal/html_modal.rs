use std::{
    cell::{ RefCell, RefMut }, rc::Rc
};
use serde_json::Value;
use html5ever::{
    parse_document,
    parse_fragment,
    tendril::{
        TendrilSink,
        StrTendril
    }, 
    tree_builder::TreeBuilderOpts,
    ParseOpts,
    serialize::{
        SerializeOpts, 
        serialize,
    },
    QualName
};
use rcdom::{
    Handle, 
    NodeData, 
    RcDom,
    SerializableHandle,
    Node
};

struct AttrInfo {
    name: String,
    value: String
}

pub fn process_string<T>(
    html: &String,
    modal: &T,
    opts: Option<ParseOpts>) -> String
    where
        T: serde::ser::Serialize
    {
        match opts {
            None => {
                let default_opts = ParseOpts {
                        tree_builder: TreeBuilderOpts {
                            drop_doctype: true,
                            ..Default::default()
                        },
                    ..Default::default()
                };

                let mut dom = parse_document(RcDom::default(), default_opts.clone())
                    .from_utf8()
                    .read_from(&mut html.as_bytes())
                    .unwrap();
                
                process_document(&mut dom, modal, Some(default_opts))
            }
            Some(value) => {
                let mut dom = parse_document(RcDom::default(), value.clone())
                    .from_utf8()
                    .read_from(&mut html.as_bytes())
                    .unwrap();
                
                process_document(&mut dom, modal, Some(value))
            }
        }
    }

pub fn process_document<T>(
    dom: &mut RcDom,
    modal: &T,
    opts: Option<ParseOpts>) -> String 
    where
        T: serde::ser::Serialize
    {
        let json_value: Value = serde_json::to_value(&modal).unwrap();
        let document = &dom.document;

        inner_process_document(&dom, 0, document, None, &json_value, &Value::Null, &opts.unwrap());
        
        let mut bytes = vec![];
        let document_clone: SerializableHandle = dom.document.clone().into();
        serialize(&mut bytes, &document_clone, SerializeOpts::default()).unwrap();
        let result = String::from_utf8(bytes).unwrap();

        result
    }

fn inner_process_document(
    dom: &RcDom, 
    indent: usize, 
    handle: &Handle,
    parent_children: Option<&mut RefMut<'_, Vec<Rc<Node>>>>,
    modal: &Value, 
    foreach_value: &Value,
    opts: &ParseOpts)
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
                        
                        // remove original node since it contains no relavent data
                        if let Some(pos) = unwrapped_children.iter().position(|child| Rc::ptr_eq(child, handle)) {
                            unwrapped_children.remove(pos);
                        }

                        let mut new_children = vec![];
                        for val in disp_val.as_array().unwrap() {
                            for child in unwrapped_children.clone().iter() {
                                let mut bytes = vec![];
                                let document_clone: SerializableHandle = child.clone().into();
                                serialize(&mut bytes, &document_clone, SerializeOpts::default()).unwrap();
                                let result = String::from_utf8(bytes).unwrap();
                                
                                let handle_clone = parse_fragment(
                                    RcDom::default(), 
                                    opts.clone(), 
                                    QualName::new(None, ns!(html), local_name!("body")), 
                                    vec![],
                                true)
                                .one(result);

                                {
                                    let mut borrowed_children = handle_clone.document.children.borrow_mut();
                                    inner_process_document(dom, indent + 4, child, Some(&mut borrowed_children), modal, val, opts);
                                }

                                new_children.push(handle_clone.document);
                            }
                        }

                        for child in new_children {
                            for actual_child in child.children.borrow().clone() {
                                unwrapped_children.push(actual_child);
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
        inner_process_document(dom, indent + 4, child, Some(&mut children), modal, foreach_value, opts);
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

    for val in val_split {
        if disp_val.is_string() {
            break;
        }
        
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