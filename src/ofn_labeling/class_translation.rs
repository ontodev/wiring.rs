use serde_json::{Value};
use crate::ofn_labeling::property_translation as property_translation;
use std::collections::HashMap;
use crate::ofn_labeling::labeling as labeling;

//Note that (thick) triples are not OWL
pub fn translate(v : &Value, m : &HashMap<String, String>) -> Value {

    //let owl_operator: String = v[0].to_string(); 
    //match owl_operator.as_str() {

    match v[0].as_str() {
        Some("ObjectSomeValuesFrom") => translate_some_values_from(v,m), 
        Some("ObjectAllValuesFrom") => translate_all_values_from(v,m), 
        Some("ObjectHasValue") => translate_has_value(v,m), 
        Some("ObjectMinCardinality") => translate_min_cardinality(v,m), 
        Some("ObjectMinQualifiedCardinality") => translate_min_qualified_cardinality(v,m), 
        Some("ObjectMaxCardinality") => translate_max_cardinality(v,m), 
        Some("ObjectMaxQualifiedCardinality") => translate_max_qualified_cardinality(v,m), 
        Some("ObjectExactCardinality") => translate_exact_cardinality(v,m), 
        Some("ObjectExactQualifiedCardinality") => translate_exact_qualified_cardinality(v,m), 
        Some("ObjectHasSelf") => translate_has_self(v,m), 
        Some("ObjectIntersectionOf") => translate_intersection_of(v,m), 
        Some("ObjectUnionOf") => translate_union_of(v,m), 
        Some("ObjectOneOf") => translate_one_of(v,m), 
        Some("ObjectComplementOf") => translate_complement_of(v,m), 

        Some("DataSomeValuesFrom") => translate_some_values_from(v,m), 
        Some("DataAllValuesFrom") => translate_all_values_from(v,m), 
        Some("DataMinCardinality") => translate_min_cardinality(v,m), 
        Some("DataMinQualifiedCardinality") => translate_min_qualified_cardinality(v,m), 
        Some("DataMaxCardinality") => translate_max_cardinality(v,m), 
        Some("DataMaxQualifiedCardinality") => translate_max_qualified_cardinality(v,m), 
        Some("DataExactCardinality") => translate_exact_cardinality(v,m), 
        Some("DataExactQualifiedCardinality") => translate_exact_qualified_cardinality(v,m), 
        Some("DataHasSelf") => translate_has_self(v,m), 
        Some("DataHasValue") => translate_has_value(v,m), 

        //NB: these are not OWL class expressions but datatypes
        //TODO: Refactor this into a dedicated file for datatypes 
        Some("DataIntersectionOf") => translate_intersection_of(v,m), 
        Some("DataUnionOf") => translate_union_of(v,m), 
        Some("DataOneOf") => translate_one_of(v,m), 
        Some("DataComplementOf") => translate_complement_of(v,m), 


        Some("ObjectInverseOf") => property_translation::translate_inverse_of(v,m), 
        Some(_) => panic!(),  //there is no data inverse
        None => labeling::substitute(v,m),//substitute labels for entities
    }
}


//TODO: technically, all the constructor specific translations are not necessary and
//could be replaced by a simple identify function that simply recurses into nested constructors

pub fn translate_some_values_from(v : &Value, m : &HashMap<String, String>) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m); 
    let filler: Value = translate(&v[2],m); 

    let expression = vec![operator, property, filler];
    Value::Array(expression)
} 

pub fn translate_all_values_from(v : &Value, m : &HashMap<String, String>) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m); 
    let filler: Value = translate(&v[2],m); 

    let expression = vec![operator, property, filler];
    Value::Array(expression) 
} 

pub fn translate_has_value(v : &Value, m : &HashMap<String, String>) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m); 
    let filler: Value = translate(&v[2],m); 
    let expression = vec![operator, property, filler];
    Value::Array(expression) 
} 

pub fn translate_has_self(v : &Value, m : &HashMap<String, String>) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m); 
    let expression = vec![operator, property];
    Value::Array(expression) 
} 

pub fn translate_min_cardinality(v : &Value, m : &HashMap<String, String>) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m); 
    let cardinality: Value = translate(&v[2],m); 
    let expression = vec![operator, property, cardinality];
    Value::Array(expression) 
} 

pub fn translate_min_qualified_cardinality(v : &Value, m : &HashMap<String, String>) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m); 
    let cardinality: Value = translate(&v[2],m); 
    let filler: Value = translate(&v[3],m); 
    let expression = vec![operator, property, cardinality, filler];
    Value::Array(expression) 
} 

pub fn translate_max_cardinality(v : &Value, m : &HashMap<String, String>) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m); 
    let cardinality: Value = translate(&v[2],m); 

    let expression = vec![operator, property, cardinality];
    Value::Array(expression) 
} 

pub fn translate_max_qualified_cardinality(v : &Value, m : &HashMap<String, String>) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m); 
    let cardinality: Value = translate(&v[2],m); 
    let filler: Value = translate(&v[3],m); 
    let expression = vec![operator, property, cardinality, filler];
    Value::Array(expression) 
} 

pub fn translate_exact_cardinality(v : &Value, m : &HashMap<String, String>) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m); 
    let cardinality: Value = translate(&v[2],m); 

    let expression = vec![operator, property, cardinality];
    Value::Array(expression) 
} 

pub fn translate_exact_qualified_cardinality(v : &Value, m : &HashMap<String, String>) -> Value {

    let operator: Value = v[0].clone();
    let property: Value = translate(&v[1],m); 
    let cardinality: Value = translate(&v[2],m); 
    let filler: Value = translate(&v[3],m); 

    let expression = vec![operator, property, cardinality, filler];
    Value::Array(expression) 
} 

pub fn translate_list(v : &[Value], m : &HashMap<String, String>) -> Value {

    let mut res = Vec::new();
    for argument in v {
        let t: Value = translate(&argument,m); 
        res.push(t) 
    }
    Value::Array(res) 
}

pub fn translate_intersection_of(v : &Value, m : &HashMap<String, String>) -> Value {

    let operator: Value = v[0].clone();
    let mut operands : Value = translate_list(&(v.as_array().unwrap())[1..],m);

    let mut res = Vec::new();
    let r = operands.as_array_mut().unwrap();
    res.push(operator);
    res.append(r); 
    Value::Array(res) 
} 

pub fn translate_union_of(v : &Value, m : &HashMap<String, String>) -> Value {

    let operator: Value = v[0].clone();
    let mut operands : Value = translate_list(&(v.as_array().unwrap())[1..],m);

    let mut res = Vec::new();
    let r = operands.as_array_mut().unwrap();
    res.push(operator);
    res.append(r); 
    Value::Array(res) 
} 

pub fn translate_one_of(v : &Value, m : &HashMap<String, String>) -> Value {

    let operator: Value = v[0].clone();
    let mut operands : Value = translate_list(&(v.as_array().unwrap())[1..],m);

    let mut res = Vec::new();
    let r = operands.as_array_mut().unwrap();
    res.push(operator);
    res.append(r); 
    Value::Array(res) 
} 

pub fn translate_complement_of(v : &Value, m : &HashMap<String, String>) -> Value {

    let operator: Value = v[0].clone(); 
    let argument: Value = translate(&v[1],m); 
    let v = vec![operator, argument];
    Value::Array(v) 
} 
