use serde_json::{Value};
use crate::owl::typing as owl;

pub fn translate(v : &Value) -> owl::OWL {

    let owl_operator: String = v[0].to_string();

     match owl_operator.as_str() {
         "\"ObjectInverseOf\"" => translate_inverse_of(v), 
         _ => owl::OWL::Named(v.to_string().replace("\"","")),//return named entity (without quotes)
     }
}

pub fn translate_inverse_of(v : &Value) -> owl::OWL {

    let argument: owl::OWL = translate(&v[1]); 

    let argument_o : owl::Object = owl::Object{object : argument};

    let res : owl::InverseOf = owl::InverseOf{ owl_inverse_of : vec![argument_o]}; 
    owl::OWL::InverseOf(res) 
} 

