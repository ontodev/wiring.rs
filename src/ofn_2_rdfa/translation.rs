use serde_json::{Value};
use serde_json::json; 
use std::collections::HashMap;

use crate::ofn_2_rdfa::axiom_translation as axiom_translation; 

/// Given an OFN S-expression (encoded in JSON),
/// return its corresponding representation in RDFa (encoded in hiccup JSON) 
///
/// Examples
///
/// let ofn_string = r#"["SubClassOf","obo:IAO_0000120",["ObjectSomeValuesFrom","obo:BFO_0000050","obo:OBI_0500000"]]"#; 
/// let ofn = ofn_2_man::parser::parse(&ofn_string); //TODO: refactor parser
/// let rdfa = ofn_2_rdfa::translation::ofn_2_rdfa(&ofn);
/// println!("{}", rdfa); 
pub fn ofn_2_rdfa(v : &Value, subject_2_label: &HashMap<String,String>) -> Value {

    match v[0].as_str() {
        Some("SubClassOf") => axiom_translation::translate_subclass_of_axiom(&v[1], &v[2], subject_2_label),
        Some("DisjointClasses") => json!("todo"),
        Some("DisjointUnionOf") => json!("todo"),
        Some("EquivalentClasses") => json!("todo"), 
        Some(_) => panic!(),
        None => panic!(), 
    } 
} 
