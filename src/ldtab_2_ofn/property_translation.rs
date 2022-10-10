use crate::owl::thick_triple as owl;
use serde_json::{Value};

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//           Object Properties
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

/// Given an OWL expressions owl
/// return its corresponding OFN S-expression.
pub fn translate(owl: &owl::OWL) -> Value {
     match &*owl {
        owl::OWL::Named(x) => translate_named(x.to_string()),
        owl::OWL::InverseOf(x) => translate_inverse_of(x),
        _ => Value::String(String::from("ERROR: Not a property")), //TODO: proper error handling
     }
}

/// Given a string, return a JSON string
pub fn translate_named(s: String) -> Value {
    Value::String(s)
}

/// Given an owl:inverseOf expression,
/// return its corresponding OFN S-expression
pub fn translate_inverse_of(owl: &owl::InverseOf) -> Value { 
    let inverse_of = translate(&owl.owl_inverse_of[0].object);

    let operator = Value::String(String::from("ObjectInverseOf"));
    let v = vec![operator, inverse_of];
    Value::Array(v) 
}
