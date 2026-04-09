use crate::constants::*;
use regex::Regex;
use serde_json::json;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::LazyLock;

static RE_LANGUAGE_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^\"(.+)\"@(.*)$").unwrap());
static RE_DATATYPE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^\"(.+)\"\\^\\^(.*)$").unwrap());
static RE_LITERAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^\"(.+)\"(.*)$").unwrap());
static RE_URI: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^<(.+)>$").unwrap());
static RE_CURIE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^(.+):(.+)$").unwrap());

/// Generates a blank node ID for an LDTab object.
pub fn generate_blank_node_id(v: &Value) -> String {
    let sorted = sort_value(v);
    let s = sorted.to_string();
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    let hash = hasher.finalize();
    format!("<ldtab:blanknode:{:x}>", hash)
}

pub fn translate_literal(s: &str) -> Value {
    if RE_LANGUAGE_TAG.is_match(s) {
        match RE_LANGUAGE_TAG.captures(s) {
            Some(x) => json!(format!("@{}", &x[2])),
            None => json!("Error"),
        }
    } else if RE_DATATYPE.is_match(s) {
        match RE_DATATYPE.captures(s) {
            Some(x) => json!(format!("{}", &x[2])),
            None => json!("Error"),
        }
    } else {
        json!(XSD_STRING)
    }
}

pub fn translate_string(s: &str) -> Value {
    if RE_LITERAL.is_match(s) {
        translate_literal(s)
    } else if RE_URI.is_match(s) {
        json!(LDTAB_IRI)
    } else if RE_CURIE.is_match(s) {
        json!(LDTAB_IRI)
    } else {
        json!("ERROR")
    }
}

pub fn translate_datatype(v: &Value) -> Value {
    match v {
        Value::String(s) => translate_string(&s),
        Value::Array(_x) => json!(LDTAB_JSON_LIST),
        Value::Object(_x) => json!(LDTAB_JSON_MAP),
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
        Value::Array(a) => sort_array(a), //TODO: exclude _JSONLIST
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
        let sorted_value;

        if key == "object"
            && v.contains_key("datatype")
            && v.get("datatype").unwrap() == &json!(LDTAB_JSON_LIST)
        {
            //check if value is none
            match value.as_array() {
                Some(_val) => {}
                None => {
                    println!("NOT AN ARRAY {:?}", value);
                }
            }

            let sorted_values: Vec<Value> = value
                .as_array()
                .unwrap()
                .into_iter()
                .map(|x| sort_value(x))
                .collect();
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
