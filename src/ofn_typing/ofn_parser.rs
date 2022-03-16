use serde_json::{Value};
use crate::ofn_typing::axiom_translation as axiom_translation; 
use crate::ofn_typing::class_translation as class_translation; 
use std::collections::HashMap;
use std::collections::HashSet;

pub fn parse_ofn(v: &Value, m : &HashMap<String, HashSet<String>>) -> Value { 
    match v[0].as_str() {
        Some("SubClassOf") => axiom_translation::translate_subclass_of_axiom(v,m),
        Some("DisjointClasses") => axiom_translation::translate_disjoint_classes_axiom(v,m),
        Some("DisjointUnionOf") => axiom_translation::translate_disjoint_union_of_axiom(v,m),
        Some("EquivalentClasses") => axiom_translation::translate_equivalent_classes_axiom(v,m),
        Some("ThinTriple") => axiom_translation::translate_thin_triple(v),

        Some("SomeValuesFrom") => class_translation::translate_some_values_from(v,m), 
        Some("AllValuesFrom") =>  class_translation::translate_all_values_from(v,m), 
        Some("HasValue") => class_translation::translate_has_value(v,m), 
        Some("MinCardinality") => class_translation::translate_min_cardinality(v,m), 
        Some("MaxCardinality") => class_translation::translate_max_cardinality(v,m), 
        Some("ExactCardinality") => class_translation::translate_exact_cardinality(v,m), 

        Some("ObjectSomeValuesFrom") => class_translation::id(v,m),
        Some("ObjectAllValuesFrom") => class_translation::id(v,m),
        Some("ObjectHasValue") =>  class_translation::id(v,m),
        Some("ObjectMinCardinality") =>  class_translation::id(v,m),
        Some("ObjectMinQualifiedCardinality") => class_translation::id(v,m), 
        Some("ObjectMaxCardinality") =>  class_translation::id(v,m),
        Some("ObjectMaxQualifiedCardinality") => class_translation::id(v,m), 
        Some("ObjectExactCardinality") =>  class_translation::id(v,m),
        Some("ObjectExactQualifiedCardinality") => class_translation::id(v,m), 
        Some("ObjectHasSelf") => class_translation::id(v,m), 
        Some("ObjectIntersectionOf") => class_translation::id(v,m), 
        Some("ObjectUnionOf") => class_translation::id(v,m), 
        Some("ObjectOneOf") => class_translation::id(v,m), 
        Some("ObjectComplementOf") => class_translation::id(v,m), 

        Some("ObjectInverseOf") => class_translation::id(v,m),  //there is no data inverse
        Some(_) => panic!(),
        None => Value::String(String::from(v.as_str().unwrap())),
    } 
} 
