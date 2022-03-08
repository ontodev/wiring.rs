use serde_json::{Value};
use crate::ofn_typing::class_translation as class_translation; //TODO: class translation
use std::collections::HashMap;
use std::collections::HashSet;


pub fn translate_subclass_of_axiom(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value {

    //translate OWL classes
    let subclass : Value = class_translation::translate(&v[1], m);
    let superclass : Value = class_translation::translate(&v[2], m); 

    let operator = Value::String(String::from("SubClassOf"));
    let v = vec![operator, subclass, superclass];
    Value::Array(v) 
}

pub fn translate_disjoint_classes_axiom(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value {
    let mut operands : Value = class_translation::translate_list(&(v.as_array().unwrap())[1..], m); 

    let operator = Value::String(String::from("DisjointClasses"));
    let mut disjoint = vec![operator];
    let arguments = operands.as_array_mut().unwrap();
    disjoint.append(arguments);
    Value::Array(disjoint.to_vec())
}

pub fn translate_disjoint_union_of_axiom(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value {

    let lhs : Value = class_translation::translate(&v[1], m);
    let operands : Value = class_translation::translate_list(&(v.as_array().unwrap())[2..], m); 

    let operator = Value::String(String::from("DisjointUnionOf"));
    let v = vec![operator, lhs, operands];
    Value::Array(v) 
}


//TODO::   equivalent classe  (we have a custom encoding for this and need a case distinction
//between binary axioms and n-ary axioms)
pub fn translate_equivalent_classes_axiom(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value {
    let number_of_operands =  (v.as_array().unwrap())[1..].len();
    if number_of_operands == 2 {
        let lhs : Value = class_translation::translate(&v[1],m);
        let rhs : Value = class_translation::translate(&v[2],m); 

        let operator = Value::String(String::from("EquivalentClasses"));
        let v = vec![operator, lhs, rhs];
        Value::Array(v) 

    } else {

        let operands : Value = class_translation::translate_list(&(v.as_array().unwrap())[1..],m); 
        let operator = Value::String(String::from("EquivalentClasses"));
        let v = vec![operator, operands];
        Value::Array(v) 
    }
}

//TODO: need to distinguish:
//-types
//-Object Property Assertions
//-Data Property Assertions
//-Annotation assertions
//-same as 
//-property axioms ...
//
//the type cannot always be determined by looking at the predicate alone
//so, we need to use the type look-up table here as well
pub fn translate_thin_triple(v : &Value) -> Value {
    //this just creates a copy of an OFN-S thin triple
    let s = v[1].as_str().unwrap();
    let p = v[2].as_str().unwrap();
    let o = v[3].as_str().unwrap();

    let subject = Value::String(String::from(s));
    let predicate = Value::String(String::from(p));
    let object = Value::String(String::from(o));

    let operator = Value::String(String::from("ThinTriple"));
    let v = vec![operator, subject, predicate, object];
    Value::Array(v) 

}
