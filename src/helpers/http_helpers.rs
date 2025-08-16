use actix_web::{self, http::header::{AsHeaderName}, HttpRequest};

pub fn get_header_value(req: HttpRequest, key: impl AsHeaderName) -> String {
    // Gets the value of a header as a String. Returns empty is header does not exist.
    let header = req.headers().get(key);
    
    if header == None {
        return String::new();
    }

    match header.unwrap().to_str() {
        Ok(value) => {
            return value.to_string();
        }
        Err(_) => {
            return String::new();
        }
    }
}