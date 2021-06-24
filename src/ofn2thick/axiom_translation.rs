use serde_json::{Value};
use serde_json::json; 
use crate::owl::typing as owl;
use crate::ofn2thick::class_translation as class_translation; 
use rand::Rng; 

//use crate::ofn2thick::owl as owl; 

pub fn translate_subclass_of_axiom(v : &Value) -> String {

    //translate OWL classes
    let subclass = class_translation::translate(&v[1]);
    let superclass = class_translation::translate(&v[2]); 

    //serialise to JSON
    let sub_json = json!(subclass);
    let sup_json = json!(superclass);

    //convert to string
    let sub_string = sub_json.to_string();
    let sup_string = sup_json.to_string();

    //putting it all together
    let expression = format!("{{\"subject\": {}, \"predicate\": \"rdfs:subClassOf\", \"object\": {}}}", sub_string, sup_string);
    expression
}

pub fn translate_disjoint_classes_axiom(v : &Value) -> String {

    // TODO make sure generated blank node ID's are unique
    // this will be done as part of some post-processing
    let mut rng = rand::thread_rng(); 
    let blank_id: u8 = rng.gen();

    let operands : owl::OWL = class_translation::translate_list(&(v.as_array().unwrap())[1..]); 
    let operads_json = json!(operands); 
    let operands_string = operads_json.to_string();

    let expression = format!("{{\"subject\": _:gen{}, \"predicate\": \"owl:AllDisjointClasses\", \"object\": {{\"owl:members\": {}}}}}", blank_id, operands_string);
    expression 
}

pub fn translate_disjoint_union_of_axiom(v : &Value) -> String {

    let lhs = class_translation::translate(&v[1]);
    let operands : owl::OWL = class_translation::translate_list(&(v.as_array().unwrap())[2..]); 

    let lhs_json = json!(lhs);
    let operads_json = json!(operands); 

    let lhs_string =  lhs_json.to_string();
    let operands_string = operads_json.to_string();

    let expression = format!("{{\"subject\": {}, \"predicate\": \"owl:disjointUnionOf\", \"object\": {}}}", lhs_string, operands_string);
    expression 
}

//TODO::  disjointUnion, equivalent classe
