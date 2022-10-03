use serde_json::{Value};
use crate::ofn_2_thick::axiom_translation as axiom_translation; 

pub fn ofn_2_thick(v : &Value) -> Value {

     match v[0].as_str() {
         Some("SubClassOf") => axiom_translation::translate_subclass_of_axiom(v),
         Some("DisjointClasses") => axiom_translation::translate_disjoint_classes_axiom(v),
         Some("DisjointUnionOf") => axiom_translation::translate_disjoint_union_of_axiom(v),
         Some("EquivalentClasses") => axiom_translation::translate_equivalent_classes_axiom(v),
         Some(_) => panic!(),
         None => panic!(),
     } 
}
