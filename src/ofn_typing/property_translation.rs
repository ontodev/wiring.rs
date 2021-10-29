use serde_json::{Value};
use std::collections::HashMap;
use std::collections::HashSet;

pub fn translate(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let owl_operator: String = v[0].to_string();

     match owl_operator.as_str() {
         "\"ObjectInverseOf\"" => translate_inverse_of(v,m), 
         _ => Value::String(v.to_string())
     }
}

pub fn is_object_property(v : &Value, m : &HashMap<String, HashSet<String>>) -> bool {

    let owl_operator : String ;

    if let Value::Array(_) = v { 
        owl_operator = v[0].to_string(); //compound expression
    } else {
        owl_operator = v.to_string();    //named entity
    } 

     match owl_operator.clone().as_str() {
         "\"ObjectInverseOf\"" => true,
         _ => object_type_look_up(owl_operator.clone(), m),
     } 
}

pub fn is_data_property(v : &Value, m : &HashMap<String, HashSet<String>>) -> bool {

    if let Value::Array(_) = v { 
        return false;
    } else { 
        let s = v.to_string();    //named entity
        match m.get(&s) {
            Some(set) => set.contains("owl:DatatypeProperty"),
            _ => false, 
        }
    } 
}

pub fn object_type_look_up(s : String, m: &HashMap<String, HashSet<String>>) -> bool { 
    match m.get(&s) {
        Some(set) => set.contains("owl:ObjectProperty"),
        _ => false,
    }
}

pub fn translate_inverse_of(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value { 

    let operator = Value::String(String::from("ObjectInverseOf"));
    let argument: Value = translate(&v[1],m); 
    let v = vec![operator, argument]; 
    Value::Array(v)
}

