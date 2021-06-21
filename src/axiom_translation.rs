use serde_json::{Result};

use crate::class_translation as class_translation; 

//TODO: error handling (don't use unrwap() like that...)
pub fn translate_subclass_of_axiom(sub: &str, sup: &str) -> String {

    let subclass: class_translation::OWL = serde_json::from_str(sub).unwrap(); 
    let superclass: class_translation::OWL = serde_json::from_str(sup).unwrap(); 

    let lhs : String = class_translation::thick2ofn(&subclass);
    let rhs: String = class_translation::thick2ofn(&superclass); 
    let expression = format!("[\"SubClassOf\",{},{}]", lhs, rhs);
    expression 
}


