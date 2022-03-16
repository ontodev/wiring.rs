use serde_json::{Value, Result as SResult};
use crate::owl::thick_triple as tt;
use crate::ldtab2ofn::axiom_translation as axiom_translation; 

pub fn parse_thick_triple_object(object : &str) -> tt::OWL {
    let triple_json: SResult<tt::OWL> = serde_json::from_str(object); 

    match triple_json {
        Err(_) => tt::OWL::Named(String::from(object)),
        Ok(x) => x,
    } 
}

pub fn parse_thick_triple(subject: &str, predicate: &str, object: &str) -> Value {

    let subject_json = parse_thick_triple_object(subject); 
    let object_json = parse_thick_triple_object(object); 

    match predicate {
        "rdfs:subClassOf"  => axiom_translation::translate_subclass_of_axiom(&subject_json, &object_json),
        "owl:equivalentClass" => axiom_translation::translate_equivalent_class(&subject_json, &object_json),
        "owl:AllDisjointClasses" => axiom_translation::translate_disjoint_classes(&object_json),
        "owl:disjointUnionOf" => axiom_translation::translate_disjoint_union(&subject_json,&object_json),
        "owl:disjointWith" => axiom_translation::translate_disjoint_with(&subject_json, &object_json), 
        _ => axiom_translation::translate_thin_triple(subject, predicate, object),
    }
}

