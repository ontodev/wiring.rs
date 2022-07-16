use serde_json::{Value};
use crate::ofn_labeling::property_translation as property_translation;
use std::collections::HashMap;
use crate::ofn_labeling::labeling as labeling;

pub fn translate(v : &Value, m : &HashMap<String, String>) -> Value {

    match v[0].as_str() {
        Some("SomeValuesFrom") => id(v,m), 
        Some("AllValuesFrom") => id(v,m), 
        Some("HasValue") => id(v,m), 
        Some("MinCardinality") => id(v,m), 
        Some("MinQualifiedCardinality") => id(v,m), 
        Some("MaxCardinality") => id(v,m), 
        Some("MaxQualifiedCardinality") => id(v,m), 
        Some("ExactCardinality") => id(v,m), 
        Some("ExactQualifiedCardinality") => id(v,m), 
        Some("HasSelf") => id(v,m), 
        Some("IntersectionOf") => id(v,m), 
        Some("UnionOf") => id(v,m), 
        Some("OneOf") => id(v,m), 
        Some("ComplementOf") => id(v,m), 

        Some("ObjectSomeValuesFrom") => id(v,m), 
        Some("ObjectAllValuesFrom") => id(v,m), 
        Some("ObjectHasValue") => id(v,m), 
        Some("ObjectMinCardinality") => id(v,m), 
        Some("ObjectMinQualifiedCardinality") => id(v,m), 
        Some("ObjectMaxCardinality") => id(v,m), 
        Some("ObjectMaxQualifiedCardinality") => id(v,m), 
        Some("ObjectExactCardinality") => id(v,m), 
        Some("ObjectExactQualifiedCardinality") => id(v,m), 
        Some("ObjectHasSelf") => id(v,m), 
        Some("ObjectIntersectionOf") => id(v,m), 
        Some("ObjectUnionOf") => id(v,m), 
        Some("ObjectOneOf") => id(v,m), 
        Some("ObjectComplementOf") => id(v,m), 

        Some("DataSomeValuesFrom") => id(v,m), 
        Some("DataAllValuesFrom") => id(v,m), 
        Some("DataMinCardinality") => id(v,m), 
        Some("DataMinQualifiedCardinality") => id(v,m), 
        Some("DataMaxCardinality") => id(v,m), 
        Some("DataMaxQualifiedCardinality") => id(v,m), 
        Some("DataExactCardinality") => id(v,m), 
        Some("DataExactQualifiedCardinality") => id(v,m), 
        Some("DataHasSelf") => id(v,m), 
        Some("DataHasValue") => id(v,m), 

        //NB: these are not OWL class expressions but datatypes
        //TODO: Refactor this into a dedicated file for datatypes 
        Some("DataIntersectionOf") => id(v,m), 
        Some("DataUnionOf") => id(v,m), 
        Some("DataOneOf") => id(v,m), 
        Some("DataComplementOf") => id(v,m), 

        Some("ObjectInverseOf") => property_translation::translate_inverse_of(v,m), 
        Some(_) => panic!(),
        None => labeling::substitute(v,m),
    }
}


pub fn id(v : &Value, m : &HashMap<String, String>) -> Value { 

    let mut res = Vec::new();
    let operator = Value::String(String::from(v[0].as_str().unwrap()));
    res.push(operator);

    let arguments  = &(v.as_array().unwrap())[1..]; 
    for argument in arguments  {
        let test : Value = translate(argument, m);
        res.push(test);
    } 
    Value::Array(res)
}

pub fn translate_list(v : &[Value], m : &HashMap<String, String>) -> Value {

    let mut res = Vec::new();
    for argument in v {
        let t: Value = translate(&argument,m); 
        res.push(t) 
    }
    Value::Array(res) 
}



