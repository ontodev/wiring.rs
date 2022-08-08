use serde_json::{Value};
use serde_json::json; 

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

//TODO: handle datatypes/literals
pub fn translate_value(v : &Value) -> Value {
    json!("asd")
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
               "annotation" : annotation }) 
    } else { 
        json!({property_str : vec![value]})
    } 
}

pub fn translate_annotation_list(v : &Value) -> Value {
    json!("asd")
}

pub fn translate(v : &Value) -> Value {
    //TODO
    //transalte Annotation + AnnotationList
    json!("asd")
}

