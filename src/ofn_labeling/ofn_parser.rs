use serde_json::{Value};
use crate::ofn_labeling::axiom_translation as axiom_translation; 
use std::collections::HashMap;

//pub fn parse_ofn(t: &str, m : &HashMap<String,String>) -> String {
//    //deserialise JSON as a (serde) Value
//    let thick_triple: Value = serde_json::from_str(t).unwrap(); 
//    //start the translation process
//    let out = translate_triple(&thick_triple, m); 
//    out 
//}

pub fn parse_ofn(t: &Value, m : &HashMap<String,String>) -> String {
    //deserialise JSON as a (serde) Value
    //let thick_triple: Value = serde_json::from_str(t).unwrap(); 
    //start the translation process
    let out = translate_triple(t, m); 
    out 
}

pub fn translate_triple(v : &Value, m :&HashMap<String, String>) -> String {

    let owl_operator: String = v[0].to_string();

     let res : String = match owl_operator.as_str() {
         "\"SubClassOf\"" => axiom_translation::translate_subclass_of_axiom(v,m),
         "\"DisjointClasses\"" => axiom_translation::translate_disjoint_classes_axiom(v,m),
         "\"DisjointUnionOf\"" => axiom_translation::translate_disjoint_union_of_axiom(v,m),
         "\"EquivalentClasses\"" => axiom_translation::translate_equivalent_classes_axiom(v,m),
         _ => v.to_string(),//return named entity TODO: this should be an error?
     };

     res
} 
