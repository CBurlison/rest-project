use std::{
    cell::{RefCell, RefMut}, rc::Rc
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

    match opts {
        None => {
            let default_opts = ParseOpts {
                    tree_builder: TreeBuilderOpts {
                        drop_doctype: true,
                        ..Default::default()
                    },
                ..Default::default()
            };

            inner_process_document(&dom, document, None, &json_value, &Value::Null, &default_opts);
        }
        Some(value) => {
            inner_process_document(&dom, document, None, &json_value, &Value::Null, &value);
        }
    }
    
    let mut bytes = vec![];
    let document_clone: SerializableHandle = dom.document.clone().into();
    serialize(&mut bytes, &document_clone, SerializeOpts::default()).unwrap();
    let result = String::from_utf8(bytes).unwrap();

    result
}

fn inner_process_document(
    dom: &RcDom,
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

                match attr_info.name.to_lowercase().as_str() {
                    "value" => {
                        match_value(handle, modal, &attr_info.value);
                    }
                    "foreach-value" => {
                        match_value(handle, foreach_value, &attr_info.value);
                    }
                    "if" => {
                        match_if(handle, modal, &attr_info.value);
                    }
                    "foreach-if" => {
                        match_if(handle, foreach_value, &attr_info.value);
                    }
                    "foreach" => {
                        let unwrapped_children = parent_children.unwrap();
                        match_foreach(dom, handle, modal, opts, attr_info, unwrapped_children);
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
        inner_process_document(dom, child, Some(&mut children), modal, foreach_value, opts);
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

fn match_value(handle: &Rc<Node>, modal: &Value, attr_val: &String) {
    let disp_val = get_display_value(modal, attr_val);
    
    let mut text = StrTendril::new();
    let mut text_str = "".to_string();

    if disp_val.is_string() {
        text_str = disp_val.as_str().unwrap().to_string();
    }
    else if disp_val.is_boolean() {
        text_str = disp_val.as_bool().unwrap().to_string();
    }
    else if disp_val.is_f64() {
        text_str = disp_val.as_f64().unwrap().to_string();
    }
    else if disp_val.is_i64() {
        text_str = disp_val.as_i64().unwrap().to_string();
    }
    else if disp_val.is_u64() {
        text_str = disp_val.as_u64().unwrap().to_string();
    }
    else if disp_val.is_number() {
        text_str = disp_val.as_number().unwrap().to_string();
    }

    if text_str == "" {
        return;
    }
    
    let text_handle = text.try_push_bytes(text_str.as_bytes());
    match text_handle {
        Ok(_) => {
            handle.children.borrow_mut().insert(0, Node::new(NodeData::Text { contents: RefCell::new(text) }));
        }
        Err(_) => {}
    }
}

fn match_if(handle: &Rc<Node>, modal: &Value, attr_val: &String) {
    let disp_val = get_display_value(modal, attr_val);
                        
    if !disp_val.as_bool().unwrap() {
        let mut children = handle.children.borrow_mut();

        for _ in 0..children.len() {
            children.remove(0);
        }
    }
}

fn match_foreach(dom: &RcDom, handle: &Rc<Node>, modal: &Value, opts: &ParseOpts, attr_info: AttrInfo, parent_children: &mut RefMut<'_, Vec<Rc<Node>>>) {
    let disp_val = get_display_value(modal, &attr_info.value);
                 
    let base_handle_clone = clone_handle(opts, handle);
    let clone_children = base_handle_clone.document.children.borrow_mut();
 
    // remove original node since it contains no relavent data
    if let Some(pos) = parent_children.iter().position(|child| Rc::ptr_eq(child, handle)) {
        parent_children.remove(pos);
    }

    for val in disp_val.as_array().unwrap() {
        for child in clone_children.clone().iter() {
            let child_clone = clone_handle(opts, child);

            {
                inner_process_document(dom, &child_clone.document, Some(parent_children), modal, val, opts);
            }

            for actual_child in child_clone.document.children.take() {
                parent_children.push(actual_child);
            }
        }
    }
}

fn get_display_value(modal: &Value, attr_val: &String) -> Value {
    let val_split = attr_val.split(".");
    let mut disp_val = modal;

    for val in val_split {
        if disp_val.is_object() {
            disp_val = &disp_val[val];
        }
        else {
            break;
        }
    }
    disp_val.clone()
}

fn clone_handle(opts: &ParseOpts, child: &Rc<Node>) -> RcDom {
    let result = node_to_string(child);

    parse_fragment(
        RcDom::default(), 
        opts.clone(), 
        QualName::new(None, ns!(html), local_name!("body")), 
        vec![],
    true)
    .one(result)
}

fn node_to_string(child: &Rc<Node>) -> String {
    let mut bytes = vec![];
    let document_clone: SerializableHandle = child.clone().into();
    serialize(&mut bytes, &document_clone, SerializeOpts::default()).unwrap();
    String::from_utf8(bytes).unwrap()
}
