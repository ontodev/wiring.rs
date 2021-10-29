use serde_json::{Value};
use crate::ofn_typing::property_translation as property_translation;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn translate(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let owl_operator: String = v[0].to_string();

     match owl_operator.as_str() {
         "\"SomeValuesFrom\"" => translate_some_values_from(v,m), 
         "\"AllValuesFrom\"" =>  translate_all_values_from(v,m), 
         "\"HasValue\"" => translate_has_value(v,m), 
         "\"MinCardinality\"" => translate_min_cardinality(v,m), 
         "\"MaxCardinality\"" => translate_max_cardinality(v,m), 
         "\"ExactCardinality\"" => translate_exact_cardinality(v,m), 

         "\"ObjectSomeValuesFrom\"" => id(v,m),
         "\"ObjectAllValuesFrom\"" => id(v,m),
         "\"ObjectHasValue\"" =>  id(v,m),
         "\"ObjectMinCardinality\"" =>  id(v,m),
         "\"ObjectMinQualifiedCardinality\"" => id(v,m), 
         "\"ObjectMaxCardinality\"" =>  id(v,m),
         "\"ObjectMaxQualifiedCardinality\"" => id(v,m), 
         "\"ObjectExactCardinality\"" =>  id(v,m),
         "\"ObjectExactQualifiedCardinality\"" => id(v,m), 
         "\"ObjectHasSelf\"" => id(v,m), 

         //TODO: (note DataIntersection is not a class expression)
         //"\"IntersectionOf\"" => id(v,m), 
         //"\"UnionOf\"" => id(v,m), 
         //"\"OneOf\"" => id(v,m), 
         //"\"ComplementOf\"" => id(v,m), 

         "\"ObjectIntersectionOf\"" => id(v,m), 
         "\"ObjectUnionOf\"" => id(v,m), 
         "\"ObjectOneOf\"" => id(v,m), 
         "\"ObjectComplementOf\"" => id(v,m), 

         "\"ObjectInverseOf\"" => id(v,m),  //there is no data inverse
         _ => Value::String(String::from(v.as_str().unwrap())),
     }
} 

pub fn is_class_expression(v : &Value, m : &HashMap<String, HashSet<String>>) -> bool {

    let owl_operator : String ;

    if let Value::Array(_) = v { 
        owl_operator = v[0].to_string(); //compound expression
    } else {
        owl_operator = v.to_string();    //named entity
    } 

     match owl_operator.clone().as_str() {
         //abstract
         "\"SomeValuesFrom\"" => true,
         "\"AllValuesFrom\"" => true,
         "\"HasValue\"" => true,
         "\"MinCardinality\"" => true,
         "\"MaxCardinality\"" => true,
         "\"ExactCardinality\"" => true,

         //data - note that data restrictions are still class expressions
         "\"DataSomeValuesFrom\"" => true,
         "\"DataAllValuesFrom\"" => true,
         "\"DataHasValue\"" => true,
         "\"DataMinCardinality\"" => true,
         "\"DataMaxCardinality\"" => true,
         "\"DataExactCardinality\"" => true,

         "\"ObjectSomeValuesFrom\"" => true,
         "\"ObjectAllValuesFrom\"" => true,
         "\"ObjectHasValue\"" =>  true,
         "\"ObjectMinCardinality\"" =>  true,
         "\"ObjectMinQualifiedCardinality\"" => true,
         "\"ObjectMaxCardinality\"" =>  true,
         "\"ObjectMaxQualifiedCardinality\"" => true,
         "\"ObjectExactCardinality\"" =>  true,
         "\"ObjectExactQualifiedCardinality\"" => true, 
         "\"ObjectHasSelf\"" => true,

         //object (not that DataIntersections, etc. are NOT class expressions
         "\"ObjectIntersectionOf\"" => true,
         "\"ObjectUnionOf\"" => true,
         "\"ObjectOneOf\"" => true,
         "\"ObjectComplementOf\"" => true,
         _ => type_look_up(owl_operator.clone(), m),
     }
}

//TODO: refactor this into a data type translation 
//pub fn is_data_type_expression(v : &Value, m : &HashMap<String, HashSet<String>>) -> bool {
//
//}

pub fn type_look_up(s : String, m: &HashMap<String, HashSet<String>>) -> bool { 
    match m.get(&s) {
        Some(set) => set.contains("owl:Class"),
        _ => false,
    }
}

pub fn id(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value { 

    let mut res = Vec::new();
    //let operator = Value::String(String::from(v[0].to_string()));
    let operator = Value::String(String::from(v[0].as_str().unwrap()));
    res.push(operator);

    let arguments  = &(v.as_array().unwrap())[1..]; 
    for argument in arguments  {
        let test : Value = translate(argument, m);
        res.push(test);
    } 
    Value::Array(res)
}

pub fn translate_some_values_from(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let property: Value = translate(&v[1],m); 
    let filler: Value = translate(&v[2],m); 

    //TODO: check data type
    if is_class_expression(&v[2], m) || property_translation::is_object_property(&v[1],m) { 
        let operator = Value::String(String::from("ObjectSomeValuesFrom"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    } else if property_translation::is_data_property(&v[1],m) {
        let operator = Value::String(String::from("DataSomeValuesFrom"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    } else { 
        let operator = Value::String(String::from("ErrorSomeValuesFrom"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    }
} 

pub fn translate_all_values_from(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let property: Value = translate(&v[1],m); 
    let filler: Value = translate(&v[2],m); 

    //TODO: check data type
    if is_class_expression(&v[2], m) || property_translation::is_object_property(&v[1],m) {
        let operator = Value::String(String::from("ObjectAllValuesFrom"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    } else if property_translation::is_data_property(&v[1],m) {
        let operator = Value::String(String::from("DataAllValuesFrom"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    } else { 
        let operator = Value::String(String::from("ErrorAllValuesFrom"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    }
} 

pub fn translate_has_value(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let property: Value = translate(&v[1],m); 
    let filler: Value = translate(&v[2],m); 

    if is_class_expression(&v[2], m) || property_translation::is_object_property(&v[1],m) {
        let operator = Value::String(String::from("ObjectHasValue"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    } else if property_translation::is_data_property(&v[1],m) {
        let operator = Value::String(String::from("DataHasValue"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    } else {
        let operator = Value::String(String::from("ErrorHasValue"));
        let v = vec![operator, property, filler];
        Value::Array(v) 
    }
} 

pub fn translate_min_cardinality(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let property: Value = translate(&v[1],m); 
    let cardinality: Value = translate(&v[2],m); 

    if property_translation::is_object_property(&v[1],m) { 
        let operator = Value::String(String::from("ObjectMinCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    } else if property_translation::is_data_property(&v[1],m) {
        let operator = Value::String(String::from("DataMinCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    } else { 
        let operator = Value::String(String::from("ErrorMinCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    } 
} 

pub fn translate_max_cardinality(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let property: Value = translate(&v[1],m); 
    let cardinality: Value = translate(&v[2],m); 

    if property_translation::is_object_property(&v[1],m) {
        let operator = Value::String(String::from("ObjectMaxCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    } else if property_translation::is_data_property(&v[1],m) {
        let operator = Value::String(String::from("DataMaxCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    } else {
        let operator = Value::String(String::from("ErrorMaxCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    } 
} 

pub fn translate_exact_cardinality(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let property: Value = translate(&v[1],m); 
    let cardinality: Value = translate(&v[2],m); 

    if property_translation::is_object_property(&v[1],m) {
        let operator = Value::String(String::from("ObjectExactCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    } else if property_translation::is_data_property(&v[1],m) {
        let operator = Value::String(String::from("DataExactCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    } else {
        let operator = Value::String(String::from("ErrorExactCardinality"));
        let v = vec![operator, property, cardinality];
        Value::Array(v) 
    }
}

pub fn translate_list(v : &[Value], m : &HashMap<String, HashSet<String>>) -> Value {

    let mut res = Vec::new();
    for argument in v {
        let t: Value = translate(&argument,m); 
        res.push(t) 
    }
    Value::Array(res) 
}

//TODO: check arguments for propert type
pub fn translate_intersection_of(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let mut res = Vec::new();
    let operator = Value::String(String::from("ObjectIntersectionOf"));
    let mut operands : Value = translate_list(&(v.as_array().unwrap())[1..],m);
    let r = operands.as_array_mut().unwrap();
    res.push(operator);
    res.append(r); 
    Value::Array(res) 

} 

pub fn translate_union_of(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let mut res = Vec::new();
    let operator = Value::String(String::from("ObjectUnionOf"));
    let mut operands : Value = translate_list(&(v.as_array().unwrap())[1..],m);
    let r = operands.as_array_mut().unwrap();
    res.push(operator);
    res.append(r); 
    Value::Array(res)
} 

pub fn translate_one_of(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let mut res = Vec::new();
    let operator = Value::String(String::from("ObjectOneOf"));
    let mut operands : Value = translate_list(&(v.as_array().unwrap())[1..],m);
    let r = operands.as_array_mut().unwrap();
    res.push(operator);
    res.append(r); 
    Value::Array(res) 
} 

pub fn translate_complement_of(v : &Value, m : &HashMap<String, HashSet<String>>) -> Value {

    let argument: Value = translate(&v[1],m); 
    let operator = Value::String(String::from("ObjectComplementOf"));
    let v = vec![operator, argument];
    Value::Array(v) 
} 
