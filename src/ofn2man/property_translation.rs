use serde_json::{Value};

pub fn translate(v : &Value) -> String {

    let owl_operator: String = v[0].to_string();

     match owl_operator.as_str() {
         "\"ObjectInverseOf\"" => translate_inverse_of(v), 
         _ => v.to_string().replace("\"",""),//return named entity (without quotes)
     }
}

pub fn translate_inverse_of(v : &Value) -> String {

    let argument: String = translate(&v[1]); 

    format!("inverse ( {} )", argument) 
}
