use serde_json::{Value};
use crate::util::structural_identity as structural_identity;
use std::collections::HashMap;

pub fn translate(v : &Value, m : &HashMap<String, String>) -> Value {
    structural_identity::translate(v,m,&substitute)
} 

pub fn substitute(v : &Value, m : &HashMap<String, String>) -> Value {

    //TODO: find a better way to deal with quotes
    let element : String = v.to_string().replace("\"","");
    let split: Vec<&str> = element.split(":").collect();
    let prefix = split[0];
    let identifier = split[1];

    if m.contains_key(prefix) { 
        let expanded_prefix  = m.get(prefix).unwrap().to_string();
        let iri = format!("<{}{}>", expanded_prefix, identifier); //introduce single quotes
        Value::String(String::from(iri)) 
    } else {
        Value::String(String::from(element)) 
    }
}
