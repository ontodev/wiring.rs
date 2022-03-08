use crate::thick2ofn::class_translation as class_translation; 
use crate::owl::typing as owl;
use serde_json::{Value};

pub fn translate_subclass_of_axiom(sub: &str, sup: &str) -> Value {

    let subclass: owl::OWL = serde_json::from_str(sub).unwrap(); 
    let superclass: owl::OWL = serde_json::from_str(sup).unwrap(); 

    let lhs : Value = class_translation::translate(&subclass);
    let rhs: Value = class_translation::translate(&superclass); 

    let operator = Value::String(String::from("SubClassOf"));
    let v = vec![operator, lhs, rhs];
    Value::Array(v) 
}

pub fn translate_equivalent_class(sub: &str, sup: &str) -> Value {

    let subject: owl::OWL = serde_json::from_str(sub).unwrap(); 
    let object: owl::OWL = serde_json::from_str(sup).unwrap(); 

    let lhs : Value = class_translation::translate(&subject);
    let mut rhs: Value = class_translation::translate(&object); 

    match object {
        owl::OWL::RDFList(_) => {
            let operator = Value::String(String::from("EquivalentClasses"));
            let mut equivalent = vec![operator];
            let arguments = rhs.as_array_mut().unwrap();
            //equivalent.push(lhs); //LHS is a (generated) blank node
            equivalent.append(arguments);
            Value::Array(equivalent.to_vec())
        },
        _ => {

            let operator = Value::String(String::from("EquivalentClasses"));
            let v = vec![operator, lhs, rhs];
            Value::Array(v) 
        },
    }
}

pub fn translate_disjoint_classes(ops: &str) -> Value {

    let operands : owl::OWL = serde_json::from_str(ops).unwrap(); 
    let mut arguments: Value = class_translation::translate(&operands); 

    let operator = Value::String(String::from("DisjointClasses"));
    let mut disjoint = vec![operator];
    let arguments = arguments.as_array_mut().unwrap();
    disjoint.append(arguments);
    Value::Array(disjoint.to_vec())
}

pub fn translate_disjoint_with(lhs: &str, rhs: &str) -> Value {

    let l: owl::OWL = serde_json::from_str(lhs).unwrap(); 
    let r: owl::OWL = serde_json::from_str(rhs).unwrap(); 

    let lhs : Value = class_translation::translate(&l);
    let rhs: Value = class_translation::translate(&r); 

    let operator = Value::String(String::from("DisjointClasses"));
    let v = vec![operator, lhs, rhs];
    Value::Array(v) 
}

pub fn translate_disjoint_union(u: &str, ops: &str) -> Value {

    let union: owl::OWL = serde_json::from_str(u).unwrap(); 
    let operands: owl::OWL = serde_json::from_str(ops).unwrap(); 

    let lhs : Value = class_translation::translate(&union);
    let mut rhs: Value = class_translation::translate(&operands); 

    let operator = Value::String(String::from("DisjointUnionOf"));
    let mut union = vec![operator];
    union.push(lhs);
    let arguments = rhs.as_array_mut().unwrap();
    union.append(arguments);
    Value::Array(union.to_vec())
}

pub fn translate_thin_triple(v : &Value) -> Value {

    let s = v["subject"].as_str().unwrap();
    let p = v["predicate"].as_str().unwrap();
    let o = v["object"].as_str().unwrap();

    let subject = Value::String(String::from(s));
    let predicate = Value::String(String::from(p));
    let object = Value::String(String::from(o));

    let operator = Value::String(String::from("ThinTriple"));
    let v = vec![operator, subject, predicate, object];
    Value::Array(v) 
}


