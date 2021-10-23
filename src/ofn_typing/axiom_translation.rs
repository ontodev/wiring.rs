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
