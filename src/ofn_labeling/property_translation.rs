use serde_json::{Value};
use std::collections::HashMap;

pub fn translate(v : &Value, m : &HashMap<String, String>) -> Value {

    let owl_operator: String = v[0].to_string();

     match owl_operator.as_str() {
         "\"ObjectInverseOf\"" => translate_inverse_of(v,m),
         _ => Value::String(v.to_string())
     }
}

pub fn translate_inverse_of(v : &Value, m : &HashMap<String, String>) -> Value {

    let operator: Value = v[0].clone();
    let argument: Value = translate(&v[1],m);

    let v = vec![operator, argument];
    Value::Array(v)
}

