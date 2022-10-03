use serde_json::{Value};

pub fn translate(v : &Value) -> String { 
    match v[0].as_str() {
         Some("ObjectInverseOf") => translate_inverse_of(v), 
         Some(_) => panic!(),
         None => String::from(v.as_str().unwrap()),//return named entity (without quotes)
     }
}

pub fn translate_inverse_of(v : &Value) -> String {

    let argument: String = translate(&v[1]); 

    format!("inverse ( {} )", argument) 
}
