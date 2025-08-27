use serde_json::Value;

pub fn process_string<T: serde::ser::Serialize>(
    html: &String,
    modal: &T) -> String
{
    let json_value: Value = serde_json::to_value(&modal).unwrap();
    let mut foreach_vals: Vec<Option<Value>> = vec![];

    parse(html, &json_value, &mut foreach_vals)
}

fn parse(str: &String, modal: &Value, foreach_modal: &mut Vec<Option<Value>>) -> String {
    let mut escaped = false;
    let mut ret_vec: Vec<u8> = vec![];
    let bytes = str.as_bytes();
    let mut i = 0;
    let bytes_len = bytes.len();

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
            let mut token_type = String::new();
            let mut token_key = String::new();

            while i < bytes_len && bytes[i] != b':' {
                token_type.push(bytes[i] as char);
                i += 1;
            }
            if i < bytes_len && bytes[i] == b':' {
                i += 1;
            }

            while i < bytes_len && bytes[i] != b';' && bytes[i] != b'{' {
                token_key.push(bytes[i] as char);
                i += 1;
            }
            if i < bytes_len && bytes[i] == b';' {
                i += 1;
            }

            match token_type.to_lowercase().as_str() {
                "value" => {
                    let val = get_display_string(modal, &token_key);
                    ret_vec.extend_from_slice(val.as_bytes());
                }
                "forvalue" => {
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
                "for" => {
                    while i < bytes_len && bytes[i] != b'{' { i += 1; }

                    if i < bytes_len && bytes[i] == b'{' {
                        i += 1;
                        let mut brace_count = 1;
                        let start = i;

                        while i < bytes_len && brace_count > 0 {
                            if bytes[i] == b'{' { brace_count += 1; }
                            if bytes[i] == b'}' { brace_count -= 1; }
                            i += 1;
                        }

                        let end = i - 1;
                        let inner = String::from_utf8(bytes[start..end].to_vec()).unwrap_or_default();
                        let disp_val = get_display_value(modal, &token_key);
                        
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
                "forfor" => {
                    let mut parts = token_key.splitn(2, '.');

                    if let (Some(idx_str), Some(key)) = (parts.next(), parts.next()) {
                        if let Ok(idx) = idx_str.parse::<usize>() {
                            if let Some(Some(fe_mod)) = foreach_modal.get(idx) {

                                while i < bytes_len && bytes[i] != b'{' { i += 1; }

                                if i < bytes_len && bytes[i] == b'{' {
                                    i += 1;
                                    let mut brace_count = 1;
                                    let start = i;

                                    while i < bytes_len && brace_count > 0 {
                                        if bytes[i] == b'{' { brace_count += 1; }
                                        if bytes[i] == b'}' { brace_count -= 1; }
                                        i += 1;
                                    }

                                    let end = i - 1;
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
                "if" => {
                    while i < bytes_len && bytes[i] != b'{' { i += 1; }

                    if i < bytes_len && bytes[i] == b'{' {
                        i += 1;
                        let mut brace_count = 1;
                        let start = i;

                        while i < bytes_len && brace_count > 0 {
                            if bytes[i] == b'{' { brace_count += 1; }
                            if bytes[i] == b'}' { brace_count -= 1; }
                            i += 1;
                        }

                        let end = i - 1;
                        let inner = String::from_utf8(bytes[start..end].to_vec()).unwrap_or_default();
                        let disp_val = get_display_value(modal, &token_key);

                        if disp_val.as_bool().unwrap_or(false) {
                            let parsed = parse(&inner, modal, foreach_modal);
                            ret_vec.extend_from_slice(parsed.as_bytes());
                        }
                    }
                }
                "forif" => {
                    let mut parts = token_key.splitn(2, '.');
                    if let (Some(idx_str), Some(key)) = (parts.next(), parts.next()) {
                        if let Ok(idx) = idx_str.parse::<usize>() {
                            if let Some(Some(fe_mod)) = foreach_modal.get(idx) {

                                while i < bytes_len && bytes[i] != b'{' { i += 1; }

                                if i < bytes_len && bytes[i] == b'{' {
                                    i += 1;
                                    let mut brace_count = 1;
                                    let start = i;

                                    while i < bytes_len && brace_count > 0 {
                                        if bytes[i] == b'{' { brace_count += 1; }
                                        if bytes[i] == b'}' { brace_count -= 1; }
                                        i += 1;
                                    }

                                    let end = i - 1;
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

fn get_display_value(modal: &Value, attr_val: &String) -> Value {
    let val_split = attr_val.split(".");
    let mut disp_val = modal;

    for val in val_split {
        if disp_val.is_object() {
            if val.contains("[") {
                let mut index_split = val.split('[');
                while let Some(key) = index_split.next() {
                    if key.len() == 0 {
                        break;
                    }

                    if !key.ends_with(']') {
                        disp_val = &disp_val[key];
                    }

                    if disp_val.is_array() {
                        let index = key[0..key.len()-1].parse::<usize>();

                        match index {
                            Ok(idx) => {
                                disp_val = &disp_val.as_array().unwrap()[idx];
                            }
                            Err(_) => {}
                        }
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

    if disp_val.is_string() {
        return String::from(disp_val.as_str().unwrap());
    }
    else if disp_val.is_boolean() {
        return disp_val.as_bool().unwrap().to_string();
    }
    else if disp_val.is_f64() {
        return disp_val.as_f64().unwrap().to_string();
    }
    else if disp_val.is_i64() {
        return disp_val.as_i64().unwrap().to_string();
    }
    else if disp_val.is_u64() {
        return disp_val.as_u64().unwrap().to_string();
    }
    else if disp_val.is_number() {
        return disp_val.as_number().unwrap().to_string();
    }

    String::new()
}
