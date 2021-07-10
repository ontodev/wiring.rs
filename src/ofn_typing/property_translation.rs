use serde_json::{Value};
use std::collections::HashMap;
use std::collections::HashSet;

pub fn translate(v : &Value, m : &HashMap<String, HashSet<String>>) -> String {

    let owl_operator: String = v[0].to_string();

     match owl_operator.as_str() {
         "\"ObjectInverseOf\"" => translate_inverse_of(v,m), 
         _ => v.to_string().replace("\"",""),//return named entity (without quotes)
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
         _ => type_look_up(owl_operator.clone(), m),
     } 
}

pub fn type_look_up(s : String, m: &HashMap<String, HashSet<String>>) -> bool { 
    match m.get(&s) {
        Some(set) => set.contains("owl:ObjectProperty"),
        _ => false,
    }
}

pub fn translate_inverse_of(v : &Value, m : &HashMap<String, HashSet<String>>) -> String { 
    let argument: String = translate(&v[1],m); 
    let expression = format!("[\"ObjectInverseOf\",{}]", argument);
    expression 
} 

