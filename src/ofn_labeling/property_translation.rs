use serde_json::{Value};
use std::collections::HashMap;

pub fn translate(v : &Value, m : &HashMap<String, String>) -> String {

    let owl_operator: String = v[0].to_string();

     match owl_operator.as_str() {
         "\"ObjectInverseOf\"" => translate_inverse_of(v,m), 
         _ => v.to_string().replace("\"",""),//return named entity (without quotes)
     }
}

pub fn translate_inverse_of(v : &Value, m : &HashMap<String, String>) -> String {

    let argument: String = translate(&v[1],m); 
    let expression = format!("[\"ObjectInverseOf\",{}]", argument);
    expression 
} 

