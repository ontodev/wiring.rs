use serde_json::{Value, Result as SResult};
use serde_json::json;
use crate::owl::thick_triple as tt;
use crate::ldtab2ofn::axiom_translation as axiom_translation; 

pub fn parse_thick_triple_object(object : &str) -> tt::OWL {
    let triple_json: SResult<tt::OWL> = serde_json::from_str(object); 

    match triple_json {
        Err(_) => tt::OWL::Named(String::from(object)),
        Ok(x) => x,
    } 
}

pub fn parse_string(input : &str) -> String {

    let input: Value = serde_json::from_str(input).unwrap(); 

    match input {
        Value::String(x) => String::from(x),
        _ => panic!("Not a string"),
    }
}

pub fn parse_ldtab(input : &str) -> Value {
    let thick_triple: Value = serde_json::from_str(input).unwrap(); 

    let subject = thick_triple["subject"].to_string();
    let predicate = thick_triple["predicate"].to_string();
    let object = thick_triple["object"].to_string();

    let annotation = thick_triple["annotation"].to_string(); 
    //TODO: translate annotation

    //TODO merge logical OFN with annotation

    parse_thick_triple(&subject, &predicate, &object) 
}

//TODO: rename this to translate
pub fn parse_thick_triple(subject: &str, predicate: &str, object: &str) -> Value {

    let subject_json = parse_thick_triple_object(subject); 
    let predicate_json = parse_string(predicate); //Assumption: this is a string
    let object_json = parse_thick_triple_object(object); 

    //TODO: add support for annotations

    match predicate_json.as_str() {
        "rdfs:subClassOf"  => axiom_translation::translate_subclass_of_axiom(&subject_json, &object_json),
        "owl:equivalentClass" => axiom_translation::translate_equivalent_class(&subject_json, &object_json),
        "owl:AllDisjointClasses" => axiom_translation::translate_disjoint_classes(&object_json),
        "owl:disjointUnionOf" => axiom_translation::translate_disjoint_union(&subject_json,&object_json),
        "owl:disjointWith" => axiom_translation::translate_disjoint_with(&subject_json, &object_json), 
        "rdf:type" => axiom_translation::translate_rdf_type(&subject_json, &object_json),
        "rdfs:domain" => axiom_translation::translate_domain(&subject_json, &object_json),
        "rdfs:range" => axiom_translation::translate_range(&subject_json, &object_json),
        "owl:inverseOf" => axiom_translation:: translate_inverse_object_properties(&subject_json, &object_json),
        "owl:equivalentProperty"  => axiom_translation::translate_equivalent_properties(&subject_json, &object_json),
        "owl:propertyDisjointWith" => axiom_translation::translate_property_disjoint_with(&subject_json, &object_json),
        "owl:AllDisjointProperties" => axiom_translation::translate_all_disjoint_properties(&subject_json, &object_json),
        "rdfs:subPropertyOf" => axiom_translation::translate_sub_property_of(&subject_json, &object_json),
        "owl:AllDifferent" => axiom_translation::translate_all_different(&subject_json, &object_json),
        "owl:differentFrom" => axiom_translation::translate_different_from(&subject_json, &object_json),
        "owl:sameAs" => axiom_translation::translate_same_as(&subject_json, &object_json),
        "owl:AllSameAs" => axiom_translation::translate_all_same_as(&subject_json, &object_json),
        "owl:propertyChainAxiom" => axiom_translation::translate_property_chain(&subject_json, &object_json),
        "owl:NegativePropertyAssertion" => axiom_translation::translate_negative_property_assertion(&subject_json, &object_json),
        "owl:hasKey" => axiom_translation::translate_has_key(&subject_json, &object_json),

        _ => axiom_translation::translate_thin_triple(subject, predicate, object),
    }
}

