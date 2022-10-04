use serde_json::{Value};
use crate::util::structural_identity as structural_identity;
use std::collections::HashMap;

pub fn translate(v : &Value, m : &HashMap<String, String>) -> Value {
    structural_identity::translate(v,m,&substitute)
} 

pub fn substitute(v : &Value, m : &HashMap<String, String>) -> Value {

    let element = v.as_str().unwrap();
    //remove enclosing angle brackets < > of an IRI
    let element = rem_first_and_last(element);

    for (key, value) in m { 
        //remove enclosing angle brackets < > of prefix IRI
        let trimmed_key = rem_first_and_last(key);
        //if element.contains(key) {
        if element.starts_with(trimmed_key) {
            let iri = element.replace(trimmed_key, value); 
            return Value::String(String::from(iri))
        }
    }
    Value::String(String::from(element)) 
}

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}
