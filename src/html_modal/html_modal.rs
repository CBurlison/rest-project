use serde_json::Value;

const MAX_TOKEN_LEN: usize = 1000;

/// - Parse and process the modal token values found in the supplied String. A new String is returned as a result.
/// 
/// - The format of tokens are as follows: \@\[token type\]:\[value key\];
/// 
/// Example: @value:name;
/// 
/// 
/// - Value keys can include indexes for things such as Vecs or arrays, and multiple keys for going multiple structs deep.
/// 
/// Example: @value:names\[1\].first;
/// 
/// 
/// 
/// - Valid token types are; 
/// 
/// 1) value       - Displays the value of the key provided.
/// 
/// 2) if          - Displays the contents inside of the {} if the provided value is a true bool.
/// 
/// 3) for         - Repeats the contents inside of the {} for each value within the provided collection value.
/// 
/// 4) forvalue    - Displays the value of the key provided, with the value originating from a for loop. The first element of the key must be an index of the loop level.
/// 
/// Example: @forvalue:0.name;
/// 
/// 5) forif       - Displays the contents inside of the {} if the provided value is a true bool, with the value originating from a for loop. The first element of the key must be an index of the loop level.
/// 
/// Example: @forif:1.is_admin;
/// 
/// 6) forfor      - Repeats the contents inside of the {} for each value within the provided collection value. The first element of the key must be an index of the loop level.
/// 
/// Example: @forfor:2.user_groups;
/// 
/// # Examples
///
/// ```
/// 
/// #[derive(Serialize)]
/// struct User {
///     name: String,
///     test_bool: bool
/// }
/// 
/// let html = String::from(r#"
///     <!DOCTYPE html>
///     <meta charset="utf-8">
///     <title>Hello, world!</title>
///     <body>
///         /@value:name; Example <br/>@value:name;
///         @if:test_true;{
///         this is displaying!
///         }
///     </body>
/// "#);
///
/// let user = User {
///     name: String::from("Test Name"),
///     test_bool: true
/// };
///
/// let result = html_modal::process_string(&html, &user);
/// ```
pub fn process_string<T: serde::ser::Serialize>(
    html: &String,
    modal: &T) -> String
{
    let json_value: Value = serde_json::to_value(&modal).unwrap_or_default();
    let mut foreach_vals: Vec<Option<Value>> = vec![];

    parse(html, &json_value, &mut foreach_vals)
}

fn parse(str: &String, modal: &Value, foreach_modal: &mut Vec<Option<Value>>) -> String {
    let mut escaped = false;
    let mut ret_vec: Vec<u8> = vec![];
    let bytes = str.as_bytes();
    let bytes_len = bytes.len();

    let mut i = 0;
    while i < bytes_len {
        let ch = bytes[i];
        if ch == b'/' {
            escaped = true;
            ret_vec.push(ch);
            i += 1;
            continue;
        }

        if escaped {
            escaped = false;
            ret_vec.push(ch);
            i += 1;
            continue;
        }

        if ch == b'@' {
            i += 1;

            let token_type = parse_token_type(bytes, bytes_len, &mut ret_vec, &mut i);

            let token_key = parse_token_key(bytes, bytes_len, &mut ret_vec, &mut i);

            match token_type.to_lowercase().as_str() {
                "value" => {
                    parse_value(modal, &mut ret_vec, &token_key);
                }
                "forvalue" => {
                    parse_forvalue(foreach_modal, &mut ret_vec, &token_key);
                }
                "for" => {
                    parse_for(modal, foreach_modal, &mut ret_vec, bytes, bytes_len, &mut i, &token_key);
                }
                "forfor" => {
                    parse_forfor(modal, foreach_modal, &mut ret_vec, bytes, bytes_len, &mut i, &token_key);
                }
                "if" => {
                    parse_if(modal, foreach_modal, &mut ret_vec, bytes, bytes_len, &mut i, &token_key);
                }
                "forif" => {
                    parse_forif(modal, foreach_modal, &mut ret_vec, bytes, bytes_len, &mut i, &token_key);
                }
                _ => {
                    ret_vec.extend_from_slice(format!("@{}:{};", token_type, token_key).as_bytes());
                }
            }
        } else {
            ret_vec.push(ch);
            i += 1;
        }
    }

    String::from_utf8(ret_vec).unwrap_or_default()
}

fn parse_token_type(bytes: &[u8], bytes_len: usize, ret_vec: &mut Vec<u8>, i: &mut usize) -> String {
    let mut token_type: Vec<u8> = vec![];
    while *i < bytes_len && bytes[*i] != b':' {
        token_type.push(bytes[*i]);
        *i += 1;

        if token_type.len() >= MAX_TOKEN_LEN {
            ret_vec.extend_from_slice(&token_type);
            Default::default()
        }
    }

    if *i < bytes_len && bytes[*i] == b':' {
        *i += 1;
    }

    if *i >= bytes_len {
        ret_vec.extend_from_slice(&token_type);
        Default::default()
    }

    String::from_utf8(token_type).unwrap_or_default()
}

fn parse_token_key(bytes: &[u8], bytes_len: usize, ret_vec: &mut Vec<u8>, i: &mut usize) -> String {
    let mut token_key: Vec<u8> = vec![];
    while *i < bytes_len && bytes[*i] != b';' && bytes[*i] != b'{' {
        token_key.push(bytes[*i]);
        *i += 1;

        if token_key.len() >= MAX_TOKEN_LEN {
            ret_vec.extend_from_slice(&token_key);
            Default::default()
        }
    }

    if *i < bytes_len && bytes[*i] == b';' {
        *i += 1;
    }

    if *i >= bytes_len {
        ret_vec.extend_from_slice(&token_key);
        Default::default()
    }

    String::from_utf8(token_key).unwrap_or_default()
}

fn parse_value(modal: &Value, ret_vec: &mut Vec<u8>, token_key: &String) {
    let val = get_display_string(modal, token_key);
    ret_vec.extend_from_slice(val.as_bytes());
}

fn parse_forvalue(foreach_modal: &mut Vec<Option<Value>>, ret_vec: &mut Vec<u8>, token_key: &String) {
    let mut parts = token_key.splitn(2, '.');

    if let (Some(idx_str), key) = (parts.next(), parts.next()) {
        if let Ok(idx) = idx_str.parse::<usize>() {
            if let Some(Some(fe_mod)) = foreach_modal.get(idx) {
                let val = get_display_string(fe_mod, &key.unwrap_or_default().to_string());
                ret_vec.extend_from_slice(val.as_bytes());
            }
        }
    }
}

fn parse_for(modal: &Value, foreach_modal: &mut Vec<Option<Value>>, ret_vec: &mut Vec<u8>, bytes: &[u8], bytes_len: usize, i: &mut usize, token_key: &String) {
    while *i < bytes_len && bytes[*i] != b'{' { *i += 1; }

    if *i < bytes_len && bytes[*i] == b'{' {
        *i += 1;
        let mut brace_count = 1;
        let start = *i;

        while *i < bytes_len && brace_count > 0 {
            if bytes[*i] == b'{' { brace_count += 1; }
            if bytes[*i] == b'}' { brace_count -= 1; }
            *i += 1;
        }

        let end = *i - 1;
        let inner = String::from_utf8(bytes[start..end].to_vec()).unwrap_or_default();
        let disp_val = get_display_value(modal, token_key);
    
        if let Some(arr) = disp_val.as_array() {
            for val in arr.iter() {
                foreach_modal.push(Some(val.clone()));
                let parsed = parse(&inner, modal, foreach_modal);
                ret_vec.extend_from_slice(parsed.as_bytes());
                foreach_modal.pop();
            }
        }
    }
}

fn parse_forfor(modal: &Value, foreach_modal: &mut Vec<Option<Value>>, ret_vec: &mut Vec<u8>, bytes: &[u8], bytes_len: usize, i: &mut usize, token_key: &String) {
    let mut parts = token_key.splitn(2, '.');

    if let (Some(idx_str), Some(key)) = (parts.next(), parts.next()) {
        if let Ok(idx) = idx_str.parse::<usize>() {
            if let Some(Some(fe_mod)) = foreach_modal.get(idx) {

                while *i < bytes_len && bytes[*i] != b'{' { *i += 1; }

                if *i < bytes_len && bytes[*i] == b'{' {
                    *i += 1;
                    let mut brace_count = 1;
                    let start = *i;

                    while *i < bytes_len && brace_count > 0 {
                        if bytes[*i] == b'{' { brace_count += 1; }
                        if bytes[*i] == b'}' { brace_count -= 1; }
                        *i += 1;
                    }

                    let end = *i - 1;
                    let inner = String::from_utf8(bytes[start..end].to_vec()).unwrap_or_default();
                    let disp_val = get_display_value(fe_mod, &key.to_string());
                
                    if let Some(arr) = disp_val.as_array() {
                        for val2 in arr.iter() {
                            foreach_modal.push(Some(val2.clone()));
                            let parsed = parse(&inner, modal, foreach_modal);
                            ret_vec.extend_from_slice(parsed.as_bytes());
                            foreach_modal.pop();
                        }
                    }
                }
            }
        }
    }
}

fn parse_if(modal: &Value, foreach_modal: &mut Vec<Option<Value>>, ret_vec: &mut Vec<u8>, bytes: &[u8], bytes_len: usize, i: &mut usize, token_key: &String) {
    while *i < bytes_len && bytes[*i] != b'{' { *i += 1; }

    if *i < bytes_len && bytes[*i] == b'{' {
        *i += 1;
        let mut brace_count = 1;
        let start = *i;

        while *i < bytes_len && brace_count > 0 {
            if bytes[*i] == b'{' { brace_count += 1; }
            if bytes[*i] == b'}' { brace_count -= 1; }
            *i += 1;
        }

        let end = *i - 1;
        let inner = String::from_utf8(bytes[start..end].to_vec()).unwrap_or_default();
        let disp_val = get_display_value(modal, token_key);

        if disp_val.as_bool().unwrap_or(false) {
            let parsed = parse(&inner, modal, foreach_modal);
            ret_vec.extend_from_slice(parsed.as_bytes());
        }
    }
}

fn parse_forif(modal: &Value, foreach_modal: &mut Vec<Option<Value>>, ret_vec: &mut Vec<u8>, bytes: &[u8], bytes_len: usize, i: &mut usize, token_key: &String) {
    let mut parts = token_key.splitn(2, '.');
    if let (Some(idx_str), Some(key)) = (parts.next(), parts.next()) {
        if let Ok(idx) = idx_str.parse::<usize>() {
            if let Some(Some(fe_mod)) = foreach_modal.get(idx) {

                while *i < bytes_len && bytes[*i] != b'{' { *i += 1; }

                if *i < bytes_len && bytes[*i] == b'{' {
                    *i += 1;
                    let mut brace_count = 1;
                    let start = *i;

                    while *i < bytes_len && brace_count > 0 {
                        if bytes[*i] == b'{' { brace_count += 1; }
                        if bytes[*i] == b'}' { brace_count -= 1; }
                        *i += 1;
                    }

                    let end = *i - 1;
                    let inner = String::from_utf8(bytes[start..end].to_vec()).unwrap_or_default();
                    let disp_val = get_display_value(fe_mod, &key.to_string());

                    if disp_val.as_bool().unwrap_or(false) {
                        let parsed = parse(&inner, modal, foreach_modal);
                        ret_vec.extend_from_slice(parsed.as_bytes());
                    }
                }
            }
        }
    }
}

fn get_display_value(modal: &Value, attr_val: &String) -> Value {
    let val_split = attr_val.split(".");
    let mut disp_val = modal;

    for val in val_split {
        if disp_val.is_object() {
            if val.contains("[") {
                let mut index_split = val.split('[');
                while let Some(key) = index_split.next() {
                    if key.len() == 0 {
                        continue;
                    }

                    if !key.ends_with(']') {
                        disp_val = &disp_val[key];
                    }

                    match disp_val {
                        Value::Array(arr) => {
                            let index = key[0..key.len()-1].parse::<usize>();

                            match index {
                                Ok(idx) => {
                                    disp_val = &arr[idx];
                                }
                                Err(_) => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
            else {
                disp_val = &disp_val[val];
            }
        }
        else {
            break;
        }
    }
    disp_val.clone()
}

fn get_display_string(modal: &Value, attr_val: &String) -> String {
    let disp_val = get_display_value(modal, attr_val);

    match disp_val {
        Value::String(val) => {
            return val;
        }
        Value::Bool(val) => {
            return val.to_string();
        }
        Value::Number(val) => {
            return val.to_string();
        }
        _ => {
            return String::new();
        }
    }
}
