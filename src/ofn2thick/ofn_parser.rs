use serde_json::{Value};
use crate::ofn2thick::axiom_translation as axiom_translation; 

pub fn parse_ofn(t: &str) -> String {
    //deserialise JSON as a (serde) Value
    let thick_triple: Value = serde_json::from_str(t).unwrap(); 
    //start the translation process
    let out = translate_triple(&thick_triple); 
    out 
}

pub fn translate_triple(v : &Value) -> String {

    let owl_operator: String = v[0].to_string();

     let res : String = match owl_operator.as_str() {
         "\"SubClassOf\"" => axiom_translation::translate_subclass_of_axiom(v),
         "\"DisjointClasses\"" => axiom_translation::translate_disjoint_classes_axiom(v),
         _ => v.to_string(),//return named entity TODO: this should be an error
     };

     res
} 
