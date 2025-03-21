use crate::owl::thick_triple as owl;
use serde_json::Value;

pub fn translate(v: &Value) -> owl::OWL {
    match v[0].as_str() {
        Some("ObjectInverseOf") => translate_inverse_of(v),
        Some(_) => panic!(),
        None => owl::OWL::Named(String::from(v.as_str().unwrap())),
    }
}

pub fn translate_inverse_of(v: &Value) -> owl::OWL {
    let argument: owl::OWL = translate(&v[1]);

    let argument_o: owl::Object = owl::Object { object: argument, datatype: String::from("_IRI"), meta: None };

    let res: owl::InverseOf = owl::InverseOf {
        owl_inverse_of: vec![argument_o],
    };
    owl::OWL::InverseOf(res)
}
