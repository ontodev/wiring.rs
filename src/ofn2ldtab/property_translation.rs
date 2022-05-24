use serde_json::{Value};
use crate::owl::thick_triple as owl;
use crate::ofn2ldtab::util as util;

pub fn translate(v : &Value) -> owl::OWL {

    match v[0].as_str() {
        Some("ObjectInverseOf") => translate_inverse_of(v), 
        Some(_) => panic!(),
        //None => owl::OWL::Named(String::from(v.as_str().unwrap())),
        None => translate_named_entity(&v),
    }
}

pub fn translate_named_entity(v: &Value) -> owl::OWL {
        let o: String = String::from(v.as_str().unwrap());
        let d: String = String::from(util::translate_datatype(&v).as_str().unwrap()); 

        let terminal = owl::TerminalObject{object : o, datatype: d, meta: None };
        owl::OWL::TerminalObject(terminal) 
}

pub fn translate_inverse_of(v : &Value) -> owl::OWL {

    let argument: owl::OWL = translate(&v[1]); 

    let argument_o : owl::Object = owl::Object{object : argument, datatype : String::from(util::translate_datatype(&v[1]).as_str().unwrap()), meta: None};

    let res : owl::InverseOf = owl::InverseOf{ owl_inverse_of : vec![argument_o]}; 
    owl::OWL::InverseOf(res) 
} 

