use serde_json::{Value};
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

pub fn translate_class_assertion(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value {

    let class : Value = class_translation::translate(&v[1], m); 
    let individual : Value = v[2].clone();

    let operator = Value::String(String::from("ClassAssertion"));
    let v = vec![operator, class, individual];
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

pub fn translate_sub_data_property_of(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value {

    let lhs : Value = property_translation::translate(&v[1], m);
    let rhs : Value = property_translation::translate(&v[2], m);

    let operator = Value::String(String::from("SubDataPropertyOf"));
    let v = vec![operator, lhs, rhs];
    Value::Array(v) 
}

pub fn translate_inverse_object_properties(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value {

    let lhs : Value = property_translation::translate(&v[1], m);
    let rhs : Value = property_translation::translate(&v[2], m);

    let operator = Value::String(String::from("InverseObjectProperties"));
    let v = vec![operator, lhs, rhs];
    Value::Array(v) 

}

pub fn translate_object_property_range(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value { 
    let property: Value = property_translation::translate(&v[1],m); 
    let range: Value = class_translation::translate(&v[2],m); 

    let operator = Value::String(String::from("ObjectPropertyRange"));
    let v = vec![operator, property, range];
    Value::Array(v) 
}

pub fn translate_data_property_range(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value { 
    let property: Value = property_translation::translate(&v[1],m); 
    let range: Value = class_translation::translate(&v[2],m); 

    let operator = Value::String(String::from("DataPropertyRange"));
    let v = vec![operator, property, range];
    Value::Array(v) 
}

pub fn translate_annotation_property_range(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value { 
    let property: Value = property_translation::translate(&v[1],m); 
    let range: Value = class_translation::translate(&v[2],m); 

    let operator = Value::String(String::from("AnnotationPropertyRange"));
    let v = vec![operator, property, range];
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

pub fn translate_data_property_domain(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value {
    let property: Value = property_translation::translate(&v[1],m); 
    let range: Value = class_translation::translate(&v[2],m); 

    let operator = Value::String(String::from("DataPropertyDomain"));
    let v = vec![operator, property, range];
    Value::Array(v) 
}

pub fn translate_object_property_domain(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value {
    let property: Value = property_translation::translate(&v[1],m); 
    let range: Value = class_translation::translate(&v[2],m); 

    let operator = Value::String(String::from("ObjectPropertyDomain"));
    let v = vec![operator, property, range];
    Value::Array(v) 
}

pub fn translate_annotation_property_domain(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value {
    let property: Value = property_translation::translate(&v[1],m); 
    let range: Value = class_translation::translate(&v[2],m); 

    let operator = Value::String(String::from("AnnotationPropertyDomain"));
    let v = vec![operator, property, range];
    Value::Array(v) 
}

pub fn translate_domain(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value {

    let property: Value = property_translation::translate(&v[1],m); 
    let range: Value = class_translation::translate(&v[2],m); 

    let operator = 
    if property_translation::is_object_property(&property, m) { 
        Value::String(String::from("ObjectPropertyDomain")) 
    } else if property_translation::is_data_property(&property, m) { 
        Value::String(String::from("DataPropertyDomain"))
    } else if property_translation::is_annotation_property(&property, m) { 
        Value::String(String::from("AnnotationPropertyDomain"))
    } else {
        panic!("Unknown Domain axiom")
    };

    let v = vec![operator, property, range];
    Value::Array(v)
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

pub fn translate_disjoint_properties(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value { 
    let mut operands = Vec::new();
    let mut found_object_property = false;
    let mut found_data_property = false;

    for argument in &(v.as_array().unwrap())[1..] {
        let a: Value = property_translation::translate(&argument,m); 
        operands.push(a.clone());

        if property_translation::is_data_property(&a,m) {
            found_data_property = true;
        }

        if property_translation::is_object_property(&a,m) {
            found_object_property = true;
        } 
    }

    let operator = 
    if found_data_property && !found_object_property {
        Value::String(String::from("DisjointDataProperties"))
    } else if found_object_property && !found_data_property { 
        Value::String(String::from("DisjointObjectProperties"))
    } else { 
        panic!("Unknown disjoint expression")
    }; 

    let mut axiom = vec![operator];
    for o in operands {
        axiom.push(o); 
    } 
    Value::Array(axiom) 
}

pub fn translate_equivalent_properties(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value { 
    let mut operands = Vec::new();
    let mut found_object_property = false;
    let mut found_data_property = false;

    for argument in &(v.as_array().unwrap())[1..] {
        let a: Value = property_translation::translate(&argument,m); 
        operands.push(a.clone());

        if property_translation::is_data_property(&a,m) {
            found_data_property = true;
        }

        if property_translation::is_object_property(&a,m) {
            found_object_property = true;
        } 
    }

    let operator = 
    if found_data_property && !found_object_property {
        Value::String(String::from("EquivalentDataProperties"))
    } else if found_object_property && !found_data_property { 
        Value::String(String::from("EquivalentObjectProperties"))
    } else { 
        panic!("Unknown Equivalent expression")
    }; 

    let mut axiom = vec![operator];
    for o in operands {
        axiom.push(o); 
    } 
    Value::Array(axiom) 
}

pub fn translate_functional_property(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value { 

    let property = property_translation::translate(&v[1],m);

    let operator = 
    if property_translation::is_data_property(&property,m) {
        Value::String(String::from("FunctionalDataProperty")) 
    } else if property_translation::is_object_property(&property,m) {
        Value::String(String::from("FunctionalObjectProperty"))
    } else {
        panic!("Unkown functional property")
    };

    let axiom = vec![operator, property];
    Value::Array(axiom) 
}

pub fn translate_equivalent_axiom(v : &Value, m : &HashMap<String,HashSet<String>>) -> Value { 
    let mut operands = Vec::new();
    let mut found_class_expression = false;
    let mut found_data_range = false;

    for argument in &(v.as_array().unwrap())[1..] {
        let a: Value = class_translation::translate(&argument,m); 
        operands.push(a.clone());

        if class_translation::is_data_range(&a,m) {
            found_data_range = true;
        }

        if class_translation::is_class_expression(&a,m) {
            found_class_expression = true;
        } 
    }

    let operator = 
    if found_data_range && !found_class_expression {
        Value::String(String::from("DatatypeDefinition"))
    } else if found_class_expression && !found_data_range { 
        Value::String(String::from("EquivalentClasses"))
    } else { 
        panic!("Unknown Equivalent expression")
    }; 

    let mut axiom = vec![operator];
    for o in operands {
        axiom.push(o); 
    } 
    Value::Array(axiom) 
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

        //TODO: operands shoudn't be wrapped in an array?
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
        _ => class_translation::translate_assertion(v,m),
    } 

}
