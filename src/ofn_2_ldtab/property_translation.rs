use crate::ofn_2_ldtab::util;
use serde_json::json;
use serde_json::Value;

pub fn translate(v: &Value) -> Value {
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

pub fn translate_inverse_of(v: &Value) -> Value {
    let argument: Value = translate(&v[1]);

    let argument_o: Value = json!({"object" : argument,
                                   "datatype" : String::from(util::translate_datatype(&argument).as_str().unwrap())});

    json!({"<http://www.w3.org/2002/07/owl#inverseOf>" : vec![argument_o]})
}

pub fn get_object(v: &Value) -> Value {
    let o: Value = translate(&v);
    let d: String = String::from(util::translate_datatype(&o).as_str().unwrap());

    json!({"object" : o,
           "datatype" : d})
}

pub fn translate_list(v: &[Value]) -> Value {
    let mut list = Vec::new();

    for e in v.iter() {
        let e_object: Value = get_object(&e);
        list.push(e_object);
    }

    Value::Array(list)
}
