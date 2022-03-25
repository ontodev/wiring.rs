use serde_json::{Value};
use serde_json::json; 
use std::collections::HashMap;
use crate::ofn_2_rdfa::class_translation as class_translation; 

pub fn is_named_class(ofn: &Value) -> bool {

    match ofn[0].as_str() {
         Some("ObjectSomeValuesFrom") => false, 
         Some("ObjectAllValuesFrom") => false,
         Some("ObjectHasValue") => false,
         Some("ObjectMinCardinality") => false,
         Some("ObjectMinQualifiedCardinality") => false,
         Some("ObjectMaxCardinality") => false,
         Some("ObjectMaxQualifiedCardinality") => false,
         Some("ObjectExactCardinality") => false,
         Some("ObjectExactQualifiedCardinality") => false,
         Some("ObjectHasSelf") => false,
         Some("ObjectIntersectionOf") => false,
         Some("ObjectUnionOf") => false,
         Some("ObjectOneOf") => false,
         Some("ObjectComplementOf") => false,
         Some(_) => true,
         None => true, 
    } 
}

pub fn get_type(ofn: &Value) -> &str {

     match ofn[0].as_str() {
         Some("ObjectSomeValuesFrom") => "owl:Restriction", 
         Some("ObjectAllValuesFrom") => "owl:Restriction",
         Some("ObjectHasValue") => "owl:Restriction", 
         Some("ObjectMinCardinality") => "owl:Restriction", 
         Some("ObjectMinQualifiedCardinality") => "owl:Restriction", 
         Some("ObjectMaxCardinality") => "owl:Restriction", 
         Some("ObjectMaxQualifiedCardinality") => "owl:Restriction", 
         Some("ObjectExactCardinality") => "owl:Restriction", 
         Some("ObjectExactQualifiedCardinality") => "owl:Restriction", 
         Some("ObjectHasSelf") => "owl:Restriction", 
         Some("ObjectIntersectionOf") => "owl:Class", 
         Some("ObjectUnionOf") => "owl:Class", 
         Some("ObjectOneOf") => "owl:Class", 
         Some("ObjectComplementOf") => "owl:Class", 
         None => ofn.as_str().unwrap(),
         Some(_) => panic!(),
     }
}

pub fn type_opening(ofn: &Value) -> Value {
    if is_named_class(&ofn) {
        //json!({"about": ofn})
        json!({"resource": ofn})
    } else {
        json!({"typeof": get_type(&ofn)}) 
    }
}

pub fn translate_subclass_of_axiom(sub: &Value, sup: &Value, subject_2_label: &HashMap<String,String>) -> Value {
    let opening = type_opening(sub);
    let sub_class = class_translation::translate(sub, subject_2_label, None);
    let sup_class = class_translation::translate(sup, subject_2_label, Some("rdfs:subClassOf"));
    json!(["div", opening, sub_class, " SubClassOf ", sup_class]) 
}
