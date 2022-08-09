use serde_json::{Value};
use serde_json::json; 
use std::collections::HashMap;
use regex::Regex; 

pub fn is_annotation(v : &Value) -> bool { 
    match v.clone() { 
        Value::Array(x) => { 
            match x[0].as_str(){
                Some("Annotation") => true,
                Some("AnnotationList") => true, //NB: this is used for RDF support
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

pub fn strip_annotation(v : &Value) -> Value {
    if has_annotation(&v) {
        let mut c = v.clone();
        let a = c.as_array_mut().unwrap();
        a.remove(1);
        json!(a) 
    } else {
        v.clone()
    } 
}

pub fn get_owl(v : &Value) -> Value { 
    strip_annotation(v)
}

pub fn get_annotation(v : &Value) -> Value {
    if has_annotation(&v) {
        v.as_array().unwrap()[1].clone()
    } else {
        Value::Null
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

pub fn translate_annotation(v : &Value) -> Value {

    let stripped = strip_annotation(v);
    let stripped = stripped.as_array().unwrap();

    let property = stripped[1].clone();
    let property_str = property.as_str().unwrap();
    let value = translate_value(&stripped[2].clone());

    if has_annotation(v) { 
        let annotation = translate(&get_annotation(v));
        json!({property_str : vec![value],
               "annotation" : annotation })  //TODO: homogenise format 
    } else { 
        json!({property_str : vec![value]})
    } 
}

pub fn translate_annotation_list(v : &Value) -> Value {

    let mut property_2_value = HashMap::new();
    let mut clone = v.clone();
    let list = clone.as_array_mut().unwrap();
    list.remove(0);  //drop "AnnotationList" operator

    for annotation in list {
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

pub fn translate(v : &Value) -> Value {

    match v.clone() { 
        Value::Array(x) => { 
            match x[0].as_str(){
                Some("Annotation") => translate_annotation(v),
                Some("AnnotationList") => translate_annotation_list(v), //NB: this is used for RDF support
                Some(_) => panic!(),
                None => panic!(), 
            }
        }
        _ => panic!(),
    }
}

