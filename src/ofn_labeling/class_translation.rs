use serde_json::{Value};
use crate::ofn_labeling::property_translation_serde as property_translation;
use std::collections::HashMap;
use crate::ofn_labeling::labeling_serde as labeling;

//Note that (thick) triples are not OWL
pub fn translate(v : &Value, m : &HashMap<String, String>) -> Value {

    let owl_operator: String = v[0].to_string();

     match owl_operator.as_str() {
         "\"ObjectSomeValuesFrom\"" => translate_some_values_from(v,m), 
         "\"ObjectAllValuesFrom\"" => translate_all_values_from(v,m), 
         "\"ObjectHasValue\"" => translate_has_value(v,m), 
         "\"ObjectMinCardinality\"" => translate_min_cardinality(v,m), 
         "\"ObjectMinQualifiedCardinality\"" => translate_min_qualified_cardinality(v,m), 
         "\"ObjectMaxCardinality\"" => translate_max_cardinality(v,m), 
         "\"ObjectMaxQualifiedCardinality\"" => translate_max_qualified_cardinality(v,m), 
         "\"ObjectExactCardinality\"" => translate_exact_cardinality(v,m), 
         "\"ObjectExactQualifiedCardinality\"" => translate_exact_qualified_cardinality(v,m), 
         "\"ObjectHasSelf\"" => translate_has_self(v,m), 
         "\"ObjectIntersectionOf\"" => translate_intersection_of(v,m), 
         "\"ObjectUnionOf\"" => translate_union_of(v,m), 
         "\"ObjectOneOf\"" => translate_one_of(v,m), 
         "\"ObjectComplementOf\"" => translate_complement_of(v,m), 
         "\"ObjectInverseOf\"" => property_translation::translate_inverse_of(v,m), 
         _ => labeling::substitute(v,m),//substitute labels for entities
     }
} 


//TODO: introduce identify function

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
