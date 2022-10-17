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
///
/// # Examples
///
/// ```
/// use serde_json::{Value};
/// use wiring_rs::ldtab_2_ofn::property_translation as translation;
/// use wiring_rs::owl::thick_triple as owl;
///
/// let inverse_of = r#"{"owl:inverseOf":[{"datatype":"_IRI","object":"obo:IAO_0000120"}]}"#;
/// let inverse_of_owl : owl::InverseOf = serde_json::from_str(inverse_of).unwrap(); 
///
/// let inverse_of_ofn : Value = translation::translate_inverse_of(&inverse_of_owl);
///
/// let inverse_of_ofn_expected_string = r#"["ObjectInverseOf","obo:IAO_0000120"]"#;
///
/// let inverse_of_expected : Value = serde_json::from_str(inverse_of_ofn_expected_string).unwrap();
/// assert_eq!(inverse_of_ofn, inverse_of_expected); 
/// ``` 
pub fn translate_inverse_of(owl: &owl::InverseOf) -> Value { 
    let inverse_of = translate(&owl.owl_inverse_of[0].object);

    let operator = Value::String(String::from("ObjectInverseOf"));
    let v = vec![operator, inverse_of];
    Value::Array(v) 
}
