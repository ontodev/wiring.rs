use serde_json::{Value};
use std::collections::HashMap;
use std::collections::HashSet;

pub fn translate(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

     match v[0].as_str() {
         Some("ObjectInverseOf") => translate_inverse_of(v,m), 
         Some(_) => panic!(),
         None => Value::String(String::from(v.as_str().unwrap())),
     }
}

pub fn is_object_property(v : &Value, m : &HashMap<String, HashSet<String>>) -> bool {

     match v[0].as_str() { 
         Some("ObjectInverseOf") => true,
         Some(_) => panic!(),
         None => object_type_look_up(String::from(v.as_str().unwrap()), m),
     } 
}

//TODO: test this
pub fn is_data_property(v : &Value, m : &HashMap<String, HashSet<String>>) -> bool {

    let key = match v.as_str() {
        Some(s) => String::from(s),
        None => v.to_string() 
    };

    match m.get(&key) {
        Some(set) => set.contains("owl:DatatypeProperty"),//we are using JSON Strings here
        _ => false, 
    } 
}

pub fn is_annotation_property(v : &Value, m : &HashMap<String, HashSet<String>>) -> bool {

    let key = match v.as_str() {
        Some(s) => String::from(s),
        None => v.to_string() 
    };

    match m.get(&key) {
        Some(set) => set.contains("owl:AnnotationProperty"),
        _ => false, 
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

