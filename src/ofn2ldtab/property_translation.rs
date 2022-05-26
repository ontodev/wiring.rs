use serde_json::{Value};
use serde_json::json;
use crate::ofn2ldtab::util as util;

pub fn translate(v : &Value) -> Value {

    match v[0].as_str() {
        Some("ObjectInverseOf") => translate_inverse_of(v), 
        Some(_) => panic!(),
        //None => owl::OWL::Named(String::from(v.as_str().unwrap())),
        None => translate_named_entity(&v),
    }
}

pub fn translate_named_entity(v: &Value) -> Value {
        let o: String = String::from(v.as_str().unwrap());
        let d: String = String::from(util::translate_datatype(&v).as_str().unwrap()); 

        json!({"object" : o, "datatype" : d}) 
}

pub fn translate_inverse_of(v : &Value) -> Value {

    let argument: Value = translate(&v[1]); 

    let argument_o : Value = json!({"object" : argument,
                                    "datatype" : String::from(util::translate_datatype(&v[1]).as_str().unwrap())});

    json!({"owl:inverseOf" : vec![argument_o]}) 
} 

