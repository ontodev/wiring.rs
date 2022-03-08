use serde_json::{Value};
use std::collections::HashMap;

pub fn translate(v : &Value, m : &HashMap<String, String>) -> Value {

     match v[0].as_str() {
         Some("ObjectInverseOf") => translate_inverse_of(v,m), 
         Some(_) => panic!(),
         None => Value::String(String::from(v.as_str().unwrap())),
     } 
}

pub fn translate_inverse_of(v : &Value, m : &HashMap<String, String>) -> Value {

    let operator = Value::String(String::from("ObjectInverseOf"));
    let argument: Value = translate(&v[1],m);

    let v = vec![operator, argument];
    Value::Array(v)
}

