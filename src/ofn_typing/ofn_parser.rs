use serde_json::{Value};
use crate::ofn_typing::axiom_translation as axiom_translation; 
use std::collections::HashMap;
use std::collections::HashSet;

pub fn parse_ofn(t: &Value, m : &HashMap<String, HashSet<String>>) -> Value { 
    //start the translation process
    let out = translate_triple(&t, m); 
    out 
}

pub fn translate_triple(v : &Value, m :&HashMap<String, HashSet<String>>) -> Value {

    let owl_operator: String = v[0].to_string();

     let res : Value = match owl_operator.as_str() {
         "\"SubClassOf\"" => axiom_translation::translate_subclass_of_axiom(v,m),
         "\"DisjointClasses\"" => axiom_translation::translate_disjoint_classes_axiom(v,m),
         "\"DisjointUnionOf\"" => axiom_translation::translate_disjoint_union_of_axiom(v,m),
         "\"EquivalentClasses\"" => axiom_translation::translate_equivalent_classes_axiom(v,m),
         _ => Value::String(v.to_string()),
     };

     res
} 
