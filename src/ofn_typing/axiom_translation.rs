use serde_json::{Value};
use serde_json::json;
use crate::ofn_typing::class_translation as class_translation; //TODO: class translation
use crate::ofn_typing::property_translation as property_translation;
use crate::ofn_util::signature as signature;
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

pub fn translate_declaration(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value {

    let operand : Value = class_translation::translate(&v[1], m);

    let operator = Value::String(String::from("Declaration"));
    let v = vec![operator, operand];
    Value::Array(v) 
}

pub fn translate_sub_object_property_of(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value {

    let lhs : Value = property_translation::translate(&v[1], m);
    let rhs : Value = property_translation::translate(&v[2], m);

    let operator = Value::String(String::from("SubObjectPropertyOf"));
    let v = vec![operator, lhs, rhs];
    Value::Array(v) 

}

pub fn translate_range(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value {

    let property: Value = property_translation::translate(&v[1],m); 
    let range: Value = class_translation::translate(&v[2],m); 

    if property_translation::is_object_property(&property, m) || class_translation::is_class_expression(&range,m) { 
        let operator = Value::String(String::from("ObjectPropertyRange"));
        let v = vec![operator, property, range];
        Value::Array(v)

    } else if property_translation::is_data_property(&property, m) || class_translation::is_data_range(&range,m) { 
        let operator = Value::String(String::from("DataPropertyRange"));
        let v = vec![operator, property, range];
        Value::Array(v) 
    } else if property_translation::is_annotation_property(&property, m) { 
        let operator = Value::String(String::from("AnnotationPropertyRange"));
        let v = vec![operator, property, range];
        Value::Array(v) 
    } else {
        panic!("Unknown Range axiom")
    } 
}

pub fn translate_sub_property_of(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value {

    //get signature
    let identifiers = signature::extract_identifiers(&v);

    //check whether signature has object properties of data properties
    let mut is_object_property = false;
    let mut is_data_property = false;
    let mut is_annotation_property = false;

    for id in identifiers {
        match id {
            Value::String(x) => {
                if m.contains_key(&x) {
                    let types = m.get(&x).unwrap();
                    if types.contains("owl:ObjectProperty") {
                        is_object_property = true;
                    }
                    if types.contains("owl:DatatypeProperty")  { 
                        is_data_property = true;
                    }
                    if types.contains("owl:AnnotationProperty") {
                        is_annotation_property = true; 
                    }
                } 
            },
            _ => panic!("Not an entity"), 
        }
    }


    let operator  =
    if is_object_property && !is_data_property && !is_annotation_property  {
        Value::String(String::from("SubObjectPropertyOf"))
    } else if is_data_property && !is_object_property && !is_annotation_property {
        Value::String(String::from("SubDataPropertyOf"))
    } else if is_annotation_property && !is_data_property && !is_object_property {
        Value::String(String::from("SubAnnotationPropertyOf"))
    } else if is_object_property || is_data_property || is_annotation_property {
        panic!("Incorrect type information")
    } else { 
        panic!("Missing type information")
    };

    let lhs : Value = property_translation::translate(&v[1], m);
    let rhs : Value = property_translation::translate(&v[2], m);

    let v = vec![operator, lhs, rhs];
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
//-Object Property Assertions
//-Data Property Assertions
//-Annotation assertions
//-same as 
//-property axioms ...
//
//the type cannot always be determined by looking at the predicate alone
//so, we need to use the type look-up table here as well
pub fn translate_thin_triple(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value {

    match v[2].as_str() {

        Some("rdf:type") => class_translation::translate_rdf_type(v,m),
        //TODO: translate annotation (and then check what kind of annotation)
        _ => class_translation::translate_annotation_assertion(v,m),
    } 

}
