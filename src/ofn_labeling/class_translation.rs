use serde_json::{Value};
use crate::ofn_labeling::property_translation as property_translation;
use std::collections::HashMap;
use crate::ofn_labeling::labeling as labeling;

//Note that (thick) triples are not OWL
pub fn translate(v : &Value, m : &HashMap<String, String>) -> String {

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


pub fn translate_some_values_from(v : &Value, m : &HashMap<String, String>) -> String {

    //translate recursively
    //let op: &Value = &v[0];//don't need OWL constructor 
    let property: String = translate(&v[1],m); 
    let filler: String = translate(&v[2],m); 

    let expression = format!("[\"ObjectSomeValuesFrom\",{},{}]", property, filler); 
    expression
} 

pub fn translate_all_values_from(v : &Value, m : &HashMap<String, String>) -> String {

    let property: String = translate(&v[1],m); 
    let filler: String = translate(&v[2],m); 
    let expression = format!("[\"ObjectAllValuesFrom\",{},{}]", property, filler);
    expression 
} 

pub fn translate_has_value(v : &Value, m : &HashMap<String, String>) -> String {

    let property: String = translate(&v[1],m); 
    let filler: String = translate(&v[2],m); 
    let expression = format!("[\"ObjectHasValue\",{},{}]", property, filler);
    expression
} 

pub fn translate_has_self(v : &Value, m : &HashMap<String, String>) -> String {

    let property: String = translate(&v[1],m); 
    let expression = format!("[\"ObjectHasSelf\",{}]", property);
    expression 
} 

pub fn translate_min_cardinality(v : &Value, m : &HashMap<String, String>) -> String {

    let property: String = translate(&v[1],m); 
    let cardinality: String = translate(&v[2],m); 
    let expression = format!("[\"ObjectMinCardinality\",{},{}]", property, cardinality);
    expression 
} 

pub fn translate_min_qualified_cardinality(v : &Value, m : &HashMap<String, String>) -> String {

    let property: String = translate(&v[1],m); 
    let cardinality: String = translate(&v[2],m); 
    let filler: String = translate(&v[3],m); 
    let expression = format!("[\"ObjectMinQualifiedCardinality\",{},{},{}]", property, cardinality, filler);
    expression
} 

pub fn translate_max_cardinality(v : &Value, m : &HashMap<String, String>) -> String {

    let property: String = translate(&v[1],m); 
    let cardinality: String = translate(&v[2],m); 

    let expression = format!("[\"ObjectMaxCardinality\",{},{}]", property, cardinality);
    expression

} 

pub fn translate_max_qualified_cardinality(v : &Value, m : &HashMap<String, String>) -> String {

    let property: String = translate(&v[1],m); 
    let cardinality: String = translate(&v[2],m); 
    let filler: String = translate(&v[3],m); 

    let expression = format!("[\"ObjectMaxQualifiedCardinality\",{},{},{}]", property, cardinality, filler);
    expression 
} 

pub fn translate_exact_cardinality(v : &Value, m : &HashMap<String, String>) -> String {

    let property: String = translate(&v[1],m); 
    let cardinality: String = translate(&v[2],m); 
    let expression = format!("[\"ObjectExactCardinality\",{},{}]", property, cardinality);
    expression 
} 

pub fn translate_exact_qualified_cardinality(v : &Value, m : &HashMap<String, String>) -> String {

    let property: String = translate(&v[1],m); 
    let cardinality: String = translate(&v[2],m); 
    let filler: String = translate(&v[3],m); 

    let expression = format!("[\"ObjectExactQualifiedCardinality\",{},{},{}]", property, cardinality, filler);
    expression 
} 

pub fn translate_list(v : &[Value], m : &HashMap<String, String>) -> String {

    if v.len() == 1 {
        let first: String = translate(&v[0],m);
        first
    } else { 

        let first: String = translate(&v[0],m); 
        let rest: String = translate_list(&v[1..],m);
        format!("{},{}", first, rest) 
    } 
}

pub fn translate_intersection_of(v : &Value, m : &HashMap<String, String>) -> String {

    let operands : String = translate_list(&(v.as_array().unwrap())[1..],m);
    let expression = format!("[\"ObjectIntersectionOf\",{}]", operands);
    expression 
} 

pub fn translate_union_of(v : &Value, m : &HashMap<String, String>) -> String {

    let operands : String = translate_list(&(v.as_array().unwrap())[1..],m);
    let expression = format!("[\"ObjectUnionOf\",{}]", operands);
    expression 
} 

pub fn translate_one_of(v : &Value, m : &HashMap<String, String>) -> String {

    let operands : String = translate_list(&(v.as_array().unwrap())[1..],m);
    let expression = format!("[\"ObjectOneOf\",{}]", operands);
    expression 
} 

pub fn translate_complement_of(v : &Value, m : &HashMap<String, String>) -> String {

    let argument: String = translate(&v[1],m); 
    let expression = format!("[\"ObjectComplementOf\",{}]", argument);
    expression 
} 
