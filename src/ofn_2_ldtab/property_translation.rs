use serde_json::{Value};
use serde_json::json;
use crate::ofn_2_ldtab::util as util;

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
        json!(o) 
}

pub fn translate_inverse_of(v : &Value) -> Value {

    let argument: Value = translate(&v[1]); 

    let argument_o : Value = json!({"object" : argument,
                                    "datatype" : String::from(util::translate_datatype(&v[1]).as_str().unwrap())});

    json!({"owl:inverseOf" : vec![argument_o]}) 
} 

pub fn get_object(v : &Value) -> Value {
    let o: Value = translate(&v);
    let d: String = String::from(util::translate_datatype(&v).as_str().unwrap());

    json!({"object" : o,
           "datatype" : d}) 
}

pub fn translate_list(v : &[Value]) -> Value {

    //TODO: refactor common parts
    if v.len() == 1 {

        let first_o : Value = get_object(&v[0]);
        let rest_o : Value = get_object(&json!("rdf:nil"));


        json!({"rdf:first" : vec![first_o],
               "rdf:rest" : vec![rest_o]}) 
    } else { 

        //let first: Value = translate(&v[0]); 
        let rest: Value = translate_list(&v[1..]);//datatype is necessarily _JSON?

        let first_o : Value = get_object(&v[0]);
        let rest_o : Value = json!({"object" : rest, "datatype" : String::from("_JSON")});
        //let rest_o : Value = get_object(rest);
        //
        json!({"rdf:first" : vec![first_o],
               "rdf:rest" : vec![rest_o]}) 
    }
}

