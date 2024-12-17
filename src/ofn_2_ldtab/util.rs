use regex::Regex;
use serde_json::json;
use serde_json::{Map, Value};
use std::collections::HashMap;

pub fn translate_literal(s: &str) -> Value {
    let language_tag = Regex::new("^\"(.+)\"@(.*)$").unwrap();
    let datatype = Regex::new("^\"(.+)\"\\^\\^(.*)$").unwrap();

    if language_tag.is_match(s) {
        match language_tag.captures(s) {
            Some(x) => json!(format!("@{}", &x[2])),
            None => json!("Error"),
        }
    } else if datatype.is_match(s) {
        match datatype.captures(s) {
            Some(x) => json!(format!("{}", &x[2])),
            None => json!("Error"),
        }
    } else {
        json!("_plain")
    }
}

pub fn translate_string(s: &str) -> Value {
    let literal = Regex::new("^\"(.+)\"(.*)$").unwrap();
    let uri = Regex::new("^<(.+)>$").unwrap();
    let curie = Regex::new("^(.+):(.+)$").unwrap();

    if literal.is_match(s) {
        translate_literal(s)
    } else if uri.is_match(s) {
        json!("_IRI")
    } else if curie.is_match(s) {
        json!("_IRI")
    } else {
        json!("ERROR")
    }
}

pub fn translate_datatype(v: &Value) -> Value {
    match v {
        Value::String(s) => translate_string(&s),
        Value::Array(_x) => json!("_JSONLIST"),
        Value::Object(_x) => json!("_JSONMAP"),
        _ => json!("error"),
    }
    //check array & object
    //check string
}

pub fn sort_value(v: &Value) -> Value {
    match v {
        Value::String(_s) => v.clone(),
        Value::Bool(_b) => v.clone(),
        Value::Number(_n) => v.clone(),
        Value::Array(a) => sort_array(a),//TODO: exclude _JSONLIST
        Value::Object(o) => sort_object(o),
        Value::Null => v.clone(),
    }
}

pub fn sort_object(v: &Map<String, Value>) -> Value {
    //serde objects are sorted by keys:
    //"By default the map is backed by a BTreeMap."
    let mut map = Map::new();

    //sort nested values
    for (key, value) in v.iter() {

        let mut sorted_value = Value::Null;

        if key == "object" && v.contains_key("datatype") && v.get("datatype").unwrap() == &json!("_JSONLIST") {
            let sorted_values: Vec<Value> = value.as_array().unwrap().into_iter().map(|x| sort_value(x)).collect();
            sorted_value = Value::Array(sorted_values);
        } else {
            sorted_value = sort_value(value);
        }
        map.insert(key.clone(), sorted_value);
    }
    Value::Object(map)
}


pub fn sort_array(v: &Vec<Value>) -> Value {
    //sort nested values
    let sorted_values: Vec<Value> = v.into_iter().map(|x| sort_value(x)).collect();

    //get string representation
    let mut string_2_value = HashMap::new();
    let mut sorted = Vec::new();

    for i in sorted_values.iter() {
        let s = i.to_string();
        sorted.push(s.clone());
        string_2_value.insert(s.clone(), i.clone());
    }

    //sort string representation
    sorted.sort();

    //build new sorted value vector
    let mut res = Vec::new();
    for i in sorted.iter() {
        let value = string_2_value.get(i).unwrap();
        res.push(value.clone());
    }

    Value::Array(res)
}
