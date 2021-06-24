use crate::owl::typing as owl;

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//           Object Properties
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

pub fn translate(b: &owl::OWL) -> String {
     match &*b {
        owl::OWL::Named(x) => translate_named(x.to_string()),
        owl::OWL::InverseOf(x) => translate_inverse_of(x),
        _ => String::from("ERROR: Not a property"), //TODO: proper error handling
     }
}

pub fn translate_named(s: String) -> String {
    let expression = format!("\"{}\"", s);
        expression
}

pub fn translate_inverse_of(s: &owl::InverseOf) -> String { 
    let inverse_of = translate(&s.owl_inverse_of[0].object);
    let expression = format!("[\"ObjectInverseOf\",{}]", inverse_of);
    expression
}
