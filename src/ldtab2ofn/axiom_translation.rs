use crate::ldtab2ofn::class_translation as class_translation; 
use crate::owl::thick_triple as owl;
use serde_json::{Value};

pub fn translate_subclass_of_axiom(subclass: &owl::OWL, superclass: &owl::OWL) -> Value {

    let lhs : Value = class_translation::translate(subclass);
    let rhs: Value = class_translation::translate(superclass); 

    let operator = Value::String(String::from("SubClassOf"));
    let v = vec![operator, lhs, rhs];
    Value::Array(v) 
}

pub fn translate_equivalent_class(subject: &owl::OWL, object: &owl::OWL) -> Value {

    let lhs : Value = class_translation::translate(subject);
    let mut rhs: Value = class_translation::translate(object); 

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

pub fn translate_disjoint_classes(operands: &owl::OWL) -> Value {

    let mut arguments: Value = class_translation::translate(operands); 

    let operator = Value::String(String::from("DisjointClasses"));
    let mut disjoint = vec![operator];
    let arguments = arguments.as_array_mut().unwrap();
    disjoint.append(arguments);
    Value::Array(disjoint.to_vec())
}

pub fn translate_disjoint_with(l: &owl::OWL, r: &owl::OWL) -> Value {

    let lhs : Value = class_translation::translate(l);
    let rhs: Value = class_translation::translate(r); 

    let operator = Value::String(String::from("DisjointClasses"));
    let v = vec![operator, lhs, rhs];
    Value::Array(v) 
}

pub fn translate_disjoint_union(union: &owl::OWL, operands: &owl::OWL) -> Value {

    let lhs : Value = class_translation::translate(union);
    let mut rhs: Value = class_translation::translate(operands); 

    let operator = Value::String(String::from("DisjointUnionOf"));
    let mut union = vec![operator];
    union.push(lhs);
    let arguments = rhs.as_array_mut().unwrap();
    union.append(arguments);
    Value::Array(union.to_vec())
}

pub fn translate_thin_triple(s: &str, p: &str, o: &str) -> Value {

    let subject = Value::String(String::from(s));
    let predicate = Value::String(String::from(p));
    let object = Value::String(String::from(o));

    let operator = Value::String(String::from("ThinTriple"));
    let v = vec![operator, subject, predicate, object];
    Value::Array(v) 
} 
