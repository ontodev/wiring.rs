use serde_json::{Value};
use crate::ofn_typing::axiom_translation as axiom_translation; 
use crate::ofn_typing::class_translation as class_translation; 
use std::collections::HashMap;
use std::collections::HashSet;

//TODO: factor this out
pub fn is_annotation(v : &Value) -> bool { 
    match v.clone() { 
        Value::Array(x) => { 
            match x[0].as_str(){
                Some("Annotation") => true,
                //Some("AnnotationList") => true, //NB: this shouldn't occur
                Some(_) => false,
                None => false, 
            }
        }
        _ => false,
    } 
}

pub fn strip_annotations(v : &Value) -> Value {

    let mut res = Vec::new();
    let original = &v.as_array().unwrap()[0..];
    for element in original { 
        if !is_annotation(element){
            res.push(element.clone());
        } 
    } 
    Value::Array(res) 
}

pub fn has_annotation(v : &Value) -> bool { 
    match v.clone() {
        Value::Array(x) => is_annotation(&x[1]), //look into second argument
        _ => false,
    } 
}

pub fn get_annotations(v : &Value) -> Vec<Value> {
    if has_annotation(&v) {

        let mut res = Vec::new();
        let candidates = &v.as_array().unwrap()[0..];
        for candidate in candidates  {
            if is_annotation(candidate){
                res.push(candidate.clone());
            } 
        }
        res
    } else {
        Vec::new()//empty vector
    } 
}

pub fn get_owl(v : &Value) -> Value { 
    strip_annotations(v)
}

pub fn parse_ofn(v: &Value, m : &HashMap<String, HashSet<String>>) -> Value { 

    let annotations = get_annotations(v);
    let v = &get_owl(v);//TODO: rename v to owl

    let ofn = 
    match v[0].as_str() {
        Some("SubClassOf") => axiom_translation::translate_subclass_of_axiom(v,m),
        Some("DisjointClasses") => axiom_translation::translate_disjoint_classes_axiom(v,m),
        Some("DisjointUnionOf") => axiom_translation::translate_disjoint_union_of_axiom(v,m),
        Some("EquivalentClasses") => axiom_translation::translate_equivalent_classes_axiom(v,m),
        Some("ClassAssertion") => axiom_translation::translate_class_assertion(v,m),
        Some("DataPropertyDomain") => axiom_translation::translate_data_property_domain(v,m),
        Some("ObjectPropertyDomain") => axiom_translation::translate_object_property_domain(v,m),
        Some("AnnotationPropertyDomain") => axiom_translation::translate_annotation_property_domain(v,m),
        Some("DataPropertyRange") => axiom_translation::translate_data_property_range(v,m),
        Some("ObjectPropertyRange") => axiom_translation::translate_object_property_range(v,m),
        Some("AnnotationPropertyRange") => axiom_translation::translate_annotation_property_range(v,m),
        Some("InverseObjectProperties") => axiom_translation::translate_inverse_object_properties(v,m), 
        Some("SubDataPropertyOf") => axiom_translation::translate_sub_data_property_of(v,m), 

        Some("AnnotationAssertion") => v.clone(), 
        Some("DifferentIndividuals") => v.clone(), 
        Some("ObjectPropertyAssertion") => v.clone(), 
        Some("NegativeObjectPropertyAssertion") => v.clone(),
        Some("SubAnnotationPropertyOf") => v.clone(), 
        Some("DataPropertyAssertion") => v.clone(),

        //TODO: these could take ObjectInverses as arguments
        //but, they should already be typed correctly 
        Some("TransitiveObjectProperty") => v.clone(), 
        Some("SymmetricObjectProperty") => v.clone(), 
        Some("AsymmetricObjectProperty") => v.clone(), 
        Some("IrreflexiveObjectProperty") => v.clone(), 
        Some("ReflexiveObjectProperty") => v.clone(), 
        Some("InverseFunctionalObjectProperty") => v.clone(), 
        Some("FunctionalObjectProperty") => v.clone(), 
        Some("FunctionalDataProperty") => v.clone(), 
        Some("EquivalentObjectProperties") => v.clone(), 
        Some("DisjointObjectProperties") => v.clone(), 
        Some("Import") => v.clone(), 
        Some("OntologyAnnotation") => v.clone(),
        Some("EquivalentDataProperties") => v.clone(),
        Some("SameIndividual") => v.clone(),
        Some("DifferentIndividual") => v.clone(),
        Some("DisjointDataProperties") => v.clone(),
        Some("NegativeDataPropertyAssertion") => v.clone(),

        //TODO: ThinTriples need to be typed as well
        Some("ThinTriple") => axiom_translation::translate_thin_triple(v,m),
        Some("Equivalent") => axiom_translation::translate_equivalent_axiom(v,m),
        Some("EquivalentProperties") => axiom_translation::translate_equivalent_properties(v,m),
        Some("FunctionalProperty") => axiom_translation::translate_functional_property(v,m),
        Some("DisjointProperties") => axiom_translation::translate_disjoint_properties(v,m),
        Some("Declaration") => axiom_translation::translate_declaration(v,m),

        Some("SubPropertyOf") => axiom_translation::translate_sub_property_of(v,m),
        Some("Range") => axiom_translation::translate_range(v,m),
        Some("Domain") => axiom_translation::translate_domain(v,m),

        //no typing necessary, i.e., this axiom cannot contain ambiguous OFN S-expressions 
        //introduce id?
        Some("SubObjectPropertyOf") => axiom_translation::translate_sub_object_property_of(v,m),

        Some("SomeValuesFrom") => class_translation::translate_some_values_from(v,m), 
        Some("AllValuesFrom") =>  class_translation::translate_all_values_from(v,m), 
        Some("HasValue") => class_translation::translate_has_value(v,m), 
        Some("MinCardinality") => class_translation::translate_min_cardinality(v,m), 
        Some("MaxCardinality") => class_translation::translate_max_cardinality(v,m), 
        Some("ExactCardinality") => class_translation::translate_exact_cardinality(v,m), 

        //TODO: axioms 
        //Some("DatatypeDefinition") => translate_datatype_definition(v),
        ////Some("HasKey") => translate_has_key(v), //no property type support in OFN S
        //Some("HasKey") => panic!("HasKey operator currently not supported"), 

        //TODO: deprecated
        //Some("MinQualifiedCardinality") => class_translation::translate_min_cardinality(v,m), 
        //Some("MaxQualifiedCardinality") => class_translation::translate_max_cardinality(v,m), 
        //Some("ExactQualifiedCardinality") => class_translation::translate_exact_cardinality(v,m), 
        //TODO: these should be typed correctly already
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
    };


    //merge logical OFN with annotation OFN
    let rest = &ofn.as_array().unwrap()[1..];

    let mut res = vec![ofn[0].clone()];

    for annotation in annotations {
        res.push(annotation.clone());
    } 

    for r in rest {
        res.push(r.clone());
    }

    Value::Array(res)

} 
