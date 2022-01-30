use serde_json::{Value};
use crate::ofn_typing::axiom_translation as axiom_translation; 
use std::collections::HashMap;
use std::collections::HashSet;

pub fn parse_ofn(v: &Value, m : &HashMap<String, HashSet<String>>) -> Value { 
     match v[0].as_str() {
         Some("SubClassOf") => axiom_translation::translate_subclass_of_axiom(v,m),
         Some("DisjointClasses") => axiom_translation::translate_disjoint_classes_axiom(v,m),
         Some("DisjointUnionOf") => axiom_translation::translate_disjoint_union_of_axiom(v,m),
         Some("EquivalentClasses") => axiom_translation::translate_equivalent_classes_axiom(v,m),
         Some(_) => panic!(),
         None => Value::String(String::from(v.as_str().unwrap())),
     } 
} 
