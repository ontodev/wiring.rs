use regex::Regex;
use serde_json::{Value};
use serde_json::json; 

pub fn translate_literal(s: &str) -> Value {

    let language_tag = Regex::new("^\"(.+)\"@(.*)$").unwrap();
    let datatype = Regex::new("^\"(.+)\"\\^\\^(.*)$").unwrap();

    if language_tag.is_match(s) {
        match language_tag.captures(s){
            Some(x) =>  json!(format!("@{}", &x[2])),
            None => json!("Error"), 
        } 
    } else if datatype.is_match(s) {
        match datatype.captures(s){
            Some(x) =>  json!(format!("{}", &x[2])),
            None => json!("Error"), 
        } 
    } else {
        json!("asd")
    }
}

pub fn translate_string(s: &str) -> Value {

    let literal = Regex::new("^\"(.+)\"(.*)$").unwrap(); 
    let uri = Regex::new("^<(.+)>$").unwrap(); 
    let curie = Regex::new("^(.+):(.+)$").unwrap();

    if literal.is_match(s) {
        //todo translate literals
        //json!("_LITERAL")
        translate_literal(s)
    } else if uri.is_match(s) { 
        json!("_IRI")
    } else if curie.is_match(s) {
        json!("_IRI")
    } else {
        json!("ERROR")
    } 
}

pub fn translate_datatype(v : &Value) -> Value {

    match v {
        Value::String(s) => translate_string(&s), 
        Value::Array(x) => json!("_JSON"),
        Value::Object(x) => json!("_JSON"), 
        _ => json!("error"),
    }
    //check array & object
    //check string

}

