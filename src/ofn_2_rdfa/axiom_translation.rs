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
         Some("ObjectSomeValuesFrom") => "<http://www.w3.org/2002/07/owl#Restriction>", 
         Some("ObjectAllValuesFrom") => "<http://www.w3.org/2002/07/owl#Restriction>",
         Some("ObjectHasValue") => "<http://www.w3.org/2002/07/owl#Restriction>", 
         Some("ObjectMinCardinality") => "<http://www.w3.org/2002/07/owl#Restriction>", 
         Some("ObjectMinQualifiedCardinality") => "<http://www.w3.org/2002/07/owl#Restriction>", 
         Some("ObjectMaxCardinality") => "<http://www.w3.org/2002/07/owl#Restriction>", 
         Some("ObjectMaxQualifiedCardinality") => "<http://www.w3.org/2002/07/owl#Restriction>", 
         Some("ObjectExactCardinality") => "<http://www.w3.org/2002/07/owl#Restriction>", 
         Some("ObjectExactQualifiedCardinality") => "<http://www.w3.org/2002/07/owl#Restriction>", 
         Some("ObjectHasSelf") => "<http://www.w3.org/2002/07/owl#Restriction>", 
         Some("ObjectIntersectionOf") => "<http://www.w3.org/2002/07/owl#Class>", 
         Some("ObjectUnionOf") => "<http://www.w3.org/2002/07/owl#Class>", 
         Some("ObjectOneOf") => "<http://www.w3.org/2002/07/owl#Class>", 
         Some("ObjectComplementOf") => "<http://www.w3.org/2002/07/owl#Class>", 
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
    let sup_class = class_translation::translate(sup, subject_2_label, Some("<http://www.w3.org/2000/01/rdf-schema#subClassOf>"));
    json!(["div", opening, sub_class, " SubClassOf ", sup_class]) 
}
