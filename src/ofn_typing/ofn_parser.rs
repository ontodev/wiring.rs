use serde_json::{Value};
use crate::ofn_typing::axiom_translation as axiom_translation; 
use crate::ofn_typing::class_translation as class_translation; 
use std::collections::HashMap;
use std::collections::HashSet;

//TODO: handle annotations properly
pub fn parse_ofn(v: &Value, m : &HashMap<String, HashSet<String>>) -> Value { 
    match v[0].as_str() {
        Some("SubClassOf") => axiom_translation::translate_subclass_of_axiom(v,m),
        Some("DisjointClasses") => axiom_translation::translate_disjoint_classes_axiom(v,m),
        Some("DisjointUnionOf") => axiom_translation::translate_disjoint_union_of_axiom(v,m),
        Some("EquivalentClasses") => axiom_translation::translate_equivalent_classes_axiom(v,m),
        //TODO: ThinTriples need to be typed as well
        Some("ThinTriple") => axiom_translation::translate_thin_triple(v),
        Some("Declaration") => axiom_translation::translate_declaration(v,m),

        Some("SomeValuesFrom") => class_translation::translate_some_values_from(v,m), 
        Some("AllValuesFrom") =>  class_translation::translate_all_values_from(v,m), 
        Some("HasValue") => class_translation::translate_has_value(v,m), 
        Some("MinCardinality") => class_translation::translate_min_cardinality(v,m), 
        Some("MaxCardinality") => class_translation::translate_max_cardinality(v,m), 
        Some("ExactCardinality") => class_translation::translate_exact_cardinality(v,m), 

        //TODO: axioms 
        //Some("SubObjectPropertyOf") => translate_sub_object_property_of(v),
        //Some("EquivalentObjectProperties") => translate_equivalent_object_properties(v),
        //Some("DisjointObjectProperties") => translate_disjoint_object_properties(v),
        //Some("InverseObjectProperties") => translate_inverse_object_properties(v),
        //Some("ObjectPropertyDomain") => translate_object_property_domain(v),
        //Some("ObjectPropertyRange") => translate_object_property_range(v),
        //Some("FunctionalObjectProperty") => translate_functional_object_property(v),
        //Some("InverseFunctionalObjectProperty") => translate_inverse_functional_object_property(v),
        //Some("SubDataPropertyOf") => translate_sub_dataproperty_of(v),
        //Some("EquivalentDataProperties") => translate_equivalent_data_properties(v),
        //Some("DisjointDataProperties") => translate_disjoint_data_properties(v),
        //Some("DataPropertyDomain") => translate_data_property_domain(v),
        //Some("DataPropertyRange") => translate_data_property_range(v),
        //Some("FunctionalDataProperty") => translate_functional_data_property(v),

        //Some("DatatypeDefinition") => translate_datatype_definition(v),
        ////Some("HasKey") => translate_has_key(v), //no property type support in OFN S
        //Some("HasKey") => panic!("HasKey operator currently not supported"),

        //Some("SameIndividual") => translate_same_individual(v),
        //Some("DifferentIndividual") => translate_different_individuals(v),

        //Some("ClassAssertion") => translate_class_assertion(v),
        //Some("ObjectPropertyAssertion") => translate_object_property_assertion(v),
        //Some("NegativeObjectPropertyAssertion") => translate_negative_object_property_assertion(v),
        //Some("DataPropertyAssertion") => translate_data_property_assertion(v),
        //Some("NegativeDataPropertyAssertion") => translate_negative_data_property_assertion(v),

        //Some("AnnotationAssertion") => translate_annotation_assertion(v),
        //Some("SubAnnotationPropertyOf") => translate_sub_annotation_assertion(v),
        //Some("AnnotationPropertyDomain") => translate_annotation_property_domain(v),
        //Some("AnnotationPropertyRange") => translate_annotation_property_domain(v),

        //TODO:
        //Some("MinQualifiedCardinality") => class_translation::translate_min_cardinality(v,m), 
        //Some("MaxQualifiedCardinality") => class_translation::translate_max_cardinality(v,m), 
        //Some("ExactQualifiedCardinality") => class_translation::translate_exact_cardinality(v,m), 
        //Some("IntersectionOf") => translate_intersection_of(v), 
        //Some("UnionOf") => translate_union_of(v), 
        //Some("OneOf") => translate_one_of(v), 
        //Some("ComplementOf") => translate_complement_of(v), 
        //Some("InverseOf") => property_translation::translate_inverse_of(v),

        //TODO: we need to list all constructors here
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
