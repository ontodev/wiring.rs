use crate::thick2ofn::class_translation as class_translation; 
use crate::owl::typing as owl;
use serde_json::{Value};

pub fn translate_subclass_of_axiom(sub: &str, sup: &str) -> String {

    let subclass: owl::OWL = serde_json::from_str(sub).unwrap(); 
    let superclass: owl::OWL = serde_json::from_str(sup).unwrap(); 

    let lhs : String = class_translation::translate(&subclass);
    let rhs: String = class_translation::translate(&superclass); 
    let expression = format!("[\"SubClassOf\",{},{}]", lhs, rhs);
    expression 
}

pub fn translate_equivalent_class(sub: &str, sup: &str) -> String {

    let subject: owl::OWL = serde_json::from_str(sub).unwrap(); 
    let object: owl::OWL = serde_json::from_str(sup).unwrap(); 

    let lhs : String = class_translation::translate(&subject);
    let rhs: String = class_translation::translate(&object); 

    match object {
        owl::OWL::RDFList(_) => {
            let expression = format!("[\"EquivalentClasses\",{}]", rhs);
            let operator = Value::String(String::from("EquivalentClasses"));
            let v = vec![operator, Value::String(rhs)];
            expression 
        },
        _ => {
            let expression = format!("[\"EquivalentClasses\",{},{}]", lhs, rhs);
            expression 
        },
    }
}

pub fn translate_disjoint_classes(ops: &str) -> String {

    let operands : owl::OWL = serde_json::from_str(ops).unwrap(); 
    let arguments: String = class_translation::translate(&operands); 
    let expression = format!("[\"DisjointClasses\",{}]", arguments);
    expression 
}

pub fn translate_disjoint_union(u: &str, ops: &str) -> String {

    let union: owl::OWL = serde_json::from_str(u).unwrap(); 
    let operands: owl::OWL = serde_json::from_str(ops).unwrap(); 

    let lhs : String = class_translation::translate(&union);
    let rhs: String = class_translation::translate(&operands); 
    let expression = format!("[\"DisjointUnionOf\",{},{}]", lhs, rhs);
    expression 
}


