use serde_json::{Value};
use crate::ofn_util::structural_identity as structural_identity;
use std::collections::HashMap;

pub fn translate(v : &Value, m : &HashMap<String, String>) -> Value {
    structural_identity::translate(v,m,&substitute)
} 

pub fn substitute(v : &Value, m : &HashMap<String, String>) -> Value {

    //TODO: find a better way to deal with quotes
    let element : String = v.to_string().replace("\"",""); 

    for (key, value) in m { 
        if element.contains(key) {
            //TODO: make sure that the identified prefix is indeed a prefix
            let iri = element.replace(key, value); 
            return Value::String(String::from(iri))
        }
    }
    Value::String(String::from(element)) 
}
