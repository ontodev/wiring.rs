use crate::owl::thick_triple as owl;
use serde_json::Value;

//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
//           Object Properties
//;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

pub fn translate(b: &owl::OWL) -> Value {
    match &*b {
        owl::OWL::Named(x) => translate_named(x.to_string()),
        owl::OWL::InverseOf(x) => translate_inverse_of(x),
        _ => Value::String(String::from("ERROR: Not a property")), //TODO: proper error handling
    }
}

pub fn translate_named(s: String) -> Value {
    Value::String(s)
}

pub fn translate_inverse_of(s: &owl::InverseOf) -> Value {
    let inverse_of = translate(&s.owl_inverse_of[0].object);

    let operator = Value::String(String::from("ObjectInverseOf"));
    let v = vec![operator, inverse_of];
    Value::Array(v)
}
