use serde_json::{Value};
use serde_json::json; 
use std::collections::HashMap;

use crate::ofn_2_rdfa::axiom_translation as axiom_translation; 

pub fn translate(v : &Value) -> Value {

    match v[0].as_str() {
        Some("SubClassOf") => axiom_translation::translate_subclass_of_axiom(&v[1], &v[2]),
        Some("DisjointClasses") => json!("todo"),
        Some("DisjointUnionOf") => json!("todo"),
        Some("EquivalentClasses") => json!("todo"), 
        Some(_) => panic!(),
        None => panic!(), 
    } 
} 
