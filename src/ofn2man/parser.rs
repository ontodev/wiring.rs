use serde_json::{Value};
use crate::ofn2man::axiom_translation as axiom_translation; 

pub fn parse_ofn(t: &str) -> String {
    //deserialise JSON as a (serde) Value
    let thick_triple: Value = serde_json::from_str(t).unwrap(); 
    //start the translation process
    let out = translate_triple(&thick_triple); 
    out 
}

pub fn translate_triple(v : &Value) -> String {

    match v[0].as_str() {
         Some("SubClassOf") => axiom_translation::translate_subclass_of_axiom(v),
         Some("DisjointClasses") => axiom_translation::translate_disjoint_classes_axiom(v),
         Some("DisjointUnionOf") => axiom_translation::translate_disjoint_union_of_axiom(v),
         Some("EquivalentClasses") => axiom_translation::translate_equivalent_classes_axiom(v),
         Some(_) => panic!(),
         None => panic!(),
     }
}
