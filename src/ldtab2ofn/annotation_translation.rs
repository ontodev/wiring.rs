use serde_json::{Value, Result as SResult, Map};
use serde_json::json;

pub fn translate_annotations(m : &Map<String, Value>) -> Vec<Value> {

    let mut annotations = Vec::new();

    for (k, v) in m {

        let property = Value::String(String::from(k)); 

        match v {
            //traverse list of objects
            Value::Array(array) =>  {
                for object in array { 
                    //unpack object
                    let value = object["object"].clone(); 
                    //TODO:
                    //(horned OWL currently doesn't support recrusive annotations)
                    //1. check whether object has an annotation
                    //2. translate annotation recursively
                    //3. insert annotation at the second index
                    let annotation_operator = Value::String(String::from("Annotation")); 
                    let annotation = vec![annotation_operator, property.clone(), value];
                    annotations.push(Value::Array(annotation));
                } 
            } 
            _ => panic!("Not an array"), 
        }; 
    }
    annotations 
}

pub fn translate_annotation(m : &Map<String, Value>) -> Value {

    let operator = Value::String(String::from("Annotation")); 
    let mut annotation = vec![operator];

    for (k, v) in m {

        let property = Value::String(String::from(k)); 
        annotation.push(property);

        match v {
            //traverse list of objects
            Value::Array(array) =>  {
                for object in array {

                    //unpack object
                    let value = object["object"].clone(); 
                    //TODO:
                    //(horned OWL currently doesn't support recrusive annotations)
                    //1. check whether object has an annotation
                    //2. translate annotation recursively
                    //3. insert annotation at the second index
                    annotation.push(value);
                } 
            } 
            _ => panic!("Not an array"), 
        }; 
    } 

    Value::Array(annotation)

}

pub fn translate_annotation_list(m : &Map<String, Value>) -> Value {

    let operator = Value::String(String::from("AnnotationList")); 
    let mut annotation_list = vec![operator];

    for (k, v) in m {

        let property = Value::String(String::from(k)); 
        //annotation.push(property);

        match v {
            //traverse list of objects
            Value::Array(array) =>  {
                for object in array { 
                    //unpack object
                    //TODO: use datatype key to encode literals correctly
                    let value = object["object"].clone(); 
                    //TODO:
                    //(horned OWL currently doesn't support recrusive annotations)
                    //1. check whether object has an annotation
                    //2. translate annotation recursively
                    //3. insert annotation at the second index
                    let annotation_operator = Value::String(String::from("Annotation")); 
                    let annotation = vec![annotation_operator, property.clone(), value];
                    annotation_list.push(Value::Array(annotation));
                } 
            } 
            _ => panic!("Not an array"), 
        }; 
    } 

    Value::Array(annotation_list) 
}

pub fn is_single_annotation(m : &Map<String, Value>) -> bool {

    //TODO improve this 
    if m.len() > 1 {
        false
    }
    else { 
        //there is at most one key 
        let mut res = false;
        for v in m.values() {
            //this key  has only one value
            if v.as_array().unwrap().len() ==  1 {
                res = true;
            } 
        } 
        res
    }
}


pub fn translate(annotation : &Value) -> Vec<Value> { 
     match annotation {
        Value::Object(x) => translate_annotations(&x),
        Value::Null => Vec::new(),
        //{
        //    if is_single_annotation(&x) {
        //        translate_annotation(&x)
        //    } else {
        //        translate_annotation_list(&x)
        //    }
        //},
        //Value::String(x) => annotation.clone(),
        _ => panic!("Not a valid annotation map"), 
    } 
}
