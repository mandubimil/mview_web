use actix_web::Error;
use bytes::Bytes;
use std::collections::HashMap;
use actix_session::Session;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use json::JsonValue;


pub fn get_config_file() -> Result<JsonValue, Error>{
    let file = File::open("mview_web.conf").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let result = json::parse(&contents).unwrap();

    return Ok(result);    
}

pub fn get_service_ip_port() -> Result<String, Error>{
    let injson: JsonValue = get_config_file().unwrap();
    let service_ip_port = injson["service_ip_port"].as_str().unwrap();

    return Ok(service_ip_port.to_string());    
}

pub fn body_to_hash(
    body: Bytes,
) -> Result<(HashMap<String, String>,HashMap<String, String>,HashMap<String, String>), Error> {
    let body_str = std::str::from_utf8(&body).unwrap();

    let result = json::parse(body_str);
    let injson: JsonValue = match result {
        Ok(v) => v,
        Err(e) => json::object! {"err" => e.to_string()},
    };

    let mut para1: HashMap<String, String> = HashMap::new();
    if !injson["para1"].is_empty() {
        para1 = serde_json::from_str(&injson["para1"].dump()).unwrap();
    }

    let mut para2: HashMap<String, String> = HashMap::new();
    if !injson["para2"].is_empty() {
        para2 = serde_json::from_str(&injson["para2"].dump()).unwrap();
    }

    let mut para3: HashMap<String, String> = HashMap::new();
    if !injson["para3"].is_empty() {
        para3 = serde_json::from_str(&injson["para3"].dump()).unwrap();
    }

    return Ok((para1, para2, para3));
}

pub fn check_session(session: &Session) -> Result<bool, Error> {
    if let Some(check_login) = session.get::<String>("check_login").unwrap() {
        log(30, &format!("세션 확인 : {}", check_login));

        // session.set("check_login", "ok_check")?;
        // session.set("user_id", "mandu".to_string())?;    
        // return Ok(true);        

        if check_login == "ok_check" {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    return Ok(false);
}

pub fn log(log_level: i32, log_str : &str){
    let session_log_level = 30;

    if session_log_level >= log_level{
        println!("log {}/{}. {}", session_log_level, log_level, log_str);
    }
}