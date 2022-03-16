use serde_json::{Value};
use crate::ofn_labeling::axiom_translation as axiom_translation; 
use crate::ofn_labeling::class_translation as class_translation; 
use crate::ofn_labeling::property_translation as property_translation;
use std::collections::HashMap;

//TODO: there is no 'real' parsing going on
//TODO: could refactor the whole labelling translation into a single file
//since OFN-S expressions are handled in a uniform manner 
pub fn parse_ofn(v: &Value, m : &HashMap<String,String>) -> Value { 

    match v[0].as_str() {
        Some("SubClassOf") => axiom_translation::translate_subclass_of_axiom(v,m),
        Some("DisjointClasses") => axiom_translation::translate_disjoint_classes_axiom(v,m),
        Some("DisjointUnionOf") => axiom_translation::translate_disjoint_union_of_axiom(v,m),
        Some("EquivalentClasses") => axiom_translation::translate_equivalent_classes_axiom(v,m),
        Some("ThinTriple") => axiom_translation::translate_thin_triple(v,m),

        Some("ObjectSomeValuesFrom") => class_translation::id(v,m), 
        Some("ObjectAllValuesFrom") => class_translation::id(v,m), 
        Some("ObjectHasValue") => class_translation::id(v,m), 
        Some("ObjectMinCardinality") => class_translation::id(v,m), 
        Some("ObjectMinQualifiedCardinality") => class_translation::id(v,m), 
        Some("ObjectMaxCardinality") => class_translation::id(v,m), 
        Some("ObjectMaxQualifiedCardinality") => class_translation::id(v,m), 
        Some("ObjectExactCardinality") => class_translation::id(v,m), 
        Some("ObjectExactQualifiedCardinality") => class_translation::id(v,m), 
        Some("ObjectHasSelf") => class_translation::id(v,m), 
        Some("ObjectIntersectionOf") => class_translation::id(v,m), 
        Some("ObjectUnionOf") => class_translation::id(v,m), 
        Some("ObjectOneOf") => class_translation::id(v,m), 
        Some("ObjectComplementOf") => class_translation::id(v,m), 

        Some("DataSomeValuesFrom") => class_translation::id(v,m), 
        Some("DataAllValuesFrom") => class_translation::id(v,m), 
        Some("DataMinCardinality") => class_translation::id(v,m), 
        Some("DataMinQualifiedCardinality") => class_translation::id(v,m), 
        Some("DataMaxCardinality") => class_translation::id(v,m), 
        Some("DataMaxQualifiedCardinality") => class_translation::id(v,m), 
        Some("DataExactCardinality") => class_translation::id(v,m), 
        Some("DataExactQualifiedCardinality") => class_translation::id(v,m), 
        Some("DataHasSelf") => class_translation::id(v,m), 
        Some("DataHasValue") => class_translation::id(v,m), 

        //NB: these are not OWL class expressions but datatypes
        //TODO: Refactor this into a dedicated file for datatypes 
        Some("DataIntersectionOf") => class_translation::id(v,m), 
        Some("DataUnionOf") => class_translation::id(v,m), 
        Some("DataOneOf") => class_translation::id(v,m), 
        Some("DataComplementOf") => class_translation::id(v,m), 

        Some("ObjectInverseOf") => property_translation::translate_inverse_of(v,m), 

        Some(_) => panic!(),
        None => Value::String(String::from(v.as_str().unwrap())),
    }
} 
