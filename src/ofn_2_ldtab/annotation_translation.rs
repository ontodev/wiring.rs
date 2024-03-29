use serde_json::{Value};
use serde_json::json; 
use std::collections::HashMap;
use regex::Regex; 

pub fn is_annotation(v : &Value) -> bool { 
    match v.clone() { 
        Value::Array(x) => { 
            match x[0].as_str(){
                Some("Annotation") => true,
                //Some("AnnotationList") => true, //NB: this shouldn't occur
                Some(_) => false,
                None => false, 
            }
        }
        _ => false,
    } 
}

pub fn has_annotation(v : &Value) -> bool { 
    match v.clone() {
        Value::Array(x) => is_annotation(&x[1]), //look into second argument
        _ => false,
    } 
}

pub fn strip_annotations(v : &Value) -> Value {

    let mut res = Vec::new();
    let original = &v.as_array().unwrap()[0..];
    for element in original { 
        if !is_annotation(element){
            res.push(element.clone());
        } 
    } 
    Value::Array(res) 
}

pub fn get_owl(v : &Value) -> Value { 
    strip_annotations(v)
}


pub fn get_annotations(v : &Value) -> Vec<Value> {
    if has_annotation(&v) {

        let mut res = Vec::new();
        let candidates = &v.as_array().unwrap()[0..];
        for candidate in candidates  {
            if is_annotation(candidate){
                res.push(candidate.clone());
            } 
        }
        res
    } else {
        Vec::new()//empty vector
    } 
}


pub fn translate_literal(s: &str) -> Value {

    let language_tag = Regex::new("^\"(.+)\"@(.*)$").unwrap();
    let datatype = Regex::new("^\"(.+)\"\\^\\^(.*)$").unwrap();
    let plain = Regex::new("^\"(.+)\"$").unwrap();

    if language_tag.is_match(s) {
        match language_tag.captures(s){
            //Some(x) =>  json!(format!("@{}", &x[2])),
            Some(x) => { let lang = format!("@{}", &x[2]);
                json!({"object" : &x[1],
                       "datatype" : lang}) 
            }, 
            None => json!("Error"), 
        } 
    } else if datatype.is_match(s) {
        match datatype.captures(s){
            Some(x) => { let data = format!("{}", &x[2]); 
                json!({"object" : &x[1],
                        "datatype" : data})},
            None => json!("Error"), 
        } 
    } else if plain.is_match(s) {
        match plain.captures(s){
            Some(x) => { 
                json!({"object" : &x[1],
                        "datatype" : "_plain"})},
            None => json!("Error"), 
        } 
    
    } else {
        json!("error")
        //json!({"object" : s, "datatype": "_plain"})
    }
}

pub fn translate_value(v : &Value) -> Value { 

    let s = v.as_str().unwrap(); 

    let literal = Regex::new("^\"(.+)\"(.*)$").unwrap(); 
    let uri = Regex::new("^<(.+)>$").unwrap(); 
    let curie = Regex::new("^(.+):(.+)$").unwrap();

    if literal.is_match(s) {
        translate_literal(s)
    } else if uri.is_match(s) { 
        json!({"object" : s,
               "datatype" : "_IRI"})
    } else if curie.is_match(s) {
        json!({"object" : s,
               "datatype" : "_IRI"})
    } else {
        json!("ERROR")
    } 
}

pub fn translate_annotations(annotations : &Vec<Value>) -> Value {

    let mut property_2_value = HashMap::new(); 
    for annotation in annotations {
        let a = annotation.as_array().unwrap();
        let property = a[1].clone();
        let property_str = String::from(property.as_str().unwrap());
        let value = translate_value(&a[2].clone()); 

        //collect all annotations with the same property 
        if !property_2_value.contains_key(&property_str) {
            let mut vec = Vec::new(); 
            vec.push(value); 
            property_2_value.insert(property_str,vec);
        } else {
            property_2_value.get_mut(&property_str).unwrap().push(value); 
        } 
    } 
    json!(property_2_value) 
}

