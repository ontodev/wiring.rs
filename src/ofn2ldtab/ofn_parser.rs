use serde_json::{Value};
use crate::ofn2ldtab::axiom_translation as axiom_translation; 
use crate::ofn2ldtab::util as util;

pub fn parse_ofn(t: &str) -> Value {
    //deserialise JSON as a (serde) Value
    let thick_triple: Value = serde_json::from_str(t).unwrap(); 
    //start the translation process
    let out = translate_triple(&thick_triple); 
    out 
}

pub fn translate_triple(v : &Value) -> Value {

    let ldtab_triple = match v[0].as_str() {
        Some("SubClassOf") => axiom_translation::translate_subclass_of_axiom(v),
        Some("DisjointClasses") => axiom_translation::translate_disjoint_classes_axiom(v),
        Some("DisjointUnionOf") => axiom_translation::translate_disjoint_union_of_axiom(v),
        Some("EquivalentClasses") => axiom_translation::translate_equivalent_classes_axiom(v),
        Some(_) => panic!(),
        None => panic!(),
    };

    //ensure that all triples for LDTab conform to the same order
    util::sort_value(&ldtab_triple) 
} 
